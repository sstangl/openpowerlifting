import json
import sys
import csv

from csv import DictWriter


def lifts_str(event):
    lifts_l = []
    if "S" in event:
        lifts_l.append("squat")
    if "B" in event:
        lifts_l.append("bench")
    if "D" in event:
        lifts_l.append("dead")
    return ",".join(lifts_l)


def gen_divisions(config_d):
    for sex in ["M", "F"]:
        for age in config_d["age"]:
            for eq_d in config_d["equipment"]:
                for event in config_d["events"]:
                    if sex == "M":
                        wtcls_l = config_d["m_wtcls"]
                    elif sex == "F":
                        wtcls_l = config_d["f_wtcls"]
                    else:
                        raise ValueError(f"Unknown sex:{sex}")
                    # for each division also generate a division for guest lifters
                    for guest in [False, True]:
                        for (i, wtcls,) in enumerate(wtcls_l):
                            div_d = {
                                "name": "",
                                "gender": "",
                                "rawOrEquipped": "",
                                "lifts": "",
                                "scoreBy": "",
                                "weightClassName": wtcls,
                                "maxWeight": "9999" if wtcls.endswith("+") else wtcls,
                                "usaplDivisionCode": "",
                                "hideOnBoard": ""
                            }
                            if i == 0:
                                div_d.update({
                                    "name": (
                                        f"{sex} {age} {eq_d['show']} {event}"
                                        f"{' GUEST' if guest else ''}"
                                    ),
                                    "gender": "Male" if sex == "M" else "Female",
                                    "rawOrEquipped": eq_d["lc"],
                                    "lifts": lifts_str(event),
                                    "scoreBy": "Total",
                                    # TODO - confirm this does nothing
                                    "usaplDivisionCode": "G" if guest else "O",
                                })
                            yield div_d


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.stderr.write(f"Usage: {sys.argv[0]} config_path output_path")
    config_path = sys.argv[1]
    with open(config_path, "rt") as config_f:
        config_d = json.load(config_f)
    output_path = sys.argv[2]
    with open(output_path, "wt") as output_f:
        dw = DictWriter(
            output_f,
            [
                "name", "gender", "rawOrEquipped", "lifts", "scoreBy", "weightClassName",
                "maxWeight", "usaplDivisionCode", "hideOnBoard"
            ],
            quoting=csv.QUOTE_ALL
        )
        dw.writeheader()
        for div_row_d in gen_divisions(config_d):
            dw.writerow(div_row_d)
