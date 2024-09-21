#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# RPS posts meets to a page separated by year.
# Each meet has a distinct URL, which is saved in the repo.
#


from bs4 import BeautifulSoup
import sys
import os
import datetime
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


# URL needs updating every year.
MEETSURL = "http://meets.revolutionpowerlifting.com/results/2024-meet-results/"
if datetime.datetime.now().strftime("%Y") != "2024":
    print("Warning: RPS fetch URL needs updating for new year.", file=sys.stderr)
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    meetul = soup.find("ul", {"class": "display-pages-listing"})

    urls = []
    for a in meetul.find_all('a'):
        urls.append(a['href'])

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(MEETSURL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[RPS]'), unentered)


if __name__ == '__main__':
    main()
