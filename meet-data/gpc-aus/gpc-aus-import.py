#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the new GPC-AUS website
# Note that to use this script you will need Selenium installed
# and geckodriver on your path.

# on Ubuntu:
# sudo apt-get install python3-pip firefox-geckodriver
# sudo pip3 install selenium

from bs4 import BeautifulSoup
from selenium import webdriver
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import Select
from selenium.webdriver.common.by import By
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


def get_pages(url):
    foptions = Options()
    foptions.headless = True
    driver = webdriver.Firefox(options=foptions)
    driver.get(url)

    pages = []

    time.sleep(2)

    dropdowns = driver.find_elements(by=By.TAG_NAME, value='select')
    Select(dropdowns[0]).select_by_visible_text('100')

    source = driver.page_source
    pages += [source]

    buttons = driver.find_element(by=By.CLASS_NAME,
                                  value='wpb_tabs_nav.ui-tabs-nav.clearfix')
    buttons = buttons.find_elements(by=By.TAG_NAME, value='li')

    ii = 1
    for button in buttons[1:]:
        button.click()
        Select(dropdowns[ii]).select_by_visible_text('100')
        source = driver.page_source
        pages += [source]
        ii += 1

    driver.quit()
    return pages


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    name = soup.find('h1', {'class': 'entry-title'}).text
    state = soup.find('span', {'class': 'meta-category'}).a['class'][0].upper()
    date = soup.find('span', {'class': 'meta-date date published'}).text

    if state == 'NATIONAL':
        state = ''

    name = name.replace('GPC ', '')

    date_split = date.split('/')
    date = date_split[2]+'-'+date_split[1]+'-'+date_split[0]

    fed = 'GPC-AUS'
    country = 'Australia'
    town = ''

    row = [fed, date, country, state, town, name]
    csv.rows = [row]
    return csv


def getresults(soup):
    csv = Csv()

    # Get the results table.
    table = None
    table_parents = soup.find_all(
        'div', style=lambda x: x and 'visibility: visible;' in x)
    for div in table_parents:
        if div.find('table'):
            table = div.find('table')

    if table is None:
        error("Couldn't find the results table.")

    data = table.find('tbody')

    if csv.fieldnames == []:
        # Get column information.
        headers = [x.text.strip()
                   for x in table.find('thead').find_all('th')]

        iterable = iter(range(len(headers)))
        for ii in iterable:
            h = headers[ii]
            if h == 'Name':
                csv.fieldnames += ['Name']
            elif h == 'Age':
                csv.fieldnames += ['Age']
            elif h == 'Div':
                csv.fieldnames += ['Division']
            elif h in ['BWt (Kg)', 'BWt']:
                csv.fieldnames += ['BodyweightKg']
            elif h in ['WtCls (Kg)', 'WtCls']:
                csv.fieldnames += ['WeightClassKg']
            elif h == 'Glossbrenner':
                csv.fieldnames += ['IGNORE']
            elif h == 'Lot #':
                csv.fieldnames += ['IGNORE']
            elif h == 'RH Sq':
                csv.fieldnames += ['IGNORE']
            elif h == 'Squat 1':
                csv.fieldnames += ['Squat1Kg']
            elif h == 'Squat 2':
                csv.fieldnames += ['Squat2Kg']
            elif h == 'Squat 3':
                csv.fieldnames += ['Squat3Kg']
            elif h == 'Squat 4':
                csv.fieldnames += ['Squat4Kg']
            elif h == 'Best Squat':
                csv.fieldnames += ['Best3SquatKg']
            elif h == 'RH BP':
                csv.fieldnames += ['IGNORE']
            elif h == 'Bench 1':
                csv.fieldnames += ['Bench1Kg']
            elif h == 'Bench 2':
                csv.fieldnames += ['Bench2Kg']
            elif h == 'Bench 3':
                csv.fieldnames += ['Bench3Kg']
            elif h == 'Bench 4':
                csv.fieldnames += ['Bench4Kg']
            elif h == 'Best Bench':
                csv.fieldnames += ['Best3BenchKg']
            elif h == 'Sub Total':
                csv.fieldnames += ['IGNORE']
            elif h == 'Deadlift 1':
                csv.fieldnames += ['Deadlift1Kg']
            elif h == 'Deadlift 2':
                csv.fieldnames += ['Deadlift2Kg']
            elif h == 'Deadlift 3':
                csv.fieldnames += ['Deadlift3Kg']
            elif h == 'Deadlift 4':
                csv.fieldnames += ['Deadlift4Kg']
            elif h == 'Best Deadlift':
                csv.fieldnames += ['Best3DeadliftKg']
            elif h in ['PL Total', 'TOTAL', 'Pl Total', 'Push Pull Total']:
                csv.fieldnames += ['TotalKg']
            elif h == 'Coeff Score':
                csv.fieldnames += ['IGNORE']
            elif h == 'Age & Coeff':
                csv.fieldnames += ['IGNORE']
            elif h in ['Place code', 'PLACE']:
                csv.fieldnames += ['IGNORE']
            elif h == 'Pl-Div-WtCl':
                csv.fieldnames += ['Place']
            elif h == 'Team Pts':
                csv.fieldnames += ['IGNORE']
            elif h == 'Team':
                csv.fieldnames += ['Team']
            elif h in ['Events', 'Event']:
                csv.fieldnames += ['Event']
            elif h in ['DivSex', 'Div Sex']:
                csv.fieldnames += ['Sex']
            elif h == 'DivCat':
                csv.fieldnames += ['IGNORE']
            elif h == 'DivAge':
                csv.fieldnames += ['IGNORE']
            elif h == 'Date':
                csv.fieldnames += ['IGNORE']
            elif h == 'Meet':
                csv.fieldnames += ['IGNORE']
            elif h == '':
                csv.fieldnames += ['IGNORE']
            elif h == 'Age Div':
                if 'Division' in csv.fieldnames:
                    csv.fieldnames[csv.fieldnames.index('Division')] = 'IGNORE'
                csv.fieldnames += ['Division']
            else:
                error("Unknown column name: \"%s\"" % h)

    for tr in data.find_all('tr'):
        row = [x.text if x.text != 'null' else '' for x in tr.find_all('td')]

        csv.rows += [row]

    return csv


