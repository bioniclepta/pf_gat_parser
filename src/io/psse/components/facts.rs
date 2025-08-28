use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Facts {
    /// ## NAME: FACTS name
    /// ### Type: String
    /// ### Default: " "
    pub deivce_name: String,
    /// ## I: Bus number the FACTS leaves from
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub from_bus: i32,
    /// ## J: Bus number the FACTS goes to
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub to_bus: i32,
    /// ## MODE: FACTS control mode
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Out of service
    /// * 1 : In service
    /// * 2 : Series link bypassed and shunt link operating as a STATCOM
    /// * 3 : Series link operating at constant series impedance
    /// * 4 : Series link operating at constant series voltage
    /// * 5 : Master of device with PQ specified
    /// * 6 : Slave of device with PQ specified
    /// * 7 : Master of device with constant series voltage specified
    /// * 8 : Slave of device with constant series voltage specified
    pub control_mode: i8,
    /// ## PDES: The desired active power arriving at the terminal end bus
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 0.0
    pub desired_active_power: f64,
    /// ## QDES: The desired reactive power arriving at the terminal end bus
    /// ### Type: f64
    /// ### Units: MVAR
    /// ### Default: 0.0
    pub desired_reactive_power: f64,
    /// ## VSET: The voltage setpoint at the sending bus
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.0
    pub from_bus_volt_setpoint: f64,
    /// ## SHMX: Maximum shunt current at the sending bus
    /// ### Type: f64
    /// ### Units: MVA (at unity voltage)
    /// ### Default: 9999.0
    pub max_shunt_current: f64,
    /// ## TRMX: Maximum bridge active power transfer
    /// ### Type: f64
    /// ### Units: MW
    /// ### Default: 9999.0
    pub max_bridge_active_power: f64,
    /// ## VTMN: Minimum voltage at the terminal end bus
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.9
    pub to_bus_min_voltage: f64,
    /// ## VTMX: Maximum voltage at the terminal end bus
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.1
    pub to_bus_max_voltage: f64,
    /// ## VSMX: Maximum series voltage
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.0
    pub max_series_voltage: f64,
    /// ## IMX: Maximum series current
    /// ### Type: f64
    /// ### Units: MVA (at unity voltage)
    /// ### Default: 0.0
    pub max_series_current: f64,
    /// ## LINX: Reactance of the dummy series element
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.05
    pub series_reactance: f64,
    /// ## RMPCT: The percent of the total MVAR required to hold the voltage at the bus controlled by the shunt element of the FACTS
    /// ### Type: f64
    /// ### Default: 100.0
    pub shunt_mvar_share: f64,
    /// ## OWNER: FACTS owner number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner: i16,
    /// ## VSREF: FACTS series voltage reference code
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Sending end voltage
    /// * 1 : Series current
    pub volt_ref_code: i8,
    /// ## FCREG: Bus ID to be regulated by the shunt element of the FACTS device
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub regulated_bus_id: i32,
    /// ## NREG: Bus node to be regulated by the shunt element of the FACTS device
    /// ### Type: i32
    /// ### Default: 0
    pub regulated_bus_node: i32,
    /// ## MNAME: FACTS name that is the IPFC master device
    /// ### Type: String
    /// ### Default: " "
    pub master_device_name: String
}

pub fn parse_facts(lines: &[&[u8]], psse_version: i8) -> Vec<Facts> {
    let parse_adder: usize = (psse_version >= 34) as usize;

    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Facts{
                deivce_name: parts[0].replace("'", "").replace("\"", "").trim().to_string(),
                from_bus: parts[1].parse().unwrap_or(0),
                to_bus: parts[2].parse().unwrap_or(0),
                control_mode: parts[3].parse().unwrap_or(1),
                desired_active_power: parts[4].parse().unwrap_or(0.0),
                desired_reactive_power: parts[5].parse().unwrap_or(0.0),
                from_bus_volt_setpoint: parts[6].parse().unwrap_or(1.0),
                max_shunt_current: parts[7].parse().unwrap_or(9999.0),
                max_bridge_active_power: parts[8].parse().unwrap_or(9999.0),
                to_bus_min_voltage: parts[9].parse().unwrap_or(0.9),
                to_bus_max_voltage: parts[10].parse().unwrap_or(1.1),
                max_series_voltage: parts[11].parse().unwrap_or(1.0),
                max_series_current: parts[12].parse().unwrap_or(0.0),
                series_reactance: parts[13].parse().unwrap_or(0.05),
                shunt_mvar_share: parts[14].parse().unwrap_or(100.0),
                owner: parts[15].parse().unwrap_or(1),
                volt_ref_code: parts[16].parse().unwrap_or(0),
                regulated_bus_id: parts[17].parse().unwrap_or(0),
                regulated_bus_node: (psse_version >= 34).then_some(parts[18].parse().unwrap_or(0)).unwrap_or(0),
                master_device_name: parts[18 + parse_adder].replace("'", "").trim().to_string(),
            })
        })
    }).collect()
}