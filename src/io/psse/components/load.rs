use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network load in the PSS/E .RAW case
pub struct Load {
    /// ## I: Bus number load is at
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub bus_id: i32,
    /// ## ID: Load identifier
    /// ### Type: String
    /// ### Default: '1'
    pub id: String,
    /// ## STATUS: Load status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Load Offline
    /// * 1 : Load Online
    pub status: i8,
    /// ## AREA: Load area number
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub area: i16,
    /// ## ZONE: Load zone number
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub zone: i16,
    /// ## PL: Active power component of the constant MVA load
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub pl_mw: f64,
    /// ## QL: Rective power component of the constant MVA load
    /// ### Type: f64
    /// ### Units: Mvar
    /// ### Default: 0.0
    pub ql_mvar: f64,
    /// ## IP: Active power component of the constant current load
    /// ### Type: f64
    /// ### Units: MW (at one (1.0) p.u. voltage)
    /// ### Default: 0.0
    pub ip_mw: f64,
    /// ## IQ: Rective power component of the constant current load
    /// ### Type: f64
    /// ### Units: Mvar (at one (1.0) p.u. voltage)
    /// ### Default: 0.0
    pub iq_mvar: f64,
    /// ## YP: Active power component of the constant admittance load
    /// ### Type: f64
    /// ### Units: MW (at one (1.0) p.u. voltage)
    /// ### Default: 0.0
    pub yp_mw: f64,
    /// ## YQ: Rective power component of the constant admittance load
    /// ### Type: f64
    /// ### Units: MW (at one (1.0) p.u. voltage)
    /// ### Default: 0.0
    pub yq_mvar: f64,
    /// ## OWNER: Load owner number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner: i16,
    /// ## SCALE: Load scaling flag
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Fixed Load
    /// * 1 : Scalable Load
    pub scale: i8,
    /// ## INTRPT: Interruptable load flag
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Non-interruptable Load
    /// * 1 : Interruptable Load
    pub interruptable: i8,
    /// ## DGENP: Distributed Generation active power component
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub dgen_mw: f64,
    /// ## DGENQ: Distributed Generation rective power component
    /// ### Type: f64
    /// ### Units: Mvar
    /// ### Default: 0.0
    pub dgen_mvar: f64,
    /// ## DGENM: Distributed Generation operation mode
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Distributed generation offline
    /// * 1 : Distributed generation online
    pub dgen_mode: i8,
    /// ## LOADTYPE: Load descriptor
    /// ### Type: String
    /// ### Default: ""
    pub load_type: String,
}

pub fn parse_loads(lines: &[&[u8]]) -> Vec<Load> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Load{
                bus_id: parts[0].parse().unwrap_or(0),
                id: parts[1].replace("'", "").trim().to_string(),
                status: parts[2].parse().unwrap_or(1),
                area: parts[3].parse().unwrap_or(0),
                zone: parts[4].parse().unwrap_or(0),
                pl_mw: parts[5].parse().unwrap_or(99.99),
                ql_mvar: parts[6].parse().unwrap_or(99.99),
                ip_mw: parts[7].parse().unwrap_or(0.0),
                iq_mvar: parts[8].parse().unwrap_or(0.0),
                yp_mw: parts[9].parse().unwrap_or(0.0),
                yq_mvar: parts[10].parse().unwrap_or(0.0),
                owner: parts[11].parse().unwrap_or(1),
                scale: parts[12].parse().unwrap_or(1),
                interruptable: parts[13].parse().unwrap_or(0),
                dgen_mw: parts.get(14).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                dgen_mvar: parts.get(15).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                dgen_mode: parts.get(16).and_then(|s| s.parse().ok()).unwrap_or(0),
                load_type: parts.get(15).and_then(|s| s.parse().ok()).unwrap_or("".to_string()),
            })
        })
    }).collect()
}