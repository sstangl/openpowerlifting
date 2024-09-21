#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Parse the USPA competition results download page and download
# any that aren't in the USPA meet results.
#


from bs4 import BeautifulSoup
import os
import shutil
import sys
import urllib.request


CUTOFF = 200  # Ignore earlier meets, since they have invalid URLs anyway.
FETCHDIR = 'fetch'  # Directory in CWD to use for fetchin'.
RESULTSURL = 'https://docs.google.com/spreadsheets/d/' \
             '1HBpedvVS8N0H75Eg9qwTxClqtV1EqO-IuctYzgiyMNI/' \
             'pubhtml/sheet?hl=en_US&headers=false&gid=0'


states = {
    'Alaska': 'AK',
    'Alabama': 'AL',
    'Arkansas': 'AR',
    'American Samoa': 'AS',
    'Arizona': 'AZ',
    'British Columbia': 'BC',  # Apparently USPA is in Canada now?
    'California': 'CA',
    'Colorado': 'CO',
    'Connecticut': 'CT',
    'District of Columbia': 'DC',
    'Delaware': 'DE',
    'Florida': 'FL',
    'Georgia': 'GA',
    'Guam': 'GU',
    'Hawaii': 'HI',
    'Iowa': 'IA',
    'Idaho': 'ID',
    'Illinois': 'IL',
    'Indiana': 'IN',
    'Kansas': 'KS',
    'Kentucky': 'KY',
    'Louisiana': 'LA',
    'Massachusetts': 'MA',
    'Massachussetts': 'MA',  # Work around spelling errors.
    'Maryland': 'MD',
    'Maine': 'ME',
    'Michigan': 'MI',
    'Minnesota': 'MN',
    'Missouri': 'MO',
    'Northern Mariana Islands': 'MP',
    'Mississippi': 'MS',
    'Montana': 'MT',
    'National': 'NA',
    'North Carolina': 'NC',
    'North Dakota': 'ND',
    'Nebraska': 'NE',
    'New Hampshire': 'NH',
    'New Jersey': 'NJ',
    'New Mexico': 'NM',
    'Nevada': 'NV',
    'New York': 'NY',
    'Ohio': 'OH',
    'Oklahoma': 'OK',
    'Oregon': 'OR',
    'Pennsylvania': 'PA',
    'Puerto Rico': 'PR',
    'Rhode Island': 'RI',
    'South Carolina': 'SC',
    'South Dakota': 'SD',
    'Tennessee': 'TN',
    'Texas': 'TX',
    'Utah': 'UT',
    'Virginia': 'VA',
    'Virgin Islands': 'VI',
    'Vermont': 'VT',
    'Washington': 'WA',
    'Wisconsin': 'WI',
    'West Virginia': 'WV',
    'Wyoming': 'WY',
}


# Just a way to name a tuple.
class Meet:

    def __init__(self, url, num, date, name, location):
        self.url = url
        self.num = ("0" * (4 - len(num))) + num
        self.date = date
        self.name = name
        self.location = location

    def __str__(self):
        return '"%s - %s %s %s"' % (self.num, self.date, self.name, self.location)

    def __repr__(self):
        return str(self)


def gethtml():
    with urllib.request.urlopen(RESULTSURL) as r:
        return r.read()


def getmeetinfo(tr):
    # The first column would have the URL.
    urltd = tr.find('td')
    if urltd is None:
        return None

    a = urltd.find('a')
    if a is None:
        return None
    url = a.get('href')

    # Then the rest of the information should be there too.
    tds = tr.find_all('td')
    num = tds[1].get_text()
    date = tds[2].get_text()
    name = tds[3].get_text()
    location = tds[4].get_text()

    return Meet(url, num, date, name, location)


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    # Ignore all the Google Sheets javascript and get to the tags.
    doc = soup.find(id='sheets-viewport')
    table = doc.find('table')

    meets = []

    # Iterate over every row of the table.
    for tr in table.find_all('tr'):
        info = getmeetinfo(tr)
        if info:
            meets.append(info)

    return meets


def getknownmeets():
    # Known meets for the USPA have numeric directory names.
    meets = []
    for name in os.listdir(os.getcwd()):
        if name.isdigit():
            meets.append(name)

    # Also check out the IGNORE file.
    for line in open("IGNORE").readlines():
        meets.append(line.strip())

    return meets


