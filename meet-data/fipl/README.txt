# Automatic data conversion

To automatically generate OPL data files for FIPL meets you can use the @labarilem/opl-tools NPM package.

Requirements: Node.js and NPM must be installed on your system. Guide: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

Usage: npx @labarilem/opl-tools@latest generate <federation> <year> <meetId|latest> <outputDir> [--isOpenDivision <true|false>]
Docs: https://www.npmjs.com/package/@labarilem/opl-tools

Example: if you want to generate OPL data for the latest meet in year 2026 that has published results on the official website, run:
npx @labarilem/opl-tools@latest generate fipl 2026 latest ./path/to/fipl/26xx

Source code is hosted on a different repository since the parser logic is not trivial.
It's open source and all contributions are welcome. Check it out here: https://github.com/labarilem/openpowerlifting-tools