use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each area in the PSS/E .RAW case
pub struct Area {
    /// ## I: Area Number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub area_id: i32,
    /// ## ISW: Bus number of the slack bus in the area
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub swing_bus_id: i32,
    /// ## PDES: Net export from the area
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub desired_interchange: f64,
    /// ## PTOL: Area interchange tolerance bandwith
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub mw_tolerance: f64,
    /// ## ARNAME: Area name
    /// ### Type: String
    /// ### Default: ""
    pub name: String,
}

pub fn parse_areas(lines: &[&[u8]]) -> Vec<Area> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Area{
                area_id: parts[0].parse().unwrap_or(0),
                swing_bus_id: parts[1].parse().unwrap_or(0),
                desired_interchange: parts[2].parse().unwrap_or(0.0),
                mw_tolerance: parts[3].parse().unwrap_or(0.0),
                name: parts[4].replace("'", "").trim().to_string(),
            })
        })
    }).collect()
}