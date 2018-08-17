#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Helper library for automatically generating usernames from the Name column.
# This logic is needed independently for testing and for compilation.
#

# UTF-8 => lookalike ASCII substitutions.
SubstitutionMap = {
    'á': 'a', 'ä': 'a', 'å': 'a', 'ą': 'a', 'ã': 'a', 'à': 'a', 'â': 'a', 'ā': 'a',
    'ắ': 'a', 'ấ': 'a', 'ầ': 'a', 'ặ': 'a', 'ạ': 'a', 'ă': 'a', 'ả': 'a', 'ậ': 'a',
    'æ': 'ae',
    'ć': 'c', 'ç': 'c', 'č': 'c',
    'đ': 'd', 'ð': 'd', 'ď': 'd',
    'é': 'e', 'ê': 'e', 'ë': 'e', 'è': 'e', 'ě': 'e', 'ę': 'e', 'ē': 'e', 'ế': 'e',
    'ễ': 'e', 'ể': 'e', 'ề': 'e', 'ệ': 'e', 'ė': 'e', 'ə': 'e',
    'ğ': 'g', 'ģ': 'g',
    'î': 'i', 'í': 'i', 'ï': 'i', 'ì': 'i', 'ї': 'i', 'ī': 'i', 'ĩ': 'i', 'ị': 'i',
    'ı': 'i',
    'ķ': 'k',
    'ľ': 'l', 'ĺ': 'l', 'ļ': 'l', 'ŀ': 'l',
    'ñ': 'n', 'ń': 'n', 'ň': 'n', 'ņ': 'n',
    'ø': 'o', 'ô': 'o', 'ö': 'o', 'ó': 'o', 'ő': 'o', 'õ': 'o', 'ò': 'o', 'ỗ': 'o',
    'ọ': 'o', 'ơ': 'o', 'ồ': 'o', 'ớ': 'o', 'ố': 'o',
    'ř': 'r',
    'ß': 'ss',
    'š': 's', 'ś': 's', 'ș': 's', 'ş': 's',
    'ț': 't', 'ť': 't',
    'þ': 'th',
    'ü': 'u', 'ů': 'u', 'ú': 'u', 'ù': 'u', 'ū': 'u', 'ű': 'u', 'ư': 'u', 'ứ': 'u',
    'ũ': 'u', 'ữ': 'u', 'ự': 'u', 'ừ': 'u',
    'ý': 'y', 'ỳ': 'y', 'ỹ': 'y',
    'ł': 'w',
    'ž': 'z', 'ż': 'z', 'ź': 'z',
}


EastAsianRanges = [
    # compatibility ideographs
    {"from": ord(u"\u3300"), "to": ord(u"\u33ff")},
    # compatibility ideographs
    {"from": ord(u"\ufe30"), "to": ord(u"\ufe4f")},
    # compatibility ideographs
    {"from": ord(u"\uf900"), "to": ord(u"\ufaff")},
    # compatibility ideographs
    {"from": ord(u"\U0002F800"), "to": ord(u"\U0002fa1f")},
    {"from": ord(u"\u30a0"), "to": ord(u"\u30ff")},          # Japanese Kana
    # cjk radicals supplement
    {"from": ord(u"\u2e80"), "to": ord(u"\u2eff")},
    {"from": ord(u"\u4e00"), "to": ord(u"\u9fff")},
    {"from": ord(u"\u3400"), "to": ord(u"\u4dbf")},
    {"from": ord(u"\U00020000"), "to": ord(u"\U0002a6df")},
    {"from": ord(u"\U0002a700"), "to": ord(u"\U0002b73f")},
    {"from": ord(u"\U0002b740"), "to": ord(u"\U0002b81f")},
    # included as of Unicode 8.0
    {"from": ord(u"\U0002b820"), "to": ord(u"\U0002ceaf")}
]


def sub_from(m, c):
    try:
        return m[c]
    except KeyError:
        return c


def is_eastasian(char):
    return any([range["from"] <= ord(char) <= range["to"] for range in EastAsianRanges])


def get_username(name):
    # Although the input string is UTF-8, the username must be ASCII.
    # Instead of dropping non-ASCII characters, just make look-alike
    # substitutions.

    # If the name has East Asian characters, store the unicode numbers
    if any(is_eastasian(c) for c in name):
        ret = 'jp-'+''.join([str(ord(c))for c in name])
    else:  # Name is latin
        name_lower = name.lower()
        name_ascii = map(lambda c: sub_from(SubstitutionMap, c), name_lower)
        name_alnum = filter(str.isalnum, name_ascii)
        ret = ''.join(name_alnum)
    return ret
