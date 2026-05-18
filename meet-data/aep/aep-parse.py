#!/usr/bin/python3
"""
AEP meet results parser.

Handles three formats from powerliftingspain.es:
  - DETAILED SCORESHEET (.xls binary, separate files per sex, for Master/Open)
  - OpenLifter XLS (single file with "Data Entry FEM/MAS" sheets)
  - OpenLifter OPL CSV (direct export, almost ready for OPL submission)

Usage:
  python3 aep-parse.py --url https://powerliftingspain.es/some-meet/
  python3 aep-parse.py --url https://powerliftingspain.es/some-meet/ --dry-run
"""

import argparse
import io
import os
import re

import pandas as pd
import requests
import xlrd
from bs4 import BeautifulSoup
from datetime import datetime


# ──────────────────────────────────────────────────────────────────────────────
# Constants
# ──────────────────────────────────────────────────────────────────────────────

WEIGHT_CLASSES_F = [47, 52, 57, 63, 69, 76, 84]
WEIGHT_CLASSES_M = [59, 66, 74, 83, 93, 105, 120]

EQUIPMENT_MAP = {
    'RAW': 'Raw',
    'SLEEVES': 'Sleeves',
    'RAW SLEEVES': 'Sleeves',
    'WRAPS': 'Wraps',
    'RAW WRAPS': 'Wraps',
    'SINGLE-PLY': 'Single-ply',
    'MULTI-PLY': 'Multi-ply',
}

# Maps various division strings (uppercased) to OPL standard division names
DIVISION_MAP = {
    'OPEN': 'Open',
    'JUNIOR': 'Junior',
    'JNR': 'Junior',
    'SUB-JUNIOR': 'Sub-Junior',
    'SUBJUNIOR': 'Sub-Junior',
    'TEEN': 'Teen',
    'TEEN 16-17': 'Teen 16-17',
    'TEEN 18-19': 'Teen 18-19',
    'MASTERS 1': 'M1', 'MASTERS1': 'M1', 'M1': 'M1',
    'MASTERS 2': 'M2', 'MASTERS2': 'M2', 'M2': 'M2',
    'MASTERS 3': 'M3', 'MASTERS3': 'M3', 'M3': 'M3',
    'MASTERS 4': 'M4', 'MASTERS4': 'M4', 'M4': 'M4',
    'SOI': '__SOI__',  # Adapted bench-press-only category; handled specially below
}

OPL_COLUMNS = [
    'Place', 'Name', 'Sex', 'BirthDate', 'Age', 'Division', 'Equipment',
    'BodyweightKg', 'WeightClassKg', 'Team',
    'Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Best3SquatKg',
    'Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Best3BenchKg',
    'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Best3DeadliftKg',
    'TotalKg', 'Event',
]


# ──────────────────────────────────────────────────────────────────────────────
# Utilities
# ──────────────────────────────────────────────────────────────────────────────

def switch_firstname_order_to_beginning(fullname: str) -> str:
    """
    Switches the first name from the end of the string to the beginning.
    Handles compound surnames (de, del, la, van, ...) and initials.
    """
    words_for_compounds = ['da', 'de', 'del', 'la', 'las', 'los', 'mac', 'mc',
                           'van', 'von', 'y', 'i', 'san', 'santa']
    fullname_words = fullname.split(' ')

    def is_compound_name(name_parts):
        return (len(name_parts) > 1 and
                all(p.lower() not in words_for_compounds for p in name_parts))

    processed_name = []
    prev_word = ""
    for word in fullname_words:
        if word.lower() in words_for_compounds:
            if processed_name:
                prev_word += processed_name.pop() + ' ' + word + ' '
            else:
                prev_word += word + ' '
        else:
            processed_name.append(
                prev_word.strip() + ' ' + word if prev_word else word)
            prev_word = ""

    if processed_name:
        if is_compound_name(processed_name[-1].split()):
            first_name = processed_name.pop(0)
            processed_name.append(first_name)

    processed_name_with_initials = []
    for part in processed_name:
        if len(part) == 2 and part[1] == '.':
            if processed_name_with_initials:
                processed_name_with_initials[-1] += ' ' + part
            else:
                processed_name_with_initials.append(part)
        else:
            processed_name_with_initials.append(part)

    if len(processed_name_with_initials) == 2:
        processed_name_with_initials = list(reversed(processed_name_with_initials))
    elif len(processed_name_with_initials) >= 3:
        for _ in range(len(processed_name_with_initials) - 2):
            processed_name_with_initials.insert(
                0, processed_name_with_initials.pop())

    return " ".join(processed_name_with_initials)


def compute_weight_class(bw: float, sex: str) -> str:
    limits = WEIGHT_CLASSES_F if sex == 'F' else WEIGHT_CLASSES_M
    for limit in limits:
        if bw <= limit:
            return str(limit)
    return str(limits[-1]) + '+'


