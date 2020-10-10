#!/usr/bin/env python3

# Import data from the AllPowerlifting website. This script should only be used
# for meets where AllPowerlifting is the primary source.

from bs4 import BeautifulSoup
import errno
from oplcsv import Csv
import os
import sys
import urllib.request
import re
import subprocess

scriptsdir = os.path.dirname(os.path.realpath(__file__))
importdir = os.getcwd()


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read().decode('utf-8')


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def getdirname(url):
    dirname = url.split('results/')[1][:-1]
    dirname = dirname.replace('/', '-')
    return dirname


def getmeetinfo(soup):
    month_dict = {'january': '01', 'febuary': '02', 'march': '03', 'april': '04',
                  'may': '05', 'june': '06', 'july': '07', 'august': '08',
                  'september': '09', 'october': '10', 'november': '11',
                  'december': '12'}

    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    # Get the info table.
    infotable = soup.find('div', {'class': 'col-lg-6'})

    if infotable == []:
        error("Couldn't find the info table.")

    # Get the fed.
    fedrow = infotable.find_all('dd')[0]

    fed = fedrow.text
    fed = fed[fed.find("(")+1:fed.find(")")]

    # Correct fed names to the form that we use
    if fed in ['RPF', 'SPPF', 'MPF', 'FPVO', 'FPIO', 'FPYNAO']:
        fed = 'FPR'
    elif fed in ['UPF']:
        fed = 'UkrainePF'
    elif fed == 'ROP':
        fed = 'RPU'

    # A list of Russian cities and their states as a timesaver

    state_dict = {'Izhevsk': 'UD', 'St. Petersburg': 'LEN', 'Khanty-Mansiysk': 'KHM',
                  'Sochi': 'KDA', 'Blagoveshchensk': 'AMU', 'Moscow': 'MOW',
                  'Belogorsk': 'AMU', 'Cherepovets ': 'VLG', 'Perm': 'PER',
                  'Kostroma': 'KOS', 'Samara': 'SAM', 'Zelenogorsk': 'LEN',
                  'Berdsk': 'NVS', 'Vologda': 'VLG', 'Irkutsk': 'IRK',
                  'Noyabrsk': 'YAN', 'Krasnoyarsk': 'KYA', 'Kaluga': 'KLU',
                  'Sayansk': 'IRK', 'Leninsk-Kuznetsky': 'KEM', 'Gai': 'ORE',
                  'Chelyabinsk': 'CHE', 'Bogotol': 'KYA', 'Ruzayevka': 'MO',
                  'Oryol': 'ORL', 'Yekaterinburg': 'SVE', 'Ukhta': 'KO',
                  'Yoshkar-Ola': 'ME', 'Yaroslavl': 'YAR', 'Voskresensk': 'MOW',
                  'Novgorod': 'NOV', 'Surgut': 'KHM', 'Myski': 'KEM',
                  'Volgograd': 'VGG', 'Vladimir': 'VLA', 'Stavropol': 'STA',
                  'Kungur': 'PER', 'Kursk': 'KRS', 'Voronezh': 'VOR',
                  'Arsenyev': 'PRI', 'Vladikavkaz': 'SE', 'Saratov': 'SAR',
                  'Belgorod': 'BEL', 'Nakhodka': 'PRI', 'Krasnodar': 'KDA',
                  'Krymsk': 'KDA', 'Vladivostok': 'PRI', 'Novorossiysk': 'KDA',
                  'Novosibirsk': 'NVS', 'Barnaul': 'ALT', 'Rostov-on-Don': 'ROS',
                  'Kazan': 'TA', 'Nizhny Novgorod': 'NIZ', 'Kavalerovo': 'PRI',
                  'Suzdal': 'VLA', 'Taganrog': 'ROS', 'Khabarovsk': 'KHA',
                  'Beryozovsky': 'SVE', 'Togliatti': 'SAM', 'Engels': 'SAR',
                  'Rzhev': 'TVE', 'Omsk': 'OMS', 'Anapa': 'KDA', 'Gurievsk': 'KGD',
                  'Tomsk': 'TOM', 'Lipetsk': 'LIP', 'Tyumen': 'TYU', 'Gulkevichi': 'KDA',
                  'Penza': 'PNZ', 'Zabaykalsk': 'ZAB', 'Tambov': 'TAM'}

    town_aliases = {'Hanti-Mansiysk': 'Khanty-Mansiysk',
                    'Saint Petersburg': 'St. Petersburg',
                    'Leninsk-Kuznetskiy': 'Leninsk-Kuznetsky',
                    'Ruzaevka': 'Ruzayevka',
                    'Yekaterinbourg': 'Yekaterinburg', 'Uhkta': 'Ukhta',
                    'Челябинск': 'Chelyabinsk', 'Пермь': 'Perm', 'Курск': 'Kursk',
                    'Екатеринбург': 'Yekaterinburg', 'Вологда': 'Vologda',
                    'Краснодар': 'Krasnodar', 'Crymsk': 'Krymsk',
                    'Санкт-Петербург': ' St. Petersburg', 'Саратов': 'Saratov',
                    'Владивосток': 'Vladivostok', 'Крымск': 'Krymsk',
                    'Ставрополь': 'Stavropol', 'Новороссийск': 'Novorossiysk',
                    'Rostov-na-Donu': 'Rostov-on-Don',
                    'Nizhniy Novgorod': 'Nizhny Novgorod', 'Калуга': 'Kaluga',
                    'пгт Кавалерово': 'Kavalerovo', 'Суздаль': 'Suzdal',
                    'Благовещенск': 'Blagoveshchensk', 'Москва': 'Moscow',
                    'Воронеж': 'Voronezh', 'Казань': 'Kazan',
                    'Ростов-на-Дону': 'Rostov-on-Don', 'Тольятти': 'Togliatti',
                    'Череповец': 'Cherepovets', 'Белгород': 'Belgorod',
                    'Нижний Новгород': 'Nizhny Novgorod', 'Волгоград': 'Volgograd',
                    'Новосибирск': 'Novosibirsk', 'Togliatty': 'Togliatti',
                    'Blagoveschensk': 'Blagoveshchensk'}

    country_aliases = {'Россия': 'Russia', 'Slovak Republic': 'Slovakia',
                       'Czech Republic': 'Czechia', 'Tadjikistan': 'Tajikistan',
                       'Австрия': 'Austria'}

    # Get the location.
    venuerow = fedrow.find_next_sibling().find_next_sibling()
    venue = venuerow.a['title']
    if venue[-2:] == ', ':
        venue = venue[:-2]

    split = venue.split(', ')
    if len(split) > 1:
        country = split[0]
        town = split[-1]
    else:
        country = venue
        town = ''

    if country in country_aliases:
        country = country_aliases[country]

    if town in town_aliases:
        town = town_aliases[town]

    state = ''
    if town in state_dict:
        state = state_dict[town]

    # Get the date.
    daterow = venuerow.find_next_sibling().find_next_sibling()

    longdate = daterow.text
    longdate = longdate.replace(" - ", "-")

    if ' ' in longdate and longdate.count(' ') == 2:
        [day, month, year] = longdate.strip().split(' ')
        year = year.strip()

        if "-" in day:
            day = day.split("-")[0]

        for month_key in month_dict.keys():
            if month.lower() in month_key:
                month = month_dict[month_key]
                break
        if len(day) == 1:
            day = '0' + day

        date = year + '-' + month + '-' + day
    else:
        date = ''

    # Get the competition name.
    h2 = soup.find('h2')
    if len(h2) != 1:
        error("Couldn't find the competition name.")
    name = h2.text.replace(';', '').replace(',', '')

    row = [fed, date, country, state, town, name]
    for i, r in enumerate(row):
        row[i] = r.replace(',', ' ').replace('  ', ' ').strip()
    csv.rows = [row]
    return csv


