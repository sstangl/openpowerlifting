#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from NASA.
# All the results are on one page!


from bs4 import BeautifulSoup
import sys
import os
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


FEDURL = "http://nasa-sports.com/results/"
BASEURL = "http://nasa-sports.com"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    content = soup.find_all("main")
    if len(content) == 0:
        error("Page layout seems to have changed.")
    elif len(content) > 1:
        error("Multiple result areas found.")

    urls = []
    for a in content[0].find_all('a'):
        url = a['href']
        if 'http' not in url:
            url = BASEURL + url
        if url not in urls:
            urls.append(url)

    return urls


def main():
    quick = ('--quick' in sys.argv)

    meetlist = []

    html = oplprobe.gethtml(FEDURL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    if not quick or len(unentered) <= 5:
        oplprobe.print_meets(color('[NASA]'), unentered)
    else:
        print(color('[NASA] ') + '%d meets remaining.' % len(unentered))


if __name__ == '__main__':
    main()
