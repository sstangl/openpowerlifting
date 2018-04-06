#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Helper library for dealing with the Name column.
#


def standardize_upper_ascii(s):
    s = s.upper()
    s = s.replace('Ň', 'N')
    s = s.replace('Ö', 'O')
    s = s.replace('Ã', 'A')
    s = s.replace('Ä', 'A')
    s = s.replace('Ü', 'U')
    s = s.replace('Ø', 'O')
    s = s.replace('É', 'E')
    s = s.replace('Å', 'A')
    s = s.replace('Á', 'A')
    s = s.replace('Ó', 'O')
    s = s.replace('Ñ', 'N')
    s = s.replace('Í', 'I')
    s = s.replace('Ú', 'U')
    s = s.replace('Ć', 'C')
    s = s.replace('Č', 'C')
    s = s.replace('Ý', 'Y')
    s = s.replace('Ž', 'Z')
    s = s.replace('Š', 'S')
    s = s.replace('Ł', 'W')
    s = s.replace('Ů', 'U')
    s = s.replace('Æ', 'AE')
    s = s.replace('È', 'E')
    s = s.replace('Ê', 'E')
    s = s.replace('Î', 'I')
    s = s.replace('Ë', 'E')
    s = s.replace('Đ', 'D')
    s = s.replace('Ð', 'D')  # Not a duplicate.
    s = s.replace('Ě', 'E')
    s = s.replace('Ç', 'C')
    s = s.replace('Ô', 'O')
    s = s.replace('Ï', 'I')
    return s


# Levenshtein algorithm taken from
# https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Levenshtein_distance#Python.
def levenshtein(s1, s2):
    if len(s1) < len(s2):
        return levenshtein(s2, s1)

    # len(s1) >= len(s2)
    if len(s2) == 0:
        return len(s1)

    previous_row = range(len(s2) + 1)
    for i, c1 in enumerate(s1):
        current_row = [i + 1]
        for j, c2 in enumerate(s2):
            # j+1 instead of j since previous_row and current_row are one
            # character longer than s2.
            insertions = previous_row[j + 1] + 1
            deletions = current_row[j] + 1
            substitutions = previous_row[j] + (c1 != c2)
            current_row.append(min(insertions, deletions, substitutions))
        previous_row = current_row

    return previous_row[-1]


# Converts a part of a name to a four-character code, based on English phonetics,
# which can be used to classify equivalent names. Intended for use on surnames.
#
# Algorithm described in Section 9.3:
# https://web.archive.org/web/20090107221831/http://www.cs.utah.edu/contest/2005/NameMatching.pdf
def phonex(s):
    s = s.replace('-', '')
    s = standardize_upper_ascii(s)

    assert s.isalpha()
    assert len(s) > 1  # No abbreviations.

    # 1. Remove all trailing 'S' characters at the end of the name.
    s = s.rstrip('S')

    # 2. Convert leading letter-pairs is follows:
    #    KN -> N, PH -> F, WR -> R
    if s.startswith('KN') or s.startswith('WR'):
        s = s[1:]
    elif s.startswith('PH'):
        s = 'F' + s[2:]

    # 3. Convert leading single letters as follows:
    #    H -> Remove; E,I,O,U,Y -> A; K,Q -> C; P -> B;
    #    J -> G; V -> F; Z -> S
    if s[0] == 'H':
        s = s[1:]
    elif s[0] in 'EIOUY':
        s = 'A' + s[1:]
    elif s[0] in 'K:':
        s = 'C' + s[1:]
    elif s[0] == 'P':
        s = 'B' + s[1:]
    elif s[0] == 'J':
        s = 'J' + s[1:]
    elif s[0] == 'V':
        s = 'V' + s[1:]
    elif s[0] == 'Z':
        s = 'Z' + s[1:]

    # 4. Retain the first letter of the pre-processed name, and drop all occurrences
    #    of A,E,H,I,O,U,W,Y in other positions.
    k = s[0]
    for c in s[1:]:
        if c not in 'AEHIOUWY':
            k += c
    s = k

    # 5. Assign the following numbers to the remaining letters after the first:
    #    B,F,P,V -> 1; C,G,J,K,Q,S,X,Z -> 2;
    #    D,T -> 3 (only if not followed by C);
    #    L -> 4 (only if not followed by a vowel or end of name)
    #    M,N -> 5 (ignore next letter if either D or G)
    #    R -> 6 (only if not followed by vowel or end of name)
    # Ignore the current letter if it would repeat the most recently-added
    # digit.
    i = 1
    k = ''
    while i < len(s):
        c = s[i]
        c_next = s[i + 1] if i + 1 < len(s) else ''
        n = ''

        if c in 'BFPV':
            n = '1'
        elif c in 'CGJKQSXZ':
            n = '2'
        elif c in 'DT':
            if c_next != 'C':
                n = '3'
        elif c == 'L':
            if c_next not in 'AEIOUY':  # The empty string is in all strings.
                n = '4'
        elif c in 'MN':
            n = '5'
            if c_next in 'DG':
                i += 1
        elif c == 'R':
            if c_next not in 'AEIOUY':
                n = '6'
        else:
            raise ValueError("Unhandled character: %s" % c)

        # Ignore repeats.
        if not k or n != k[-1]:
            k += n

        i += 1

    # 6. Convert to the form "Letter, Digit, Digit, Digit" by adding trailing zeros
    #    (if there are fewer than three digits) or by dropping rightmost digits if
    #    there are more than three).
    s = s[0] + k[0:3] + ('0' * (3 - len(k)))
    return s


# By default, try to establish names that collide, testing the algorithms.
if __name__ == '__main__':
    import oplcsv
    import sys

    csv = oplcsv.Csv(sys.argv[1])
    nameidx = csv.index('Name')
    names = [r[nameidx] for r in csv.rows]

    h = {}
    counts = {}

    for name in names:
        comps = name.split()[0:2]
        if len(comps) != 2:
            continue

        if not comps[0].isalpha() or len(comps[0]) <= 1:
            continue
        if not comps[1].isalpha() or len(comps[1]) <= 1:
            continue

        first = phonex(comps[0])
        second = phonex(comps[1])

        key = '%s-%s' % (first, second)

        if key not in h:
            h[key] = [name]
        elif name not in h[key]:
            h[key].append(name)

        if name not in counts:
            counts[name] = 1
        else:
            counts[name] += 1

    for k, v in h.items():
        if len(v) == 1:
            continue

        # Also, just for the moment, only consider the ones that
        # have a bunch of samples.
        if max([counts[n] for n in v]) < 10:
            continue

        for i in range(1, len(v)):
            # For the moment, since there are so many name conflicts,
            # just consider the ones that have a minimal edit distance.
            if levenshtein(v[0], v[i]) == 1:
                print([(n, counts[n]) for n in v])
                break
