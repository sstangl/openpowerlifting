#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from JPA.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe

URLS = ["https://www.jpa-powerlifting.or.jp/championships.php"]
BASEURL = "https://www.jpa-powerlifting.or.jp/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for div in soup.find_all('table'):
        for meet in div.find_all('tr'):
            tds = meet.find_all('td')
            if len(tds) < 6:
                continue

            if not tds[5].find('a'):
                continue

            a_list = tds[5].find_all('a')

            for a in a_list:
                url = a['href']

                if 'international' in url:
                    continue

                if 'http' not in url:
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

    oplprobe.print_meets(color('[JPA]'), unentered)


if __name__ == '__main__':
    main()
