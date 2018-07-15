#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new WABDL meets.

from bs4 import BeautifulSoup
import datetime
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


# URL may need updating every year.
# In particular, to not miss the last few results of the year.
if datetime.datetime.now().strftime("%Y") != "2018":
    print("Warning: WABDL fetch URL may need updating for new year.", file=sys.stderr)


URLS = [
    "http://wabdl.org/results-2/",
    "http://wabdl.org/worlds/"
]
BASE_URL = "http://wabdl.org"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    div = soup.find("div", {"class": "last_column"})
    assert div, "No <div> element with class `last_column` found."

    urls = []
    links = div.find_all("a")
    for link in links:
        url = link['href'].strip()

        if '://' not in url:
            url = BASE_URL + url

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meets_list = []
    for url in URLS:
        html_content = oplprobe.gethtml(url)
        meets_list += getmeetlist(html_content)

    with open(FEDDIR + '/todo/URLLIST') as fd:
        for url in fd:
            url = url.strip()
            if url not in meets_list:
                meets_list.append(url)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meets_list, entered)

    oplprobe.print_meets(color('[WABDL]'), unentered)


if __name__ == '__main__':
    main()
