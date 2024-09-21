#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from GPC-AUS.
# Note that to use this script you will need Selenium installed
# and geckodriver on your path.

from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.firefox.options import Options as FirefoxOptions
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

PAGES = 14
URL = "https://gpcaustralia.com/competition-results/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getpages(url, pagerange):
    LOGGER.setLevel(logging.ERROR)
    options = FirefoxOptions()
    options.add_argument("--headless")

    driver = webdriver.Firefox(options=options)
    driver.get(url)

    pages = []
    time.sleep(2)
    for page in pagerange:
        if page != 1:

            button = driver.find_element_by_link_text(str(page))

            button.click()
            time.sleep(2)

        pages.append(driver.page_source)
    driver.quit()

    return pages


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []

    table = soup.find('div', id='newsPanelResults')
    for div in table.find_all('div', {'class': 'col-sm-10'}):

        url = div.find('a')['href']

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

    oplprobe.print_meets(color('[GPC-AUS]'), unentered)


if __name__ == '__main__':
    main()
