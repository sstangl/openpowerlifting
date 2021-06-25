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
### 3. Install the the Rust programming language using `rustup`:

  ```bash
  curl https://sh.rustup.rs -sSf | sh
  ```

### 4. Add cargo package manager to your PATH
  ```bash
  sudo nano ~/.bash_profile
  ```
 your PATH variable should contain `$HOME/.cargo/bin`, for example `export PATH="$HOME/.cargo/bin:{other_stuff}:$PATH"`

### 5. Install Python3 (if needed)

```bash
brew install python
```

### 6. Install pip

```bash
sudo easy_install pip
```

### 7. Install dependencies:

```bash
sudo pip install toml beautifulsoup4 flake8
```

```bash
brew install make npm ansible parallel
```

```bash
npm install uglify-js -g
```

### 8. Run the Makefile
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
