#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from World Powerlifting New Zealand
# Results are pdfs of inconsistent layout, some are text,
# others are screenshots, they are all some variant of
# NextLifter as far as I can see.

# International WP events are also linked to, which we ignore,
# and can be identified by URL.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ['https://www.sporty.co.nz/worldpowerliftingnz/Results-1/Historic-results',
        'https://www.sporty.co.nz/worldpowerliftingnz/Results-1/Results-2021',
        'https://www.sporty.co.nz/worldpowerliftingnz/Results-1/Results-1']

BASEURL = "https://www.sporty.co.nz/worldpowerliftingnz/Results-1"
NZURLPREFIX = "https://www.sporty.co.nz/asset/downloadasset?id="
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    # this one is a bit odd - there is a widget
    # <div class="splitter-column-sortable" data-key="splitter_col1">
    # containing escaped HTML in the data-widgetsettings attr, in which we have
    # <a data-cke-saved-href="https://sporty.co.nz/asset/downloadasset?id=$ID">
    urls = []

    widget_div = soup.find(
        lambda tag:
        tag.name == 'div' and
        tag.has_attr('class') and
        'splitter-column-sortable' in tag['class'] and
        tag.has_attr('data-key') and
        'splitter_col1' in tag['data-key'] and
        tag.has_attr('data-widgetsettings')
    )

    div_soup = BeautifulSoup(widget_div['data-widgetsettings'], 'html.parser')

    # WP international results are included, so look specifically for URLs
    # for NZ comps
    for a in div_soup.find_all(
        lambda tag:
        tag.name == 'a' and
        tag.has_attr('data-cke-saved-href') and
        tag['data-cke-saved-href'].startswith(NZURLPREFIX)
    ):
        urls.append(a['data-cke-saved-href'])

    return urls


def main():
    meetlist = []

    for url in URLS:

        try:
            html = oplprobe.gethtml(url, raise_on_redirect=True)
            meetlist = meetlist + getmeetlist(html)

        except oplprobe.UnexpectedRedirect:
            pass

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[WP-NZ]'), unentered)


if __name__ == '__main__':
    main()