def map_equipment(raw: str) -> str:
    return EQUIPMENT_MAP.get(raw.upper().strip(), raw)


def map_division(raw: str) -> str:
    key = raw.upper().strip()
    if key in DIVISION_MAP:
        return DIVISION_MAP[key]
    # Try partial match for strings like "Women's Raw Open", "Men's Raw AEP2"
    for pattern, result in DIVISION_MAP.items():
        if pattern in key:
            return result
    # AEP category codes (AEP1, AEP2, AEP3) → Open
    if re.search(r'\bAEP\d?\b', key):
        return 'Open'
    return raw.title()


def best_of(attempts: list) -> float:
    """Return the max of positive attempts, or NaN if none succeeded."""
    valid = [float(a) for a in attempts
             if pd.notna(a) and str(a).strip() not in ('', 'nan') and float(a) > 0]
    return max(valid) if valid else float('nan')


def master_division_from_birth_year(birth_year: int, meet_year: int) -> str:
    """IPF age-group rule: age = meet_year - birth_year (full calendar years)."""
    age = meet_year - birth_year
    if age < 40:
        return 'Open'
    elif age <= 49:
        return 'M1'
    elif age <= 59:
        return 'M2'
    elif age <= 69:
        return 'M3'
    else:
        return 'M4'


def parse_weight_class_str(raw: str) -> str:
    """Parse '-57kg', '120+kg', '-83kg' → '57', '120+', '83'."""
    raw = raw.strip()
    raw = re.sub(r'kg$', '', raw, flags=re.IGNORECASE).strip()
    raw = raw.lstrip('-')
    return raw


def is_weight_class_row(val: str) -> bool:
    return bool(re.match(r'^-?\d+\+?kg$', val.strip(), re.IGNORECASE))


def normalise_place(raw) -> str:
    s = str(raw).strip()
    if s in ('—', '--', 'DQ', 'DSQ'):
        return 'DQ'
    if s in ('NS', 'DNS'):
        return 'NS'
    try:
        return str(int(float(s)))
    except (ValueError, TypeError):
        return s


def clean_apostrophe(val: str) -> str:
    """Remove leading apostrophe used by Excel/OpenLifter to prevent auto-formatting."""
    return str(val).lstrip("'").strip()


MONTHS_ES = {
    'enero': 1, 'febrero': 2, 'marzo': 3, 'abril': 4,
    'mayo': 5, 'junio': 6, 'julio': 7, 'agosto': 8,
    'septiembre': 9, 'octubre': 10, 'noviembre': 11, 'diciembre': 12,
}


def parse_date_from_spanish(location: str, year: int) -> str:
    """Extract date from strings like 'Cantabria 22 de marzo', return 'YYYY-MM-DD'."""
    m = re.search(r'(\d{1,2})\s+de\s+(\w+)', location, re.IGNORECASE)
    if m and year:
        day = int(m.group(1))
        month = MONTHS_ES.get(m.group(2).lower())
        if month:
            return f"{year}-{month:02d}-{day:02d}"
    return ''


# ──────────────────────────────────────────────────────────────────────────────
# Format detection
# ──────────────────────────────────────────────────────────────────────────────

def detect_format(content: bytes, filename: str) -> str:
    """
    Returns one of:
      'OPL_CSV'            — OpenLifter direct OPL export
      'DATA_CSV'           — OpenLifter awards CSV (same structure, no OPL header)
      'OPENLIFTER_XLS'     — XLS with "Data Entry" sheets
      'DETAILED_SCORESHEET'— Binary XLS from AEP scoring software
      'UNKNOWN'
    """
    fname_lower = filename.lower()

    if fname_lower.endswith('.csv'):
        try:
            first_line = content[:80].decode('utf-8', errors='ignore')
            if 'OPL Format' in first_line:
                return 'OPL_CSV'
            # Check if header row looks like OpenLifter awards CSV
            if 'Place' in first_line and 'Name' in first_line and 'Sex' in first_line:
                return 'OPL_CSV'
            # Generic CSV with OpenLifter column names
            if 'Awards Division' in first_line or 'Body Weight' in first_line:
                return 'DATA_CSV'
        except Exception:
            pass
        return 'UNKNOWN'

    # XLS/XLSX
    try:
        xl = pd.ExcelFile(io.BytesIO(content))
        sheet_names_lower = [s.lower() for s in xl.sheet_names]
        if any('data entry' in s for s in sheet_names_lower):
            return 'OPENLIFTER_XLS'
    except Exception:
        pass

    try:
        wb = xlrd.open_workbook(file_contents=content)
        ws = wb.sheet_by_index(0)
        if ws.nrows > 2:
            row2_val = str(ws.row(2)[0].value).strip()
            if 'DETAILED SCORESHEET' in row2_val.upper():
                return 'DETAILED_SCORESHEET'
    except Exception:
        pass

    return 'UNKNOWN'


