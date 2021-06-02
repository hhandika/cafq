mod cli;
mod finder;
mod merger;

#[macro_use]
extern crate lazy_static;

use clap::crate_version;

fn main() {
    let version = crate_version!();
    cli::parse_cli(version);
}