def getresults(soup, url):
    csv = Csv()

    subpages = soup.find('ul', {'class': 'nav-tabs'})
    csv.append_column('Tested')
    csv.append_column('Equipment')

    if subpages:
        csv.append_column('SheetName')

        links = subpages.find_all('a')

        div_names = subpages.find_all('li')
        ii = 0

        for ii in range(len(links)):

            link = links[ii]
            subpage_ext = link['href'].split('/')[:-1][-1]

            subpage_url = url + subpage_ext
            subpage_html = gethtml(subpage_url)
            subpage_html = subpage_html.replace('<del>', '<del>-')

            div_name = re.sub(r'\d', '',
                              div_names[ii].a.find(text=True, recursive=False)).strip()

            subpage_soup = BeautifulSoup(subpage_html, 'html.parser')
            subpage_csv = getpagedata(subpage_soup)

            if 'Name' in subpage_csv.fieldnames:
                [russian_names, birthyears] = get_cyrillic_names_and_birthyears(
                    subpage_url)

                subpage_csv.append_column('CyrillicName')
                cyrnameidx = subpage_csv.index('CyrillicName')
                nameidx = subpage_csv.index('Name')

                if 'BirthYear' not in subpage_csv.fieldnames:
                    subpage_csv.append_column('BirthYear')
                byidx = subpage_csv.index('BirthYear')

                for ii in range(len(subpage_csv.rows)):
                    if subpage_csv.rows[ii][nameidx] != russian_names[ii]:
                        subpage_csv.rows[ii][cyrnameidx] = russian_names[ii]

                    subpage_csv.rows[ii][byidx] = birthyears[ii]

            subpage_csv.append_column('SheetName')
            for row in subpage_csv.rows:
                row[-1] = div_name.lower()

            csv.cat(subpage_csv)

    else:
        # The english site is currently broken
        if 'en.' in url:
            [russian_names, birthyears] = get_cyrillic_names_and_birthyears(
                url)

        csv.append_column('CyrillicName')
        cyrnameidx = csv.index('CyrillicName')
        nameidx = csv.index('Name')

        if 'BirthYear' not in csv.fieldnames:
            csv.append_column('BirthYear')
        byidx = csv.index('BirthYear')

        for ii in range(len(csv.rows)):
            if csv.rows[ii][nameidx] != russian_names[ii]:
                csv.rows[ii][cyrnameidx] = russian_names[ii]

            csv.rows[ii][byidx] = birthyears[ii]

    return csv


