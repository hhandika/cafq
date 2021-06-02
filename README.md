# CaFq

[![Build Status](https://www.travis-ci.com/hhandika/fastq-merger.svg?branch=main)](https://www.travis-ci.com/hhandika/fastq-merger)

CaFq is a command-line tool to concat fastq reads. The runtime is slower than concatenating gunzipped fastq files using the `cat` command, such as: `cat file_L001_R1.fastq.gz file_L002_R1.fastq.gz > file_R1.fastq.gz`. The app, however, allows for batch concatenating fastq files in nested directories with minimal effort (zero Bash coding skill needed) and a better control over your files (minimal chance making mistakes that can cause losing data). The final file is typically smaller than using the `cat` command.

## Installation

To install, please install [the Rust Compiler](https://www.rust-lang.org/learn/get-started) first, and then:

```
cargo install --git https://github.com/hhandika/cafq
```

Confirm the app properly installed:

```
cafq --version
```

It should show the app version.

## Usages

To find the files and create input for the app:

```
cafq new -d [your-parent-sequence-directory]
```

It will create an input files `caft-input.conf` that contains a list of found multi-lane fastq sequences. For each list, it contains the id and the path to the files. For example: 

```
[seq]
sample_1:/home/you/sequences/sample/
sample_2:/home/you/sequecens/sample/
```

To concat all the files in the list:

```
cafq concat
```

## Command Options


```
USAGE:
    cafq <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    concat    Concatenates lanes for each fastq read
    help      Prints this message or the help of the given subcommand(s)
    new       Finds sequences and generate input files
```

To check options for each subcommand:

```
cafq <SUBCOMMAND> --help
```