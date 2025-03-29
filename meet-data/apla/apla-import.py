#!/usr/bin/env python3

import os
import shutil
import subprocess
import sys
from datetime import datetime

BASE_DIR = os.path.dirname(os.path.realpath(__file__))
OPL_DATA_ROOT = os.path.dirname(os.path.dirname(BASE_DIR))
SCRIPTS_DIR = os.path.join(OPL_DATA_ROOT, "scripts")

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(SCRIPTS_DIR)
    from oplcsv import Csv


def clean_meet_name(name, year):
    """
    Standardize the meet name by removing year and federation.
    """
    # Remove the year if it appears anywhere
    name = name.replace(str(year), "").replace(str(year)[2:], "")

    # Remove federation references if they appear
    name = name.replace("APA", "").replace("APLA", "").replace("IPF", "")

    # Clean up extra spaces and standardize
    name = " ".join(name.split())

    return name.strip()


def process_meets(import_dir, base_dir):
    """
    Process all meets in the import directory.
    """
    try:
        processed_dirs = []
        failed_meets = []

        for filename in os.listdir(import_dir):
            if not filename.endswith(".opl.csv"):
                continue

            input_file = os.path.join(import_dir, filename)
            meet_number = find_next_meet_number(base_dir)
            meet_dir = os.path.join(base_dir, meet_number)
            os.makedirs(meet_dir, exist_ok=True)

            try:
                convert_meet_csv(input_file, meet_dir)
                processed_dirs.append(meet_dir)
                print(f"Processed meet {filename} -> {meet_number}/")
            except Exception as e:
                print(f"Error processing {filename}: {str(e)}", file=sys.stderr)
                failed_meets.append((filename, str(e)))
                # Remove the directory if processing failed
                if os.path.exists(meet_dir):
                    shutil.rmtree(meet_dir)
                continue

        if failed_meets:
            print("\nThe following meets failed to process:")
            for meet, error in failed_meets:
                print(f"- {meet}: {error}")

        return processed_dirs

    except Exception as e:
        print(f"Critical error in process_meets: {str(e)}")
        raise


def parse_args():
    """
    Handle command line arguments for the meet import directory.
    """
    if len(sys.argv) != 2:
        print("Usage: python apla-import.py <import_directory>")
        sys.exit(1)
    return sys.argv[1]


def find_next_meet_number(base_dir):
    """
    Find the next available meet number for the current year.
    """
    current_year = str(datetime.now().year)[2:]  # Get last 2 digits of current year
    existing_dirs = [
        d
        for d in os.listdir(base_dir)
        if d.startswith(current_year) and len(d) == 4 and d[2:].isdigit()
    ]

    if not existing_dirs:
        return f"{current_year}01"

    last_number = max(int(d[2:]) for d in existing_dirs)
    return f"{current_year}{last_number + 1:02d}"


def run_dos2unix(filepath):
    """
    Run dos2unix on a file.
    """
    try:
        subprocess.run(["dos2unix", filepath], check=True, capture_output=True)
    except subprocess.CalledProcessError as e:
        print(f"Error: dos2unix failed for {filepath}: {e}", file=sys.stderr)


def map_equipment(equipment_str):
    """
    Map LiftingCast equipment strings to OPL format.
    """
    equipment_str = equipment_str.lower().replace(" ", "")
    if equipment_str in {"raw", "classic"}:
        return "Raw"
    elif "single" in equipment_str or "eq" in equipment_str:
        return "Single-ply"
    print(f"Error: Could not detect equipment from '{equipment_str}', defaulted to Raw")
    return "Raw"


def get_division(division_str):
    """
    Get standardized division from LiftingCast format.
    """
    # Clean the input string
    div_clean = (
        division_str.lower()
        .replace("'", "")
        .replace("-", "")
        .replace(" ", "")
        .replace("womens", "")
        .replace("women", "")
        .replace("woman", "")
        .replace("female", "")
        .replace("males", "")
        .replace("male", "")
        .replace("mens", "")
        .replace("men", "")
        .replace("man", "")
    )

    # Check for Sub-Junior
    if any(term in div_clean for term in ["subjunior", "subjnr", "subj", "sub", "sj"]):
        return "Sub-Junior"

    # Check for Junior
    if any(term in div_clean for term in ["junior", "jnr", "jr", "jnior", "j"]):
        return "Junior"

    # Check for Masters with roman numerals or digits
    if (
        "master" in div_clean
        or "m" == div_clean
        or "m" in div_clean
        and any(c.isdigit() for c in div_clean)
    ):
        # Try to find a digit or roman numeral
        if "iv" in div_clean:
            return "Masters 4"
        elif "iii" in div_clean:
            return "Masters 3"
        elif "ii" in div_clean:
            return "Masters 2"
        elif "i" in div_clean:
            return "Masters 1"
        elif "4" in div_clean:
            return "Masters 4"
        elif "3" in div_clean:
            return "Masters 3"
        elif "2" in div_clean:
            return "Masters 2"
        elif "1" in div_clean:
            return "Masters 1"
        # If just "masters" or "master" with no number, default to Open
        else:
            print(f"Error: No number found in '{division_str}', defaulted to Open")
            return "Open"

    # Check for Open
    if "open" in div_clean:
        return "Open"

    # If no match found for any division, return Open
    print(
        f"Error: Could not identify division from '{division_str}', defaulted to Open"
    )
    return "Open"


