#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Library for common functionality for probe scripts.
#

import os
import sys
import urllib.request
import urllib.parse


class UnexpectedRedirect(Exception):
    pass


def die(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def gethtml(url, raise_on_redirect=False):
    request = urllib.request.Request(url)
    request.add_header('User-Agent', 'Mozilla/5.0 Gecko/20100101 Firefox/52.0')

    with urllib.request.urlopen(request, timeout=10) as r:
        if raise_on_redirect and r.geturl() != url:
            raise UnexpectedRedirect(r.geturl())
        return r.read()


def getenteredurls(feddir):
    urls = set()
    for dirname, subdirs, files in os.walk(feddir):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r', encoding='utf-8') as fd:
                for k in fd.readlines():
                    urls.add(k.strip())
    return urls


def getunenteredurls(meetlist, enteredmeets):
    # Calculate some variants of the entered meets.
    variants = set()
    for k in enteredmeets:

        curr_variants = set()

        curr_variants.add(k.replace("https://", "http://"))
        curr_variants.add(k.replace("http://", "https://"))

        # Add space variants
        curr_variants.update([v.replace("%20", " ") for v in curr_variants])
        curr_variants.update([v.replace(" ", "%20") for v in curr_variants])

        # Check with and without www.
        curr_variants.update([v.replace("://www.", "://") for v in curr_variants])
        curr_variants.update([v.replace("://", "://www.") for v in curr_variants])

        # Add the version with unicode characters converted to the %xx version
        curr_variants.update([urllib.parse.unquote(v) for v in curr_variants])

        # Check with and without .html
        curr_variants.update([v.replace(".html", "") for v in curr_variants])
        curr_variants.update([v+".html" for v in curr_variants if ".html" not in v])

        # Add the version with unicode converted to idna
        idna_variants = set()
        for v in curr_variants:
            url_parts = list(urllib.parse.urlsplit(v))
            url_parts[1] = url_parts[1].encode('idna').decode('utf-8')
            url_idna = urllib.parse.urlunsplit(url_parts)
            idna_variants.add(url_idna)
            idna_variants.add(urllib.parse.unquote(url_idna))

        curr_variants.update(idna_variants)

        variants.update(curr_variants)
    enteredmeets.update(variants)

    unentered = []
    for m in meetlist:
        # Skip any results that list us as the authoritatize source.
        if 'www.openpowerlifting.org' in m:
            continue

        if m not in enteredmeets and m not in unentered:
            unentered.append(m)

    return unentered


# Given a federation string and a list of unentered meets, print to stdout.
def print_meets(fedstr, meetlist):
    count = len(meetlist)

    # If a quick mode was requested, just show the newest meets.
    if '--quick' in sys.argv:
        meetlist = meetlist[0:5]

    try:
        for url in meetlist:
            print("%s %s" % (fedstr, url.replace(' ', '%20')))

        if count > 3:
            print("%s %d meets remaining." % (fedstr, count))
    except BrokenPipeError:
        pass