def infer_sex_from_filename(filename: str):
    fname = filename.lower()
    if any(w in fname for w in ['mujeres', 'mujer', 'fem', 'women', 'woman']):
        return 'F'
    if any(w in fname for w in ['hombres', 'hombre', 'mas', 'masc', 'men', 'man']):
        return 'M'
    return None


# ──────────────────────────────────────────────────────────────────────────────
# Format A: DETAILED SCORESHEET
# ──────────────────────────────────────────────────────────────────────────────

def parse_detailed_scoresheet_meetinfo(wb: xlrd.Book) -> dict:
    ws = wb.sheet_by_index(0)
    raw = str(ws.row(1)[0].value).strip()
    # "Meet Name, Town, Region (Country), DD.MM.YYYY"
    parts = [p.strip() for p in raw.split(',')]

    date = ''
    for part in reversed(parts):
        m = re.search(r'(\d{1,2})\.(\d{1,2})\.(\d{4})', part)
        if m:
            day, month, year = m.groups()
            date = f"{year}-{int(month):02d}-{int(day):02d}"
            break

    raw_name = parts[0] if parts else ''
    raw_name = re.sub(
        r'^AEP\s*\d+\s*(?:[–—-]\s*)?', '', raw_name, flags=re.IGNORECASE
    ).strip()
    raw_name = ' '.join(w.capitalize() if w.islower() else w for w in raw_name.split())
    town = parts[1].strip() if len(parts) > 2 else ''

    return {
        'Federation': 'AEP',
        'Date': date,
        'MeetCountry': 'Spain',
        'MeetState': '',
        'MeetTown': town,
        'MeetName': raw_name,
    }


