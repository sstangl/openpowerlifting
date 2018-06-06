# How to Add a Meet
This guide is intended to take you step-by-step into adding a meet into the database.

## Prerequisites
1. Ubuntu (see Instructions for Setting Up the Project on Windows steps 1-8)
- Automake
- Git
- Clone of repo
- sqlite 3
- npm
- node

## Part 1: Creating `meet.csv`, `entries.csv`, `URL`,  and `results.txt` files
*Important* before you get started, remember `meet.csv`, `entries.csv`, and `URL` files need to be saved in the same folder.

### Step 1: Create the `meet.csv` file (In Excel, LibreOffice, or another spreadsheet program)
1. Use the following format for creating the meet.csv file.
- In the first row:
  - The first column should be labeled `Federation`.
  - The next column should be labeled `Date`.
  - The next column should be labeled `MeetCountry`.
  - The next column should be labeled `MeetState`.
  - The next column should be labeled `MeetTown`.
  - The next column should be labeled `MeetName`.
- In the second row:
  - The first column should contain the abbreviation of the federation (e.g. `USPF`) from which the meet you are adding is sanctioned by.
  - The next column should contain the date of the meet you are adding in the format of YEAR-MONTH-DATE (e.g. `2018-01-01`).
  - The next column should contain the abbreviation of the country (e.g. `USA`) where the meet took place.
  - The next column should contain the abbreviation of the state/province (e.g. `CA`) where the meet took place.
  - The next column should contain the name of the town/city where the meet took place (e.g. `Burbank`).
  - The next column should contain the name of the meet you are adding (e.g. `2000 USPF National Powerlifting and Bench Press Championships`).
2. Save the file as `meet.csv`in its own folder (you will be saving the next two files into the same folder).

### Step 2: Creating the `entries.csv` file (In Excel, LibreOffice, or another spreadsheet program)
1. Use the following format for creating the `entries.csv` file. Order of the columns does not matter, but the order below is most convenient
- In the first row:
  - The first column should be labeled `Place`.
  - The next column should be labeled `Name`.
  - The next column should be labeled `Sex`.
  - The next column should be labeled `Event`.
  - The next column should be labeled `Division`. 
  - The next column should be labeled `WeightClassKg`.
  - The next column should be labeled `Equipment`.
  - The next column should be labeled `BirthYear` or `Age`, depending on the federation, if reported.
  - The next column should be labeled `State` if reported.
  - The next column should be labeled `BodyweightKg` if reported.
  - The next column should be labeled `Squat1Kg` if reported.
  - The next column should be labeled `Squat2Kg` if reported.
  - The next column should be labeled `Squat3Kg`.
  - The next column should be labeled `Best3SquatKg`.
  - The next column should be labeled `Bench1Kg` if reported.
  - The next column should be labeled `Bench2Kg` if reported.
  - The next column should be labeled `Bench3Kg` if reported.
  - The next column should be labeled `Best3BenchKg`.
  - The next column should be labeled `Deadlift1Kg` if reported.
  - The next column should be labeled `Deadlift2Kg` if reported.
  - The next column should be labeled `Deadlift3Kg` if reported.
  - The next column should be labeled `Best3DeadliftKg`.
  - The next column should be labeled `TotalKg`.
- In subsequent rows:
  - The first column should contain the placing of the competitor in the meet (e.g. `1`).
  - The next column should contain the name of the competitor (e.g. `John Doe`).
  - The next column should contain the sex of the competitor. `M` for male and `F` for female.
  - The next column should contain the lifts that the competitor performed (e.g. `SBD`, `BD`, `B`, OR `D`). `S` is squat, `B` is bench, and `D` is deadlift. 
  - The next column should contain the division that the competitor competed in. This is written as the federation reported it (e.g. `R-O` or `Sub Master Men`).
  - The next column should contain the weight class of the competitor, in kilograms (e.g. `100`).
  - The next column should contain the equipment used by the competitor. `Raw`, `Wraps`, `Single-ply`, and `Multi-ply` are the equipment types.
  - The next column should contain the year the competitor was born (e.g. `1991`) or the age of the competitor at the competition (e.g. `43`), depending on the federation, if reported.
  - The next column should contain the state/province abbreviation from where the competitor is competing from (e.g. `VA`) if reported.
  - The next column should contain the bodyweight of the competitor in kilograms (e.g. `102.2`) if reported.
  - The next column should contain the competitor's first attempt squat in kilograms `Squat1Kg` if reported.
  - The next column should contain the competitor's second attempt in kilograms `Squat2Kg` if reported.
  - The next column should contain the competitor's third attempt in kilograms `Squat3Kg` if reported.
  - The next column should contain the competitor's best squat attempt in kilograms `Best3SquatKg`.
  - The next column should contain the competitor's first attempt in kilograms `Bench1Kg` if reported.
  - The next column should contain the competitor's second attempt in kilograms `Bench2Kg` if reported.
  - The next column should contain the competitor's third attempt in kilograms `Bench3Kg` if reported.
  - The next column should contain the competitor's best bench press attempt in kilograms `Best3BenchKg`.
  - The next column should contain the competitor's first attempt in kilograms `Deadlift1Kg` if reported.
  - The next column should contain the competitor's second attempt in kilograms `Deadlift2Kg` if reported.
  - The next column should contain the competitor's third attempt in kilograms `Deadlift3Kg` if reported.
  - The next column should contain the competitor's best deadlift attempt in kilograms `Best3DeadliftKg`.
  - The next column should contain the competitor's total in kilograms `TotalKg`.
