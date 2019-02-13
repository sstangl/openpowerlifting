#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes the IrishPF website for any new meets.


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


FEDDIR = os.path.dirname(os.path.realpath(__file__))
FEDURL = "http://www.irishpowerliftingfederation.com/results-2019/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    # The HTML is malformed, for example as follows:
    #   <p><a href="foo"></p><p></a></p>
    # Work around this by deleting all the <p> tags.
    html = str(html, 'utf-8')
    html = html.replace('<p>', '')
    html = html.replace('</p>', '')

    soup = BeautifulSoup(html, 'html.parser')
    content = soup.find("div", {"class": "fl-page-content"})

    urls = []
    for a in content.find_all('a'):
        url = a['href']
        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(FEDURL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[IrishPF]'), unentered)


if __name__ == '__main__':
    main()
