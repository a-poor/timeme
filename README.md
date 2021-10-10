# timeme

[![Crate Version](https://img.shields.io/crates/v/timeme.svg)](https://crates.io/crates/timeme)
[![Rust](https://github.com/a-poor/timeme/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/a-poor/timeme/actions/workflows/rust.yml)

_created by Austin Poor_

A CLI for timing command line program execution, written in Rust.

# Installation

```bash
cargo install timeme
```

## Quick Start

Here's a quick look at `timeme`'s help:

```bash
$ timeme --help
timeme 0.2.0

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

