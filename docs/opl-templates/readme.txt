OpenPowerlifting Templates Readme
------

From the whole team, sincerely thank you for helping add meet data!
This template pack is intended to make that as straightforward as possible for you.

For all the details below, you don't need to stress about getting everything perfect.
As long as it's pretty close, there are usually only a few small, easy edits for us to make to fix it up.

If you have any questions at all, shoot us an email or join our team chat at https://openpl.zulipchat.com.

------

CSV Data Format

OpenPowerlifting data is submitted in the CSV format, which is a textual spreadsheet format.

Four documents are required for every meet:
- "meet.csv" contains details about the meet (Fed, date, name etc).
- "entries.csv" contains details about the lifters and their lifts.
- "original.txt" or "original.csv" is a copy of the original results for future reference.
- "URL" contains links to the competition results on the federation's website, one link per line.

The order of columns is fixed in the "meet.csv", but can be in any order in the "entries.csv".
The template file shows the most common ordering.

You can test the meet.csv and entries.csv files on our online tool here: https://www.openpowerlifting.org/dev.

If all is well, submit the files by attaching them in an email to issues@openpowerlifting.org.
You should receive an automatic reply with a ticket number, letting you know that it was received successfully.

------

MEET.CSV

The "meet.csv" file describes where and when the meet took place.
The order of the columns is fixed, and should follow the template exactly.

Federation
Mandatory. The federation that hosted the meet.
Note that this may be different than the international federation that provided sanction to the meet. For example, USPA meets are sanctioned by the IPL, but we record USPA meets as USPA.
The full list of valid Federation values is defined by https://gitlab.com/openpowerlifting/opl-data/-/blob/main/modules/opltypes/src/federation.rs.

Comments in that file help explain what each federation value means.

Date
Mandatory. The start date of the meet in ISO 8601 format.
ISO 8601 looks like YYYY-MM-DD: as an example, 1996-12-04 would be December 4th, 1996.
Meets that last more than one day only have the start date recorded.
If a meet spans many days, you can optionally use an "EntryDate" column in the entries.csv to record the exact date of each entry.

MeetCountry
Mandatory. The country in which the meet was held.
The full list of valid Country values is defined by https://gitlab.com/openpowerlifting/opl-data/-/blob/main/modules/opltypes/src/country.rs.

MeetState
Optional. The state, province, or region in which the meet was held.
The full list of valid State values is defined by https://gitlab.com/openpowerlifting/opl-data/-/blob/main/modules/opltypes/src/states.rs.

MeetName
Mandatory. The name of the meet.
The name is defined to never include the year or the federation.
For example, the meet officially called "2019 USAPL Raw National Championships" would have the MeetName "Raw National Championships".

------

ENTRIES.CSV

This file contains information about lifters and their lifts.
The order of the columns doesn't matter, and you can change them to be whatever is easiest to enter for you.
For an empty lifting cell, we prefer having a blank field, instead of writing out "0".

The "entries.csv" format supports a LOT of columns. We list only the most common/useful ones here.
In general, if results contain some data, don't delete it! We probably have a column for it, or we can add one.

Name
Mandatory. The lifter's name, usually in "Firstname Lastname" order, unless in a non-English language.
Lifters who share the same name are distinguished by use of a # symbol followed by a unique number.
For example, two lifters both named John Doe would have Name values John Doe #1 and John Doe #2 respectively.

CyrillicName
Optional. If the lifter's name is usually written in a different character set, keep that information and put it in a column like this one.
Other such columns exist too: ChineseName, KoreanName, JapaneseName, etc.

Sex
Mandatory. The sex category in which the lifter competed, M, F, or Mx.
Mx (pronounced Muks) is a gender-neutral title — like Mr and Ms — originating from the UK. It is a catch-all sex category that is particularly appropriate for non-binary lifters.
The Sex column is defined by modules/opltypes/src/sex.rs.

Event
Mandatory. The type of competition that the lifter entered.
Values are as follows:

- SBD: Squat-Bench-Deadlift, also commonly called "Full Power".
- BD: Bench-Deadlift, also commonly called "Ironman" or "Push-Pull".
- SD: Squat-Deadlift, very uncommon.
- SB: Squat-Bench, very uncommon.
- S: Squat-only.
- B: Bench-only.
- D: Deadlift-only.

