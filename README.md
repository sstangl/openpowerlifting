# The OpenPowerlifting Project

[![Build Status](https://gitlab.com/openpowerlifting/opl-data/badges/master/pipeline.svg)](https://gitlab.com/openpowerlifting/opl-data/commits/master)

A permanent, accurate, convenient, accessible, open archive of the world's powerlifting data.<br/>
Presentation of this data is available at [OpenPowerlifting.org](https://www.openpowerlifting.org).

**Powerlifting to the People.**

## Licensing

### Code Licensing

All OpenPowerlifting code is Free/Libre software under the GNU AGPLv3+.<br/>
Please refer to the LICENSE file.

### Data Licensing

OpenPowerlifting data (`*.csv`) under `meet-data/` is contributed to the public domain.

The OpenPowerlifting database contains facts that, in and of themselves,<br/>
are not protected by copyright law. However, the copyright laws of some jurisdictions<br/>
may cover database design and structure.

To the extent possible under law, all data (`*.csv`) in the `meet-data/` folder is waived</br>
of all copyright and related or neighboring rights. The work is published from the United States.

Although you are under no requirement to do so, if you incorporate OpenPowerlifting</br>
data into your project, please consider adding a statement of attribution,</br>
so that people may know about this project and help contribute data.

Sample attribution text:

> This page uses data from the OpenPowerlifting project, https://www.openpowerlifting.org.<br/>
> You may download a copy of the data at https://gitlab.com/openpowerlifting/opl-data.

If you modify the data or add useful new data, please consider contributing<br/>
the changes back so the entire powerlifting community may benefit.

## Development Installation

### Fedora 29

First, install the "nightly" version of the Rust programming language using `rustup`:

```bash
curl https://sh.rustup.rs -sSf | sh
```

When a menu appears, choose "Customize installation".<br/>
Press the Enter key until it asks `Default toolchain?`. Type `nightly` and press Enter.<br/>
Continue pressing Enter at the remaining prompts until Rust is installed.

Log out and log back in to allow `~/.cargo/bin` to be part of your default shell `$PATH`.

Install dependencies:

```bash
sudo dnf install make npm python3-toml python3-beautifulsoup4 python3-flake8 ansible parallel uglify-js
```

Build the project and run the server:

```bash
make
cd server
cargo run --release
```

### Docker

To run the server using Docker, simply build and run:

```
docker build -t opl .
docker run -p 8000:8000 opl
```

Access at http://localhost:8000/ per usual.
