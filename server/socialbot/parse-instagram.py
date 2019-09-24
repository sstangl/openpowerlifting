#!/usr/bin/env python3
# Parses the OpenPowerlifting account on Instagram for follower counts.

import datetime
import urllib.request

URL = "https://www.instagram.com/openpowerlifting/"


def get_html(url):
    with urllib.request.urlopen(url) as fp:
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


def main():
    html = get_html(URL)
    followers = get_followers(html)
    date = get_date()
    print(date + ": **" + '{:,}'.format(followers) + "** graph enthusiasts")


if __name__ == "__main__":
    main()
