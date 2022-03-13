#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from Powerlifting Australia.
# What a nice federation! Results are already in tables!

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["https://powerliftingaustralia.com/results/"]
BASEURL = "https://powerliftingaustralia.com"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    # Just look at all links from the entire page.
    # There seems to be a parsing error when looking just at the 'post-content'
    # div, where some <a> tags at the end get overlooked for some unknown
    # reason.

    urls = []
    for a in soup.find_all('a'):
        # Apparently some <a> tags don't have an href.
        try:
            url = a['href']
        except KeyError:
            continue

        # All PA results are in /wp-content/uploads/results/.
        if 'wp-content' not in url or 'results' not in url:
            continue

        if 'http' not in url:
            url = BASEURL + url
        if url not in urls:
            urls.append(url)

    return urls


def getenteredurls(feddir):
    """Reimplementation because PA renamed their .htm files to .html"""

    urls = set()
    for dirname, subdirs, files in os.walk(feddir):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r') as fd:
                for k in fd.readlines():
                    url = k.strip()
                    urls.add(url)

                    # Also add a version with ".html"
                    if url.endswith("htm"):
                        urls.add(url + "l")
    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    with open(FEDDIR + os.sep + 'URLLIST') as fd:
        for k in fd.readlines():
            meetlist.append(k.strip())

    entered = getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[PA]'), unentered)


if __name__ == '__main__':
    main()
