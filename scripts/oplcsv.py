#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# A better CSV manipulation library for the OpenPowerlifting format.
#


class Csv:

    def __init__(self, filename=None):
        if filename:
            with open(filename, 'r', encoding='utf-8') as fd:
                self.fieldnames = fd.readline().rstrip().split(',')
                self.rows = [x.rstrip("\r\n").split(',')
                             for x in fd.readlines()]
        else:
            self.fieldnames = []
            self.rows = []

    def __len__(self):
        return len(self.rows)

    def index(self, name):
        return self.fieldnames.index(name)

    def append_column(self, name):
        self.fieldnames.append(name)
        for row in self.rows:
            row.append('')

    def append_columns(self, namelist):
        self.fieldnames += namelist
        addend = ['' for x in namelist]
        for row in self.rows:
            row += addend

    def insert_column(self, index, name):
        self.fieldnames.insert(index, name)
        for row in self.rows:
            row.insert(index, '')

    def remove_column_by_index(self, idx):
        del self.fieldnames[idx]
        for row in self.rows:
            del row[idx]

    def remove_column_by_name(self, name):
        for i, header in enumerate(self.fieldnames):
            if header == name:
                self.remove_column_by_index(i)
                return

    # Integrate another Csv object into the current one.
    def cat(self, other):
        for header in other.fieldnames:
            if header not in self.fieldnames:
                self.append_column(header)

        # An array mapping index in other.fieldnames to index in
        # self.fieldnames.
        mapping = [self.index(header) for header in other.fieldnames]

        for row in other.rows:
            build = ['' for x in range(0, len(self.fieldnames))]

            for i, cell in enumerate(row):
                build[mapping[i]] = cell

            self.rows.append(build)

    def write(self, fd):
        fd.write(','.join(self.fieldnames) + "\n")
        for row in self.rows:
            fd.write(','.join(row) + "\n")