2. Save the file as `entries.csv` in the same folder as meet.csv.

For meets that are in pounds:
1. Change anything `Kg` to `LBS`.
2. Once completed with step two of part two open Ubuntu and go to the directory you just added
- Go to the federation's directory for which you are trying to add a meet.
  - Type: `cd openpowerlifting`
  - Type: `cd meet-data`
  - Type: `cd ${FEDNAME}`
- Go to the directory you just added. 
  - Type: `cd ${DIRNAME}`
3. Run `../../../scripts/csv-tokg entries.csv`
4. Run `../../../scripts/fix-weightclasses entries.csv`

Tips
1. Look at recent USAPL `meet-data` files for an example of how to format `entries.csv`.
2. If some data is not reported you can leave the column for that data blank.
3. Some competitors may be listed more than once. This happens because sometimes competitors enter more than one division.

### Step 3: Creating the URL file (in Notepad or another text editor)
1. Copy and paste or type the URL of the meet results into Notepad/any text editor.
2. Save the file as `URL` in the same folder as the two previous files. Note that this file has no extension.

### Step 4: Creating the `results.txt` file (in Ubuntu)
This step requires that you have the URL of the `.pdf` file of the meet results.
1. Install poppler `sudo apt-get install poppler-utils`.
2. Go to the federation's directory for which you are trying to add the meet. Type `cd openpowerlifting/meet-data/${FEDNAME}` (where `${FEDNAME}` is the name of the federation you are uploading to).
3. At this point, you can go ahead and create a new directory for the meet. Type `mkdir ${DIRNAME}` (where `${DIRNAME}` is the name of the directory you will create). Name the directory starting with the last two digits of the year of the meet (1999 the last two digits would be 81) and ending with the number meet it was for that year (`9901` is the first meet of 1999 from the federation and  `1524` is the twenty-fourth meet of 2015 from the federation.).
4. Go to the directory you just added. Type `cd ${DIRNAME}` (where `${DIRNAME}` is the name of the directory you created).
5. Download the `.pdf` file of the meet results. Type `wget ${URL}` (where `${URL}` is the URL of the `.pdf` file) to .
6. Type `ls` to figure out the name of the `.pdf` downloaded.
7. Convert the `.pdf` file to a `.txt` file. Type `pdftotext -layout ${PDF}` (where `${PDF}` is the name of the `.pdf` file downloaded) which.
8. Rename the `${TXT}` file to `results.txt`. Type `mv ${TXT} results.txt` (where ${TXT} is same name of the `.pdf` earlier, with a `.txt` file extension instead).
9. Remove the `.pdf` file downloaded in step two. Type `rm *.pdf`. *Be careful* to only include no spaces between `*` and `.pdf` because `rm *.pdf` will delete all your files!
You are now ready to upload the `entries.csv`, `results.csv`, and `URL` files created earlier.

## Part 2: Uploading meet 

### Step 1:  Add to directory (In Ubuntu)
1. Go to the directory you just added, you are probably already there (you can check where you are by typing `pwd`). If so omit this step. Type `cd openpowerlifting/meet-data/${FEDNAME}/${DIRNAME}` (where `${FEDNAME}` is the name of the federation you are uploading to and `${DIRNAME}` is the directory you just added).
2. Upload the folder where you stored `meet.csv`, `entries.csv`, and `URL`. Type `cp /mnt/${FILEPATH}* ./`(where ${FILEPATH} is the file path to the folder where you stored `meet.csv`, `entries.csv`, and `URL`). *Note*: if your folder has a space use a backslash (e.g. `/Documents/open\ pl/USPF/1701/* ./`)

### Step 2: Check your work (In Ubuntu)
1. You need to return to the main project directory so type `cd openpowerlifting`.
2. Check your work. Type `make check`. You will see any errors that you have made in the data. If you have errors fix them and repeat the previous step *and this step* before continuing. If you have no errors you may continue.

### Step 3: Tell `git` about changes (In Ubuntu)
1. Tell `git` about the files you added. Type `git add meet-data/${FEDNAME}/${DIRNAME}` (where `${FEDNAME}` is the name of the federation you are uploaded to and `${DIRNAME}` is the name of the directory you created).
2. Next, make sure you told `git` by typing `git status`, it should show the files you added.

### Step 4: Upload (In Ubuntu)
1. You need to add `entries.csv` file so type `git add entries.csv`.
2. You need to add `meet.csv` file so type `git add meet.csv`.
3. You need to add `URL` file so type `git add URL`.
4. You need to add `results.txt` file so type `git add results.txt`
5. Next, commit the above files by typing `git commit -m  "${DIRNAME}" ` (where `${DIRNAME}` is the directory you just added).
6. Push the above files by typing `git remote add mine ${URL_TO_YOUR_FORK}`.
7. You will be asked for your GitLab username and password.

### Step 5: Send merge request (On GitLab.com)
1. Go to GitLab.com.
2. Find your profile.
3. Go to your repository.
4. Create a new merge request

Now you have successfully uploaded a meet!

### Miscellaneous Ubuntu Tips
1. Typing `ls` will give you a list of all things under the directory that you are currently under.
2. Typing `pwd` will tell you where you are at.
3. Typing `cd` will return you to your home directory.
4. Hitting the tab key will autocomplete what you are typing. For example: hitting the tab key after `cd open` will auto complete to `cd openpowerlifting`.
5. `.` means the current directory and `..` means the parent directory