# Returns just the Russian names and BirthYears for lifters
def get_cyrillic_names_and_birthyears(url):
    # Get the html for the Russian site
    html = gethtml(url.replace('http://en.', 'http://'))

    soup = BeautifulSoup(html, 'html.parser')

    names = []
    birthyears = []


# Get the results table.
    table = soup.find('table', {'class': 'table-xs'})
    if table == []:
        error("Couldn't find the results table.")
    trs = table.find_all('tr')

    # Get column information.
    headers = [x.text for x in trs[1].find_all('th')]
    nameidx = None
    byidx = None
    for ii in range(len(headers)):
        if headers[ii] in ['Спортсмен', 'Имя']:
            nameidx = ii
        elif headers[ii] == 'ГР':
            byidx = ii

        if nameidx and byidx:
            break

    assert nameidx is not None

    for tr in trs[1:]:
        row = [x.text for x in tr.find_all('td')]

        if len(row) > 1:
            tds = tr.find_all('td')
            name = ' '.join(tds[nameidx].text.split())

            names.append(name)

            if byidx:
                by = ' '.join(tds[byidx].text.replace(
                    '—', '').replace('-', '').split())
            else:
                by = ''

            birthyears.append(by)

    return [names, birthyears]


def getpagedata(soup):
    csv = Csv()

    # Get the results table.
    table = soup.find('table', {'class': 'table-xs'})
    if table == []:
        error("Couldn't find the results table.")

    trs = table.find_all('tr')

    # Get column information.
    headers = [x.text for x in trs[1].find_all('th')]
    csv.fieldnames = []
    for h in headers:
        # Allpowerlifting is annoying and have removed the place header
        # on their new site, it's the second column (always?)
        if len(csv.fieldnames) == 1:
            csv.fieldnames += ['Place']
        elif h in [' ', '\xa0', '\n']:
            csv.fieldnames += ['IGNORE']
        elif h in ['Pl', 'М']:
            csv.fieldnames += ['Place']
        elif h in ['Lifter', 'Name']:
            csv.fieldnames += ['Name']
        elif h in ['Спортсмен', 'Имя']:
            csv.fieldnames += ['CyrillicName']
        elif h in ['Age cls', 'Возр', 'Age']:
            csv.fieldnames += ['Division']
        elif h in ['BY', 'ГР', 'YOB']:
            csv.fieldnames += ['BirthYear']
        elif h == 'From':
            csv.fieldnames += ['IGNORE']
        elif h in ['Body wt', 'Вес', 'Weight']:
            csv.fieldnames += ['BodyweightKg']
        elif h in ['SQ1', 'П1', 'S1']:
            csv.fieldnames += ['Squat1Kg']
        elif h in ['SQ2', 'П2', 'S2']:
            csv.fieldnames += ['Squat2Kg']
        elif h in ['SQ3', 'П3', 'S3']:
            csv.fieldnames += ['Squat3Kg']
        elif h in ['SQ', 'П', 'S']:
            csv.fieldnames += ['Best3SquatKg']
        elif h in ['BP1', 'Ж1', 'B1']:
            csv.fieldnames += ['Bench1Kg']
        elif h in ['BP2', 'Ж2', 'B2']:
            csv.fieldnames += ['Bench2Kg']
        elif h in ['BP3', 'Ж3', 'B3']:
            csv.fieldnames += ['Bench3Kg']
        elif h in ['BP', 'Ж', 'B']:
            csv.fieldnames += ['Best3BenchKg']
        elif h in ['DL1', 'Т1', 'D1']:
            csv.fieldnames += ['Deadlift1Kg']
        elif h in ['DL2', 'Т2', 'D2']:
            csv.fieldnames += ['Deadlift2Kg']
        elif h in ['DL3', 'Т3', 'D3']:
            csv.fieldnames += ['Deadlift3Kg']
        elif h in ['DL', 'Т', 'D']:
            csv.fieldnames += ['Best3DeadliftKg']
        elif h in ['Total', 'Сумма']:
            csv.fieldnames += ['TotalKg']
        elif h in ['Nat', 'Откуда', 'Стр', 'Location']:
            csv.fieldnames += ['Country']
        elif h == 'G':
            csv.fieldnames += ['IGNORE']
        elif h in ['R', 'W', 'S/M', 'S', 'G', 'Points']:  # Resh, Wilks, Schwartz/Malone
            csv.fieldnames += ['IGNORE']
        elif h == 'Team':
            csv.fieldnames += ['IGNORE']
        elif h in ['Coach', 'Тренер', 'Trainer', 'Location']:
            csv.fieldnames += ['IGNORE']
        else:
            error("Unknown column name: \"%s\"" % h)

    # These columns are added from the category rows.
    csv.fieldnames += ['WeightClassKg', 'Sex']

    wcstate = ''

    # If unspecified assume male and fix later
    sexstate = 'M'

    for tr in trs[0:]:
        row = [x.text for x in tr.find_all(['td', 'th'])]

        # Rows of length >1 are actual results, of length 1 are categories.
        if len(row) == 1:
            # Extract sex information.
            text = row[0].lower()
            if 'women' in text or 'женщины' in text:
                sexstate = 'F'
            elif 'men' in text or 'мужчины' in text:
                sexstate = 'M'
            elif 'category' in text:
                wcstate = ' '.join(text.replace(
                    'category ', '').replace(' kg', '').split())
                if '+' in wcstate:
                    wcstate = wcstate.replace('+', '')+'+'

        elif tr.find_all('th') == []:  # Otherwise it's a header
            row = []

            for td in tr.find_all('td'):

                if td.has_attr('class') and td.attrs['class'] == ['font-weight-bold',
                                                                  'general',
                                                                  'border-right',
                                                                  'text-right']:
                    continue

                text = td.text

                # Get the country data
                if td.find('i'):
                    text = td.i['class'][0]
                    text = text.replace('flag-icon-', '')
                    country_codes = {'ru': 'Russia', 'kz': 'Kazakhstan',
                                     'az': 'Azerbaijan', 'ua': 'Ukraine',
                                     'by': 'Belarus', 'tj': 'Tajikistan',
                                     'ie': 'Ireland', 'lv': 'Latvia',
                                     'kg': 'Kyrgyzstan', 'md': 'Moldova',
                                     'mn': 'Mongolia', 'us': 'USA',
                                     'at': 'Austria', 'hu': 'Hungary',
                                     'cz': 'Czechia', 'de': 'Germany',
                                     'pl': 'Poland', 'sk': 'Slovakia',
                                     'it': 'Italy', 'se': 'Sweden',
                                     'fr': 'France', 'ch': 'Switzerland',
                                     'sr': 'Serbia', 'am': 'Armenia',
                                     'ir': 'Iran', 'ee': 'Estonia',
                                     'gb': 'UK', 'mt': 'Malta',
                                     'za': 'South Africa', 'au': 'Australia',
                                     'ro': 'Romania', 'be': 'Belgium',
                                     'il': 'Israel'}
                    if text in country_codes:
                        text = country_codes[text]

                if text is None:
                    text = ''

                text = ' '.join(text.split())
                text = text.replace('- ', '-')
                row.append(text.strip().replace(',', ' ').replace('—', ''))

            row = row + [wcstate, sexstate]
            csv.rows += [row]

    return csv


