#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the NSF.


from bs4 import BeautifulSoup
import os
import sys
import urllib.parse


try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://results.kraft.is/meets",
        "http://results.kraft.is/meets/2018",
        # The URLs below were already fully-added.
        # "http://results.kraft.is/meets/2017",
        # "http://results.kraft.is/meets/2016",
        # "http://results.kraft.is/meets/2015",
        # "http://results.kraft.is/meets/2014",
        # "http://results.kraft.is/meets/2013",
        # "http://results.kraft.is/meets/2012",
        # "http://results.kraft.is/meets/2011",
        # "http://results.kraft.is/meets/2010",
        # "http://results.kraft.is/meets/2009",
        # "http://results.kraft.is/meets/2008",
        # "http://results.kraft.is/meets/2007",
        # "http://results.kraft.is/meets/2006",
        # "http://results.kraft.is/meets/2005",
        # "http://results.kraft.is/meets/2004",
        # "http://results.kraft.is/meets/2003",
        # "http://results.kraft.is/meets/2002",
        # "http://results.kraft.is/meets/2001",
        # "http://results.kraft.is/meets/2000",
        # "http://results.kraft.is/meets/1999",
        # "http://results.kraft.is/meets/1998",
        # "http://results.kraft.is/meets/1997",
        # "http://results.kraft.is/meets/1996",
        # "http://results.kraft.is/meets/1995",
        # "http://results.kraft.is/meets/1994",
        # "http://results.kraft.is/meets/1993",
        # "http://results.kraft.is/meets/1992",
        # "http://results.kraft.is/meets/1991",
        # "http://results.kraft.is/meets/1990",
        # "http://results.kraft.is/meets/1989",
        # "http://results.kraft.is/meets/1988",
        # "http://results.kraft.is/meets/1987",
        # "http://results.kraft.is/meets/1986",
        # "http://results.kraft.is/meets/1985",
        # "http://results.kraft.is/meets/1984",
        # "http://results.kraft.is/meets/1983",
        # "http://results.kraft.is/meets/1982",
        # "http://results.kraft.is/meets/1981",
        # "http://results.kraft.is/meets/1980",
        # "http://results.kraft.is/meets/1979",
        # "http://results.kraft.is/meets/1978",
        # "http://results.kraft.is/meets/1977",
        # "http://results.kraft.is/meets/1976",
        # "http://results.kraft.is/meets/1975",
        # "http://results.kraft.is/meets/1974",
        # "http://results.kraft.is/meets/1973",
        # "http://results.kraft.is/meets/1972",
        # "http://results.kraft.is/meets/1971",
        ]
BASEURL = "http://results.kraft.is"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read().decode('utf-8')


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []

    table = soup.find(
        'div', {'class': 'col-md-8 offset-md-2 col-xs-12 meetpg-block'})

    for a in table.find_all('a'):
        url = a['href']

        if any(int_str in a.text for int_str in ['IPF ', 'NPF ', 'EPF ', 'DM ', 'GBPF ',
                                                 'Nordic', 'International', 'Arnold',
                                                 'World', 'European']):
            continue

        if 'gallery' in url:
            continue

        if 'http' not in url:
            url = BASEURL + url
            url = url.replace('../', '')
        if url not in urls:
            urls.append(url)

    return urls


def getenteredurls():
    urls = []
    for dirname, subdirs, files in os.walk(FEDDIR):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r') as fd:
                for k in fd.readlines():
                    urls.append(k.strip())
    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[Kraft]'), unentered)


if __name__ == '__main__':
    main()
