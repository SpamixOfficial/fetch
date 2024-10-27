# Bluefetch

Minimal fetch command written in rust

## NOTE
This tool is still in development so things might change quickly or (worst case scenario) break.

If something breaks, please refer to this documentation or file an issue on GitLab or GitHub!
(GitLab is preferred!)

## Installation

First git clone and enter the repository:

```
$ git clone https://gitlab.com/SpamixOfficial/fetch && cd fetch
```

There's multiple ways to install fetch:


### Manually

Use this if neither of the options before worked for you


Prerequisites:

- Rust version 2021
- Cargo

For a system-wide install (requires sudo)
```
# cargo build --release && sudo cp ./target/release/fetch /usr/local/bin/fetch
```
For an user install
``` 
$ cargo build --release && cp ./target/release/fetch ~/.local/bin/fetch
```
You may change the install paths above to a path in your PATH variable

### Using cargo make 

Prerequisites:

- Rust version 2021
- Cargo
- [cargo make](https://github.com/sagiegurari/cargo-make)

For a system-wide install (requires sudo)
```
# cargo make install
```
For an user install
```
$ cargo make install-user
```

### Using make (NEEDS BASH)

Use this if you want to use `make` 

Prerequisites:

- Rust version 2021
- Cargo
- make
- bash

For a system-wide install (requires sudo)

You can specify `BIN_LOCATION=` if you want to change where it is installed 

Default install location is `/usr/local/bin/`
```
# make install 
```
For an user install

You can specify `USER_BIN_LOCATION=` if you want to change where it is installed 

Default install location is `~/.local/bin/`

```
$ make install INSTALL_MODE="USER"
```

## Usage
Use `bluefetch` for the default output

Use the `--os-logo` parameter for specifying another logo (must be on the list below)

Use as follows:

```
# Name is the name of an OS on the list below
$ bluefetch --os-logo NAME
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
$ bluefetch --help
Usage: bluefetch [OPTIONS]
Minimal but advanced fetch tool written in rust

Positional Arguments:

Options:
    -h	--help		Use this to print this help message
    -c	--config	Manually specify the config file
     	--os-logo	Manually specify OS logo
    -d	--debug		

Exit Statuses:
    0	Everything went well
    1	An error occurred


SpamixOfficial 2024
```

## Documentation (and configuration!)
Check the [navigation file](docs/README.md) for all documentation pages. 

This includes the configuration pages, in case you were looking for the configuration