def expand_fourths(csv, bestlift, lift4):
    bestidx = csv.index(bestlift)

    for row in csv.rows:
        row[bestidx] = row[bestidx].lstrip().strip()
        if ' ' in row[bestidx].strip():
            if lift4 not in csv.fieldnames:
                csv.insert_column(bestidx+1, lift4)
            lift4idx = csv.index(lift4)

            [row[bestidx], row[lift4idx]] = row[bestidx].split(' ')
            row[bestidx] = row[bestidx].strip()
            row[lift4idx] = row[lift4idx].strip()


def fix_place(csv):
    placeidx = csv.index('Place')
    totalidx = csv.index('TotalKg')
    for row in csv.rows:
        row[placeidx] = row[placeidx].replace('.', '')
        if row[placeidx] in ['', '-', 'нк', 'н']:
            row[placeidx] = 'DQ'
            row[totalidx] = ''  # Instead of a zero.
        elif row[placeidx] == 'DQ':  # Allpowerlifting marks doping disquals as DQ
            row[placeidx] = 'DD'
            row[totalidx] = ''  # Instead of a zero.
        elif row[totalidx] in ['0.0', '0']:
            row[totalidx] = ''
            row[placeidx] = 'DQ'


def fix_totals(csv):
    if 'TotalKg' not in csv.fieldnames:
        csv.insert_column(len(csv.fieldnames), 'TotalKg')

    totalidx = csv.index('TotalKg')
    eventidx = csv.index('Event')

    for row in csv.rows:
        total = 0
        if ('S' in row[eventidx] and 'Best3SquatKg' in csv.fieldnames and
                row[csv.index('Best3SquatKg')] != ''):
            total += float(row[csv.index('Best3SquatKg')])
        if ('B' in row[eventidx] and 'Best3BenchKg' in csv.fieldnames and
                row[csv.index('Best3BenchKg')] != ''):
            total += float(row[csv.index('Best3BenchKg')])
        if ('D' in row[eventidx] and 'Best3DeadliftKg' in csv.fieldnames and
                row[csv.index('Best3DeadliftKg')] != ''):
            total += float(row[csv.index('Best3DeadliftKg')])

        row[totalidx] = str(total)


