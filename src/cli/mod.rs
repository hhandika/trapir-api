use std::time::Instant;

use clap::Parser;

use crate::cli::img::Cli;

pub mod img;

pub fn parse_cli() {
    let time = Instant::now();
    let args = Cli::parse();
}
