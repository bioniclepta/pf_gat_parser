use std::str::from_utf8;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network voltage source converter (VSC) DC branch in the PSS/E .RAW case
pub struct MultiTermDCLine {
    /// ## NAME: DC line name
    /// ### Type: String
    /// ### Default: " "
    pub name: String,
    /// ## NCONV: The number of AC converter station buses in the multi-terminal DC line
    /// ### Type: i32
    /// ### Default: 0
    pub num_converters: i32,
    /// ## NDCBS: The number of DC buses in the multi-terminal DC line
    /// ### Type: i32
    /// ### Default: 0
    pub num_dc_buses: i32,
    /// ## NDCLN: The number of DC links in the multi-terminal DC line
    /// ### Type: i32
    /// ### Default: 0
    pub num_dc_links: i32,
    /// ## MDC: Control mode
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Multi-terminal DC line blocked
    /// * 1 : Power control
    /// * 2 : current control
    pub control_mode: i8,
    /// ## VCONV: Bus ID of the AC converter station that controls the voltage on the positive pole of the multi-terminal DC line
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub positive_ac_conv_bus_id: i32,
    /// ## VCMOD: Mode switch dc voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub mode_switch_dc_volt: f64,
    /// ## VCONVN: Bus ID of the AC converter station that controls the voltage on the negative pole of the multi-terminal DC line
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub negative_ac_conv_bus_id: i32,

    // Line 2

    /// ## IB: AC converter bus ID
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub ac_conv_bus_1_id: i32,
    /// ## N: The number of bridges in series
    /// ### Type: i32
    /// ### Default: 0
    pub series_bridges: i32,
    /// ## ANGMX: Maximum alpha or gamma
    /// ### Type: f64
    /// ### Units: Degrees
    /// ### Default: 0.0
    pub max_alpha_gamma: f64,
    /// ## ANGMX: Minimum alpha or gamma
    /// ### Type: f64
    /// ### Units: Degrees
    /// ### Default: 0.0
    pub min_alpha_gamma: f64,
    /// ## RC: Commutating resistance per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub bridge_r: f64,
    /// ## XC: Commutating reactance per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub bridge_x: f64,
    /// ## EBAS: Primary base AC voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub base_ac_volt: f64,
    /// ## TR: Transformer ratio
    /// ### Type: f64
    /// ### Default: 1.0
    pub xfmr_ratio: f64,
    /// ## TAP: Transformer tap setting
    /// ### Type: f64
    /// ### Default: 1.0
    pub xfmr_tap: f64,
    /// ## TAP: Transformer maximum tap setting
    /// ### Type: f64
    /// ### Default: 1.5
    pub xfmr_max_tap: f64,
    /// ## TAP: Transformer minimum tap setting
    /// ### Type: f64
    /// ### Default: 0.00625
    pub xfmr_min_tap: f64,
    /// ## SETVL: Converter setpoint
    /// ### Type: f64
    /// ### Default: 0.0
    pub conv_setpoint: f64,
    /// ## DCPF: Converter participation factor
    /// ### Type: f64
    /// ### Default: 1.0
    pub conv_participation_factor: f64,
    /// ## DCPF: Rectifier margin
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.0
    pub rec_margin: f64,
    /// ## CNVCOD: Converter code
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * \>=0 : Converter is on the positive pole of the multi-terminal DC line
    /// * < 0 : Converter is on the negative pole of the multi-terminal DC line
    pub conv_code: i8,

    // Line 3

    /// ## IDC: DC converter bus number
    /// ### Type: i32
    /// ### Default: 1
    /// ## Range
    /// * 1 - num_converters (NDCBS)
    pub dc_bus_num: i32,
    /// ## IB: AC converter bus number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 0 - 999997
    pub ac_conv_bus_2_id: i32,
    /// ## AREA: Multi-terminal DC line area number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub area: i16,
    /// ## ZONE: Multi-terminal DC line zone number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub zone: i16,
    /// ## DCNAME: DC bus name
    /// ### Type: String
    /// ### Default: " "
    pub dc_bus_name: String,
    /// ## IDC2: Second DC converter bus number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 0 - num_converters (NDCBS)
    pub dc_bus_num_2: i32,
    /// ## RGRND: Resistance to ground at dc_bus_num (IDC)
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub resist_to_ground: f64,
    /// ## OWNER: Multi-terminal DC line owner number
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner: i16,
}

pub fn parse_multiterminal_dc_line(lines: &[&[u8]]) -> Vec<MultiTermDCLine> {
    //Check if there is even data before proceeding
    if lines.len() == 0 {return Vec::new();}
    let mut mt_dc_lines: Vec<MultiTermDCLine> = Vec::new();
    let mut i: usize = 0;
    while i < lines.len() {
        if let (Ok(line1_str), Ok(line2_str), Ok(line3_str)) = (
            from_utf8(lines[i]), from_utf8(lines[i + 1]), from_utf8(lines[i + 2])
        ) {
            let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
            let parts2: Vec<&str> = line2_str.split(',').map(|s| s.trim()).collect();
            let parts3: Vec<&str> = line3_str.split(',').map(|s| s.trim()).collect();

            let mt_dc_line: MultiTermDCLine = MultiTermDCLine {
                name: parts1[0].replace("'", "").trim().to_string(),
                num_converters: parts1[1].parse().unwrap_or(0),
                num_dc_buses: parts1[2].parse().unwrap_or(0),
                num_dc_links: parts1[3].parse().unwrap_or(0),
                control_mode: parts1[4].parse().unwrap_or(0),
                positive_ac_conv_bus_id: parts1[5].parse().unwrap_or(0),
                mode_switch_dc_volt: parts1[6].parse().unwrap_or(0.0),
                negative_ac_conv_bus_id: parts1[7].parse().unwrap_or(0),

                // Line 2

                ac_conv_bus_1_id: parts2[0].parse().unwrap_or(0),
                series_bridges: parts2[1].parse().unwrap_or(0),
                max_alpha_gamma: parts2[2].parse().unwrap_or(0.0),
                min_alpha_gamma: parts2[3].parse().unwrap_or(0.0),
                bridge_r: parts2[4].parse().unwrap_or(0.0),
                bridge_x: parts2[5].parse().unwrap_or(0.0),
                base_ac_volt: parts2[6].parse().unwrap_or(0.0),
                xfmr_ratio: parts2[7].parse().unwrap_or(1.0),
                xfmr_tap: parts2[8].parse().unwrap_or(1.0),
                xfmr_max_tap: parts2[9].parse().unwrap_or(1.5),
                xfmr_min_tap: parts2[10].parse().unwrap_or(0.00625),
                conv_setpoint: parts2[11].parse().unwrap_or(0.0),
                conv_participation_factor: parts2[12].parse().unwrap_or(1.0),
                rec_margin: parts2[13].parse().unwrap_or(0.0),
                conv_code: parts2[14].parse().unwrap_or(1),

                // Line 3
                dc_bus_num: parts3[0].parse().unwrap_or(1),
                ac_conv_bus_2_id: parts3[1].parse().unwrap_or(0),
                area: parts3[2].parse().unwrap_or(1),
                zone: parts3[3].parse().unwrap_or(1),
                dc_bus_name: parts3[4].replace("'", "").trim().to_string(),
                dc_bus_num_2: parts3[5].parse().unwrap_or(1),
                resist_to_ground: parts3[6].parse().unwrap_or(0.0),
                owner: parts3[7].parse().unwrap_or(1),
            };
            
            mt_dc_lines.push(mt_dc_line);

            i += 3;
        }        
    }
    mt_dc_lines
}