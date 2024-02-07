#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# SSF meets have the date on the linking page, not the meet page
# Try and find the information for a meet and write it to the meet file
# Call this with the folder number
# Depending on your internet speed you may need to adjust DELAY
# Note that to use this script you will need Selenium installed
# and geckodriver on your path.

from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.firefox.options import Options
import os
import sys
import time
import logging
import io
from selenium.webdriver.remote.remote_connection import LOGGER

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv

PAGES = 4
DELAY = 1
URL = "http://online.styrkelyft.se/web/oldContest.aspx"
BASEURL = "http://online.styrkelyft.se/web/"


def write_csv_with_lf(csv_obj, filename):
    with io.StringIO() as buffer:
        csv_obj.write(buffer)
        buffer.seek(0)  # Reset buffer position to the beginning
        with open(filename, 'wb') as file:
            txt = buffer.read().encode('utf-8')
            file.write(txt)


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
    driver.get(url)

    pages = []

    try:
        for page in pagerange:
            if page != 1:
                driver.execute_script(
                    "javascript:__doPostBack('ctl00$ContentPlaceHolder2$usersGridView',\
                    'Page$%i')" % page)
            time.sleep(DELAY)

            pages.append(driver.page_source)
    finally:
        # Close firefox instance
        driver.quit()
    return pages


def getmeetinfo(html):
    soup = BeautifulSoup(html, 'html.parser')

    info = {}

    table = soup.find('table')

    for tr in table.find_all('tr'):
        tds = tr.find_all('td')
        a = tr.a

        if a:
            url = a['href']

            if 'contestResult' not in url:
                continue

            date = tds[1].text

            if 'http' not in url:
                url = BASEURL + url

            if url not in info:
                info[url] = date

    return info


def get_immediate_subdirectories(a_dir):
    return [name for name in os.listdir(a_dir)
            if os.path.isdir(os.path.join(a_dir, name))]


def main(folders):
    meet_info = {}
    pages = getpages(URL, range(1, PAGES+1))

    for page in pages:
        meet_info.update(getmeetinfo(page))

    for folder in folders:
        folderpath = os.getcwd() + os.sep + folder
        url = ''
        with open(folderpath + os.sep + 'URL', 'r') as fd:
            url = fd.readline().strip()

        if url in meet_info.keys():
            meetcsv = Csv(folderpath + os.sep + 'meet.csv')
            date = meet_info[url]
            meetcsv.rows[0][meetcsv.index('Date')] = date
            write_csv_with_lf(meetcsv, os.path.join(folder, 'meet.csv'))


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: %s meet_folder" % sys.argv[0])
    else:
        main(sys.argv[1:])
