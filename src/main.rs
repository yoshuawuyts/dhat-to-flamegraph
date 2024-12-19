//! Convert dhat JSON output to a collapsed flamegraph format
//!
//! ## Usage
//!
//! ```text
//! Usage: dhat-to-flamegraph <INPUT> [OUTPUT]
//!
//! Arguments:
//!   <INPUT>   The dhat JSON file
//!   [OUTPUT]  Where to write the output file [default: dhat.folded]
//!
//! Options:
//!   -h, --help  Print help
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

fn main() -> Result<(), Error> {
    let Args { input, output } = Args::parse();
    let file = fs::File::open(input)?;
    let dhat: dhat::Dhat = serde_json::from_reader(file)?;
    let lines = dhat
        .program_points
        .iter()
        .map(|program_point| {
            let trace = program_point
                .frames
                .iter()
                .map(|frame_idx| dhat.frame_table[*frame_idx].clone())
                .collect::<Vec<_>>();
            folded::Trace {
                trace,
                frequency: program_point.total_bytes,
            }
        })
        .collect::<Vec<_>>();
    let folded = folded::Folded { lines };
    fs::write(&output, folded.to_string())?;
    println!("wrote to {output:?}");
    Ok(())
}