def fix_place(csv):
    if 'Place' not in csv.fieldnames:
        return
    placeidx = csv.index('Place')
    for row in csv.rows:
        row[placeidx] = row[placeidx].split('-')[0]
        if row[placeidx] in ['', '0']:
            row[placeidx] = 'DQ'


def remove_zeros(csv):
    newcsv = Csv()
    newcsv.fieldnames = csv.fieldnames

    for row in csv.rows:
        newcsv.rows += [['' if x in ['0', 'BMB'] else x for x in row]]

    return newcsv


def add_sex(csv):
    if 'Sex' not in csv.fieldnames:
        csv.append_column('Sex')

    dividx = csv.index('Division')
    sexidx = csv.index('Sex')
    for row in csv.rows:
        if 'M-' in row[dividx]:
            row[sexidx] = 'M'
        else:
            row[sexidx] = 'F'


def fix_names(csv):
    nameidx = csv.index('Name')
    for row in csv.rows:
        row[nameidx] = re.sub(r' \(.*?\)', '', row[nameidx])
        row[nameidx] = row[nameidx].replace(' - ', '-')
        row[nameidx] = row[nameidx].title()


def fix_events(csv):
    if 'Event' not in csv.fieldnames:
        return

    eventidx = csv.index('Event')
    for row in csv.rows:
        event = row[eventidx]
        if event == 'PL':
            row[eventidx] = 'SBD'
        elif event == 'PP':
            row[eventidx] = 'BD'
        elif event in ['BP', 'BE']:
            row[eventidx] = 'B'
        elif event == 'DL':
            row[eventidx] = 'D'


def fix_wc(csv):
    wcidx = csv.index('WeightClassKg')
    sexidx = csv.index('Sex')
    for row in csv.rows:
        if row[wcidx] == 'SHW' and row[sexidx] == 'M':
            row[wcidx] = '140+'
        elif row[wcidx] == 'SHW':
            row[wcidx] = '110+'


def add_equipment(csv):

    if 'Equipment' not in csv.fieldnames:
        csv.append_column('Equipment')

    if 'Event' not in csv.fieldnames:
        return

    eqpidx = csv.index('Equipment')
    dividx = csv.index('Division')
    eventidx = csv.index('Event')

    for row in csv.rows:
        if 'E' in row[dividx]:
            row[eqpidx] = 'Multi-ply'
        elif 'S' in row[eventidx]:
            row[eqpidx] = 'Wraps'
        else:
            row[eqpidx] = 'Raw'


def remove_empty_cols_ignore_fieldname(csv):
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

    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)


# Attempts are given, but not the Best3SquatKg columns, etc.
def calc_best_lift(csv, col, attemptlist):
    if col in csv.fieldnames:
        return
    for k in attemptlist:
        assert k in csv.fieldnames

    csv.insert_column(csv.index(attemptlist[-1]) + 1, col)

    for row in csv.rows:
        best = 0
        for k in attemptlist:
            try:
                attempt = float(row[csv.index(k)])
            except ValueError:
                attempt = 0
            if attempt > best:
                best = attempt
        if best > 0:
            row[csv.index(col)] = str(best)


def main(dirname, url):

    pages = get_pages(url)

    soup = BeautifulSoup(pages[0], 'html.parser')
    meetcsv = getmeetinfo(soup)

    entriescsv = Csv()
    for page in pages:
        soup = BeautifulSoup(page, 'html.parser')
        currcsv = getresults(soup)
        entriescsv.cat(currcsv)

    if len(entriescsv.rows) == 0:
        error("No rows found!")

    remove_empty_cols_ignore_fieldname(entriescsv)
    fix_names(entriescsv)
    fix_events(entriescsv)
    fix_place(entriescsv)
    add_equipment(entriescsv)
    add_sex(entriescsv)
    fix_wc(entriescsv)
    entriescsv = remove_zeros(entriescsv)
    entriescsv.append_column('BirthDate')

    if ('Squat1Kg' in entriescsv.fieldnames and
            'Best3SquatKg' not in entriescsv.fieldnames):
        calc_best_lift(entriescsv, 'Best3SquatKg', [
                       'Squat1Kg', 'Squat2Kg', 'Squat3Kg'])
    if ('Bench1Kg' in entriescsv.fieldnames and
            'Best3BenchKg' not in entriescsv.fieldnames):
        calc_best_lift(entriescsv, 'Best3BenchKg', [
                       'Bench1Kg', 'Bench2Kg', 'Bench3Kg'])
    if ('Deadlift1Kg' in entriescsv.fieldnames and
            'Best3DeadliftKg' not in entriescsv.fieldnames):
        calc_best_lift(entriescsv, 'Best3DeadliftKg', [
                       'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg'])

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
