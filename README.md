# byte_printer

Byte pretty printer

## Description

There didn't seem to be a byte pretty printer that formatted in both decimal
and binary (GB & GiB) so this aims to fix that.

Aimed at usage within a
seperate program (hence being a library!) it takes in a number of bytes
 (as an f64) and then outputs that as a formatted string, in either
  decimal representation (GB) or in binary representation (GiB).

## Issues and Contributing

For contribution please see `CONTRIBUTING.md`, any issues found are welcome to be reported!

## Local Development

Firstly clone this repository.

This library uses [prek](https://github.com/https://github.com/j178/prek/tree/master) for pre-commit linting and formatting, to get this working on your system, first follow the install guide in the prek repository readme.
When prek is installed, open a terminal within this repository's source folder and run the command `prek init`. This will keep the current configuration and install the necessary pre-commit shim.

## Tests

Despite being a small library, byte_printer comes with some unittests. You can run these through the inbuilt `cargo test` command.