def remove_records(csv):
    def getevtindices(csv, fieldl):
        indexlist = []
        for f in fieldl:
            try:
                indexlist.append(csv.index(f))
            except ValueError:
                pass
        return indexlist

    squatidxl = getevtindices(
        csv, ['Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Squat4Kg', 'Best3SquatKg'])
    benchidxl = getevtindices(
        csv, ['Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Bench4Kg', 'Best3BenchKg'])
    deadliftidxl = getevtindices(
        csv, ['Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Deadlift4Kg',
              'Best3DeadliftKg'])

    attempt_idcs = squatidxl + benchidxl + deadliftidxl
    if 'TotalKg' in csv.fieldnames:
        attempt_idcs = attempt_idcs + [csv.index('TotalKg')]

    for row in csv.rows:
        for col in attempt_idcs:
            row[col] = row[col].replace('  ', ' ').lstrip().strip()
            splitentry = row[col].split(' ')
            row[col] = ' '.join([s for s in splitentry if s.replace(
                '.', '', 1).replace('-', '', 1).isdigit()])


def unreverse_names(csv):
    nameidx = None
    if 'Name' in csv.fieldnames:
        nameidx = csv.index('Name')
    cyrillicnameidx = csv.index('CyrillicName')

    for row in csv.rows:

        # Maiden names are given in brackets, remove these
        row[cyrillicnameidx] = re.sub(
            r'\(.*?\)', '', row[cyrillicnameidx]).strip()

        if nameidx:
            # Maiden names are given in brackets, remove these
            row[nameidx] = re.sub(r'\(.*?\)', '', row[nameidx]).strip()
            # Remove nicknames
            row[nameidx] = re.sub('".*?"', '', row[nameidx])
            row[nameidx] = row[nameidx].replace('  ', ' ').strip()

            parts = row[nameidx].split()
            if len(parts) > 1:
                fixed = [parts[-1]] + parts[:-1]
                row[nameidx] = ' '.join(fixed)

        parts = row[cyrillicnameidx].split()
        if len(parts) > 1:
            fixed = [parts[-1]] + parts[:-1]
            row[cyrillicnameidx] = ' '.join(fixed)


