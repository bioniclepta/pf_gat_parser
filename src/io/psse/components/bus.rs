use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network bus in the PSS/E .RAW case
pub struct Bus {
    /// ## I: Bus Number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub id: i32,
    /// ## NAME: Bus Name
    /// ### Type: String
    /// ### Default: twelve (12) blanks
    pub name: String,
    /// ## BASKV: Bus base voltage (kV)
    /// ### Type: f64
    /// ### Default: 0.0
    pub base_kv: f64,
    /// ## IDE: Bus type code
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : Load / Passive Bus
    /// * 2 : Generator / Plant Bus
    /// * 3 : Swing Bus
    /// * 4 : Disconnected Bus (isolated)
    pub type_code: i8,
    /// ## AREA: Bus area number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub area: i16,
    /// ## ZONE: Bus zone number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub zone: i16,
    /// ## OWNER: Bus owner number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner: i16,
    /// ## VM: Bus voltage magnitude
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.0
    pub vm_pu: f64,
    /// ## VA: Bus voltage phase angle
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.0
    pub va_deg: f64,
    /// ## NVHI: Normal voltage magnitude high limit
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.1
    pub nvhi: f64,
    /// ## NVLO: Normal voltage magnitude low limit
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.9
    pub nvlo: f64,
    /// ## EVHI: Emergency voltage magnitude high limit
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.1
    pub evhi: f64,
    /// ## EVLO: Emergency voltage magnitude low limit
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.9
    pub evlo: f64,
}

pub fn parse_buses(lines: &[&[u8]]) -> Vec<Bus> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Bus{
                id: parts[0].parse().unwrap_or(0),
                name: parts[1].replace("'", "").trim().to_string(),
                base_kv: parts[2].parse().unwrap_or(999.99),
                type_code: parts[3].parse().unwrap_or(1),
                area: parts[4].parse().unwrap_or(0),
                zone: parts[5].parse().unwrap_or(0),
                owner: parts[6].parse().unwrap_or(0),
                vm_pu: parts[7].parse().unwrap_or(0.95),
                va_deg: parts[8].parse().unwrap_or(0.0),
                nvhi: parts[9].parse().unwrap_or(1.1),
                nvlo: parts[10].parse().unwrap_or(0.9),
                evhi: parts[11].parse().unwrap_or(1.1),
                evlo: parts[12].parse().unwrap_or(0.9),
            })
        })
    }).collect()
}