#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new British Powerlifting national meets.

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


RESULTS_URL = "https://www.britishpowerlifting.org/results"
BASE_URL = "https://www.britishpowerlifting.org/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    divs = soup.find_all("div", {"class": "c12"})
    assert divs, "No <div> element with class `c12` found."

    urls = []
    for div in divs:
        links = div.find_all("a")
        for link in links:
            url = link['href'].strip()

            # All results are in a documents/ subdir.
            if 'documents/' not in url:
                continue

            if '://' not in url:
                url = BASE_URL + url

            urls.append(url)

    return urls


def main():
    html_content = oplprobe.gethtml(RESULTS_URL)
    meets_list = getmeetlist(html_content)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meets_list, entered)

    oplprobe.print_meets(color('[BP]'), unentered)


if __name__ == '__main__':
    main()
