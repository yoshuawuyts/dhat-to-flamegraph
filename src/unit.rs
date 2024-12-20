/// Which allocation unit to use
#[derive(clap::ValueEnum, Clone, Copy, Default)]
pub(crate) enum Unit {
    /// Measure allocations in bytes (default)
    #[default]
    Bytes,
    /// Measure allocations in blocks, useful to find allocation counts
    Blocks,
    /// Measure allocations in lifetimes, useful to find short-lived allocations
    Lifetimes,
}
