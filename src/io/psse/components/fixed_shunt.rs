use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network fixed shunt in the PSS/E .RAW case
pub struct FixedShunt {
    /// ## I: Bus Number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub bus_id: i32,
    /// ## ID: Fixed shunt Identifier
    /// ### Type: String
    /// ### Default: ""
    pub id: String,
    /// ## STATUS: Fixed Shunt status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Fixed Shunt Offline
    /// * 1 : Fixed Shunt Online
    pub status: i8,
    /// ## GL: Active component of shunt admittance to ground
    /// ### Type: f64
    /// ### Units: MW (at one (1.0) p.u. voltage)
    /// ### Default: 0.0
    pub gl_mw: f64,
    /// ## BL: Rective component of shunt admittance to ground
    /// ### Type: f64
    /// ### Units: MW (at one (1.0) p.u. voltage)
    /// ### Default: 0.0
    pub bl_mvar: f64,
}

impl From<Vec<String>> for FixedShunt {

    fn from(values: Vec<String>) -> Self {

        FixedShunt {
            bus_id: values.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
            id: values.get(1).and_then(|s| s.parse().ok()).unwrap_or("1".to_string()),
            status: values.get(2).and_then(|s| s.parse().ok()).unwrap_or(1),
            gl_mw: values.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            bl_mvar: values.get(4).and_then(|s| s.parse().ok()).unwrap_or(0.0)
        }
    }
}

pub fn parse_fixedshunts(lines: &[&[u8]]) -> Vec<FixedShunt> {
    //Check if there is even data before proceeding
    if lines.len() == 0 {return Vec::new();}
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(FixedShunt{
                bus_id: parts[0].parse().unwrap_or(0),
                id: parts[1].replace("'", "").trim().to_string(),
                status: parts[2].parse().unwrap_or(1),
                gl_mw: parts[3].parse().unwrap_or(0.0),
                bl_mvar: parts[4].parse().unwrap_or(0.0)
            })
        })
    }).collect()
}