def parse_detailed_scoresheet(content: bytes, sex: str):
    """
    Returns (entries_df, meet_data_dict).
    sex: 'M' or 'F' (determined from filename by caller).
    """
    wb = xlrd.open_workbook(file_contents=content, formatting_info=True)
    ws = wb.sheet_by_index(0)
    meet_data = parse_detailed_scoresheet_meetinfo(wb)

    def cell_value_signed(row_cells, col_idx):
        """Return float value, negated if cell has strikethrough formatting."""
        cell = row_cells[col_idx]
        val = cell.value
        if val is None or str(val).strip() == '':
            return float('nan')
        try:
            fval = float(val)
        except (ValueError, TypeError):
            # Strip record/annotation suffixes like "151.0-w2", "130.0-r"
            m = re.match(r'^-?\d+\.?\d*', str(val).strip())
            if m:
                fval = float(m.group())
            else:
                return float('nan')
        if cell.xf_index is not None:
            font = wb.font_list[wb.xf_list[cell.xf_index].font_index]
            if font.struck_out:
                return -abs(fval)
        return fval

    def parse_dob(raw: str):
        """Returns (birth_date_str_or_None, birth_year_int_or_None)."""
        raw = raw.strip()
        if re.match(r'^\d{2}\.\d{2}\.\d{2}$', raw):
            day, month, yr2 = raw.split('.')
            year = (1900 if int(yr2) > 25 else 2000) + int(yr2)
            # "01.01.YY" is a year-only placeholder — treat as birth year only
            if int(day) == 1 and int(month) == 1:
                return None, year
            return f"{year:04d}-{int(month):02d}-{int(day):02d}", None
        # xlrd returns numeric cells as floats; "1983" comes back as "1983.0"
        m = re.match(r'^(\d{4})(?:\.0)?$', raw)
        if m:
            return None, int(m.group(1))
        return None, None

    current_division = 'Open'
    current_weight_class = None
    skip_section = False
    entries = []

    for row_idx in range(4, ws.nrows):
        row = ws.row(row_idx)
        val0 = str(row[0].value).strip()
        val1 = str(row[1].value).strip()

        # Skip fully empty rows
        if not val0 and not val1:
            continue

        # Classify row when col1 is empty (section headers)
        if val0 and not val1:
            val0_lower = val0.lower()
            if val0_lower in ('rnk',):
                continue
            if 'abbreviations' in val0_lower:
                break  # No more athlete data after this
            if 'team' in val0_lower and ('point' in val0_lower or '(' in val0):
                skip_section = True
                continue
            if 'best lifter' in val0_lower:
                skip_section = True
                continue
            if is_weight_class_row(val0):
                skip_section = False
                current_weight_class = parse_weight_class_str(val0)
                continue
            # Division header
            skip_section = False
            current_division = map_division(val0)
            current_weight_class = None
            continue

        # '__SOI__' is used to flag the SOI section internally.
        # Athletes in SOI (adapted bench-only) get Place='G' and their master
        # division computed from birth year (IPF rule: meet_year - birth_year).
        in_soi = (current_division == '__SOI__')

        if skip_section:
            continue

        # Data row: col0 is place, col1 is name
        if not val1:
            continue

        place = 'G' if in_soi else normalise_place(val0)

        # Name: strip "(AI)*" notation for international athletes
        name_raw = val1
        name_raw = re.sub(r'\s*\(AI\)\*?\s*', ' ', name_raw).strip()
        # Some cells use "LAST, First" — remove the separator comma
        # Some cells have trailing periods (annotation artifacts) — remove them
        name_raw = name_raw.replace(',', ' ').split()
        name_raw = ' '.join(w.rstrip('.') for w in name_raw)
        name = switch_firstname_order_to_beginning(name_raw)

        dob_raw = str(row[2].value).strip()
        birth_date, birth_year = parse_dob(dob_raw)

        if in_soi:
            meet_year = int(meet_data['Date'][:4]) if meet_data.get('Date') else 0
            division_out = (master_division_from_birth_year(birth_year, meet_year)
                            if birth_year and meet_year else 'Open')
        else:
            division_out = current_division

        try:
            bw = float(row[4].value)
        except (ValueError, TypeError):
            bw = float('nan')

        s1 = cell_value_signed(row, 7)
        s2 = cell_value_signed(row, 8)
        s3 = cell_value_signed(row, 9)
        b1 = cell_value_signed(row, 11)
        b2 = cell_value_signed(row, 12)
        b3 = cell_value_signed(row, 13)
        d1 = cell_value_signed(row, 15)
        d2 = cell_value_signed(row, 16)
        d3 = cell_value_signed(row, 17)

        try:
            total_raw = str(row[19].value).strip()
            m_total = re.match(r'^-?\d+\.?\d*', total_raw)
            total = float(m_total.group()) if m_total else float('nan')
        except (ValueError, TypeError):
            total = float('nan')

        all_squats_empty = all(pd.isna(x) for x in [s1, s2, s3])
        all_deads_empty = all(pd.isna(x) for x in [d1, d2, d3])
        if all_squats_empty and all_deads_empty:
            event = 'B'
        elif all_squats_empty:
            event = 'D'
        else:
            event = 'SBD'

        # Source XLS may leave Total empty for bench-only or other athletes;
        # compute it from the best lifts when possible.
        if pd.isna(total):
            best_s = best_of([s1, s2, s3])
            best_b = best_of([b1, b2, b3])
            best_d = best_of([d1, d2, d3])
            if event == 'B' and not pd.isna(best_b):
                total = best_b
            elif event == 'SBD' and not any(
                pd.isna(x) for x in [best_s, best_b, best_d]
            ):
                total = best_s + best_b + best_d

        wc = current_weight_class if current_weight_class else (
            compute_weight_class(bw, sex) if not pd.isna(bw) else '')

        entry = {
            'Place': place,
            'Name': name,
            'Sex': sex,
            'BirthDate': birth_date or '',
            'Age': '',
            'Division': division_out,
            'Equipment': 'Raw',
            'BodyweightKg': bw,
            'WeightClassKg': wc,
            'Squat1Kg': s1, 'Squat2Kg': s2, 'Squat3Kg': s3,
            'Best3SquatKg': best_of([s1, s2, s3]),
            'Bench1Kg': b1, 'Bench2Kg': b2, 'Bench3Kg': b3,
            'Best3BenchKg': best_of([b1, b2, b3]),
            'Deadlift1Kg': d1, 'Deadlift2Kg': d2, 'Deadlift3Kg': d3,
            'Best3DeadliftKg': best_of([d1, d2, d3]),
            'TotalKg': total,
            'Event': event,
        }
        if birth_year:
            entry['BirthYear'] = birth_year
        entries.append(entry)

    return pd.DataFrame(entries), meet_data


# ──────────────────────────────────────────────────────────────────────────────
# Format B: OpenLifter XLS
# ──────────────────────────────────────────────────────────────────────────────

