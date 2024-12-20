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
use folded::Folded;
use inferno::flamegraph;
use std::{
    fs::{self, File},
    io::{Stdout, Write},
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
    let mut writer = match &output {
        Some(output) => Writer::File(File::create(&output)?),
        None => Writer::Stdout(std::io::stdout()),
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

enum Writer {
    File(File),
    Stdout(Stdout),
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Writer::File(file) => file.write(buf),
            Writer::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Writer::File(file) => file.flush(),
            Writer::Stdout(stdout) => stdout.flush(),
        }
    }
}
