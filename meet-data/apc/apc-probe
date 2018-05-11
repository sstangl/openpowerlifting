#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the APC federation.
# Most of their meet results are on their Results page, but often
# they just post recent results to their main page.
#

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))

# APC often forgets about their Results page and just drops links
# on their index, so both have to be checked.
URLS = [
    "http://www.americanpowerliftingcommittee-usa.com/",
    "http://www.americanpowerliftingcommittee-usa.com/meet_results%20apc.htm",
]
# Also this page needs checkin'.
IRONDAWGURL = "http://irondawg.com/apc%20results.html"
BASEURL = "http://www.americanpowerliftingcommittee-usa.com/"


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        # All the results have the text "Results" somewhere in the link.
        if 'results' not in a.text.lower():
            continue

        url = a['href']
        if 'http://' not in url:
            url = BASEURL + url

        if url == 'http://' or url == IRONDAWGURL:
            continue

        if url not in urls:
            urls.append(url)

    return urls


def get_irondawg_meetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        url = a['href']
        if 'http://' not in url:
            url = 'http://irondawg.com/' + url

        if url == 'http://irondawg.com/index.htm':
            continue

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = get_irondawg_meetlist(oplprobe.gethtml(IRONDAWGURL))
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[APC]'), unentered)


if __name__ == '__main__':
    main()
