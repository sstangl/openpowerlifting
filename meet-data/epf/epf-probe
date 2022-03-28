#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the EPF, which uses IPF-style everything.
# Probably run by the same people, I imagine.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["https://www.europowerlifting.org/results.html"]
BASEURL = "https://www.europowerlifting.org/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;35m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    content = soup.find_all("div", {"id": "content"})
    if len(content) == 0:
        error("Page layout seems to have changed.")
    elif len(content) > 1:
        error("Multiple result areas found.")

    urls = []
    for a in content[0].find_all('a'):
        url = a['href']
        if 'http' not in url:
            url = BASEURL + url

        url = url.replace('europowerlifting.org//', 'europowerlifting.org/')

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

    oplprobe.print_meets(color('[EPF]'), unentered)


if __name__ == '__main__':
    main()
