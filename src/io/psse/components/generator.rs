use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network generator in the PSS/E .RAW case
pub struct Generator {
    /// ## I: Bus number load is at
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub bus_id: i32,
    /// ## ID: Generator identifier
    /// ### Type: String
    /// ### Default: '1'
    pub id: String,
    /// ## PG: Generator active power output
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub pgen: f64,
    /// ## QG: Generator reactive power output
    /// ### Type: f64
    /// ### Units: Mvar
    /// ### Default: 0.0
    pub qgen: f64,
    /// ## QT: Maximum Generator reactive power output
    /// ### Type: f64
    /// ### Units: Mvar
    /// ### Default: 9999.0
    pub qmax: f64,
    /// ## QB: Minimum Generator reactive power output
    /// ### Type: f64
    /// ### Units: Mvar
    /// ### Default: -9999.0
    pub qmin: f64,
    /// ## VS: Regulated voltage setpoint
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.0
    pub voltage_set: f64,
    /// ## IREG: Regulated bus id
    /// ### Type: i32
    /// ### Default: 0
    pub reg_bus_id: i32,
    /// ## NREG: Regulated bus node number
    /// ### Type: i32
    /// ### Default: 0
    pub reg_bus_node: i32,
    /// ## MBASE: MVA Base of generation
    /// ### Type: f64
    /// ### Units: MVA
    /// ### Default: 100.0
    pub mbase: f64,
    /// ## ZR: Complex machine impedance (Only used in Dynamics)
    /// ### Type: f64
    /// ### Units: MVA
    /// ### Default: 0.0
    pub zr: f64,
    /// ## ZX: Complex machine impedance (Only used in Dynamics)
    /// ### Type: f64
    /// ### Units: MVA
    /// ### Default: 1.0
    pub zx: f64,
    /// ## RT: Step up transformer impedance
    /// ### Type: f64
    /// ### Units: p.u (on MVA base)
    /// ### Default: 0.0
    pub rt: f64,
    /// ## XT: Step up transformer impedance
    /// ### Type: f64
    /// ### Units: p.u (on MVA base)
    /// ### Default: 0.0
    pub xt: f64,
    /// ## GTAP: Step up transformer off-nominal turns ratio
    /// ### Type: f64
    /// ### Units: p.u (on MVA base)
    /// ### Default: 0.0
    pub gtap: f64,
    /// ## STATUS: Generator status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Generator Offline
    /// * 1 : Generator Online
    pub status: i8,
    /// ## RMPCT: Percent of total MVAR required to hold the voltage at the bus controlled by bus I
    /// ### Type: f64
    /// ### Units: p.u (on MVA base)
    /// ### Default: 100.0
    pub rmpct: f64,
    /// ## PT: Maximum Generator active power output
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 9999.0
    pub pmax: f64,
    /// ## PB: Minimum Generator active power output
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: -9999.0
    pub pmin: f64,
    /// ## BASLOD: Generator base loaded flag
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Generator is not base loaded
    /// * 1 : Generator can only be scaled down
    /// * 2 : Generator can not be scaled
    /// * 3 : Generator can only be scaled up
    pub base_loaded: i8,
    /// ## O1: Generator owner number 1
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner1: i16,
    /// ## O2: Generator owner number 2
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner2: i16,
    /// ## O3: Generator owner number 3
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner3: i16,
    /// ## O4: Generator owner number 4
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner4: i16,
    /// ## F1: Generator ownership percentage 1
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner1_percent: f64,
    /// ## F2: Generator ownership percentage 2
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner2_percent: f64,
    /// ## F3: Generator ownership percentage 3
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner3_percent: f64,
    /// ## F4: Generator ownership percentage 4
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner4_percent: f64,
    /// ## WMOD: Generator control mode
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Conventional (synchronous) machine
    /// * 1 : Renewable machine with reactive power limits specified by qmin and qmax
    /// * 2 : Renewable machine with reactive power limits specified by pgen and power factor
    /// * 3 : Renewable machine with fixed reactive power settings determined by pgen and power factor
    /// * 4 : Infeed machine
    pub machine_mode: i8,
    /// ## WPF: Generator power factor (Used when machine_mode is 2 or 3)
    /// ### Type: f64
    /// ### Default: 1.0
    pub machine_powerfactor: f64,
}

pub fn parse_generators(lines: &[&[u8]], psse_version: i8) -> Vec<Generator> {
    // a bool to int to add to the line parsing since V34 added more ratings
    let parse_adder: usize = (psse_version >= 34) as usize;
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Generator{
                bus_id: parts[0].parse().unwrap_or(0),
                id: parts[1].replace("'", "").trim().to_string(),
                pgen: parts[2].parse().unwrap_or(0.0),
                qgen: parts[3].parse().unwrap_or(0.0),
                qmax: parts[4].parse().unwrap_or(9999.0),
                qmin: parts[5].parse().unwrap_or(-9999.0),
                voltage_set: parts[6].parse().unwrap_or(1.0),
                reg_bus_id: parts[7].parse().unwrap_or(0),
                reg_bus_node: parts[8].parse().unwrap_or(0) * (parse_adder as i32),
                mbase: parts[8 + parse_adder].parse().unwrap_or(100.0),
                zr: parts[9 + parse_adder].parse().unwrap_or(0.0),
                zx: parts[10 + parse_adder].parse().unwrap_or(1.0),
                rt: parts[11 + parse_adder].parse().unwrap_or(0.0),
                xt: parts[12 + parse_adder].parse().unwrap_or(0.0),
                gtap: parts[13 + parse_adder].parse().unwrap_or(1.0),
                status: parts[14 + parse_adder].parse().unwrap_or(1),
                rmpct: parts[15 + parse_adder].parse().unwrap_or(100.0),
                pmax: parts[16 + parse_adder].parse().unwrap_or(9999.0),
                pmin: parts[17 + parse_adder].parse().unwrap_or(-9999.0),
                base_loaded: parts[18 + parse_adder].parse().unwrap_or(0),
                owner1: parts.get(19 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(1),
                owner1_percent: parts.get(20 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner2: parts.get(21 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner2_percent: parts.get(22 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner3: parts.get(23 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner3_percent: parts.get(24 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner4: parts.get(25 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner4_percent: parts.get(26 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                machine_mode: parts.get(27 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                machine_powerfactor: parts.get(28 + parse_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
            })
        })
    }).collect()
}