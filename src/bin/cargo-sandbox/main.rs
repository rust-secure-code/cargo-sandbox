//! cargo-sandbox command-line utility

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use crossterm::style::Colorize;
use std::{env, process};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    cargo_sandbox::start(&args[1..]).unwrap_or_else(|e| {
        eprintln!("{} {}", "error:".red(), e);
        process::exit(1);
    });
}
