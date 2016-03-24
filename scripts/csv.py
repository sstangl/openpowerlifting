#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# A better CSV manipulation library for the OpenPowerlifting format.
#

class Csv:
    def __init__(self, filename):
        with open(filename, 'r') as fd:
            self.fieldnames = fd.readline().strip().split(',')
            self.rows = [x.strip().split(',') for x in fd.readlines()]

    def remove_column_by_index(self, idx):
        del self.fieldnames[idx]
        for row in self.rows:
            del row[idx]

    def remove_column_by_name(self, name):
        for i, header in enumerate(self.fieldnames):
            if header == name:
                self.remove_column_by_index(i)
                return

    def write_to(self, fd):
        fd.write(','.join(self.fieldnames) + "\n")
        fd.writelines([','.join(row) + "\n" for row in self.rows])
