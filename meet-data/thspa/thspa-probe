#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the THSPA.
# Results are in a database, but the front page only shows the current year.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://www.thspa.us/results_all.aspx"]
BASEURL = "http://www.thspa.us"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all("a"):
        # Some links are just JS targets.
        try:
            url = a['href']
        except KeyError:
            continue

        # All the meet URLs have this parameter, luckily.
        if "passedMeetID=" not in url:
            continue

        if 'http' not in url:
            url = BASEURL + url.replace('./', '/')
        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[THSPA]'), unentered)
    print(color('[THSPA] ') + "Continue working through archive.")


if __name__ == '__main__':
    main()
