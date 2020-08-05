#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the LPF.


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


URLS = ["https://www.powerliftings.lv/?pg=22"]
BASEURL = "https://www.powerliftings.lv/"
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

    for a in soup.find_all('a'):
        url = a['href']

        if any(int_str in url for int_str in ['world', 'euro']):
            continue

        if not any(x in url for x in ['html', 'xls', 'doc', 'xlsx', 'docx']):
            continue

        if 'http://' not in url and 'https://' not in url:
            url = BASEURL + url
            url = url.replace('../', '')
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

    oplprobe.print_meets(color('[LPF]'), unentered)


if __name__ == '__main__':
    main()
