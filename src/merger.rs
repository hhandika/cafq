use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

use flate2::bufread::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use glob::{self, MatchOptions};
use rayon::prelude::*;
use regex::Regex;

pub fn parse_input_file(path: &str) {
    let file = File::open(path).unwrap();
    let buff = BufReader::new(file);
    let mut contents = Vec::new();

    buff.lines()
        .filter_map(|ok| ok.ok())
        .skip(1)
        .for_each(|line| {
            contents.push(line);
        });

    process_files(&contents);
}

fn process_files(contents: &[String]) {
    contents.par_iter().for_each(|line| {
        let content = line
            .split(':')
            .map(|entry| entry.trim().to_string())
            .collect::<Vec<String>>();
        assert!(content.len() > 1, "NOT ENOUGH DATA");
        let mut reads = Reads::new();
        reads.id = String::from(&content[0]);
        reads.path = String::from(&content[1]);
        let samples = reads.glob_samples();
        reads.match_path_to_reads(&samples);
        reads.sort_results();
        reads.concat_lanes_all();
        reads.print_results();
    });
}

struct Reads {
    id: String,
    path: String,
    read_1: Vec<PathBuf>,
    read_2: Vec<PathBuf>,
}

impl Reads {
    fn new() -> Self {
        Self {
            id: String::new(),
            path: String::new(),
            read_1: Vec::new(),
            read_2: Vec::new(),
        }
    }

    fn glob_samples(&self) -> Vec<PathBuf> {
        let pattern = format!("{}/{}*.f*.g*", self.path, self.id);
        let opts = MatchOptions {
            case_sensitive: true,
            ..Default::default()
        };

        glob::glob_with(&pattern, opts)
            .unwrap()
            .filter_map(|ok| ok.ok())
            .collect()
    }

    fn match_path_to_reads(&mut self, paths: &[PathBuf]) {
        paths.iter().for_each(|read| {
            let fname = read.file_name().unwrap().to_string_lossy();
            if self.re_matches_r1(&fname) {
                self.read_1.push(PathBuf::from(read));
            }
            if self.re_matches_r2(&fname) {
                self.read_2.push(PathBuf::from(read));
            }
        })
    }

    fn re_matches_r1(&self, fname: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(_|-)((?i)(read|r)1)(?:.*)(gz|gzip)").unwrap();
        }

        RE.is_match(fname)
    }

    fn re_matches_r2(&self, fname: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(_|-)((?i)(read|r)2)(?:.*)(gz|gzip)").unwrap();
        }

        RE.is_match(fname)
    }

    fn sort_results(&mut self) {
        self.read_1.sort();
        self.read_2.sort();
    }

    fn print_results(&self) {
        println!("READ 1:");
        self.read_1.iter().for_each(|file| println!("{:?}", file));
        println!("READ 2:");
        self.read_2.iter().for_each(|file| println!("{:?}", file));
    }

    fn concat_lanes_all(&self) {
        let fname_r1 = format!("{}_R1.fastq.gz", &self.id);
        let fname_r2 = format!("{}_R2.fastq.gz", &self.id);
        self.concat_lanes(&self.read_1, &fname_r1);
        self.concat_lanes(&self.read_2, &fname_r2);
    }

    fn concat_lanes(&self, read: &[PathBuf], fname: &str) {
        let dir = Path::new("raw_reads").join(&self.id);
        fs::create_dir_all(&dir).expect("CANNOT CREATE DIR");
        let path = dir.join(fname);
        let save = File::create(path).expect("CANNOT CREATE FILE");
        let mut gz = GzEncoder::new(save, Compression::default());
        read.iter().for_each(|line| {
            let file = File::open(line).unwrap();
            let buff = BufReader::new(file);
            let mut decoder = MultiGzDecoder::new(buff);
            let mut content = Vec::new();
            decoder
                .read_to_end(&mut content)
                .expect("CANNOT READ FILES");
            gz.write_all(&mut content).unwrap();
        });

        gz.finish().unwrap();
    }
}
