# Syshelper
An application to assist in troubleshooting and maintain systems

[![Rust](https://github.com/cmarquis/syshelper/actions/workflows/rust.yml/badge.svg)](https://github.com/cmarquis/syshelper/actions/workflows/rust.yml)

## Development
The application is written in Rust. As such a local rust development environment must be setup.

## Usage
The application can be compiled yourself or a pre-compiled binary can be downloaded from GitHub releases.

### USAGE:
    syshelper [FLAGS] [OPTIONS] [SUBCOMMAND]

### FLAGS:
    -h, --help       Prints help information
        --pretty     Pretty prints the output
    -V, --version    Prints version information

### OPTIONS:
    -o, --output <output>    The file to output the results to

### SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    ls      Returns the files and their sizes from the specified directory