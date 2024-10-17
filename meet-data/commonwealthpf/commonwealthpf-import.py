#!/usr/bin/env python3

import os
import sys
import subprocess
import shutil
import urllib.request
from datetime import datetime

SCRIPT_DIR = os.path.dirname(os.path.realpath(__file__))
OPL_DATA_ROOT = os.path.dirname(os.path.dirname(SCRIPT_DIR))
SCRIPTS_DIR = os.path.join(OPL_DATA_ROOT, "scripts")

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(SCRIPTS_DIR)
    from oplcsv import Csv


def run_goodlift_import(cid, temp_dir):
    """Run the goodlift-import Rust script for a given CID."""
    # Download the CSV using the Rust script
    result = subprocess.run(
        ["cargo", "run", "--bin", "goodlift-import", "--", "--cid", str(cid)],
        cwd=OPL_DATA_ROOT,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        error_msg = result.stderr.strip()
        if "CSV deserialize error" in error_msg:
            # Extract line number and byte position from the error message
            line_num = int(error_msg.split("line:")[1].split(",")[0].strip())
            byte_pos = int(error_msg.split("byte:")[1].split(")")[0].strip())

            # Read the problematic line from the original CSV
            csv_url = f"https://goodlift.info/get-competitions-report-csv.php?cid={cid}"
            with urllib.request.urlopen(csv_url) as response:
                csv_content = response.read().decode("utf-8").splitlines()
            problematic_line = (
                csv_content[line_num - 1]
                if line_num <= len(csv_content)
                else "Line not found"
            )

            # Find the exact column based on the byte position
            fields = problematic_line.split(",")
            cumulative_length = 0
            problematic_field = ""
            column_index = -1

            for i, field in enumerate(fields):
                cumulative_length += len(field) + 1  # +1 for the comma
                if cumulative_length >= byte_pos:
                    problematic_field = field
                    column_index = i
                    break

            print(f"Error processing CID {cid}:")
            print(error_msg)
            print(f"Problematic line ({line_num}): {problematic_line}")
            print(
                f"Problematic field: '{problematic_field}' (Column {column_index + 1})"
            )
            print("Detailed view of problematic line:")
            print(
                ",".join(
                    f"{i}:'{v}'" for i, v in enumerate(problematic_line.split(","))
                )
            )

            raise Exception(f"Error processing CID {cid}")
        else:
            raise Exception(
                f"Error running goodlift-import for CID {cid}:\n{error_msg}"
            )

    # Move the generated files to the temp directory
    for file in ["original.csv", "entries.csv", "meet.csv"]:
        source = os.path.join(OPL_DATA_ROOT, file)
        if os.path.exists(source):
            shutil.move(source, os.path.join(temp_dir, f"{file}.{cid}"))
        else:
            print(f"Warning: {file} not found for CID {cid}")

    return True


def create_meet_directory():
    """Create a new directory for this meet."""
    year = datetime.now().strftime("%y")
    existing_dirs = [
        d
        for d in os.listdir(SCRIPT_DIR)
        if d.startswith(year) and len(d) == 4 and d[2:].isdigit()
    ]
    meet_number = len(existing_dirs) + 1
    new_dir = f"{year}{meet_number:02d}"
    new_dir_path = os.path.join(SCRIPT_DIR, new_dir)
    os.makedirs(new_dir_path, exist_ok=True)
    return new_dir_path


def combine_entries_csv(temp_dir, output_dir):
    """Combine all entries.csv files into one using oplcsv."""
    combined_entries = Csv()
    for i, filename in enumerate(os.listdir(temp_dir)):
        if filename.startswith("original"):
            cid = filename.split(".")[-1]
            shutil.copy(
                os.path.join(temp_dir, filename),
                os.path.join(output_dir, f"original-{cid}.csv"),
            )
        elif filename.startswith("entries"):
            entries = Csv(os.path.join(temp_dir, filename))
            if i == 0:
                combined_entries = entries
            else:
                combined_entries.cat(entries)

    # Correct order of columns
    columns = [
        "Place",
        "Name",
        "Sex",
        "BirthDate",
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

    reordered_entries = Csv()
    reordered_entries.fieldnames = columns

    for row in combined_entries.rows:
        new_row = []
        for field in columns:
            if field in combined_entries.fieldnames:
                index = combined_entries.index(field)
                new_row.append(row[index])
            else:
                new_row.append("")  # Add empty string for missing fields
        reordered_entries.rows.append(new_row)

    reordered_entries.write_filename(os.path.join(output_dir, "entries.csv"))


def handle_meet_csv(temp_dir, output_dir):
    """Combine meet.csv files and correct the meet name."""
    meet_data = None
    for filename in os.listdir(temp_dir):
        if filename.startswith("meet"):
            meet_csv = Csv(os.path.join(temp_dir, filename))
            if meet_data is None:
                meet_data = meet_csv
            else:
                # Ensure all other fields match
                for i, field in enumerate(meet_csv.fieldnames):
                    if (
                        field != "MeetName"
                        and meet_data.rows[0][i] != meet_csv.rows[0][i]
                    ):
                        print(f"Warning: Mismatch in {field} across meet.csv files")

    if meet_data is None:
        print("Warning: No meet.csv files found. Creating an empty one.")
        meet_data = Csv()
        meet_data.fieldnames = [
            "Federation",
            "Date",
            "MeetCountry",
            "MeetState",
            "MeetTown",
            "MeetName",
        ]
        meet_data.rows.append([""] * len(meet_data.fieldnames))
    else:
        # Correct the meet name
        meet_name_index = meet_data.index("MeetName")
        full_meet_name = meet_data.rows[0][meet_name_index]
        current_year = datetime.now().strftime("%y")

        data_to_remove = [
            "CommonwealthPF",
            "IPF",
            "ORPF",
            "AfricanPF",
            "AsianPF",
            "EPF",
            "Womens",
            "Mens",
            "Women's",
            "Men's",
            "Mens'",
            "Womens'",
            "Equipped",
            "Classic",
            "Benchpress",
            "Bench Press",
            "Powerlifting",
            "Championships",
            "Masters",
            "Open",
            "Junior",
            "Sub-Junior",
            "Subjunior",
        ]

        if current_year in full_meet_name:
            full_meet_name = full_meet_name.replace(current_year, "")

        corrected_name = " ".join(
            word for word in full_meet_name.split() if word not in data_to_remove
        )

        corrected_name = corrected_name.strip()
        corrected_name += " Championships"

        meet_data.rows[0][meet_name_index] = corrected_name

    meet_data.write_filename(os.path.join(output_dir, "meet.csv"))


def check_files(temp_dir, cids):
    missing_files = []
    for cid in cids:
        for file in ["original.csv", "entries.csv", "meet.csv"]:
            if not os.path.exists(os.path.join(temp_dir, f"{file}.{cid}")):
                missing_files.append(f"{file}.{cid}")

    if missing_files:
        print("Warning: The following files are missing from the temporary directory:")
        for file in missing_files:
            print(f"  - {file}")
        return False
    return True


def main(cids):
    temp_dir = os.path.join(SCRIPT_DIR, "temp_goodlift_import")
    os.makedirs(temp_dir, exist_ok=True)

    output_dir = create_meet_directory()

    try:
        successful_cids = []
        for cid in cids:
            try:
                run_goodlift_import(cid, temp_dir)
                successful_cids.append(cid)
            except Exception as e:
                print(f"Error processing CID {cid}:")
                print(str(e))
                print("Continuing with next CID...")

        if not successful_cids:
            raise Exception("No data was successfully imported.")

        # Check if all expected files are present
        if not check_files(temp_dir, successful_cids):
            print("Warning: Some files are missing. Proceeding with available files.")

        # Combine entries.csv files and create a single meet.csv file
        combine_entries_csv(temp_dir, output_dir)
        handle_meet_csv(temp_dir, output_dir)

        shutil.rmtree(temp_dir)
        print(f"Import complete for CIDs: {', '.join(map(str, successful_cids))}")
        print(
            f"Results are in the {os.path.relpath(output_dir, SCRIPT_DIR)} directory."
        )
    except Exception as e:
        print(f"An error occurred during processing: {str(e)}")
        if not os.listdir(output_dir):
            os.rmdir(output_dir)
            print(f"Removed empty output directory: {output_dir}")
    finally:
        if os.path.exists(temp_dir):
            shutil.rmtree(temp_dir)
            print(f"Removed temporary directory: {temp_dir}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python commonwealthpf-import.py <cid1> <cid2> ...")
        sys.exit(1)

    cids = sys.argv[1:]
    main(cids)
