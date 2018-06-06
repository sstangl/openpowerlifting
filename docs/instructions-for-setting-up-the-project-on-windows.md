# Instructions for Setting Up the Project on Windows

Below are step-by-step instructions for setting up the project.

## Windows 10
1. Enable Linux Subsystem for Windows

   - https://docs.microsoft.com/en-us/windows/wsl/install-win10
   - Choose Ubuntu

2. Install prerequisites

   - In Ubuntu type: `sudo apt-get update`
   - Then: `sudo apt-get install automake git sqlite3 npm node.js poppler-utils`

3. Node stuff

   - In Ubuntu type: `sudo ln -s /usr/bin/nodejs /usr/bin/node`

4. Clone Repo

   - In Ubuntu type: `git clone https://gitlab.com/openpowerlifting/opl-data.git`

5. Change directory

   - In Ubuntu type: `cd openpowerlifting`

6. Build the project

   - In Ubuntu type: `make` -- this never worked
