#!/usr/bin/env python3

import shutil
import subprocess
import sys
import re
from pathlib import Path
from datetime import datetime

BASE_DIR = Path(__file__).resolve().parent
OPL_DATA_DIR = BASE_DIR.parent.parent
SCRIPTS_DIR = OPL_DATA_DIR / "scripts"

try:
    from oplcsv import Csv  # pyright: ignore[reportMissingImports]
except ImportError:
    if SCRIPTS_DIR.exists() and SCRIPTS_DIR.is_dir():
        sys.path.insert(0, str(SCRIPTS_DIR.resolve()))
    from oplcsv import Csv  # pyright: ignore[reportMissingImports]


DEFAULT_COUNTRY = "Australia"
DEFAULT_FEDERATION = "APLA"


def get_meet_number(base_dir: Path) -> str:
    """
    Finds the next available meet number for the current year.
    """
    current_year = str(datetime.now().year)[2:]  # Get last 2 digits of current year
    existing_dirs = [
        d.name
        for d in base_dir.iterdir()
        if d.is_dir()
        and d.name.startswith(current_year)
        and len(d.name) == 4
        and d.name[2:].isdigit()
    ]

    if not existing_dirs:
        return f"{current_year}01"

    last_number = max(int(d[2:]) for d in existing_dirs)
    return f"{current_year}{last_number + 1:02d}"


def clean_meet_name(name: str, year: int) -> str:
    """
    Standardises meet name.

    e.g. "2025 APA National Championships " ??? "National Championships"
    """
    if not name:
        return ""

    # Remove the year if it appears anywhere
    cleaned_name = name.replace(str(year), "").replace(str(year)[2:], "")

    # Remove federation references if they appear
    cleaned_name = (
        cleaned_name.replace("APA", "").replace("APLA", "").replace("IPF", "")
    )

    # Clean up extra spaces
    cleaned_name = " ".join(cleaned_name.split())

    # Convert to title case if required
    if cleaned_name.islower() or cleaned_name.isupper():
        cleaned_name = cleaned_name.title()

    return cleaned_name.strip()


def clean_lifter_name(name: str) -> str:
    """
    Standardises lifter name.

    e.g. "john  smith." ??? "John Smith"
    """
    if not name:
        return ""

    # Remove characters that are not letters, hyphens, apostrophes, or whitespace
    cleaned_name = re.sub(r"[^a-zA-Z'\s-]", "", name)

    # Convert to title case if required
    if cleaned_name.islower() or cleaned_name.isupper():
        cleaned_name = cleaned_name.title()

    return cleaned_name.strip()


def clean_equipment(equipment_str: str) -> str:
    """
    Standardises lifter equipment.

    e.g. "equipped" ??? "Single-ply"
    """
    equipment_str = equipment_str.lower().replace(" ", "")

    if equipment_str in {"raw", "classic"}:
        return "Raw"

    elif "single" in equipment_str or "eq" in equipment_str:
        return "Single-ply"

    else:
        print(f"Error: Could not detect equipment from '{equipment_str}'")
        return ""


def clean_division(division_str: str) -> str:
    """
    Standardises lifter division.

    e.g. "SUB JUNIOR WOMEN" ??? "Sub-Junior"
    """
    if not division_str or not isinstance(division_str, str):
        return ""

    # Clean the input string
    div_clean = division_str.strip().lower()

    # Remove all punctuation and spaces
    div_clean = re.sub(r"[^a-z0-9]", "", div_clean)

    # Remove common erroneous words
    div_clean = re.sub(
        r"(best|lifter|wom[ae]n[s]?|females?|m[ae]n[s]?|\
            males?|3lift|class|classic|raw|equipped|singleply)",
        "",
        div_clean,
    )

    # Match divisions
    if any(
        div in div_clean
        for div in ["subjunior", "subjnr", "subjr", "subj", "sub", "sj", "sbjr", "sbj"]
    ):
        return "Sub-Junior"

    elif any(div in div_clean for div in ["junior", "jnr", "jr", "jnior", "j"]):
        return "Junior"

    elif any(div in div_clean for div in ["special", "olympic", "so"]):
        return "Special Olympics"

    elif "open" in div_clean:
        return "Open"

    elif any(div in div_clean for div in ["master", "m"]):
        if "iv" in div_clean or "4" in div_clean:
            return "Masters 4"
        elif "iii" in div_clean or "3" in div_clean:
            return "Masters 3"
        elif "ii" in div_clean or "2" in div_clean:
            return "Masters 2"
        elif "i" in div_clean or "1" in div_clean:
            return "Masters 1"
        else:
            print(f"Error: No number found in '{division_str}'")
            return ""

    else:
        print(f"Error: Could not identify division from '{division_str}'")
        return ""


def clean_weightclass(bw: str) -> str:
    """
    Standardises lifter weight class.

    e.g. "U84 KG" ??? "84"
    """
    if not bw:
        return ""

    # Strip leading and trailing erroneous characters separately to avoid
    # incorrectly removing + from permitted weight classes e.g. 84+, 120+
    cleaned_wc = bw.strip().lstrip("Uu>-<+")
    cleaned_wc = re.sub(r"\s*kg$", "", cleaned_wc, flags=re.IGNORECASE).strip()

    return cleaned_wc


def run_dos2unix(filepath: Path):
    """
    Runs dos2unix on a file.
    """
    try:
        subprocess.run(["dos2unix", filepath], check=True, capture_output=True)

    except FileNotFoundError:
        print(f"dos2unix not installed; skipping conversion for {filepath.name}")

    except subprocess.CalledProcessError as e:
        print(f"Error: dos2unix failed for {filepath}: {e}", file=sys.stderr)


