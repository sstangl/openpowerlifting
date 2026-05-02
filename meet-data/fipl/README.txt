To automatically generate OPL data files for FIPL meets you can use the @labarilem/opl-tools NPM package.
Requirements: Node.js and NPM must be installed on your system. Guide: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
Usage: npx @labarilem/opl-tools generate <federation> <year> <meetId> <outputDir> [--isOpenDivision <true|false>]
Example: npx @labarilem/opl-tools generate fipl 2026 8 ./path/to/fipl/26xx
Docs: https://www.npmjs.com/package/@labarilem/opl-tools
Source code:  https://www.npmjs.com/package/@labarilem/opl-tools