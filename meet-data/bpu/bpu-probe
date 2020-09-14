#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new British Powerlifting Union meets.
# All results are helpfully posted on a single page.

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


RESULTS_URL = "http://britishpowerliftingunion.co.uk/2013/09/05/results/"

BASE_URL = "http://www.britishpowerliftingunion.co.uk/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    divs = soup.find_all("div", {"class": "content"})
    assert divs, "No <div> element with class 'content' found."

    urls = []
    for div in divs:
        links = div.find_all("a")
        for link in links:
            url = link['href'].strip()

            if '://' not in url:
                url = BASE_URL + url

            urls.append(url)

    # The earliest meets are first, so return in reverse order
    # for the benefit of --quick.
    return list(reversed(urls))


def main():
    html_content = oplprobe.gethtml(RESULTS_URL)
    meets_list = getmeetlist(html_content)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meets_list, entered)

    oplprobe.print_meets(color('[BPU]'), unentered)


if __name__ == '__main__':
    main()