def markevent(csv):
    assert 'Event' not in csv.fieldnames
    csv.append_column('Event')

    evtidx = csv.index('Event')

    def getevtindices(csv, fieldl):
        indexlist = []
        for f in fieldl:
            try:
                indexlist.append(csv.index(f))
            except ValueError:
                pass
        return indexlist

    squatidxl = getevtindices(
        csv, ['Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Best3SquatKg'])
    benchidxl = getevtindices(
        csv, ['Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Best3BenchKg'])
    deadliftidxl = getevtindices(
        csv, ['Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Best3DeadliftKg'])

    for row in csv.rows:
        evt = ''
        for i in squatidxl:
            if row[i] != '':
                evt = evt + 'S'
                break
        for i in benchidxl:
            if row[i] != '':
                evt = evt + 'B'
                break
        for i in deadliftidxl:
            if row[i] != '':
                evt = evt + 'D'
                break
        row[evtidx] = evt

# Replace random english characters that AllPowerlifting uses with unicode alternatives


def fix_cyrillic_names(csv):
    if 'CyrillicName' in csv.fieldnames:
        cyridx = csv.index('CyrillicName')
        for row in csv.rows:
            row[cyridx] = row[cyridx].replace('e', 'е').replace(
                'x', 'х').replace('o', 'о').replace('T', 'Т').replace('i', 'і').replace(
                    'O', 'О').replace('E', 'Е').replace('M', 'М').replace(
                    'I', 'І').replace('c', 'с')

# Replace random Unicode characters that AllPowerlifting uses with ASCII alternatives


def fix_latin_names(csv):
    if 'Name' in csv.fieldnames:
        nameidx = csv.index('Name')
        for row in csv.rows:
            row[nameidx] = row[nameidx].replace('і', 'i').replace(
                'О', 'O').replace('у', 'y').replace('о', 'o').replace(
                'а', 'a').replace('г', 'r').replace('с', 'c')

