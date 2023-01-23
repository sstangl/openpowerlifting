#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes the CPU main page for new meets.
# The CPU updated their website and removed raw database access.
#
# On the new site, in order to look up a meet in the database, you need
# to either already know its full name or know a lifter in that meet.
# The CPU pulls a "Latest Results" list from the database and shows that on
# the main page. So we can use that to pick up some of the new ones.
# We would miss any meets that take a while to post.


from bs4 import BeautifulSoup
import sys
import os
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))
FEDURL = "http://www.powerlifting.ca"

# The CPU website annoyingly posts results on per-province pages,
# so we have to scan them all individually.
#
# List from manually going though Events -> *Results.
PROVINCE_URLS = [
    "https://www.powerlifting.ca/lifter_database/external/prov_res_ab.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_bc.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_mb.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_nl.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_nb.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_ns.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_on.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_pe.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_qc.php",
    "https://www.powerlifting.ca/lifter_database/external/prov_res_sk.php",
    "https://www.powerlifting.ca/lifter_database/external/nats_res.php",
    "https://www.powerlifting.ca/lifter_database/external/reg_res.php",
]


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []

    for a in soup.find_all("a"):

        url = a['href']

        if 'contest_results' not in url:
            continue

        # This happens for national results, like Central Canadians.
        # The CPU website keeps changing the URLs to these meets.
        if '../../../index.php' in url:
            prefix = 'http://powerlifting.ca/cpu/index.php'
            url = url.replace('../../../index.php', prefix)

        # All the provincial meet results have incorrect URLs...
        if '/joomla30/' in url:
            url = url.replace('/joomla30/', '/cpu/')

        # Some URLs are unfortunately relative to an index.php.
        if url.startswith("contest_results?"):
            url = "https://www.powerlifting.ca/lifter_database/external/" + url

        if url.startswith("../../contest-results?"):
            url = url.replace(
                "../../", "https://www.powerlifting.ca/lifter_database/external/")

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    # Scans from new meets from the homepage, which displays the five most
    # recent results, sorted by date. This skips old results.
    # html = oplprobe.gethtml(FEDURL)
    # meetlist = meetlist + getmeetlist(html)

    for url in PROVINCE_URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)

    # The CPU keeps changing where they put the contest_results script.
    for m in list(entered):
        if "/lifter_database/contest_results.php?" in m:
            url_from = "/lifter_database/contest_results.php?"
            url_to = "/contest-results?"
            entered.add(m.replace(url_from, url_to))

        if "/cpu/index.php/contest-results?" in m:
            url_from = "/cpu/index.php/contest-results?"
            url_to = "/lifter_database/external/contest_results.php?"
            entered.add(m.replace(url_from, url_to))

        if "/lifter_database/contest_results.php?" in m:
            url_from = "/lifter_database/contest_results.php?"
            url_to = "/lifter_database/external/contest_results.php?"
            entered.add(m.replace(url_from, url_to))

    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[CPU]'), unentered)


if __name__ == '__main__':
    main()
