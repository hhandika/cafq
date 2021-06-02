use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader, Result, Write};
use std::path::{Path, PathBuf};

use flate2::bufread::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use glob::{self, MatchOptions};
use rayon::prelude::*;
use regex::Regex;

use crate::utils;

pub fn concat_fastq_files(path: &str) {
    let contents = parse_input_file(path);
    println!("Total samples: {}", contents.len());
    contents.par_iter().for_each(|(id, path)| {
        let mut reads = Merge::new(id, path);
        let samples = reads.glob_samples();
        reads.match_path_to_reads(&samples);
        reads.sort_results();
        reads.concat_lanes_all();
        reads.print_results().expect("CANNOT PRINT TO STDOUT");
    });

    println!("COMPLETED!\n");
}

fn parse_input_file(path: &str) -> HashMap<String, String> {
    let file = File::open(path).unwrap();
    let buff = BufReader::new(file);
    let mut contents = HashMap::new();
    buff.lines()
        .filter_map(|ok| ok.ok())
        .skip(1)
        .for_each(|line| {
            let content = line
                .split(':')
                .map(|entry| entry.trim().to_string())
                .collect::<Vec<String>>();
            assert!(content.len() == 2, "INVALID INPUT FORMAT");
            let id = content[0].clone();
            let path = content[1].clone();
            contents.entry(id).or_insert(path); // Avoid duplicates
        });
    contents
}

struct Merge<'a> {
    id: &'a str,
    path: &'a str,
    read_1: Vec<PathBuf>,
    read_2: Vec<PathBuf>,
}

impl<'a> Merge<'a> {
    fn new(id: &'a str, path: &'a str) -> Self {
        Self {
            id,
            path,
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

    fn concat_lanes_all(&self) {
        let fname_r1 = self.get_concat_name_r1();
        let fname_r2 = self.get_concat_name_r2();
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
            gz.write_all(&content).unwrap();
        });

        gz.finish().unwrap();
    }

    fn print_results(&self) -> Result<()> {
        let io = io::stdout();
        let mut handle = io::BufWriter::new(io);
        self.print_header();
        writeln!(handle, "\x1b[0;33mREAD 1:\x1b[0m")?;
        self.read_1
            .iter()
            .for_each(|file| writeln!(handle, "{:?}", file).unwrap());
        writeln!(handle)?;
        writeln!(handle, "\x1b[0;33mREAD 2:\x1b[0m")?;
        self.read_2
            .iter()
            .for_each(|file| writeln!(handle, "{:?}", file).unwrap());
        writeln!(handle)?;
        writeln!(handle, "\x1b[0;33mResults 2:\x1b[0m")?;
        writeln!(handle, "Read 1: {}", self.get_concat_name_r1())?;
        writeln!(handle, "Read 2: {}", self.get_concat_name_r2())?;
        writeln!(handle)?;
        Ok(())
    }

    fn print_header(&self) {
        let len = 80;
        utils::print_divider(&self.id, len);
    }

    fn get_concat_name_r1(&self) -> String {
        format!("{}_R1.fastq.gz", &self.id)
    }

    fn get_concat_name_r2(&self) -> String {
        format!("{}_R2.fastq.gz", &self.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_input_test() {
        let path = "test_files/input_test.conf";

        let contents = parse_input_file(path);
        assert_eq!(2, contents.len());
    }

    #[test]
    fn parse_duplicate_input_test() {
        let path = "test_files/input_duplicate_test.conf";

        let contents = parse_input_file(path);
        assert_eq!(2, contents.len());
    }

    #[test]
    #[should_panic]
    fn parse_invalid_input_test() {
        let path = "test_files/invalid_input_test.conf";

        let contents = parse_input_file(path);
        assert_eq!(2, contents.len());
    }

    #[test]
    fn regex_lanes_read1_test() {
        let fname = "genus_epithet_unknown_l001_read1_001.fastq.gz";
        let fname2 = "genus_epithet_unknown_l001_read2_001.fastq.gz";
        let test = Merge::new(".", ".");
        assert_eq!(true, test.re_matches_r1(fname));
        assert_eq!(false, test.re_matches_r1(fname2));
    }

    #[test]
    fn regex_lanes_read2_test() {
        let fname = "genus_epithet_unknown_l001_read1_001.fastq.gz";
        let fname2 = "genus_epithet_unknown_l001_read2_001.fastq.gz";
        let test = Merge::new(".", ".");
        assert_eq!(false, test.re_matches_r2(fname));
        assert_eq!(true, test.re_matches_r2(fname2));
    }
}
