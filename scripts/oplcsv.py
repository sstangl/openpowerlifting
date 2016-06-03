#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# A better CSV manipulation library for the OpenPowerlifting format.
#

class Csv:
    def __init__(self, filename=None):
        if filename:
            with open(filename, 'r') as fd:
                self.fieldnames = fd.readline().strip().split(',')
                self.rows = [x.strip().split(',') for x in fd.readlines()]
        else:
            self.fieldnames = []
            self.rows = []

    def __len__(self):
        return len(self.rows)

    def append_column(self, name):
        self.fieldnames.append(name)
        for row in self.rows:
            row.append('')

    def remove_column_by_index(self, idx):
        del self.fieldnames[idx]
        for row in self.rows:
            del row[idx]

    def remove_column_by_name(self, name):
        for i, header in enumerate(self.fieldnames):
            if header == name:
                self.remove_column_by_index(i)
                return

    def select_columns_by_name(self, namelist):
        x = []
        for field in self.fieldnames:
            if field not in namelist:
                x.append(field)

        for field in x:
            self.remove_column_by_name(field)

    # Integrate another Csv object into the current one.
    def cat(self, other):
        for header in other.fieldnames:
            if not header in self.fieldnames:
                self.append_column(header)

        # An array mapping index in other.fieldnames to index in self.fieldnames.
        mapping = [self.fieldnames.index(header) for header in other.fieldnames]

        for row in other.rows:
            build = ['' for x in range(0, len(self.fieldnames))]

            for i, cell in enumerate(row):
                build[mapping[i]] = cell

            self.rows.append(build)

    def write(self, fd):
        fd.write(','.join(self.fieldnames) + "\n")
        fd.writelines([','.join(row) + "\n" for row in self.rows])
