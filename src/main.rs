mod samples;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = ".";
    let len = 3;
    let sep = '_';
    let mut init = samples::Finder::new(path, len, sep);
    init.generate_input_file();
}
