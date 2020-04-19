#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new event results from the KNKF-SP.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))
URL = "https://knkf-sectiepowerliften.nl/resultaten/uitslagen/"
BASEURL = "https://knkf-sectiepowerliften.nl/"

# A special URL just for OpenLifter-style results.
OPLURL = "https://knkf-sectiepowerliften.nl/t/opl/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist_uitslagen(html):
    '''Parses the main KNKF-SP results page.'''
    soup = BeautifulSoup(html, 'html.parser')
    results = soup.find_all("div", {"class": "col-md-12"})[1]

    urls = []
    for a in results.find_all('a'):
        url = a['href']

        if 'instagram' in url:
            continue

        if 'http' not in url:
            url = BASEURL + url.replace('../../', '')

        url = url.replace('.nl//downloads', '.nl/downloads')

        if url not in urls:
            urls.append(url)

    return urls


def getmeetlist_oplcsv(html):
    '''Parses the OpenLifter-specific KNKF-SP results page.'''
    soup = BeautifulSoup(html, 'html.parser')
    results = soup.find_all("table")[0]

    urls = []
    for a in results.find_all('a'):
        url = a['href']

        if "csv" not in url.lower():
            continue

        if "http" not in url:
            url = OPLURL + url

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist_uitslagen(html)

    html = oplprobe.gethtml(OPLURL)
    meetlist = meetlist + getmeetlist_oplcsv(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[KNKF-SP]'), unentered)


if __name__ == '__main__':
    main()
