#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the Nordic Powerlifting Federation.


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
URLS = ["https://npfpower.wordpress.com/results/"]
BASEURL = "https://npfpower.wordpress.com/results/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    div = soup.find("div", {"class": "entry"})

    urls = []
    for a in div.find_all('a'):
        if a.has_attr('class'):
            continue
        url = a['href']

        if 'mailto:' in url:
            continue

        if 'https://' not in url and 'http://' not in url:
            url = BASEURL + url

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[NordicPF]'), unentered)


if __name__ == '__main__':
    main()
