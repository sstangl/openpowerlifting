#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new Fédération Française de Force meets.

from bs4 import BeautifulSoup
import os
import sys
import requests

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe

RESULTS_URL = ("https://www.ffforce.fr/fr/force-athletique-ffforce/"
               "regional-force-athletique/competitions-force-athletique/"
               "force-athletique-regional-saison-2024.html")
BASEURL = "https://www.ffforce.fr"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;37m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    # get the value attributes in the league dropdown then make POST requests
    league_selector = soup.find('select')
    options = league_selector.find_all('option')
    leagues = options[1:]
    league_values = [league['value'] for league in leagues]

    urls = []

    for league_value in league_values:

        results = requests.post(RESULTS_URL, {'jsArtSelectCategorie': league_value})
        soup_results = BeautifulSoup(results.text, 'html.parser')

        main = soup_results.find("div", "tableau-classmt")
        assert main, "No <div> element with class `tableau-classmt` found."

        links = main.find_all("div", "cel-classmt site")

        for link in links:

            a = link.find('a')

            # The FFForce has combined its calendar and results pages
            # so some meets don't have a results link yet
            if a is None:
                continue

            # Exclude the recap file that the federation periodically updates
            elif 'organisees dans les ligues' in a['href']:
                continue

            url = BASEURL + a['href']

            urls.append(url)

    return urls


def main():
    html_content = oplprobe.gethtml(RESULTS_URL)
    meets_list = getmeetlist(html_content)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meets_list, entered)

    oplprobe.print_meets(color('[FFForce]'), unentered)


if __name__ == '__main__':
    main()