# Use our form of the country names


def fix_countries(csv):
    if 'Country' in csv.fieldnames:
        countryidx = csv.index('Country')
        for row in csv.rows:
            country = row[countryidx]
            new_country = country
            if country == 'Slovak Republic':
                new_country = 'Slovakia'
            elif country == 'Czech Republic':
                new_country = 'Czechia'
            elif country == 'Tadjikistan':
                new_country = 'Tajikistan'
            row[countryidx] = new_country


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

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)

# Make use of some of the data from the sheet-name


def parse_sheetname(csv):
    if 'SheetName' in csv.fieldnames:
        sn_idx = csv.index('SheetName')
        sex_idx = csv.index('Sex')
        event_idx = csv.index('Event')
        tested_idx = csv.index('Tested')
        eqp_idx = csv.index('Equipment')

        for row in csv.rows:
            sheetname = row[sn_idx].lower()
            split_name = sheetname.split()

            if 'women' in split_name or 'wo' in split_name:
                row[sex_idx] = 'F'
                row[sn_idx] = sheetname.replace('women', '').replace('wo', '')
            elif 'men' in split_name:
                row[sex_idx] = 'M'
                row[sn_idx] = sheetname.replace('men', '')

            if 'pl' in split_name:
                row[event_idx] = 'SBD'
                row[sn_idx] = row[sn_idx].replace('pl', '')
            elif 'squat' in split_name:
                row[event_idx] = 'S'
                row[sn_idx] = row[sn_idx].replace('squat', '')
            elif 'bp' in split_name:
                row[event_idx] = 'B'
                row[sn_idx] = row[sn_idx].replace('bp', '')
            elif 'dl' in split_name:
                row[event_idx] = 'D'
                row[sn_idx] = row[sn_idx].replace('dl', '')
            elif 'pp' in split_name:
                row[event_idx] = 'BD'
                row[sn_idx] = row[sn_idx].replace('pp', '')

            if '(eq)' in split_name:
                row[eqp_idx] = 'Multi-ply'
                row[sn_idx] = row[sn_idx].replace('(eq)', '')
            elif '1ply' in split_name or 'sp' in split_name:
                row[eqp_idx] = 'Single-ply'
                row[sn_idx] = row[sn_idx].replace('1ply', '').replace('sp', '')
            elif '(raw)' in split_name or 'raw' in split_name:
                if 'S' in row[event_idx]:
                    row[eqp_idx] = 'Wraps'
                else:
                    row[eqp_idx] = 'Raw'
                row[sn_idx] = row[sn_idx].replace(
                    '(raw)', '').replace('raw', '')
            else:
                row[eqp_idx] = 'Multi-ply'

            if 'awpc' in split_name:
                row[tested_idx] = 'Yes'
                row[sn_idx] = row[sn_idx].replace('awpc', '')
            else:
                row[tested_idx] = 'No'
                row[sn_idx] = row[sn_idx].replace('wpc', '')

            row[sn_idx] = ' '.join(
                row[sn_idx].strip().replace('  ', ' ').split())
    return csv


