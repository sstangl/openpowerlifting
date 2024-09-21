#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the WPPO website
# Note that to use this script you will need Selenium installed
# and geckodriver on your path.

from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.firefox.options import Options
import errno
import os
import sys
import re
import time

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def expand_colspans(tr):

    # Get the original soup object
    enclosing_soup = tr
    while not isinstance(enclosing_soup, BeautifulSoup):
        enclosing_soup = enclosing_soup.parent

    tds = tr.find_all('td')

    # Deal with colspans
    for ii in range(0, len(tds)):
        if tds[ii].has_attr('colspan') and int(tds[ii]['colspan']) > 1:
            for jj in range(int(tds[ii]['colspan'])-1):
                blank_td = enclosing_soup.new_tag('td')
                tds[ii].insert_before(blank_td)
            del tds[ii]['colspan']

    ths = tr.find_all('th')

    # Deal with colspans
    for ii in range(0, len(ths)):
        if ths[ii].has_attr('colspan') and int(ths[ii]['colspan']) > 1:
            for jj in range(int(ths[ii]['colspan'])-1):
                blank_th = enclosing_soup.new_tag('th')
                ths[ii].insert_before(blank_th)
            del ths[ii]['colspan']

    return tr


def gethtml(url):
    foptions = Options()
    foptions.headless = True
    driver = webdriver.Firefox(options=foptions)
    driver.get(url)

    selectAllDays = driver.find_element_by_id('DateSelectAll')

    selectAllDays.click()
    time.sleep(3)
    main_source = driver.page_source

    soup = BeautifulSoup(main_source, 'html.parser')

    tds = soup.find_all('td', {'class': 'ScheduleStatus'})

    result_links = []
    for td in tds:
        link = td.a['href'].split('/')[-1]
        if link not in result_links:
            result_links += [link]

    pages = []
    for link in result_links:
        driver.get('/'.join(url.split('/')[0:-2])+'/po/'+link)
        time.sleep(2)

        pages += [driver.page_source]

    driver.quit()
    return [main_source, pages]


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    month_dict = {'Jan': '01', 'Feb': '02', 'Mar': '03', 'Apr': '04',
                  'May': '05', 'Jun': '06', 'Jul': '07', 'Aug': '08',
                  'Sep': '09', 'Oct': '10', 'Nov': '11',
                  'Dec': '12'}

    date = soup.find('div', {'id': 'TopInfoBoxSport_Stats'})
    date = date.find('div').text.split(':')[1].strip()
    date = date.split('\\n')[0]
    month = month_dict[date.split()[0]]
    day = date.split()[1]
    if len(day) == 1:
        day = '0' + day

    place = soup.find('meta', {'name': 'og:title'})['content']
    place = place.split(' - ')[0]

    town = place[:-5].strip()
    year = place[-5:].strip()

    date = year + '-' + month + '-' + day

    name = ''
    fed = 'WPPO'
    country = ''
    state = ''

    row = [fed, date, country, state, town, name]
    for i, r in enumerate(row):
        row[i] = r.replace(',', ' ').replace('  ', ' ').strip()
    csv.rows = [row]
    return csv


def getresults(soup):
    csv = Csv()

    # Get the results table.
    tables = soup.find_all('table', {'class': 'ResTableFull'})
    if len(tables) == 4:
        restable = tables[3]
    elif len(tables) == 3:
        restable = tables[2]
    else:
        error("Couldn't find the results table.")

    descriptor = soup.find('div', {'id': 'TopInfoBoxSport_EventName'})

    header_trs = expand_colspans(restable.find('tr', {'class': 'ResHead'}))

    # Get column information.
    headers = [x.text for x in header_trs.find_all('th')]

    iterable = iter(range(len(headers)))
    for ii in iterable:
        h = headers[ii]
        if h == 'Rank':
            csv.fieldnames += ['Place']
        elif h == 'Lifter':
            csv.fieldnames += ['Country', 'Name']
        elif h == 'Body Weight':
            csv.fieldnames += ['BodyweightKg']
        elif h == 'Attempts':
            csv.fieldnames += ['Bench4Kg']
            csv.fieldnames[-4:-1] = ['Bench1Kg', 'Bench2Kg', 'Bench3Kg']
        elif h == 'Result':
            csv.fieldnames += ['Best3BenchKg']
        elif h in ['Rack Height', '']:
            csv.fieldnames += ['IGNORE']
        else:
            error("Unknown column name: \"%s\"" % h)

    # These columns are added from the description.
    csv.fieldnames += ['Sex', 'WeightClassKg', 'Division']

    # Always B,Raw
    csv.fieldnames += ['Event', 'Equipment']

    wcstate = None
    divstate = None
    sexstate = None

    text = descriptor.text
    # Parse the division

    if "Women's" in text:
        sexstate = 'F'
        text = text.replace("Women's", '').strip()
    elif "Men's" in text:
        sexstate = 'M'
        text = text.replace("Men's", '').strip()

    # Extract division information.
    if 'Group A' in text:
        divstate = 'Group A'
    elif 'Group B' in text:
        divstate = 'Group B'
    elif 'Group C' in text:
        divstate = 'Group C'
    elif 'Group D' in text:
        divstate = 'Group D'
    else:
        divstate = 'Open'

    # Extract weight class information.
    if 'Up to' in text:
        wcstate = re.search('(?<=Up to ).*?(?=kg)', text).group(0)
    elif 'Over' in text:
        wcstate = re.search('(?<=Over ).*?(?=kg)', text).group(0)
        wcstate = wcstate + '+'
    else:
        error("No weightclass in descriptor: \"%s\"" % text)

    for tr in restable.find_all('tr'):
        tr = expand_colspans(tr)

        tds = tr.find_all('td')
        if len(tds) == 0:
            continue

        assert divstate
        assert sexstate is not None

        row = []
        ii = 0
        nameidx = csv.index('Name')
        for td in tds:
            text = td.text.strip()

            if td.find('a', {'class': 'playerTagCountryLink'}):
                if td.find('div', {'class': 'playerTagContainer'}):
                    text = td.find('img')['title']
                else:
                    continue
            if td.find('span') and td.find('span').has_attr('title') and ii == nameidx:
                text = td.find('span')['title']

            if text in ['-', 'RET']:
                text = ''

            if td.find('strike'):
                text = '-' + text
            row.append(text.strip())
            ii += 1

        if len(row) < 3 or [x for x in row if x != ''] == []:
            continue

        row = row + [sexstate, wcstate, divstate] + ['B', 'Raw']

        csv.rows += [row]

    return csv


