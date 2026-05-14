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
    """Finds the next available meet number for the current year."""
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


def clean_meet_name(meet_name: str, year: int) -> str:
    """Standardises meet name."""

    if not meet_name:
        return ""

    # Remove characters that are not letters, digits, hyphens, apostrophes, or whitespace
    cleaned = re.sub(r"[^a-zA-Z0-9'\s-]", "", meet_name)

    # Remove the current year and federation name if they appear anywhere
    common_errs = [str(year), str(year)[2:], "APA", "APLA", "IPF", "ORPF"]
    for err in common_errs:
        cleaned = cleaned.replace(err, "")

    # Convert to title case if required
    if cleaned.islower() or cleaned.isupper():
        cleaned = cleaned.title()

    # Strip leading/trailing and collapse extraneous whitespace
    return " ".join(cleaned.split())


def clean_lifter_name(lifter_name: str) -> str:
    """Standardises lifter name."""

    if not lifter_name:
        return ""

    # Remove characters that are not letters, hyphens, apostrophes, or whitespace
    cleaned = re.sub(r"[^a-zA-Z'\s-]", "", lifter_name)

    # Only convert to title case if all lower or all upper case to preserve
    # intentional mixed case e.g. "McCloy"
    if cleaned.islower() or cleaned.isupper():
        cleaned = cleaned.title()

    return " ".join(cleaned.split())


def clean_equipment(equipment: str) -> str:
    """Standardises lifter equipment type."""

    if not equipment:
        return ""

    cleaned = equipment.lower()

    # Remove anything that is not a letter
    cleaned = re.sub(r"[^a-z]", "", cleaned)

    if any(eq in cleaned for eq in ["raw", "classic"]):
        return "Raw"

    elif any(eq in cleaned for eq in ["single", "eq"]):
        return "Single-ply"

    print(f"Error: Could not detect equipment from '{equipment}'")
    return ""


def clean_division(division: str) -> str:
    """Standardises lifter division."""

    if not division:
        return ""

    cleaned = division.lower()

    # Remove anything that is not a letter or number
    cleaned = re.sub(r"[^a-z0-9]", "", cleaned)

    # Remove common erroneous words
    common_errs = (
        r"(best|lifter|wom[ae]n[s]?|females?|m[ae]n[s]?|males?"
        r"|3lift|threelift|benchonly|class|classic|raw|equipped|singleply)"
    )
    cleaned = re.sub(common_errs, "", cleaned)

    # Match divisions with some waterfall string-matching logic.
    #
    # NOTE: The catch-alls sometimes fall through to a false positive for a
    # match on "j" for non-junior or "m" and "i" for a non-masters lifter (e.g.
    # "primetime" would return Masters 1) which is why the order is important:
    # Sub-Junior is matched before Junior, Masters III is matched before
    # Masters II, etc., but overall the parsing is robust enough for ~99% of
    # APLA meets and a quick glance at the original.csv is enough to catch any
    # divisions that are populated incorrectly for an atypical meet structure
    # or other rare happenstance.

    if any(div in cleaned for div in ["sub", "sb", "sj"]):
        return "Sub-Junior"

    elif any(div in cleaned for div in ["jun", "jn", "jr", "j"]):
        return "Junior"

    elif any(div in cleaned for div in ["special", "olympic", "so"]):
        return "Special Olympics"

    elif "open" in cleaned:
        return "Open"

    elif any(div in cleaned for div in ["master", "m"]):
        if any(num in cleaned for num in ["iv", "4"]):
            return "Masters 4"
        elif any(num in cleaned for num in ["iii", "3"]):
            return "Masters 3"
        elif any(num in cleaned for num in ["ii", "2"]):
            return "Masters 2"
        elif any(num in cleaned for num in ["i", "1"]):
            return "Masters 1"
        else:
            print(f"Error: No number found in '{division}'")
            return ""

    print(f"Error: Could not identify division from '{division}'")
    return ""


def clean_weightclass(weightclass: str) -> str:
    """Standardises lifter weight class."""

    if not weightclass:
        return ""

    cleaned = " ".join(weightclass.split())

    # Strip leading erroneous characters separately to avoid incorrectly
    # removing "+" from permitted weight classes e.g. "84+" or "120+"
    cleaned = cleaned.lstrip("Uu>-<+")

    # Remove "kg" and any remaining whitespace e.g. "84 kg" to "84 " to "84"
    return re.sub(r"\s*kg$", "", cleaned, flags=re.IGNORECASE).strip()


def run_dos2unix(filepath: Path):
    """Runs dos2unix on a file."""
    try:
        subprocess.run(["dos2unix", filepath], check=True, capture_output=True)

    except FileNotFoundError:
        print(f"dos2unix not installed; skipping conversion for {filepath.name}")

    except subprocess.CalledProcessError as e:
        print(f"Error: dos2unix failed for {filepath}: {e}", file=sys.stderr)


def process_single_meet(input_file: Path, output_dir: Path) -> None:
    """Converts a single LiftingCast CSV to OpenPowerlifting format."""
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
        raise RuntimeError(f"Error processing meet file: {e}")


def process_all_meets(import_dir: Path, base_dir: Path) -> list[str]:
    """Processes all meets in the provided directory."""
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


if __name__ == "__main__":
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