def process_single_meet(input_file: Path, output_dir: Path) -> None:
    """
    Converts a LiftingCast CSV to OpenPowerlifting format:

    - Saves input file to original.csv
    - Maps meet data to meet.csv
    - Maps entry data to entries.csv
    """
    try:
        # Read input file
        csv_input = Csv(input_file)

        # Clean up formatting
        rows: list = [
            [("" if cell is None else cell).strip('"').strip() for cell in row]
            for row in csv_input.rows
        ]

        # Find the meet data and entries sections
        try:
            meet_data_start = next(
                i for i, row in enumerate(rows) if "Federation" in row
            )
        except StopIteration:
            raise ValueError("Could not find required meet headers in input file.")

        try:
            entries_start = next(i for i, row in enumerate(rows) if "Place" in row)
        except StopIteration:
            raise ValueError("Could not find required entries headers in input file.")

        # Default required meet data
        meet_data = {
            "Federation": DEFAULT_FEDERATION,
            "Date": "",
            "MeetCountry": DEFAULT_COUNTRY,
            "MeetState": "",
            "MeetTown": "",
            "MeetName": "",
        }

        # Extract meet data from the input file
        if meet_data_start + 1 >= len(rows):
            raise ValueError("No meet data row found after the 'Federation' header.")

        meet_row = rows[meet_data_start + 1]
        if len(meet_row) >= 6:
            meet_data["Date"] = meet_row[1] if meet_row[1] else ""
            meet_data["MeetState"] = meet_row[3] if meet_row[3] else ""
            meet_data["MeetTown"] = meet_row[4] if meet_row[4] else ""
            year = meet_row[1].split("-")[0] if meet_row[1] else datetime.now().year
            meet_data["MeetName"] = (
                clean_meet_name(meet_row[5], year) if meet_row[5] else ""
            )
        else:
            raise ValueError("Missing meet data in input file.")

        # Write meet.csv
        meet_csv = Csv()
        meet_csv.fieldnames = list(meet_data.keys())
        meet_csv.rows.append(list(meet_data.values()))
        meet_csv.write_filename(str(output_dir / "meet.csv"))

        # Process entries
        entries_headers = rows[entries_start]
        entries_start_data = entries_start + 1
        entries_data = rows[entries_start_data:]

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
            # Map header names to their entry data values for each row
            row_dict = {
                header: row_val for header, row_val in zip(entries_headers, row)
            }
            output_row = []

            for col in required_columns:
                # Get the value from the row or default to "" if column is missing
                raw_value = row_dict.get(col, "")

                # Format based on the column name
                if col == "Name":
                    output_row.append(clean_lifter_name(raw_value))
                elif col == "WeightClassKg":
                    output_row.append(clean_weightclass(raw_value))
                elif col == "Country":
                    output_row.append("Australia")
                elif col == "Equipment":
                    output_row.append(clean_equipment(raw_value))
                elif col == "Division":
                    output_row.append(clean_division(raw_value))
                else:
                    output_row.append(raw_value)

            entries_csv.rows.append(output_row)

        entries_csv.write_filename(str(output_dir / "entries.csv"))

        # Copy original file
        shutil.copy2(input_file, output_dir / "original.csv")

        # Run dos2unix on all files if they exist
        for filename in ["meet.csv", "entries.csv", "original.csv"]:
            filepath = output_dir / filename
            if filepath.exists():
                run_dos2unix(filepath)

    except (FileNotFoundError, PermissionError) as e:
        raise ValueError(f"File access error: {e}")
    except Exception as e:
        raise ValueError(f"Error processing meet file: {e}")


def process_all_meets(import_dir: Path, base_dir: Path) -> list[str]:
    """
    Processes all meets in the import directory.
    """
    processed_meets = []
    failed_meets = []

    try:
        # Filter for only the relevant files first
        input_files = sorted(list(import_dir.glob("*.opl.csv")))
        if not input_files:
            print(f"No .opl.csv files found to process in {import_dir}.")
            return []
        print(f"{len(input_files)} found to process in {import_dir}.")

        for meet_file in input_files:
            meet_number = get_meet_number(base_dir)
            meet_dir = base_dir / meet_number
            meet_dir.mkdir(exist_ok=True)

            try:
                process_single_meet(meet_file, meet_dir)
                processed_meets.append(str(meet_dir))
                print(f"Processed meet {meet_file.name} -> {meet_number}/")
            except Exception as e:
                print(f"Error processing {meet_file.name}: {str(e)}", file=sys.stderr)
                failed_meets.append((meet_file.name, str(e)))
                # Remove the directory if processing failed
                if meet_dir.exists():
                    shutil.rmtree(meet_dir)
                continue

        if failed_meets:
            print("\nThe following meets failed to process:")
            for meet_file, error in failed_meets:
                print(f"- {meet_file}: {error}")

        return processed_meets

    except Exception as e:
        print(f"Critical error: {str(e)}")
        raise


def main():
    if len(sys.argv) != 2:
        print(f"Usage: python {__file__} <import_directory>")
        sys.exit(1)

    import_dir = Path(sys.argv[1])
    if not import_dir.exists():
        sys.exit(f"Import directory not found: {import_dir}")

    processed_meets = []
    try:
        processed_meets = process_all_meets(import_dir, BASE_DIR)

        if processed_meets:
            print(f"\nSuccessfully processed {len(processed_meets)} meets.")
        else:
            print("No meets found to process.")

    except Exception as e:
        print(f"Error: {str(e)}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