def _extract_openlifter_meet_info(content: bytes, sheet_names: list) -> dict:
    """
    Extract meet name, location, and date from an OpenLifter XLS file.

    The date is stored in the formatted results sheets (e.g. "AEP-2 FEM") at the
    row labelled "Rev.", not in the "Data Entry Campeonato" sheet. Meet name and
    location come from "Data Entry Campeonato" or from the same results sheet.
    """
    meet_data = {'Federation': 'AEP', 'MeetCountry': 'Spain',
                 'MeetState': '', 'MeetTown': '', 'MeetName': '', 'Date': ''}

    location_raw = ''

    # Try "Data Entry Campeonato" for name and location
    for sheet in sheet_names:
        if 'campeonato' in sheet.lower():
            df_m = pd.read_excel(io.BytesIO(content), sheet_name=sheet,
                                 header=None, dtype=str)
            raw_name = str(df_m.iloc[0, 1]).strip()
            # Strip "AEP N –" / "AEP N -" prefix from meet name
            raw_name = re.sub(r'^AEP\s*\d+\s*[–—-]\s*', '', raw_name,
                              flags=re.IGNORECASE).strip()
            # Title-case any word that is entirely lowercase
            raw_name = ' '.join(
                w.capitalize() if w.islower() else w for w in raw_name.split()
            )
            meet_data['MeetName'] = raw_name
            location_raw = str(df_m.iloc[2, 1]).strip()
            loc_parts = [p.strip() for p in location_raw.split(',')]
            meet_data['MeetTown'] = loc_parts[0]
            break

    # Find revision date from formatted results sheet ("Rev." row, col 2)
    rev_year = None
    for sheet in sheet_names:
        sl = sheet.lower()
        if 'data entry' in sl or 'campeonato' in sl:
            continue
        df_r = pd.read_excel(io.BytesIO(content), sheet_name=sheet, header=None)
        for _, row in df_r.iterrows():
            cell1 = str(row.iloc[1]).strip() if len(row) > 1 else ''
            cell2 = row.iloc[2] if len(row) > 2 else None
            if cell1.lower() == 'rev.' and cell2 is not None and str(cell2) != 'NaT':
                try:
                    if hasattr(cell2, 'strftime'):
                        rev_year = cell2.year
                    else:
                        rev_year = int(str(cell2)[:4])
                except Exception:
                    pass
                break
        if rev_year:
            break

    # Prefer actual meet date from location string ("22 de marzo") over revision date
    if location_raw and rev_year:
        date_from_loc = parse_date_from_spanish(location_raw, rev_year)
        if date_from_loc:
            meet_data['Date'] = date_from_loc

    return meet_data


def parse_openlifter_xls(content: bytes):
    xl = pd.ExcelFile(io.BytesIO(content))
    sheet_names = xl.sheet_names

    meet_data = _extract_openlifter_meet_info(content, sheet_names)

    # Process Data Entry FEM / MAS sheets (skip Campeonato)
    dfs = []
    for sheet in sheet_names:
        sl = sheet.lower()
        if 'data entry' not in sl or 'campeonato' in sl:
            continue
        sex = 'F' if 'fem' in sl else 'M'
        df = pd.read_excel(io.BytesIO(content), sheet_name=sheet, header=0)
        dfs.append(_process_openlifter_df(df, sex))

    combined = pd.concat(dfs, ignore_index=True) if dfs else pd.DataFrame()
    return combined, meet_data


def _process_openlifter_df(df: pd.DataFrame, sex: str) -> pd.DataFrame:
    entries = []
    for _, row in df.iterrows():
        name_raw = str(row.get('Name', '')).strip()
        if not name_raw or name_raw == 'nan':
            continue
        if name_raw == name_raw.upper() and len(name_raw) > 2:
            name_raw = name_raw.title()
        name = switch_firstname_order_to_beginning(name_raw)

        place = normalise_place(row.get('Place', ''))
        bw = row.get('Body Weight (kg)', float('nan'))
        wc = str(row.get('Weight Class', '')).strip()
        div = map_division(str(row.get('Awards Division', 'Open')))
        equip = map_equipment(str(row.get('Raw/Equipped', 'RAW')))
        team = str(row.get('Team', '')).strip()
        age = row.get('Exact Age', '')

        birth_date = ''
        bd_raw = str(row.get('Birth Date', '')).strip()
        if '/' in bd_raw:
            try:
                bd = datetime.strptime(bd_raw, '%m/%d/%Y')
                birth_date = bd.strftime('%Y-%m-%d')
            except ValueError:
                pass
        elif re.match(r'\d{4}-\d{2}-\d{2}', bd_raw):
            birth_date = bd_raw[:10]
        # OpenLifter uses YYYY-01-01 as placeholder when only the year is known
        if birth_date.endswith('-01-01'):
            birth_date = ''

        def g(col):
            v = row.get(col, float('nan'))
            return float('nan') if pd.isna(v) else float(v)

        s1, s2, s3 = g('Squat 1'), g('Squat 2'), g('Squat 3')
        b1, b2, b3 = g('Bench 1'), g('Bench 2'), g('Bench 3')
        d1, d2, d3 = g('Deadlift 1'), g('Deadlift 2'), g('Deadlift 3')
        total = g('Total')

        all_squats_empty = all(pd.isna(x) for x in [s1, s2, s3])
        event = 'B' if all_squats_empty else 'SBD'

        # If event is SBD but bench or deadlift data is entirely missing,
        # the athlete bombed out and should be DQ with no total.
        if event == 'SBD':
            if (all(pd.isna(x) for x in [b1, b2, b3])
                    or all(pd.isna(x) for x in [d1, d2, d3])):
                place = 'DQ'
                total = float('nan')

        entries.append({
            'Place': place,
            'Name': name,
            'Sex': sex,
            'BirthDate': birth_date,
            'Age': '' if pd.isna(age) else int(age),
            'Division': div,
            'Equipment': equip,
            'BodyweightKg': bw,
            'WeightClassKg': wc,
            'Team': team,
            'Squat1Kg': s1, 'Squat2Kg': s2, 'Squat3Kg': s3,
            'Best3SquatKg': g('Best Squat'),
            'Bench1Kg': b1, 'Bench2Kg': b2, 'Bench3Kg': b3,
            'Best3BenchKg': g('Best Bench'),
            'Deadlift1Kg': d1, 'Deadlift2Kg': d2, 'Deadlift3Kg': d3,
            'Best3DeadliftKg': g('Best Deadlift'),
            'TotalKg': total,
            'Event': event,
        })
    return pd.DataFrame(entries)


