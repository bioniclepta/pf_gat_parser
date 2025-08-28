use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each owner in the PSS/E .RAW case
pub struct Owner {
    /// ## I: Owner number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner_id: i32,
    /// ## OWNAME: Owner name
    /// ### Type: String
    /// ### Default: " "
    pub owner_name: String,
}

pub fn parse_owners(lines: &[&[u8]]) -> Vec<Owner> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Owner{
                owner_id: parts[0].parse().unwrap_or(0),
                owner_name: parts[1].replace("'", "").trim().to_string(),
            })
        })
    }).collect()
}