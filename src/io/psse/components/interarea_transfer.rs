use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each inter-area transfer listing in the PSS/E .RAW case
pub struct InterAreaTransfer {
    /// ## ARFROM: Transfer from area number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub from_area: i32,
    /// ## ARTO: Transfer to area number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub to_area: i32,
    /// ## TRID: Transfer identifier
    /// ### Type: String
    /// ### Default: '1'
    pub transfer_id: String,
    /// ## PTRAN: Power to transfer
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub power_transfer: f64,
}

pub fn parse_area_transfers(lines: &[&[u8]]) -> Vec<InterAreaTransfer> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(InterAreaTransfer{
                from_area: parts[0].parse().unwrap_or(0),
                to_area: parts[1].parse().unwrap_or(0),
                transfer_id: parts[2].replace("'", "").trim().to_string(),
                power_transfer: parts[3].parse().unwrap_or(0.0),
            })
        })
    }).collect()
}