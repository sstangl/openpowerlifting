#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the SSF.
# Note that to use this script you will need Selenium installed
# and geckodriver on your path.

from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.firefox.options import Options
import os
import sys
import time
import logging
from selenium.webdriver.remote.remote_connection import LOGGER

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe

PAGES = 4
DELAY = 1
URL = "http://online.styrkelyft.se/web/oldContest.aspx"
BASEURL = "http://online.styrkelyft.se/web/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;34m" + s + "\033[0;m"


def getpages(url, pagerange):
    LOGGER.setLevel(logging.WARNING)
    foptions = Options()
    foptions.headless = True
    driver = webdriver.Firefox(options=foptions)
    try:
        driver.get(url)

        pages = []

        for page in pagerange:
            if page != 1:
                driver.execute_script(
                    "javascript:__doPostBack('ctl00$ContentPlaceHolder2$usersGridView',\
                    'Page$%i')" % page)
                time.sleep(DELAY)

            pages.append(driver.page_source)
    finally:
        driver.quit()

    return pages


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []

    table = soup.find('table')

    for td in table.find_all('td'):
        a = td.a
        if a:
            url = a['href']
            name = a.contents[0]

            if 'contestResult' not in url:
                continue

        # Filter out non Swedish meets
            if any(text in name for text in
                    ['VM', 'EM', 'European', 'NM', 'Barents', 'Arnold']):
                continue

            if 'http' not in url:
                url = BASEURL + url

            if url not in urls:
                urls.append(url)

    return urls


def main():
    meetlist = []
    pages = getpages(URL, range(1, PAGES+1))

    for page in pages:
        meetlist = meetlist + getmeetlist(page)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[SSF]'), unentered)
    return unentered


if __name__ == '__main__':
    main()
