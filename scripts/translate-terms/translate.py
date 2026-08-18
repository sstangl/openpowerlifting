import sys
import json
import re

from pathlib import Path

# Translate the technical terms from another language into English
# for the input file, and write the translated terms into the output file.
# This is intended for use with original.csv files, to enable more comprehensible
# per-fed parsers or more feasible manual work.

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print(
            f"Usage: {sys.argv[0]} languagecode originalfile translatedfile",
            file=sys.stderr
        )
        sys.exit(1)
    [lang_code, original_path_str, translated_path_str] = sys.argv[1:]
    lang_code_path = Path(sys.path[0]) / \
        Path("language-data") / \
        Path(f"{lang_code}.json")
    if not lang_code_path.exists:
        print(f"Unable to load language data for code {lang_code}")
        sys.exit(1)
    with open(lang_code_path, "rt") as lang_f:
        lang_dict = json.load(lang_f)
    with open(original_path_str, "rt") as orig_f:
        translatable_words = set()
        orig_str = orig_f.read()
        for orig_word_match in re.finditer(r'\w+', orig_str):
            orig_word = orig_word_match[0]
            if orig_word in lang_dict:
                translatable_words.add(orig_word)
    if len(translatable_words) < 1:
        print(f"Nothing translatable in language {lang_code}", file=sys.stderr)
        sys.exit(1)
    output_str = orig_str
    with open(translated_path_str, "wt") as output_f:
        for translatable_word in translatable_words:
            translated_word = lang_dict[translatable_word]
            output_str = re.sub(
                f'(\\W+){translatable_word}(\\W+)',
                f"\\1{translated_word}\\2",
                output_str,
                flags=re.MULTILINE
            )
        output_f.write(output_str) 