Equipment
Mandatory. The equipment category under which the lifts were performed.
Note that this does not mean that the lifter was actually wearing that equipment!
For example, GPC-affiliated federations do not have a category that disallows knee wraps.
Therefore, all lifters, even if they only wore knee sleeves, nevertheless competed in the Wraps equipment category, because they were allowed to wear wraps.
Values are as follows:

- Raw: Bare knees or knee sleeves.
- Wraps: Knee wraps were allowed.
- Single-ply: Equipped, single-ply suits.
- Multi-ply: Equipped, multi-ply suits (includes Double-ply).
- Unlimited: Equipped, multi-ply suits or rubberized gear (like Bench Daddies).
- Straps: Allowed straps on the deadlift (used mostly for exhibitions, not real meets).

Age
Optional. The age of the lifter on the start date of the meet, if known.

BirthDate
Mandatory even if there is no information, formatted as YYYY-MM-DD.
Although the column is mandatory, it's OK to just leave the fields empty.

Division
Optional. Free-form text describing the division of competition, like Open or Juniors 20-23 or Professional.
Some federations are configured in our database, which means that we have agreed on a limited set of division options for that federation, and we have rewritten their results to only use that set, and tests enforce that.
Even still, divisions are not standardized between configured federations: it really is free-form text, just to provide context.

BodyweightKg
Optional. The recorded bodyweight of the lifter at the time of competition.

WeightClassKg
Optional. The weight class in which the lifter competed.
Weight classes can be specified as a maximum or as a minimum. Maximums are specified by just the number, for example 90 means "up to (and including) 90kg."
Minimums are specified by a + to the right of the number, for example 90+ means "above (and excluding) 90kg."

Squat1Kg, Bench1Kg, Deadlift1Kg
Optional. First attempts for each of squat, bench, and deadlift, respectively. Maximum of two decimal places.
Negative values indicate failed attempts.
Not all federations report attempt information. Some federations only report Best attempts.

Squat2Kg, Bench2Kg, Deadlift2Kg
Optional. Second attempts for each of squat, bench, and deadlift, respectively. Maximum of two decimal places.
Negative values indicate failed attempts.
Not all federations report attempt information. Some federations only report Best attempts.

Squat3Kg, Bench3Kg, Deadlift3Kg
Optional. Third attempts for each of squat, bench, and deadlift, respectively. Maximum of two decimal places.
Negative values indicate failed attempts.
Not all federations report attempt information. Some federations only report Best attempts.

Squat4Kg, Bench4Kg, Deadlift4Kg
Optional. Fourth attempts for each of squat, bench, and deadlift, respectively. Maximum of two decimal places.
Negative values indicate failed attempts.
Fourth attempts are special, in that they do not count toward the TotalKg. They are used for recording single-lift records.

Best3SquatKg, Best3BenchKg, Best3DeadliftKg
Optional. Maximum of the first three successful attempts for the lift.
Rarely may be negative: that is used by some federations to report the lowest weight the lifter attempted and failed.

TotalKg
Optional. Sum of Best3SquatKg, Best3BenchKg, and Best3DeadliftKg, if all three lifts were a success.
If one of the lifts was failed, or the lifter was disqualified for some other reason, the TotalKg is empty.
Rarely, mostly for older meets, a federation will report the total but not any lift information.

Place
Mandatory. The recorded place of the lifter in the given division at the end of the meet.
Values are as follows:

- Positive number: the place the lifter came in.
- G: Guest lifter. The lifter succeeded, but wasn't eligible for awards.
- DQ: Disqualified. Note that DQ could be for procedural reasons, not just failed attempts.
- DD: Doping Disqualification. The lifter failed a drug test.
- NS: No-Show. The lifter did not show up on the meet day.

Tested
Optional. "Yes" if the lifter entered a drug-tested category, empty otherwise.
Note that this records whether the results count as drug-tested, which does not imply that the lifter actually took a drug test.
Federations typically do not report which lifters, if any, were subject to drug testing.

Country
Optional. The home country of the lifter, if known, like "USA".
The full list of valid Country values is defined by https://gitlab.com/openpowerlifting/opl-data/-/blob/main/modules/opltypes/src/country.rs

State
Optional. The home state/province/oblast/division/etc of the lifter, if known, like "CA".
The full list of valid State values is defined by https://gitlab.com/openpowerlifting/opl-data/-/blob/main/modules/opltypes/src/states.rs
Expanded names are given there in comments.

Team
Optional. The name of the lifter's team/gym/etc.
