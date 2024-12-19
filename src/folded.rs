use iter_tools::prelude::*;
use std::fmt::Display;

/// A folded stacktrace
#[derive(Debug)]
pub(crate) struct Folded {
    pub(crate) lines: Vec<Trace>,
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
