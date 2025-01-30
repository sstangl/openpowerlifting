#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from Strength Sports Australia

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(
        os.path.join(
            os.path.dirname(
                os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
            ),
            "scripts",
        )
    )
    import oplprobe


URLS = ["https://www.strengthsports.org.au/competition-results"]
BASEURL = "https://www.strengthsports.org.au/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, "html.parser")

    # Just look at all links from the entire page.
    # There seems to be a parsing error when looking just at the 'post-content'
    # div, where some <a> tags at the end get overlooked for some unknown
    # reason.

    urls = []
    for a in soup.find_all("a"):
        if (
            a.text.lower().strip().replace("\xa0", " ").replace("  ", " ")
            != "click here"
        ):
            continue

        # Apparently some <a> tags don't have an href.
        try:
            url = a["href"]
        except KeyError:
            continue

        if "http" not in url:
            url = BASEURL + url
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

    oplprobe.print_meets(color("[SSAU]"), unentered)


if __name__ == "__main__":
    main()
