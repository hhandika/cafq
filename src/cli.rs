use clap::{App, AppSettings, Arg, ArgMatches};

use crate::concat;
use crate::finder;

fn get_args(version: &str) -> ArgMatches {
    App::new("cafq")
        .version(version)
        .about("A tool to concat multi-lane fastq files")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("new")
                .about("Finds sequences and generate input files")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Specifies input directory")
                        .takes_value(true)
                        .default_value("raw_reads")
                        .value_name("DIR"),
                )
                .arg(
                    Arg::with_name("len")
                        .short("l")
                        .long("len")
                        .help("Specifies word lengths (default: 4)")
                        .takes_value(true)
                        .default_value("4")
                        .value_name("LEN"),
                )
                .arg(
                    Arg::with_name("sep")
                        .short("s")
                        .long("sep")
                        .help("Specifies the separator type (default: underscore)")
                        .takes_value(true)
                        .default_value("_")
                        .value_name("SEP"),
                ),
        )
        .subcommand(
            App::new("concat")
                .about("Concatenates lanes for each fastq read")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("Specifies an input file")
                        .takes_value(true)
                        .default_value("cafq-input.conf")
                        .value_name("INPUT"),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("Specifies an output directory")
                        .takes_value(true)
                        .default_value("raw_reads")
                        .value_name("INPUT"),
                ),
        )
        .get_matches()
}

pub fn parse_cli(version: &str) {
    let args = get_args(version);
    match args.subcommand() {
        ("new", Some(new_matches)) => new_input(new_matches),
        ("concat", Some(merge_matches)) => merge_fastq(merge_matches, version),
        _ => unreachable!(),
    }
}

fn new_input(matches: &ArgMatches) {
    let path = matches.value_of("dir").expect("IS NOT A VALID FILE PATH");
    let len = matches
        .value_of("len")
        .unwrap()
        .parse::<usize>()
        .expect("NOT AN INTEGER");
    let sep = matches
        .value_of("sep")
        .unwrap()
        .parse::<char>()
        .expect("SEPARATOR SHOULD BE A SINGLE CHARACTER");
    let mut init = finder::Finder::new(path, len, sep);
    init.generate_input_file();
}

fn merge_fastq(matches: &ArgMatches, version: &str) {
    let input = matches
        .value_of("input")
        .expect("IS NOT A VALID INPUT FILE");
    let output = matches
        .value_of("output")
        .expect("IS NOT A VALID INPUT FILE");
    println!("Starting cafq v{}...\n", version);
    concat::concat_fastq_files(input, output);
}
