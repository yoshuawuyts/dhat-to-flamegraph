//! Convert dhat JSON output to a collapsed flamegraph format
//!
//! ## Usage
//!
//! ```text
//! Convert dhat JSON output to a flamegraph
//!
//! Usage: dhat-to-flamegraph [OPTIONS] <INPUT>
//!
//! Arguments:
//!   <INPUT>
//!           The dhat JSON file to process
//!
//! Options:
//!   -o, --output <OUTPUT>
//!           Where to place the output
//!
//!           If not provided then stdout is used.
//!
//!   -f, --format <FORMAT>
//!           Which output format to use
//!
//!           Possible values:
//!           - svg:    Format as svg (default)
//!           - folded: Format as folded stack traces
//!
//!   -h, --help
//!           Print help (see a summary with '-h')
//! ```
//!
//! Usage example:
//!
//! ```bash
//! dhat-to-flamegraph fixtures/dhat-heap.json > out.svg
//! open out.svg
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

use clap::Parser;
use folded::Folded;
use inferno::flamegraph;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    str::FromStr,
};

mod dhat;
mod folded;

/// Convert dhat JSON output to a flamegraph
#[derive(Parser)]
struct Args {
    /// The dhat JSON file to process
    input: PathBuf,
    /// Where to place the output
    ///
    /// If not provided then stdout is used.
    #[clap(short, long)]
    output: Option<PathBuf>,
    /// Which output format to use
    #[clap(short, long)]
    format: Option<Format>,
}

#[derive(clap::ValueEnum, Clone, Copy, Default)]
enum Format {
    /// Format as svg (default)
    #[default]
    Svg,
    /// Format as folded stack traces
    Folded,
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "svg" => Ok(Self::Svg),
            "folded" => Ok(Self::Folded),
            s => Err(s.into()),
        }
    }
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    let Args {
        input,
        output,
        format,
    } = Args::parse();
    let file = fs::File::open(input)?;

    // Convert dhat to lines
    let dhat: dhat::Dhat = serde_json::from_reader(file)?;
    let folded = Folded::from_dhat(dhat).to_string();

    // Determine where to write the data to
    let writer = match &output {
        Some(output) => &mut File::create(&output)? as &mut dyn Write,
        None => &mut std::io::stdout(),
    };

    // Write the data
    match format.unwrap_or_default() {
        Format::Folded => write!(writer, "{folded}")?,
        Format::Svg => {
            let mut opts = flamegraph::Options::default();
            flamegraph::from_lines(&mut opts, folded.lines(), writer)?;
        }
    }
    if let Some(output) = output {
        eprintln!("wrote {output:?}");
    }
    Ok(())
}