def standardise_division(fed, csv):
    if 'Division' in csv.fieldnames:
        div_idx = csv.index('Division')
        tested_idx = csv.index('Tested')

        # IPF Feds that we are likely to use this script for
        if fed in ['FPR', 'KPF', 'UkrainePF']:
            for row in csv.rows:
                if row[div_idx] == 'T':
                    row[div_idx] = 'Sub-Juniors'
                elif row[div_idx] == 'J':
                    row[div_idx] = 'Juniors'
                elif row[div_idx] == 'O':
                    row[div_idx] = 'Open'
                elif row[div_idx] == 'M1':
                    row[div_idx] = 'Masters 1'
                elif row[div_idx] == 'M2':
                    row[div_idx] = 'Masters 2'
                elif row[div_idx] == 'M3':
                    row[div_idx] = 'Masters 3'
                elif row[div_idx] == 'M4':
                    row[div_idx] = 'Masters 4'
        elif fed in ['WPC-RUS', 'FPMO-WPC', 'FPTO', 'POKK-WPC-W']:
            for row in csv.rows:
                if row[div_idx] == 'T':
                    row[div_idx] = 'Teen'
                elif row[div_idx] == 'T1':
                    row[div_idx] = 'Teen 13-15'
                elif row[div_idx] == 'T2':
                    row[div_idx] = 'Teen 16-17'
                elif row[div_idx] == 'T3':
                    row[div_idx] = 'Teen 18-19'
                elif row[div_idx] == 'J':
                    row[div_idx] = 'Juniors 20-23'
                elif row[div_idx] == 'O':
                    row[div_idx] = 'Open'
                elif row[div_idx] == 'MS':
                    row[div_idx] = 'Submasters 33-39'
                elif row[div_idx] == 'M1':
                    row[div_idx] = 'Masters 40-44'
                elif row[div_idx] == 'M2':
                    row[div_idx] = 'Masters 45-49'
                elif row[div_idx] == 'M3':
                    row[div_idx] = 'Masters 50-54'
                elif row[div_idx] == 'M4':
                    row[div_idx] = 'Masters 55-59'
                elif row[div_idx] == 'M5':
                    row[div_idx] = 'Masters 60-64'
                elif row[div_idx] == 'M6':
                    row[div_idx] = 'Masters 65-69'
                elif row[div_idx] == 'M7':
                    row[div_idx] = 'Masters 70-74'
                elif row[div_idx] == 'M8':
                    row[div_idx] = 'Masters 75-79'
                elif row[div_idx] == 'M9':
                    row[div_idx] = 'Masters 80-84'
                elif row[div_idx] == 'M':
                    row[div_idx] = 'Masters'

                if row[tested_idx] == "Yes":
                    row[div_idx] = "Amateur "+row[div_idx]
                else:
                    row[div_idx] = "Pro " + row[div_idx]
    else:
        csv.append_column('Division')
        div_idx = csv.index('Division')
        for row in csv.rows:
            row[div_idx] = "Pro Open"
    return csv


def main(url):
    html = gethtml(url)
    soup = BeautifulSoup(html, 'html.parser')

    meetcsv = getmeetinfo(soup)
    dirname = getdirname(url)
    entriescsv = getresults(soup, url)

    if len(entriescsv.rows) == 0:
        error("No rows found!")

    remove_records(entriescsv)

    if 'Best3SquatKg' in entriescsv.fieldnames:
        expand_fourths(entriescsv, 'Best3SquatKg', 'Squat4Kg')
    if 'Best3BenchKg' in entriescsv.fieldnames:
        expand_fourths(entriescsv, 'Best3BenchKg', 'Bench4Kg')
    if 'Best3DeadliftKg' in entriescsv.fieldnames:
        expand_fourths(entriescsv, 'Best3DeadliftKg', 'Deadlift4Kg')

    unreverse_names(entriescsv)

    fix_cyrillic_names(entriescsv)
    fix_latin_names(entriescsv)

    # Figure out event information.
    markevent(entriescsv)

    entriescsv = parse_sheetname(entriescsv)
    entriescsv = standardise_division(meetcsv.rows[0][0], entriescsv)

    remove_empty_cols_ignore_fieldname(entriescsv)

    # Wilks will be automatically calculated later.
    # Feds get it wrong all the time.
    if 'Wilks' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Wilks')

    fix_totals(entriescsv)
    fix_place(entriescsv)

    fix_countries(entriescsv)

    # Remove all the columns named 'IGNORE'
    while 'IGNORE' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('IGNORE')

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

    if 'Name' not in entriescsv.fieldnames:
        subprocess.run([scriptsdir+os.sep+"calc-latin-names",
                        importdir+os.sep+dirname+os.sep+"entries.csv"])

    print("Imported into %s." % dirname)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: %s url" % sys.argv[0])
    main(sys.argv[1])