# ──────────────────────────────────────────────────────────────────────────────
# Format C: OpenLifter OPL CSV
# ──────────────────────────────────────────────────────────────────────────────

def parse_opl_csv(content: bytes):
    text = content.decode('utf-8-sig', errors='replace')
    lines = text.splitlines()

    meet_data = {'Federation': 'AEP', 'MeetCountry': 'Spain',
                 'MeetState': '', 'MeetTown': '', 'MeetName': '', 'Date': ''}

    # Find the meet info row (Federation,Date,MeetCountry,...)
    # and the data header row (Place,Name,Sex,...)
    meet_header_idx = None
    data_header_idx = None
    for i, line in enumerate(lines):
        if line.startswith('Federation,'):
            meet_header_idx = i + 1
        if line.startswith('Place,') and 'Name' in line:
            data_header_idx = i
            break

    if meet_header_idx is not None and meet_header_idx < len(lines):
        parts = lines[meet_header_idx].split(',')
        meet_data['Federation'] = clean_apostrophe(parts[0]) if parts else 'AEP'
        meet_data['Date'] = clean_apostrophe(parts[1]) if len(parts) > 1 else ''
        meet_data['MeetCountry'] = 'Spain'
        meet_data['MeetState'] = ''
        meet_data['MeetTown'] = clean_apostrophe(parts[4]) if len(parts) > 4 else ''
        raw_name = clean_apostrophe(parts[5]) if len(parts) > 5 else ''
        raw_name = re.sub(r'^AEP\s*\d+\s*[–—-]\s*', '', raw_name,
                          flags=re.IGNORECASE).strip()
        raw_name = ' '.join(
            w.capitalize() if w.islower() else w for w in raw_name.split()
        )
        meet_data['MeetName'] = raw_name

    if data_header_idx is None:
        raise ValueError("Could not find data header row in OPL CSV")

    df = pd.read_csv(io.StringIO('\n'.join(lines[data_header_idx:])),
                     dtype=str, keep_default_na=False)

    entries = []
    for _, row in df.iterrows():
        place_raw = clean_apostrophe(row.get('Place', ''))
        if place_raw in ('NS', 'DNS'):
            continue  # Skip no-show athletes

        place = normalise_place(place_raw)
        name_raw = clean_apostrophe(row.get('Name', '')).strip()
        if not name_raw:
            continue
        name = switch_firstname_order_to_beginning(name_raw)

        sex = clean_apostrophe(row.get('Sex', '')).strip()
        birth_date = clean_apostrophe(row.get('BirthDate', '')).strip()
        # OpenLifter uses YYYY-01-01 as placeholder when only birth year is known
        if birth_date.endswith('-01-01'):
            birth_date = ''
        age = clean_apostrophe(row.get('Age', '')).strip()
        equip = map_equipment(clean_apostrophe(row.get('Equipment', 'RAW')))
        div = map_division(clean_apostrophe(row.get('Division', 'Open')))
        bw = clean_apostrophe(row.get('BodyweightKg', ''))
        wc = clean_apostrophe(row.get('WeightClassKg', ''))
        team = clean_apostrophe(row.get('Team', '')).strip()
        event = clean_apostrophe(row.get('Event', 'SBD')).strip()

        def gf(col):
            v = clean_apostrophe(row.get(col, ''))
            try:
                return float(v)
            except (ValueError, TypeError):
                return float('nan')

        s1, s2, s3 = gf('Squat1Kg'), gf('Squat2Kg'), gf('Squat3Kg')
        b1, b2, b3 = gf('Bench1Kg'), gf('Bench2Kg'), gf('Bench3Kg')
        d1, d2, d3 = gf('Deadlift1Kg'), gf('Deadlift2Kg'), gf('Deadlift3Kg')
        best_s = gf('Best3SquatKg')
        best_b = gf('Best3BenchKg')
        best_d = gf('Best3DeadliftKg')
        total = gf('TotalKg')

        try:
            bw_f = float(bw)
        except (ValueError, TypeError):
            bw_f = float('nan')

        entries.append({
            'Place': place,
            'Name': name,
            'Sex': sex,
            'BirthDate': birth_date,
            'Age': age,
            'Division': div,
            'Equipment': equip,
            'BodyweightKg': bw_f,
            'WeightClassKg': wc,
            'Team': team,
            'Squat1Kg': s1, 'Squat2Kg': s2, 'Squat3Kg': s3,
            'Best3SquatKg': best_s,
            'Bench1Kg': b1, 'Bench2Kg': b2, 'Bench3Kg': b3,
            'Best3BenchKg': best_b,
            'Deadlift1Kg': d1, 'Deadlift2Kg': d2, 'Deadlift3Kg': d3,
            'Best3DeadliftKg': best_d,
            'TotalKg': total,
            'Event': event,
        })

    return pd.DataFrame(entries), meet_data