def convert_meet_csv(input_file, output_dir):
    """
    Convert LiftingCast CSV to OpenPowerlifting format.
    """
    try:
        # Read input file and clean up formatting
        csv_input = Csv(input_file)
        rows = [[cell.strip('"') for cell in row] for row in csv_input.rows]

        # Find the meet info and entries sections
        try:
            meet_info_start = next(
                i for i, row in enumerate(rows) if "Federation" in row[0]
            )
            entries_start = next(i for i, row in enumerate(rows) if "Place" in row[0])
        except StopIteration:
            raise ValueError("Could not find required headers in input file")

        # Process meet info
        meet_info = {
            "Federation": "APLA",
            "Date": "",
            "MeetCountry": "Australia",
            "MeetState": "",
            "MeetTown": "",
            "MeetName": "",
        }

        # Extract meet info from the input
        meet_row = rows[meet_info_start + 1]
        if len(meet_row) >= 6:
            meet_info["Date"] = meet_row[1] if meet_row[1] else ""
            meet_info["MeetState"] = meet_row[3] if meet_row[3] else ""
            meet_info["MeetTown"] = meet_row[4] if meet_row[4] else ""
            year = meet_row[1].split("-")[0] if meet_row[1] else datetime.now().year
            meet_info["MeetName"] = (
                clean_meet_name(meet_row[5], year) if meet_row[5] else ""
            )

        # Write meet.csv
        meet_csv = Csv()
        meet_csv.fieldnames = list(meet_info.keys())
        meet_csv.rows.append(list(meet_info.values()))
        meet_csv.write_filename(os.path.join(output_dir, "meet.csv"))

        # Process entries
        entries_headers = rows[entries_start]
        entries_data = rows[entries_start + 1:]

        # Define required columns (excluding Lift4 columns)
        required_columns = [
            "Place",
            "Name",
            "BirthDate",
            "Sex",
            "BirthYear",
            "Age",
            "Country",
            "State",
            "Equipment",
            "Division",
            "BodyweightKg",
            "WeightClassKg",
            "Squat1Kg",
            "Squat2Kg",
            "Squat3Kg",
            "Best3SquatKg",
            "Bench1Kg",
            "Bench2Kg",
            "Bench3Kg",
            "Best3BenchKg",
            "Deadlift1Kg",
            "Deadlift2Kg",
            "Deadlift3Kg",
            "Best3DeadliftKg",
            "TotalKg",
            "Event",
        ]

        # Map input columns to output columns
        column_indices = {}
        for col in required_columns:
            try:
                if col in entries_headers:
                    column_indices[col] = entries_headers.index(col)
            except ValueError:
                print(f"Error: Column {col} not found in input file")

        # Write entries.csv
        entries_csv = Csv()
        entries_csv.fieldnames = required_columns

        for row in entries_data:
            output_row = []
            for col in required_columns:
                if col == "Country":
                    output_row.append("Australia")
                elif col == "Equipment" and col in column_indices:
                    raw_equipment = row[column_indices[col]]
                    output_row.append(map_equipment(raw_equipment))
                elif col == "Division" and col in column_indices:
                    raw_division = row[column_indices[col]]
                    output_row.append(get_division(raw_division))
                elif col in column_indices:
                    output_row.append(row[column_indices[col]])
                else:
                    output_row.append("")
            entries_csv.rows.append(output_row)

        entries_csv.write_filename(os.path.join(output_dir, "entries.csv"))

        # Copy original file
        shutil.copy2(input_file, os.path.join(output_dir, "original.csv"))

        # Run dos2unix on all files if they exist
        for filename in ["meet.csv", "entries.csv", "original.csv"]:
            filepath = os.path.join(output_dir, filename)
            if os.path.exists(filepath):
                run_dos2unix(filepath)

    except Exception as e:
        raise ValueError(f"Error processing meet file: {str(e)}")


def main():
    if len(sys.argv) != 2:
        print("Usage: python apla-import.py <import_directory>")
        sys.exit(1)

    import_dir = sys.argv[1]
    if not os.path.exists(import_dir):
        sys.exit(f"Import directory not found: {import_dir}")

    try:
        processed_meets = process_meets(import_dir, BASE_DIR)

        if processed_meets:
            print("\nAll meets processed.")
        else:
            print("No meets found to process.")

    except Exception as e:
        print(f"Error: {str(e)}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
