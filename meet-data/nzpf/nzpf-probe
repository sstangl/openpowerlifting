#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the NZ Powerlifting Federation.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))
BASEURL_SPORTY = "http://www.sporty.co.nz/asset/downloadasset?id="


URL_WBOP = "http://www.sporty.co.nz/wbop/Results-1"
URL_AKL = "http://www.sporty.co.nz/aucklandpowerlifting/Results"
URL_CD = "https://www.sporty.co.nz/WellingtonCentralPowerlifting/Results-1/tab1#"
URL_NTL = "http://www.sporty.co.nz/northlandpowerlifting/" \
          "Northland-Champs-3-lift-Bench-results-Nominat/2018-2"
URL_NTL_NOV = "http://www.sporty.co.nz/northlandpowerlifting/" \
    "Novice-Competition-results/2018-1"
URL_OTA = "https://www.sporty.co.nz/otagoweightlifting/Results/POWERLIFTING-1"
URL_NZPF = "http://www.nzpowerlifting.co.nz/Results-1/National-Results"
URL_WCPA = "https://www.sporty.co.nz/WellingtonCentralPowerlifting/Results-1/tab1#"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


# Fetch Otago and National level meets
def getmeetlist_sporty(html):
    soup = BeautifulSoup(html, 'html.parser')

    div = soup.find_all("div", {"class": "splitter-column-sortable"})[1]
    splitter_settings = BeautifulSoup(
        div['data-widgetsettings'], 'html.parser')
    urls = []

    for a in splitter_settings.find_all('a'):
        if not a.has_attr('data-cke-saved-href'):
            continue
        url = a['data-cke-saved-href']

        if 'sportsground' in url:
            continue

        if url not in urls:
            urls.append(url)

    return urls

# Get the Auckland,WBOP and Northland meets which use file IDs instead of links


def getmeetlist_sporty_partial(html, code):
    soup = BeautifulSoup(html, 'html.parser')

    divs = soup.find_all("div", {"class": "splitter-column-sortable"})

    for div in divs:
        if div.has_attr('data-widgetsettings'):
            # Links are passed to widget as an html string so need to expand it again
            splitter_settings = BeautifulSoup(
                div['data-widgetsettings'], 'html.parser')
            if (splitter_settings.find(
                    "input", {"id": "hdnDocuments_" + code}) is not None):
                list_str = splitter_settings.find(
                    "input", {"id": "hdnDocuments_" + code})['value']
                break

    urls = []

    # Now need to extract the links from the list of parameters

    list_str = list_str[1:-1]  # Remove the square brackets

    split_list = list_str.split("},")
    split_list = [x + "}" for x in split_list]  # Add the curly brackets back

    for link in split_list:
        end_link = link.find('"', 7)
        file_ext = link[7:end_link]  # Now we finally have the file ID!

        url = BASEURL_SPORTY + file_ext

        if url not in urls:
            urls.append(url)
    return urls


def main():
    meetlist = []

    html_WBOP = oplprobe.gethtml(URL_WBOP)
    meetlist_WBOP = getmeetlist_sporty_partial(html_WBOP, "584859")

    html_AKL = oplprobe.gethtml(URL_AKL)
    meetlist_AKL = getmeetlist_sporty_partial(html_AKL, "357445")

    # html_NTL_NOV = oplprobe.gethtml(URL_NTL_NOV)
    meetlist_NTL_NOV = []  # getmeetlist_sporty_partial(html_NTL_NOV, "644298")

    # html_NTL = oplprobe.gethtml(URL_NTL)
    meetlist_NTL = []  # getmeetlist_sporty_partial(html_NTL, "675128")

    html_OTA = oplprobe.gethtml(URL_OTA)
    meetlist_OTA = getmeetlist_sporty(html_OTA)

    html_CD = oplprobe.gethtml(URL_CD)
    meetlist_CD = getmeetlist_sporty_partial(html_CD, "797402")

    html_NZPF = oplprobe.gethtml(URL_NZPF)
    meetlist_NZPF = getmeetlist_sporty(html_NZPF)

    html_WCPA = oplprobe.gethtml(URL_WCPA)
    meetlist_WCPA = getmeetlist_sporty_partial(html_WCPA, "797402")

    meetlist = meetlist_WBOP + meetlist_AKL + meetlist_NTL + \
        meetlist_OTA + meetlist_NZPF + meetlist_NTL_NOV + meetlist_CD + meetlist_WCPA

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[NZPF]'), unentered)
    print(color('[NZPF] ') + 'Check Canterbury Powerlifting Facebook page.')
    print(color('[NZPF] ') +
          'Check Central Districts Powerlifting Facebook page.')


if __name__ == '__main__':
    main()
