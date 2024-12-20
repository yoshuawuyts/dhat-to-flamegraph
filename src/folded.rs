use iter_tools::prelude::*;
use std::fmt::Display;

use crate::dhat::Dhat;

/// A folded stacktrace
#[derive(Debug)]
pub(crate) struct Folded {
    pub(crate) lines: Vec<Trace>,
}

impl Folded {
    pub(crate) fn from_dhat(dhat: Dhat) -> Self {
        let lines = dhat
            .program_points
            .iter()
            .map(|program_point| {
                let trace = program_point
                    .frames
                    .iter()
                    .map(|frame_idx| dhat.frame_table[*frame_idx].clone())
                    .collect::<Vec<_>>();
                Trace {
                    trace,
                    frequency: program_point.total_bytes,
                }
            })
            .collect::<Vec<_>>();
        Self { lines }
    }
}

impl Display for Folded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

/// A stack trace and a frequency
#[derive(Debug)]
pub(crate) struct Trace {
    pub(crate) trace: Vec<String>,
    pub(crate) frequency: u64,
}

impl Display for Trace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { trace, frequency } = self;
        let trace = Itertools::intersperse(trace.iter(), &";".to_string())
            .cloned()
            .collect::<String>();
        write!(f, "{trace} {frequency}")
    }
}