def getunknownmeets(meets, known):
    unknown = []
    for meet in meets:
        if meet.num not in known and int(meet.num) >= CUTOFF:
            unknown.append(meet)
    return unknown


# The meet URL is actually a URL with a link to the actual URL!
def getpdfurl(meet):
    with urllib.request.urlopen(meet.url) as r:
        soup = BeautifulSoup(r.read(), 'html.parser')

    body = soup.body.get_text()
    url = body[body.index('http'):]
    return url


def downloadpdf(meet):
    pdfurl = getpdfurl(meet)

    filename = FETCHDIR + os.sep + meet.num + '.pdf'
    useragent = 'Mozilla/5.0 (X11; Linux x86_64; rv:55.0) Gecko/20100101 Firefox/55.0'
    try:
        # The USPA site now requires User-Agent to be set.
        request = urllib.request.Request(pdfurl, headers={'User-Agent': useragent})
        with urllib.request.urlopen(request) as r, open(filename, 'wb') as fd:
            shutil.copyfileobj(r, fd)
    except urllib.error.HTTPError:
        print("Couldn't download PDF (HTTP error).")


def writemeetcsv(meet):
    filename = FETCHDIR + os.sep + meet.num + '-meet.csv'

    headers = "Federation,Date,MeetCountry,MeetState,MeetTown,MeetName"

    federation = "USPA"

    if '/' in meet.date:
        # Given like 4/16/2016.
        (month, day, year) = meet.date.split('/')
    elif ',' in meet.date:
        # Given like "April 2-3, 2016" or "May 5, 2012".
        (month, day, year) = meet.date.split(' ')

        # Sometimes abbreviated month names are used.
        if 'Jan' in month:
            month = '01'
        elif 'Feb' in month:
            month = '02'
        elif 'Mar' in month:
            month = '03'
        elif 'Apr' in month:
            month = '04'
        elif 'May' in month:
            month = '05'
        elif 'Jun' in month:
            month = '06'
        elif 'Jul' in month:
            month = '07'
        elif 'Aug' in month:
            month = '08'
        elif 'Sep' in month or month == 'Spetember':  # Way to go
            month = '09'
        elif 'Oct' in month:
            month = '10'
        elif 'Nov' in month:
            month = '11'
        elif 'Dec' in month:
            month = '12'
        else:
            print("Unknown month: %s" % month)
            sys.exit(1)

        day = day.replace(',', '')
        if '-' in day:
            day = day[:day.index('-')]
    elif meet.date.count('-') == 2:
        # USPA suddenly started giving dates as 7-23-16 for no reason.
        (month, day, year) = meet.date.split('-')

    # Ensure left-padded with zeros.
    month = ('0' * (2 - len(month))) + month
    day = ('0' * (2 - len(day))) + day
    if len(year) == 2:
        year = '20' + year
    date = "%s-%s-%s" % (year, month, day)

    country = 'USA'
    name = meet.name.replace(',', '')

    town = ""
    state = ""
    if meet.location.count(',') == 1:
        (town, state) = [x.strip() for x in meet.location.split(',')]

    if len(state) > 2:
        if state == 'Canada':
            country = 'Canada'
            state = ''
        elif state == 'Maui':  # Correct a USPA error.
            state = 'HI'
        elif state == 'CAlifornia':  # Correct another error.
            state = 'CA'
        elif state in states:
            state = states[state]

    with open(filename, 'w') as fd:
        fd.write(headers + '\n')
        fd.write("%s,%s,%s,%s,%s,%s\n" %
                 (federation, date, country, state, town, name))


def main():
    html = gethtml()
    meets = getmeetlist(html)
    known = getknownmeets()
    remote = getunknownmeets(meets, known)

    for meet in remote:
        print(' Fetching %s: %s' % (meet.num, meet.name))
        # downloadpdf(meet)
        writemeetcsv(meet)

    if len(remote) == 0:
        print(' No new meets found.')


if __name__ == '__main__':
    # Make reasonable assertions that the cwd is the USPA results directory.
    if os.getcwd().split(os.sep)[-1] != 'uspa' or not os.path.isfile('UPDATING'):
        print(" %s must be run from the USPA meet data directory." %
              sys.argv[0])
        sys.exit(1)

    if not os.path.exists(FETCHDIR):
        os.makedirs(FETCHDIR)

    main()
