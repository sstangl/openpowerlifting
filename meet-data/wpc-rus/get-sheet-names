#!/usr/bin/env python3
# Gets the sheet names of an excel file

import xlrd
import sys


def getSheetName(file_name):
    names = []
    Workbook = xlrd.open_workbook(file_name)
    names = Workbook.sheet_names()

    for name in names:
        sheet = Workbook.sheet_by_name(name)
        if sheet.nrows != 0:
            print(name)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv > entries.csv" % sys.argv[0])
        sys.exit(1)
sys.exit(getSheetName(sys.argv[1]))
