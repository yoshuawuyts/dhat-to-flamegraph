//! Convert dhat JSON output to a collapsed flamegraph format
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

use clap::Parser;
use std::{env, fs, path::PathBuf};

mod dhat;
mod folded;

#[derive(Parser)]
struct Args {
    /// The dhat JSON file
    input: PathBuf,
    /// Where to write the output file.
    #[arg(default_value=default_output().into_os_string())]
    output: PathBuf,
}

fn default_output() -> PathBuf {
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("dhat.folded");
    path
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub fn main() -> Result<(), Error> {
    let Args { input, output } = Args::parse();
    let file = fs::File::open(input)?;
    let dhat: dhat::DhatJson = serde_json::from_reader(file)?;
    println!("{dhat:#?}");
    Ok(())
}
