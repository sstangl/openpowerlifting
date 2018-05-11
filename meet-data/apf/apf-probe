#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new APF/AAPF/WPC meets.
# Also checks the results archive.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://worldpowerliftingcongress.com/wpc-results/",
        "http://worldpowerliftingcongress.com/results-archive/"]

BASEURL = "http://worldpowerliftingcongress.com"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;37m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    main = soup.find("div", {"id": "main-content"})
    divs = main.find_all("div", {"class": "wpb_wrapper"})
    assert divs

    urls = []

    for div in divs:
        for a in div.find_all('a'):
            url = a['href']
            if 'http' not in url:
                url = BASEURL + '/' + url

            # Remove some false positives.
            if '/#' in url:
                continue

            if url not in urls:
                urls.append(url.strip())

    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[APF]'), unentered)


if __name__ == '__main__':
    main()
