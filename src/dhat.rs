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
pub(crate) struct Dhat {
    /// Version number of the format. Incremented on each
    /// backwards-incompatible change. A mandatory integer.
    pub(crate) dhatFileVersion: u32,
    /// The invocation mode. A mandatory, free-form string.
    pub(crate) mode: String,
    /// The verb used before above stack frames, i.e. "<verb> at {". A
    /// mandatory string.
    pub(crate) verb: String,
    /// Are block lifetimes recorded? Affects whether some other fields are
    /// present. A mandatory boolean.
    pub(crate) bklt: bool,
    /// Are block lifetimes recorded? Affects whether some other fields are
    /// present. A mandatory boolean.
    pub(crate) bkacc: bool,
    /// Byte/bytes/blocks-position units. Optional strings. "byte", "bytes",
    /// and "blocks" are the values used if these fields are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bsu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bksu: Option<String>,
    // Time units (individual and 1,000,000x). Mandatory strings.
    pub(crate) tu: String,
    pub(crate) Mtu: String,
    /// The "short-lived" time threshold, measures in "tu"s.
    /// - bklt=true: a mandatory integer.
    /// - bklt=false: omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tuth: Option<usize>,
    /// The executed command. A mandatory string.
    pub(crate) cmd: String,
    // The process ID. A mandatory integer.
    pub(crate) pid: u32,
    /// The time of the global max (t-gmax).
    /// - bklt=true: a mandatory integer.
    /// - bklt=false: omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tg: Option<u128>,
    /// The time at the end of execution (t-end). A mandatory integer.
    pub(crate) te: u128,
    /// The program points. A mandatory array.
    #[serde(rename = "pps")]
    pub(crate) program_points: Vec<ProgramPoint>,
    /// Frame table. A mandatory array of strings.
    #[serde(rename = "ftbl")]
    pub(crate) frame_table: Vec<String>,
}

// A Rust representation of a PpInfo within DHAT's JSON file format.
#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub(crate) struct ProgramPoint {
    /// Total bytes and blocks. Mandatory integers.
    #[serde(rename = "tb")]
    pub(crate) total_bytes: u64,
    pub(crate) tbk: u64,

    /// Total lifetimes of all blocks allocated at this PP.
    /// - bklt=true: a mandatory integer.
    /// - bklt=false: omitted.
    // Derived from `PpInfo::total_lifetimes_duration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tl: Option<u128>,

    /// The maximum bytes and blocks for this PP.
    /// - bklt=true: mandatory integers.
    /// - bklt=false: omitted.
    // `PpInfo::max_bytes` and `PpInfo::max_blocks`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mb: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mbk: Option<usize>,

    /// The bytes and blocks at t-gmax for this PP.
    /// - bklt=true: mandatory integers.
    /// - bklt=false: omitted.
    // `PpInfo::at_tgmax_bytes` and `PpInfo::at_tgmax_blocks`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) gb: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) gbk: Option<usize>,

    /// The bytes and blocks at t-end for this PP.
    /// - bklt=true: mandatory integers.
    /// - bklt=false: omitted.
    // `PpInfo::curr_bytes` and `PpInfo::curr_blocks` (at termination, i.e.
    // "end").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) eb: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ebk: Option<usize>,

    // Frames. Each element is an index into `ftbl`.
    #[serde(rename = "fs")]
    pub(crate) frames: Vec<usize>,
}
