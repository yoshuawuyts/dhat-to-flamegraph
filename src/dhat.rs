use serde::Deserialize;

/// A Rust representation of DHAT's JSON file format, which is described in
/// comments in dhat/dh_main.c in Valgrind's source code.
///
/// Building this structure in order to serialize does take up some memory. We
/// could instead stream the JSON output directly to file ourselves. This would
/// be more efficient but make the code uglier.
// Copied from https://github.com/nnethercote/dhat-rs/blob/b536631fd9d9103d7191b63181f67755b5958ab5/src/lib.rs#L1826
#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub(crate) struct DhatJson {
    /// Version number of the format. Incremented on each
    /// backwards-incompatible change. A mandatory integer.
    dhatFileVersion: u32,
    /// The invocation mode. A mandatory, free-form string.
    mode: String,
    /// The verb used before above stack frames, i.e. "<verb> at {". A
    /// mandatory string.
    verb: String,
    /// Are block lifetimes recorded? Affects whether some other fields are
    /// present. A mandatory boolean.
    bklt: bool,
    /// Are block lifetimes recorded? Affects whether some other fields are
    /// present. A mandatory boolean.
    bkacc: bool,
    /// Byte/bytes/blocks-position units. Optional strings. "byte", "bytes",
    /// and "blocks" are the values used if these fields are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    bu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bsu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bksu: Option<String>,
    // Time units (individual and 1,000,000x). Mandatory strings.
    tu: String,
    Mtu: String,
    /// The "short-lived" time threshold, measures in "tu"s.
    /// - bklt=true: a mandatory integer.
    /// - bklt=false: omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    tuth: Option<usize>,
    /// The executed command. A mandatory string.
    cmd: String,
    // The process ID. A mandatory integer.
    pid: u32,
    /// The time of the global max (t-gmax).
    /// - bklt=true: a mandatory integer.
    /// - bklt=false: omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    tg: Option<u128>,
    /// The time at the end of execution (t-end). A mandatory integer.
    te: u128,
    /// The program points. A mandatory array.
    pps: Vec<PpInfoJson>,
    /// Frame table. A mandatory array of strings.
    ftbl: Vec<String>,
}

// A Rust representation of a PpInfo within DHAT's JSON file format.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub(crate) struct PpInfoJson {
    /// Total bytes and blocks. Mandatory integers.
    tb: u64,
    tbk: u64,

    /// Total lifetimes of all blocks allocated at this PP.
    /// - bklt=true: a mandatory integer.
    /// - bklt=false: omitted.
    // Derived from `PpInfo::total_lifetimes_duration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    tl: Option<u128>,

    /// The maximum bytes and blocks for this PP.
    /// - bklt=true: mandatory integers.
    /// - bklt=false: omitted.
    // `PpInfo::max_bytes` and `PpInfo::max_blocks`.
    #[serde(skip_serializing_if = "Option::is_none")]
    mb: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mbk: Option<usize>,

    /// The bytes and blocks at t-gmax for this PP.
    /// - bklt=true: mandatory integers.
    /// - bklt=false: omitted.
    // `PpInfo::at_tgmax_bytes` and `PpInfo::at_tgmax_blocks`.
    #[serde(skip_serializing_if = "Option::is_none")]
    gb: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gbk: Option<usize>,

    /// The bytes and blocks at t-end for this PP.
    /// - bklt=true: mandatory integers.
    /// - bklt=false: omitted.
    // `PpInfo::curr_bytes` and `PpInfo::curr_blocks` (at termination, i.e.
    // "end").
    #[serde(skip_serializing_if = "Option::is_none")]
    eb: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebk: Option<usize>,

    // Frames. Each element is an index into `ftbl`.
    fs: Vec<usize>,
}
