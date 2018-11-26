#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new USPF meets.
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


USPF_RESULTS_URL = "https://uspfthelegend.com/Meet_Results.html"
USPF_ILLINOIS_RESULTS_URL = "http://uspfil.com/results"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;37m" + s + "\033[0;m"


def uspf_getmeetlist(html):
    FED_URL = "https://uspfthelegend.com/"
    soup = BeautifulSoup(html, 'html.parser')

    main = soup.find_all("div", {"class": "LayoutContainer"})
    assert main, "No <div> element with class `LayoutContainer` found."

    links = main[0].find_all("a")
    assert links, "No <a> elements found."

    return list(filter(lambda link: '.pdf' in link,
                       set(map(lambda link: FED_URL + link['href'].strip(), links))))


def uspf_illinois_getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    main = soup.find_all("table", {"class": "table"})
    assert main, "No <table> element with class `table` found."

    links = main[0].find_all('a')
    assert links, 'No <a> elements found.'

    return list(filter(lambda link: 'results/' in link,
                       set(map(lambda link: link['href'].strip(), links))))


def main():
    # Get results from the main USPF site.
    html_content = oplprobe.gethtml(USPF_RESULTS_URL)
    meets_list = uspf_getmeetlist(html_content)

    # Get results from the USPF Illinois affiliate also.
    # The main site seems to omit these meets.
    html_content = oplprobe.gethtml(USPF_ILLINOIS_RESULTS_URL)
    meets_list += uspf_illinois_getmeetlist(html_content)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meets_list, entered)

    oplprobe.print_meets(color('[USPF]'), unentered)


if __name__ == '__main__':
    main()
