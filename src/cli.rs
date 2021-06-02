use clap::{App, AppSettings, Arg, ArgMatches};

use crate::finder;
use crate::merger;

fn get_args(version: &str) -> ArgMatches {
    App::new("RAT")
        .version(version)
        .about("A tool for fastq")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("new")
                .about("Find sequences and generate input files")
                .arg(
                    Arg::with_name("dir")
                        .short("d")
                        .long("dir")
                        .help("Specify input directory")
                        .takes_value(true)
                        .default_value("raw_reads")
                        .value_name("DIR"),
                )
                .arg(
                    Arg::with_name("len")
                        .short("l")
                        .long("len")
                        .help("Word lengths")
                        .takes_value(true)
                        .default_value("4")
                        .value_name("LEN"),
                )
                .arg(
                    Arg::with_name("sep")
                        .short("s")
                        .long("sep")
                        .help("Separator type")
                        .takes_value(true)
                        .default_value("_")
                        .value_name("SEP"),
                ),
        )
        .subcommand(
            App::new("merge")
                .about("Merges lanes for each fastq read")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .help("Specifies an input file")
                        .takes_value(true)
                        .default_value("yap-merge_input.conf")
                        .value_name("INPUT"),
                ),
        )
        .get_matches()
}

pub fn parse_cli(version: &str) {
    let args = get_args(version);
    match args.subcommand() {
        ("new", Some(new_matches)) => new_input(new_matches),
        ("merge", Some(merge_matches)) => merge_fastq(merge_matches),
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

fn merge_fastq(matches: &ArgMatches) {
    let input = matches
        .value_of("input")
        .expect("IS NOT A VALID INPUT FILE");
    merger::parse_input_file(input);
}
