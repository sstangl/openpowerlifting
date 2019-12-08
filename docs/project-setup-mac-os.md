# Project Setup on Mac OS (tested on Mac OS Mojave 10.14.6)

## Clone the repository
Ensure that you have git installed, and run

  ```bash
  git clone https://gitlab.com/openpowerlifting/opl-data.git
  ```


### 1. Install [brew](https://brew.sh/) package manager:
  ```bash
  /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
  ```
### 2. You will need md5sum tools, because mac-os don't have it be default, run

  ```bash
  brew install md5sha1sum
  ```
### 3. Install the "nightly" version of the Rust programming language using `rustup`:

  ```bash
  curl https://sh.rustup.rs -sSf | sh
  ```
### 4. Add cargo package manager to your PATH

  ```bash
  sudo nano ~/.bash_profile
  ```
 your PATH variable should contain `$HOME/.cargo/bin`, for example export PATH="$HOME/.cargo/bin:{other_stuff}:$PATH"

### 5. Install dependencies:

  ```bash
  brew install make npm python3-toml python3-beautifulsoup4 python3-flake8 ansible parallel uglify-js
  ```

### 6. Run the Makefile
In the `opl-data/` base directory, run

  ```bash
  make
  ```
This will run the Makefile, building the project.

## [Optional] Building the Backend

Openpowerlifting is currently developing a backend in Rust using the Rocket web
framework.  To install this subproject, see the following steps.

### Building the server
In the `server/` directory, run

  ```bash
  cargo build
  ```

### Running the server
In the `server/` directory, run

  ```bash
  cargo run
  ```

The project should now be viewable at the default location of `localhost:8000`

### Possible errors
When you run `make` in the root directory and see:

  ```bash
  cp -r client/build/* "build/data/static"
  rm "build/data/templates/static-asset-map.tera"
  rm: build/data/templates/static-asset-map.tera: No such file or directory
  make[1]: *** [clientstatics] Error 1
  ```

go to `server/templates` and check if you have `static-asset-map-mac.tera` file,if you dont,
rename file `static-asset-map-mac-os-fix.tera` to `static-asset-map.tera`. Then go to `server/client/build` folder and open
`static-asset-map.tera` file, change the hashed file names in your `server/templates/static-asset-map.tera` file to the ones you see in the
`server/client/build/static-asset-map.tera` file


then run `make` again (You should see Good luck! message if everything succeeds)
then go to the `server` folder and run `cargo run --release`
