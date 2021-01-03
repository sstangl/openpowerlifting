#!/usr/bin/env python3
# Parses the OpenPowerlifting account on Instagram for follower counts.

import datetime
import urllib.request
import random

URL_BASE = "https://www.instagram.com/"

LABELS = [
    "graph enthusiasts",
    "human souls",
    "followers",
    "casual acquaintances",
    "good friends",
    "influencers",
    "enthusiasts",
    "powerlifters",
    "lifters taking a 20-minute break between sets",
    "real human people who are definitely not dogs",
    "potential SBD athletes",
    "responsible adults",
    "irresponsible children"
]


def get_html(url):
    request = urllib.request.Request(url)
    request.add_header('User-Agent', 'Mozilla/5.0 Gecko/20100101 Firefox/52.0')

    with urllib.request.urlopen(request, timeout=10) as fp:
        html = fp.read().decode("utf-8")
    return html


def get_followers(html):
    target = '"edge_followed_by":{"count":'
    start = html.index(target)
    end = html.index('}', start)
    return int(html[start:end][len(target):])


def get_date():
    today = datetime.date.today()
    return str(today)


def main(ig_account):
    html = get_html(URL_BASE + ig_account)
    followers = get_followers(html)
    date = get_date()
    label = random.choice(LABELS)
    print(date + ": (@" + ig_account + ") **" + '{:,}'.format(followers) + "** " + label)


if __name__ == "__main__":
    import sys
    main(sys.argv[1])