# ──────────────────────────────────────────────────────────────────────────────
# Format C variant: OpenLifter awards/results CSV (no OPL header, full columns)
# ──────────────────────────────────────────────────────────────────────────────

def parse_data_csv(content: bytes, page_url: str):
    """
    Parse an OpenLifter awards CSV (same structure as Data Entry XLS sheets).
    Meet info is not embedded; town/name/date must come from the page or are left blank.
    """
    df = pd.read_csv(io.BytesIO(content), dtype=str, keep_default_na=False)

    # Detect sex from 'Gender' column if present, else from 'Sex'
    sex_col = 'Gender' if 'Gender' in df.columns else 'Sex'

    meet_data = {'Federation': 'AEP', 'MeetCountry': 'Spain',
                 'MeetState': '', 'MeetTown': '', 'MeetName': '', 'Date': ''}

    entries = []
    for _, row in df.iterrows():
        place = normalise_place(row.get('Place', ''))
        if place in ('NS', 'DNS'):
            continue
        name_raw = row.get('Name', '').strip()
        if not name_raw:
            continue
        name = switch_firstname_order_to_beginning(name_raw)

        sex_raw = row.get(sex_col, '').strip().upper()
        if sex_raw in ('F', 'FEMALE'):
            sex = 'F'
        elif sex_raw in ('M', 'MALE'):
            sex = 'M'
        else:
            sex = sex_raw

        bd_raw = row.get('Birth Date', '').strip()
        birth_date = ''
        if '/' in bd_raw:
            try:
                bd = datetime.strptime(bd_raw, '%m/%d/%Y')
                birth_date = bd.strftime('%Y-%m-%d')
            except ValueError:
                pass

        age = row.get('Exact Age', row.get('Age', '')).strip()
        equip = map_equipment(
            row.get('Raw/Equipped', row.get('Equipment', 'RAW')).strip())
        div = map_division(
            row.get('Awards Division', row.get('Division', 'Open')).strip())
        bw_str = row.get('Body Weight (kg)', row.get('BodyweightKg', '')).strip()
        wc = row.get('Weight Class', row.get('WeightClassKg', '')).strip()
        team = row.get('Team', '').strip()

        def gf(col1, col2=None):
            v = row.get(col1, row.get(col2, '') if col2 else '').strip()
            try:
                return float(v)
            except (ValueError, TypeError):
                return float('nan')

        s1 = gf('Squat 1', 'Squat1Kg')
        s2 = gf('Squat 2', 'Squat2Kg')
        s3 = gf('Squat 3', 'Squat3Kg')
        b1 = gf('Bench 1', 'Bench1Kg')
        b2 = gf('Bench 2', 'Bench2Kg')
        b3 = gf('Bench 3', 'Bench3Kg')
        d1 = gf('Deadlift 1', 'Deadlift1Kg')
        d2 = gf('Deadlift 2', 'Deadlift2Kg')
        d3 = gf('Deadlift 3', 'Deadlift3Kg')
        total = gf('Total', 'TotalKg')
        event_raw = row.get('Event', '').strip()
        event = event_raw if event_raw else (
            'B' if all(pd.isna(x) for x in [s1, s2, s3]) else 'SBD')

        try:
            bw_f = float(bw_str)
        except (ValueError, TypeError):
            bw_f = float('nan')

        entries.append({
            'Place': place,
            'Name': name,
            'Sex': sex,
            'BirthDate': birth_date,
            'Age': age,
            'Division': div,
            'Equipment': equip,
            'BodyweightKg': bw_f,
            'WeightClassKg': wc,
            'Team': team,
            'Squat1Kg': s1, 'Squat2Kg': s2, 'Squat3Kg': s3,
            'Best3SquatKg': gf('Best Squat', 'Best3SquatKg'),
            'Bench1Kg': b1, 'Bench2Kg': b2, 'Bench3Kg': b3,
            'Best3BenchKg': gf('Best Bench', 'Best3BenchKg'),
            'Deadlift1Kg': d1, 'Deadlift2Kg': d2, 'Deadlift3Kg': d3,
            'Best3DeadliftKg': gf('Best Deadlift', 'Best3DeadliftKg'),
            'TotalKg': total,
            'Event': event,
        })

    return pd.DataFrame(entries), meet_data


