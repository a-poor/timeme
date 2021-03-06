# timeme

[![Crate Version](https://img.shields.io/crates/v/timeme.svg)](https://crates.io/crates/timeme) 
[![Rust](https://github.com/a-poor/timeme/actions/workflows/rust-test.yml/badge.svg?branch=main)](https://github.com/a-poor/timeme/actions/workflows/rust-test.yml)

_created by Austin Poor_

A CLI for timing command line program execution, written in Rust.

# Installation

```bash
cargo install timeme
```

## Quick Start

Here's a quick look at `timeme`'s help:

```bash
timeme 0.2.4
$ timeme --help

Austin Poor <a-poor@users.noreply.github.com>

Times the execution of a command.

USAGE:
    timeme [OPTIONS] <CMD> [ARGS]...

ARGS:
    <CMD>        The command to time
    <ARGS>...    The arguments to pass to CMD

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -n, --number <number>    Number of times to run the command. Note: Set to <= 0 to run for at
                             least 0.2s. [default: 0]
```

And here's what it looks like in action:

```bash
$ timeme echo Hello, world!
1.281654ms (+/- 412.714µs) for 156 loops
```

You can specify the number of times to run the command.

```bash
$ timeme -n 10 echo Hello, world!
1.48318ms (+/- 326.664µs) for 10 loops
```

Or, if you leave `-n` blank, `timeme` will run the command repeatedly until
it's been run for at least 0.2 seconds.

