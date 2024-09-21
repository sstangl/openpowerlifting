#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from WPC-UKR

import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe
    import allpowerlifting_probe

WPC_UKR_URL = 'https://en.allpowerlifting.com/federations/WPC/WPC-UKR/'


FEDDIR = os.path.dirname(os.path.realpath(__file__))


def colour(s):
    return "\033[1;33m" + s + "\033[0;m"


def main():

    meetlist = allpowerlifting_probe.probefederation(WPC_UKR_URL)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(colour('[WPC-UKR]'), unentered)


if __name__ == '__main__':
    main()