# ──────────────────────────────────────────────────────────────────────────────
# File scraping
# ──────────────────────────────────────────────────────────────────────────────

def get_result_files(page_url: str) -> list:
    """
    Scrape the meet page for result files (XLS/XLSX/CSV).
    Returns list of (filename, content_bytes) tuples.
    """
    r = requests.get(page_url, timeout=15)
    r.raise_for_status()
    soup = BeautifulSoup(r.text, 'html.parser')

    seen_urls = set()
    file_urls = []
    for a in soup.find_all('a'):
        href = a.get('href', '')
        lower = href.lower()
        if lower.endswith(('.xls', '.xlsx', '.csv')) and href not in seen_urls:
            seen_urls.add(href)
            file_urls.append(href)

    results = []
    for url in file_urls:
        resp = requests.get(url, timeout=30)
        resp.raise_for_status()
        fname = url.split('/')[-1]
        results.append((fname, resp.content))

    return results


# ──────────────────────────────────────────────────────────────────────────────
# Output writing
# ──────────────────────────────────────────────────────────────────────────────

def get_newdir_path(year_2digit: str) -> str:
    script_path = os.path.abspath(os.path.dirname(__file__))
    dirs = [d for d in os.listdir(script_path)
            if os.path.isdir(os.path.join(script_path, d))
            and d.isdigit() and d.startswith(year_2digit)]
    if not dirs:
        raise ValueError(
            f"No existing directories found for year prefix '{year_2digit}'")
    return os.path.join(script_path, str(int(sorted(dirs)[-1]) + 1))


def write_outputs(entries_df: pd.DataFrame, meet_data: dict,
                  page_url: str, dry_run: bool = False):
    year_2d = meet_data['Date'][:4][-2:]
    dirpath = get_newdir_path(year_2d)

    if dry_run:
        print(f"[DRY RUN] Would create directory: {dirpath}")
        print("\nMeet data:")
        for k, v in meet_data.items():
            print(f"  {k}: {v}")
        print(f"\nEntries ({len(entries_df)} rows):")
        pd.set_option('display.max_columns', None)
        pd.set_option('display.width', 200)
        print(entries_df.to_string(index=False, float_format=lambda x: f'{x:g}'))
        return

    os.makedirs(dirpath)

    entries_df.to_csv(
        os.path.join(dirpath, 'entries.csv'),
        index=False,
        float_format='%g',
    )

    meet_cols = [
        'Federation', 'Date', 'MeetCountry', 'MeetState', 'MeetTown', 'MeetName']
    with open(os.path.join(dirpath, 'meet.csv'), 'w', encoding='utf-8') as f:
        f.write(','.join(meet_cols) + '\n')
        f.write(','.join(str(meet_data.get(c, '')) for c in meet_cols) + '\n')

    with open(os.path.join(dirpath, 'URL'), 'w', encoding='utf-8') as f:
        f.write(page_url + '\n')

    print(f"Created: {dirpath}")


# ──────────────────────────────────────────────────────────────────────────────
# Main
# ──────────────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description='Parse AEP meet results from powerliftingspain.es')
    parser.add_argument('--url', required=True,
                        help='Meet page URL (e.g. https://powerliftingspain.es/...)')
    parser.add_argument('--dry-run', action='store_true',
                        help='Print parsed data without creating files')
    args = parser.parse_args()

    print(f"Scraping: {args.url}")
    files = get_result_files(args.url)

    if not files:
        print("No XLS/XLSX/CSV files found on the page.")
        return

    all_entries = []
    meet_data = None

    for fname, content in files:
        fmt = detect_format(content, fname)
        print(f"  {fname} → {fmt}")

        if fmt == 'DETAILED_SCORESHEET':
            sex = infer_sex_from_filename(fname)
            if sex is None:
                print(f"    WARNING: cannot determine sex from '{fname}' — skipping")
                continue
            df, md = parse_detailed_scoresheet(content, sex)
            all_entries.append(df)
            if meet_data is None:
                meet_data = md

        elif fmt == 'OPENLIFTER_XLS':
            df, md = parse_openlifter_xls(content)
            all_entries.append(df)
            meet_data = md

        elif fmt == 'OPL_CSV':
            df, md = parse_opl_csv(content)
            all_entries.append(df)
            meet_data = md

        elif fmt == 'DATA_CSV':
            df, md = parse_data_csv(content, args.url)
            all_entries.append(df)
            if meet_data is None:
                meet_data = md

        else:
            print("    WARNING: unrecognised format — skipping")

    if not all_entries:
        print("No entries parsed.")
        return

    combined = pd.concat(all_entries, ignore_index=True)

    if not meet_data or not meet_data.get('Date'):
        print("WARNING: meet date not found — cannot determine output directory.")
        print("Entries preview:")
        print(combined.to_string(index=False))
        return

    write_outputs(combined, meet_data, args.url, dry_run=args.dry_run)


if __name__ == '__main__':
    main()
