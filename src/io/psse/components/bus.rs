use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    //Check if there is even data before proceeding
    if lines.len() == 0 {return Vec::new();}
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
                nvhi: parts.get(9).and_then(|s| s.parse().ok()).unwrap_or(1.1),
                nvlo: parts.get(10).and_then(|s| s.parse().ok()).unwrap_or(0.9),
                evhi: parts.get(11).and_then(|s| s.parse().ok()).unwrap_or(1.1),
                evlo: parts.get(12).and_then(|s| s.parse().ok()).unwrap_or(0.9),
            })
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bus_v35() {
        let bus_str: &'static str = "11223, 'TESTBUS',   235.0000,   5, 806, 5560,  890,   1.026054109, 228.564689898,     1.1000,     0.9000,     1.1000,     0.9000";
        let buses: Vec<&[u8]> = bus_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<Bus> = parse_buses(&buses);
        let bus: Option<&Bus> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(bus.unwrap().id, 11223);
        assert_eq!(bus.unwrap().name, "TESTBUS".to_string());
        assert_eq!(bus.unwrap().base_kv, 235.0);
        assert_eq!(bus.unwrap().type_code, 5);
        assert_eq!(bus.unwrap().evhi, 1.1);
        assert_eq!(bus.unwrap().evlo, 0.9);
    }
}