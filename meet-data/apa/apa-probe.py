#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the APA.
# APA posts meets to a page separated by year.


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
URLS = [
    "https://www.apa-wpa.com/APA/us-meet-results/2011-2020/",
    "https://www.apa-wpa.com/APA/us-meet-results/2000-2010/",
    "https://www.apa-wpa.com/APA/us-meet-results/1987-1999/"
]
BASEURL = "http://www.apa-wpa.com/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')
    entry_content = soup.find("div", {"class": "entry-content"})

    urls = []
    for a in entry_content.find_all('a'):
        url = a['href']
        if 'http://' not in url and 'https://www.apa-wpa.com/' not in url:
            if url.startswith('/'):
                url = BASEURL + url[1:]
            else:
                url = BASEURL + url

        if url not in urls and 'wpa-ukraine.com' not in url:
            urls.append(url)

    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[APA]'), unentered)


if __name__ == '__main__':
    main()
