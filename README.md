# Fetch

Simple fetch command written in rust

## Installation

There's 3 ways to install fetch:

### Option 1 (The preferred way)

Use this if you want to use `cargo make`

Prerequisites:

- Rust version 2021
- Cargo
- [cargo make](https://github.com/sagiegurari/cargo-make)

Download, build and install fetch:

```
# Git clone the Repository
$ git clone https://gitlab.com/SpamixOfficial/fetch

# Use this for a global install (requires sudo/admin privilegies)
$ cargo make install
# Use this for an user install (Only linux supports this at the moment)(sudo/admin not required)
$ cargo make install-user
```

### Option 2

Use this if you don't want to use `cargo make`, but you can use `make` instead

Prerequisites:

- Rust version 2021
- Cargo
- make

Download, build and install fetch:

```
# Git clone the Repository
$ git clone https://gitlab.com/SpamixOfficial/fetch

# Use this for a global install (requires sudo/admin privilegies)
# You can specify "BIN_LOCATION=" if you want to change where it is installed 
# Default install location is /usr/local/bin/
$ make install 

# Use this for an user install (sudo/admin not required)
# You can specify "USER_BIN_LOCATION=" if you want to change where it is installed
# Default install location is ~/.local/bin/
$ make install INSTALL_MODE="USER"
```

### Option 3

Use this if neither of the options before worked for you


Prerequisites:

- Rust version 2021
- Cargo

Build from source manually

```
# For a system-wide install (requires sudo)
$ cargo build --release && sudo cp ./target/release/fetch /usr/local/bin/fetch
# For an user install (does not require sudo)
$ cargo build --release && cp ./target/release/fetch ~/.local/bin/fetch
```

## Usage
Use `fetch` for the default output

Use the `--os-logo` parameter for specifying another logo (must be on the list below)

Use as follows:

```
# Name is the name of an OS on the list below
$ fetch --os-logo NAME
```

| OS      | Name    |
|---------|---------|
| Linux   | linux   |
| MacOS   | macos   |
| FreeBSD | freebsd |
| OpenBSD | openbsd |
| NetBSD  | netbsd  |
| Unknown | unknown |


Use the `-h` or `--help` parameter for the help page as follows:
```
$ fetch --help
Usage: fetch [OPTIONS]
Minimal and easy fetch tool written in rust

Positional Arguments:

Options:
    -h	--help		Use this to print this help message
     	--os-logo	Manually specify OS logo

Exit Statuses:
    0	Everything went well
    1	An error occurred


SpamixOfficial 2024
```

## Adding your own logo
Since only a few logos are included by default, no linux distro logos are included.

To add your own logo, create a /etc/ascii-art file and paste your art in there.

Fetch will automatically use that file if it exists
