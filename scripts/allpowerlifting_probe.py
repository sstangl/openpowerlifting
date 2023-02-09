#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Library for probing AllPowerlifting
#

from bs4 import BeautifulSoup
import oplprobe


def getmeets(html):
    base_url = 'https://en.allpowerlifting.com'
    soup = BeautifulSoup(html, 'html.parser')
    table = soup.find('table')
    urls = []
    for a in table.find_all('a'):
        url = a['href']
        url = base_url + url
        if 'results' not in url:
            continue
        if url not in urls:
            urls.append(url)

    return urls


def probefederation(fed_url):
    # Get all of the results pages
    result_page = fed_url + 'results/'
    meet_urls = []
    curr_page_meets = []
    prev_page_meets = []

    ii = 1
    while curr_page_meets != prev_page_meets or prev_page_meets == []:
        prev_page_meets = curr_page_meets
        meet_urls += prev_page_meets

        curr_page_url = result_page + '?page='+str(ii)
        curr_page_html = oplprobe.gethtml(curr_page_url)
        curr_page_meets = getmeets(curr_page_html)

        ii += 1
    return meet_urls
