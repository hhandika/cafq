mod init;

#[macro_use]
extern crate lazy_static;

fn main() {
    let path = ".";
    let len = 3;
    let sep = '_';
    let mut init = init::Init::new(path, len, sep);
    init.initialize_input_file();
    println!("Hello, world!");
}