def addtotals(csv):

    if 'TotalKg' not in csv.fieldnames:
        csv.append_column('TotalKg')
        bestbenchidx = csv.index('Best3BenchKg')
        totalidx = csv.index('TotalKg')
        for row in csv.rows:
            row[totalidx] = row[bestbenchidx]


def reverse_names(csv):
    nameidx = csv.index('Name')

    for row in csv.rows:
        split_names = row[nameidx].title().split()
        lastname_idx = 0
        for ii in range(len(split_names)):
            if split_names[ii].isupper() and len(split_names[ii].replace('.', '')) > 1:
                lastname_idx = ii

        firstname = ' '.join(split_names[lastname_idx+1:]).title().strip()
        lastname = ' '.join(split_names[0:lastname_idx+1]).strip()
        name = firstname + ' ' + lastname

        row[nameidx] = name.strip()


def remove_empty_and_ignore_cols(csv):
    def iscolempty(csv, i):
        for row in csv.rows:
            if row[i]:
                return False
        return True

    def getemptyidx(csv):
        for i, col in enumerate(csv.fieldnames):
            if iscolempty(csv, i):
                return i
        return -1

    # Remove all the columns named 'IGNORE'
    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)


def standardize_countries(csv):
    if 'Country' not in csv.fieldnames:
        return

    countryidx = csv.index('Country')
    for row in csv.rows:
        if row[countryidx] == 'Great Britain':
            row[countryidx] = 'UK'
        elif row[countryidx] == "People's Republic of China":
            row[countryidx] = 'China'
        elif row[countryidx] == "United States of America":
            row[countryidx] = 'USA'
        elif row[countryidx] == "Republic of Korea":
            row[countryidx] = 'South Korea'
        elif row[countryidx] == "Islamic Republic of Iran":
            row[countryidx] = 'Iran'
        elif row[countryidx] == "Lao People's Democratic Republic":
            row[countryidx] = 'Laos'
        elif row[countryidx] == 'United Arab Emirates':
            row[countryidx] = 'UAE'
        elif row[countryidx] == 'Republic of Moldova':
            row[countryidx] = 'Moldova'
        elif row[countryidx] == 'Chinese Taipei':
            row[countryidx] = 'Taiwan'
        elif row[countryidx] == 'Czech Republic':
            row[countryidx] = 'Czechia'
        elif row[countryidx] == "Côte d'Ivoire":
            row[countryidx] = 'Ivory Coast'


def fixplace(csv):
    if 'Country' not in csv.fieldnames:
        return

    placeidx = csv.index('Place')
    bestbenchidx = csv.index('Best3BenchKg')
    for row in csv.rows:
        if row[placeidx] == '':
            row[placeidx] = 'DQ'
        if row[bestbenchidx] == 'DNS':
            row[placeidx] = 'NS'
            row[bestbenchidx] = ''


def main(dirname, url):
    [main_html, pages_html] = gethtml(url)

    entriescsv = Csv()
    for html in pages_html:
        soup = BeautifulSoup(html, 'html.parser')

        currcsv = getresults(soup)
        entriescsv.cat(currcsv)

    main_soup = BeautifulSoup(main_html, 'html.parser')
    meetcsv = getmeetinfo(main_soup)

    if len(entriescsv.rows) == 0:
        error("No rows found!")

    addtotals(entriescsv)

    reverse_names(entriescsv)

    remove_empty_and_ignore_cols(entriescsv)

    standardize_countries(entriescsv)

    fixplace(entriescsv)

    try:
        os.makedirs(dirname)
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise
        else:
            error("Directory '%s' already exists." % dirname)

    with open(dirname + os.sep + 'entries.csv', 'w') as fd:
        entriescsv.write(fd)
    with open(dirname + os.sep + 'meet.csv', 'w') as fd:
        meetcsv.write(fd)
    with open(dirname + os.sep + 'URL', 'w') as fd:
        fd.write(url + "\n")

    print("Imported into %s." % dirname)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: %s dirname url" % sys.argv[0])
    main(sys.argv[1], sys.argv[2])
