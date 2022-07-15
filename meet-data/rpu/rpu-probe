#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/2015",
        "http://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/2016",
        "http://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/protokoly-2017",
        "http://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/protokoly-2018",
        "http://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/protokoly-2019",
        "http://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/protokoly-2020",
        "https://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/protokoly-2022",
        "https://russia-powerlifting.ru/sorevnovaniya/itogovye-protokoly/"
        "itogovye-protokoly-2021"]
BASEURL = "http://russia-powerlifting.ru"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    content = soup.find("article", {"class": "item-page"})
    alist = content.find_all("a")

    urls = []
    for a in alist:
        link = a['href']
        # A bunch of links are just '/files/foo.xls'
        if link.startswith('/'):
            link = BASEURL + link
        urls.append(link)

    return urls


def main():

    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[RPU]'), unentered)


if __name__ == '__main__':
    main()
