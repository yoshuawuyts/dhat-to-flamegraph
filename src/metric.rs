/// Which dhat metric to use
#[derive(clap::ValueEnum, Clone, Copy, Default)]
pub(crate) enum Metric {
    /// Measure all traces, output total memory usage per trace (default)
    #[default]
    Total,
    /// Measure all traces, output max memory usage per trace
    Max,
    /// Measure only the remaining traces at program end, useful to find leaks
    End,
    /// Measure only the traces at max heap usage, useful to find spikes
    HeapMax,
}
