mod merger;
mod samples;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = ".";
    let len = 4;
    let sep = '_';
    let mut init = samples::Finder::new(path, len, sep);
    init.generate_input_file();

    let input = "yap-merge_input.conf";
    merger::parse_input_file(input);
}
