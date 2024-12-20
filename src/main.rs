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
//!   -m, --metric <METRIC>
//!           Possible values:
//!           - total:    Measure all traces, output total memory usage per trace (default)
//!           - max:      Measure all traces, output max memory usage per trace
//!           - end:      Measure only the remaining traces at program end, useful to find leaks
//!           - heap-max: Measure only the traces at max heap usage, useful to find spikes
//!
//!   -u, --unit <UNIT>
//!           Possible values:
//!           - bytes:     Measure allocations in bytes (default)
//!           - blocks:    Measure allocations in blocks, useful to find allocation counts
//!           - lifetimes: Measure allocations in lifetimes, useful to find short-lived allocations
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
use metric::Metric;
use std::{
    fs::{self, File},
    io::{Stdout, Write},
    path::PathBuf,
};
use unit::Unit;

mod dhat;
mod folded;
mod metric;
mod unit;

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
    #[clap(short, long)]
    metric: Option<Metric>,
    #[clap(short, long)]
    unit: Option<Unit>,
}

#[derive(clap::ValueEnum, Clone, Copy, Default)]
enum Format {
    /// Format as svg (default)
    #[default]
    Svg,
    /// Format as folded stack traces
    Folded,
}

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    let Args {
        input,
        output,
        format,
        metric,
        unit,
    } = Args::parse();
    let file = fs::File::open(input)?;

    // Convert dhat to lines
    let dhat: dhat::Dhat = serde_json::from_reader(file)?;
    let metric = metric.unwrap_or_default();
    let unit = unit.unwrap_or_default();
    let folded = Folded::from_dhat(dhat, metric, unit).to_string();

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
