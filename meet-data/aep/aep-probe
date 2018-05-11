#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new event results from the AEP.

from bs4 import BeautifulSoup
import datetime
import os
import sys
import urllib.request

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe

# URL needs updating every year.
if datetime.datetime.now().strftime("%Y") != "2017":
    print("Warning: AEP fetch URL needs updating for new year.", file=sys.stderr)


# If NUMPAGES is very large, iteration stops at the first 404.
FEDDIR = os.path.dirname(os.path.realpath(__file__))
FEDURLS = [
    "http://www.powerhispania.net/Resultados/2017/resultados.html",
]


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html, url):
    baseurl = url.replace('Resultados.html', '')
    baseurl = baseurl.replace('resultados.html', '')

    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        url = a['href']

        if 'http:' not in url:
            url = baseurl + url

        if url not in urls:  # Sometimes the entries show up twice.
            urls.append(url)

    return urls


def main():
    meetlist = []

    # Iterate through pages until the site returns a 404.
    for url in FEDURLS:
        try:
            html = oplprobe.gethtml(url)
        except urllib.error.HTTPError:
            break
        meets = getmeetlist(html, url)
        meetlist = meetlist + meets

    with open(FEDDIR + '/todo/URLLIST') as fd:
        for line in fd:
            line = line.strip()
            if len(line) == 0 or line.startswith('#'):
                continue
            meetlist.append(line)

    entered = oplprobe.getenteredurls(FEDDIR)
    variants = set()
    for url in entered:
        if '/Resultados/' in url:
            variants.add(url.replace('/Resultados/', '/resultados/'))
        elif '/resultados/' in url:
            variants.add(url.replace('/resultados/', '/Resultados/'))
    entered = entered.union(variants)

    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[AEP]'), unentered)


if __name__ == '__main__':
    main()
