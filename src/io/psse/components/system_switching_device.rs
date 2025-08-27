use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network system switching device in the PSS/E .RAW case
pub struct SystemSwitchingDevice {
    /// ## I: Bus number the system switching device leaves from
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub from_bus: i32,
    /// ## J: Bus number the system switching device goes to
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub to_bus: i32,
    /// ## CKT: Switching device circuit identifier
    /// ### Type: String
    /// ### Default: '1'
    pub circuit: String,
    /// ## X: Switching device reactance
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub x: f64,
    /// ## RATE1: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate1: f64,
    /// ## RATE2: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate2: f64,
    /// ## RATE3: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate3: f64,
    /// ## RATE4: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate4: f64,
    /// ## RATE5: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate5: f64,
    /// ## RATE6: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate6: f64,
    /// ## RATE7: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate7: f64,
    /// ## RATE8: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate8: f64,
    /// ## RATE9: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate9: f64,
    /// ## RATE10: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate10: f64,
    /// ## RATE11: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate11: f64,
    /// ## RATE12: Switching device MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate12: f64,
    /// ## STATUS: Switching device Status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Open
    /// * 1 : Closed
    /// * 2 : Stuck closed
    pub status: i8,
    /// ## NSTATUS: Switching device Status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Open
    /// * 1 : Closed
    /// * 2 : Stuck closed
    pub normal_status: i8,
    /// ## METERD: Switching device metered end flag
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * <= 1 : From bus (I) is the metered end of the system switching device
    /// * \>=2 : To bus (J) is the metered end of the system switching device
    pub meter_end: i8,
    /// ## STYPE: Switching device type
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Generic connector
    /// * 1 : Circuit Breaker
    /// * 2 : Disconnect switch
    pub device_type: i8,
    /// ## NAME: Switching device name
    /// ### Type: String
    /// ### Default: " "
    pub name: String,
}

pub fn parse_system_switching_device(lines: &[&[u8]]) -> Vec<SystemSwitchingDevice> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            println!("{:?}", parts);
            Some(SystemSwitchingDevice{
                from_bus: parts[0].parse().unwrap_or(0),
                to_bus: parts[1].parse().unwrap_or(0),
                circuit: parts[2].replace("'", "").trim().to_string(),
                x: parts[3].parse().unwrap_or(0.0001),
                rate1: parts[6].parse().unwrap_or(0.0),
                rate2: parts[7].parse().unwrap_or(0.0),
                rate3: parts[8].parse().unwrap_or(0.0),
                rate4: parts[9].parse().unwrap_or(0.0),
                rate5: parts[10].parse().unwrap_or(0.0),
                rate6: parts[11].parse().unwrap_or(0.0),
                rate7: parts[12].parse().unwrap_or(0.0),
                rate8: parts[13].parse().unwrap_or(0.0),
                rate9: parts[14].parse().unwrap_or(0.0),
                rate10: parts[15].parse().unwrap_or(0.0),
                rate11: parts[16].parse().unwrap_or(0.0),
                rate12: parts[17].parse().unwrap_or(0.0),
                status: parts[18].parse().unwrap_or(1),
                normal_status: parts[19].parse().unwrap_or(1),
                meter_end: parts[20].parse().unwrap_or(1),
                device_type: parts[21].parse().unwrap_or(1),
                name: parts[22].replace("'", "").trim().to_string(),
            })
        })
    }).collect()
}
