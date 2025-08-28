use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each zone in the PSS/E .RAW case
pub struct Zone {
    /// ## I: Zone number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub zone_id: i32,
    /// ## NAME: Zone name
    /// ### Type: String
    /// ### Default: " "
    pub zone_name: String,
}

pub fn parse_zones(lines: &[&[u8]]) -> Vec<Zone> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Zone{
                zone_id: parts[0].parse().unwrap_or(0),
                zone_name: parts[1].replace("'", "").trim().to_string(),
            })
        })
    }).collect()
}