#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the UkrainePF
#

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ['http://powerlifting-upf.org.ua/sorevnovaniya.php',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=2',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=3',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=4',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=5',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=6',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=7',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=8',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=9',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=10',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=11',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=12',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=13',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=14',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=15',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=16',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=17',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=18',
        'http://powerlifting-upf.org.ua/sorevnovaniya.php?page=19']

base_url = 'http://powerlifting-upf.org.ua'

FEDDIR = os.path.dirname(os.path.realpath(__file__))


def colour(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        if not a.has_attr('href'):
            continue
        url = a['href']

        if 'sorevnovaniya' not in url or 'sorevnovaniya.php' in url:
            continue
        url = url.replace('../', '')
        url = base_url + "/" + url
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

    oplprobe.print_meets(colour('[UkrainePF]'), unentered)


if __name__ == '__main__':
    main()
