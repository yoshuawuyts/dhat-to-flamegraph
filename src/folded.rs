use iter_tools::prelude::*;
use std::fmt::Display;

use crate::{dhat::Dhat, metric::Metric, unit::Unit};

/// A folded stacktrace
#[derive(Debug)]
pub(crate) struct Folded {
    pub(crate) lines: Vec<Trace>,
}

impl Folded {
    pub(crate) fn from_dhat(dhat: Dhat, metric: Metric, unit: Unit) -> Self {
        let lines: Vec<Trace> = dhat
            .program_points
            .iter()
            .map(|program_point| {
                let trace = program_point
                    .frames
                    .iter()
                    .map(|frame_idx| dhat.frame_table[*frame_idx].clone())
                    .collect::<Vec<_>>();

                if let Unit::Lifetimes = unit {
                    assert!(dhat.bklt, "lifetimes were not recorded in the dhat profile, cannot use lifetime units");
                }
                match metric {
                    Metric::Max => {
                        assert!(dhat.bklt, "lifetimes were not recorded in the dhat profile, cannot compute `max` metric")
                    }
                    Metric::Total => {}
                    Metric::End => 
                        assert!(dhat.bklt, "lifetimes were not recorded in the dhat profile, cannot compute `end` metric"),
                    Metric::HeapMax => 
                        assert!(dhat.bklt, "lifetimes were not recorded in the dhat profile, cannot compute `heap max` metric"),
                }

                let frequency = match (metric, unit) {
                    (Metric::Total, Unit::Bytes) => program_point.total_bytes as u128,
                    (Metric::Total, Unit::Blocks) => program_point.total_blocks as u128,
                    (Metric::Total, Unit::Lifetimes) => program_point.total_lifetimes.unwrap(),
                    (Metric::Max, Unit::Bytes) => program_point.max_bytes.unwrap() as u128,
                    (Metric::Max, Unit::Blocks) => program_point.max_blocks.unwrap() as u128,
                    (Metric::Max, Unit::Lifetimes) => panic!("Only total lifetimes are supported"),
                    (Metric::End, Unit::Bytes) => program_point.end_bytes.unwrap() as u128,
                    (Metric::End, Unit::Blocks) => program_point.end_blocks.unwrap() as u128,
                    (Metric::End, Unit::Lifetimes) => panic!("Only total lifetimes are supported"),
                    (Metric::HeapMax, Unit::Bytes) => program_point.heap_max_bytes.unwrap() as u128,
                    (Metric::HeapMax, Unit::Blocks) => program_point.heap_max_blocks.unwrap() as u128,
                    (Metric::HeapMax, Unit::Lifetimes) => {
                        panic!("Only total lifetimes are supported")
                    }
                };
                Trace { trace, frequency }
            })
            .collect();
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
    pub(crate) frequency: u128,
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
