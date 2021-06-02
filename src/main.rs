mod cli;
mod finder;
mod merger;
mod utils;

#[macro_use]
extern crate lazy_static;

use std::time::Instant;

use clap::crate_version;

fn main() {
    let version = crate_version!();
    let time = Instant::now();
    cli::parse_cli(version);
    let duration = time.elapsed();
    if duration.as_secs() < 60 {
        println!("Execution time: {:?}", duration);
    } else {
        utils::print_formatted_duration(duration.as_secs());
    }
}
