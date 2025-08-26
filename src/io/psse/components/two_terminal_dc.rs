use std::str::from_utf8;




#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network two-terminal DC branch in the PSS/E .RAW case
pub struct TwoTerminalDc {
    /// ## NAME: Two-terminal DC line name
    /// ### Type: String
    /// ### Default: " "
    pub name: String,
    /// ## MDC: Two-terminal DC line control mode
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Two-terminal DC line blocked
    /// * 1 : Two-terminal DC line power controlled
    /// * 2 : Two-terminal DC line current controlled
    pub power_ctrl_mode: i8,
    /// ## RDC: Two-terminal DC line resistance
    /// ### Type: f64
    /// ### Default: 0.0
    pub resistance: f64,
    /// ## SETVL: Two-terminal DC line power output
    /// ### Type: f64
    /// ### Units: MW or A depending on power_ctrl_mode (MDC)
    /// ### Default: 0.0
    pub sending_pow: f64,
    /// ## VSCHD: Two-terminal DC line scheduled voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub voltage: f64,
    /// ## VCMOD: Voltage setpoint to switch to current controlled mode
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub voltage_ctrl_mode: f64,
    /// ## RCOMP: Two-terminal DC line compounding resistance
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub r_comp: f64,
    /// ## DELTI: Margin of the desired DC current/power
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 0.0
    pub dc_pow_margin: f64,
    /// ## METER: Metered end of the Two-terminal DC line
    /// ### Type: String
    /// ### Default: "I"
    /// /// ## Parameters
    /// * "I" : Meter at the inverter terminal
    /// * "R" : Meter at the rectifier terminal
    pub metered_end: String,
    /// ## DCVMIN: Two-terminal DC line minimum compounded DC voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub min_volt: f64,
    /// ## CCCITMX: Iteration limit during Newton solution for a capacitor commutated two-terminal DC line
    /// ### Type: i32
    /// ### Default: 20
    pub iteration_limit: i32,
    /// ## CCCACC: Acceleration factor during Newton solution for a capacitor commutated two-terminal DC line
    /// ### Type: f64
    /// ### Default: 1.0
    pub acceleration_fac: f64,

    //Line 2 - Rectifier Side

    /// ## IPR: Rectifier bus number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub rec_bus_id: i32,
    /// ## NBR: Number of bridges in series on the rectifier side
    /// ### Type: i32
    /// ### Default: 0
    pub rec_num_bridges: i32,
    /// ## ANMXR: Maximum rectifier firing angle
    /// ### Type: f64
    /// ### Units: Degrees
    /// ### Default: 0.0
    pub rec_max_firing_angle: f64,
    /// ## ANMNR: Minimum steady-state rectifier firing angle
    /// ### Type: f64
    /// ### Units: Degrees
    /// ### Default: 0.0
    pub rec_min_firing_angle: f64,
    /// ## RCR: Rectifier commutating transformer resistance per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub rec_comm_xfmr_r: f64,
    /// ## XCR: Rectifier commutating transformer reactance per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub rec_comm_xfmr_x: f64,
    /// ## EBASR: Rectifier primary base AC voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub rec_base_volt_ac: f64,
    /// ## TRR: Rectifier transformer ratio
    /// ### Type: f64
    /// ### Default: 1.0
    pub rec_xfmr_ratio: f64,
    /// ## TAPR: Rectifier tap setting
    /// ### Type: f64
    /// ### Default: 1.0
    pub rec_tap_setting: f64,
    /// ## TMXR: Rectifier maximum tap setting
    /// ### Type: f64
    /// ### Default: 1.5
    pub rec_max_tap_set: f64,
    /// ## TMNR: Rectifier minimum tap setting
    /// ### Type: f64
    /// ### Default: 0.51
    pub rec_min_tap_set: f64,
    /// ## STPR: Rectifier tap step
    /// ### Type: f64
    /// ### Default: 0.00625
    pub rec_tap_step: f64,
    /// ## ICR: Rectifier commutating bus id
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub rec_comm_bus_id: i32,
    /// ## NDR: Rectifier commutating bus node number
    /// ### Type: i32
    /// ### Default: 0
    pub rec_comm_bus_node: i32,
    /// ## IFR: Rectifier winding 1 (from) bus id
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub rec_from_bus: i32,
    /// ## ITR: Rectifier winding 2 (to) bus id
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub rec_to_bus: i32,
    /// ## IDR: Rectifier circuit ID
    /// ### Type: String
    /// ### Default: '1'
    pub rec_ckt_id: String,
    /// ## XCAPR: Rectifier commutating capacitor reactance magnitude per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub rec_comm_cap_x: f64,

    //Line 3 - Inverter Side
    
    /// ## IPI: Inverter bus number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub inv_bus_id: i32,
    /// ## NBI: Number of bridges in series on the inverter side
    /// ### Type: i32
    /// ### Default: 0
    pub inv_num_bridges: i32,
    /// ## ANMXI: Maximum inverter firing angle
    /// ### Type: f64
    /// ### Units: Degrees
    /// ### Default: 0.0
    pub inv_max_firing_angle: f64,
    /// ## ANMNI: Minimum steady-state inverter firing angle
    /// ### Type: f64
    /// ### Units: Degrees
    /// ### Default: 0.0
    pub inv_min_firing_angle: f64,
    /// ## RCI: Inverter commutating transformer resistance per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub inv_comm_xfmr_r: f64,
    /// ## XCI: Inverter commutating transformer reactance per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub inv_comm_xfmr_x: f64,
    /// ## EBASI: Inverter primary base AC voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub inv_base_volt_ac: f64,
    /// ## TRI: Inverter transformer ratio
    /// ### Type: f64
    /// ### Default: 1.0
    pub inv_xfmr_ratio: f64,
    /// ## TAPI: Inverter tap setting
    /// ### Type: f64
    /// ### Default: 1.0
    pub inv_tap_setting: f64,
    /// ## TMXI: Inverter maximum tap setting
    /// ### Type: f64
    /// ### Default: 1.5
    pub inv_max_tap_set: f64,
    /// ## TMNI: Inverter minimum tap setting
    /// ### Type: f64
    /// ### Default: 0.51
    pub inv_min_tap_set: f64,
    /// ## STPI: Inverter tap step
    /// ### Type: f64
    /// ### Default: 0.00625
    pub inv_tap_step: f64,
    /// ## ICI: Inverter commutating bus id
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub inv_comm_bus_id: i32,
    /// ## NDI: Inverter commutating bus node number
    /// ### Type: i32
    /// ### Default: 0
    pub inv_comm_bus_node: i32,
    /// ## IFI: Inverter winding 1 (from) bus id
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub inv_from_bus: i32,
    /// ## ITI: Inverter winding 2 (to) bus id
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub inv_to_bus: i32,
    /// ## IDI: Inverter circuit ID
    /// ### Type: String
    /// ### Default: '1'
    pub inv_ckt_id: String,
    /// ## XCAPI: Inverter commutating capacitor reactance magnitude per bridge
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub inv_comm_cap_x: f64,
}

pub fn parse_two_terminal_dc_line(lines: &[&[u8]], psse_version: i8) -> Vec<TwoTerminalDc> {
    let mut two_terminal_dc_lines: Vec<TwoTerminalDc> = Vec::new();
    let mut i: usize = 0;
    // Version 34 added NDI and NDR
    let parse_adder: usize = (psse_version >= 34) as usize;

    while i < lines.len() - 1 {
        if let (Ok(line1_str), Ok(line2_str), Ok(line3_str)) = (
            from_utf8(lines[i]), from_utf8(lines[i + 1]), from_utf8(lines[i + 2])
        ) {
            let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
            let parts2: Vec<&str> = line2_str.split(',').map(|s| s.trim()).collect();
            let parts3: Vec<&str> = line3_str.split(',').map(|s| s.trim()).collect();

            let two_terminal_dc: TwoTerminalDc = TwoTerminalDc {
                name: parts1[0].replace("'", "").replace("\"", "").trim().to_string(),
                power_ctrl_mode: parts1[1].parse().unwrap_or(0),
                resistance: parts1[2].parse().unwrap_or(0.0),
                sending_pow: parts1[3].parse().unwrap_or(0.0),
                voltage: parts1[4].parse().unwrap_or(0.0),
                voltage_ctrl_mode: parts1[5].parse().unwrap_or(0.0),
                r_comp: parts1[6].parse().unwrap_or(0.0),
                dc_pow_margin: parts1[7].parse().unwrap_or(0.0),
                metered_end: parts1[8].replace("'", "").trim().to_string(),
                min_volt: parts1[9].parse().unwrap_or(0.0),
                iteration_limit: parts1[10].parse().unwrap_or(20),
                acceleration_fac: parts1[11].parse().unwrap_or(1.0),

                // Line 2

                rec_bus_id: parts2[0].parse().unwrap_or(0),
                rec_num_bridges: parts2[1].parse().unwrap_or(0),
                rec_max_firing_angle: parts2[2].parse().unwrap_or(0.0),
                rec_min_firing_angle: parts2[3].parse().unwrap_or(0.0),
                rec_comm_xfmr_r: parts2[4].parse().unwrap_or(0.0),
                rec_comm_xfmr_x: parts2[5].parse().unwrap_or(0.0),
                rec_base_volt_ac: parts2[6].parse().unwrap_or(0.0),
                rec_xfmr_ratio: parts2[7].parse().unwrap_or(1.0),
                rec_tap_setting: parts2[8].parse().unwrap_or(1.0),
                rec_max_tap_set: parts2[9].parse().unwrap_or(1.5),
                rec_min_tap_set: parts2[10].parse().unwrap_or(0.51),
                rec_tap_step: parts2[11].parse().unwrap_or(0.00625),
                rec_comm_bus_id: parts2[12].parse().unwrap_or(0),
                rec_comm_bus_node: parts2[13].parse().unwrap_or(0) * (parse_adder as i32),
                rec_from_bus: parts2[13 + parse_adder].parse().unwrap_or(0),
                rec_to_bus: parts2[14 + parse_adder].parse().unwrap_or(0),
                rec_ckt_id: parts2[15 + parse_adder].replace("'", "").trim().to_string(),
                rec_comm_cap_x: parts2[16 + parse_adder].parse().unwrap_or(0.0),

                // Line 3

                inv_bus_id: parts3[0].parse().unwrap_or(0),
                inv_num_bridges: parts3[1].parse().unwrap_or(0),
                inv_max_firing_angle: parts3[2].parse().unwrap_or(0.0),
                inv_min_firing_angle: parts3[3].parse().unwrap_or(0.0),
                inv_comm_xfmr_r: parts3[4].parse().unwrap_or(0.0),
                inv_comm_xfmr_x: parts3[5].parse().unwrap_or(0.0),
                inv_base_volt_ac: parts3[6].parse().unwrap_or(0.0),
                inv_xfmr_ratio: parts3[7].parse().unwrap_or(1.0),
                inv_tap_setting: parts3[8].parse().unwrap_or(1.0),
                inv_max_tap_set: parts3[9].parse().unwrap_or(1.5),
                inv_min_tap_set: parts3[10].parse().unwrap_or(0.51),
                inv_tap_step: parts3[11].parse().unwrap_or(0.00625),
                inv_comm_bus_id: parts3[12].parse().unwrap_or(0),
                inv_comm_bus_node: parts3[13].parse().unwrap_or(0) * (parse_adder as i32),
                inv_from_bus: parts3[13 + parse_adder].parse().unwrap_or(0),
                inv_to_bus: parts3[14 + parse_adder].parse().unwrap_or(0),
                inv_ckt_id: parts3[15 + parse_adder].replace("'", "").trim().to_string(),
                inv_comm_cap_x: parts3[16 + parse_adder].parse().unwrap_or(0.0),
            };
            two_terminal_dc_lines.push(two_terminal_dc);

            i += 3;
        }        
    }
    two_terminal_dc_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_two_terminal_dc_v35() {
        let ttdc_v35_str: &'static str = "\"2           \",0,  25.25000,   656.30,  510.00,    1.00,  25.2500, 0.20000, 'R',    1.00,   30, 2.00000 
715600, 3,  27.500,  22.500,   1.31800,  32.45000,  330.00,  0.84260,  0.85470,  1.30000,  0.80000,  0.02250,      1121,    1,      7777,      8888, '2 ',   1.00000 
715353, 3,  27.000,  27.000,   1.32700,  29.22000,  445.00,  0.57140,  0.86250,  1.31250,  0.81250,  0.02250,      1234,    1,      5555,      9999, '2 ',   1.00000 ";
        let lines: Vec<&[u8]> = ttdc_v35_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<TwoTerminalDc> = parse_two_terminal_dc_line(&lines, 35);
        let ttdc_v35: Option<&TwoTerminalDc> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(ttdc_v35.unwrap().rec_from_bus, 7777);
        assert_eq!(ttdc_v35.unwrap().rec_to_bus, 8888);
        assert_eq!(ttdc_v35.unwrap().inv_from_bus, 5555);
        assert_eq!(ttdc_v35.unwrap().inv_to_bus, 9999);
        assert_eq!(ttdc_v35.unwrap().rec_comm_bus_id, 1121);
        assert_eq!(ttdc_v35.unwrap().inv_comm_bus_id, 1234);
        assert_eq!(ttdc_v35.unwrap().inv_comm_cap_x, 1.0);
        assert_eq!(ttdc_v35.unwrap().power_ctrl_mode, 0);
        assert_eq!(ttdc_v35.unwrap().acceleration_fac, 2.0);
        assert_eq!(ttdc_v35.unwrap().iteration_limit, 30);
    }

    #[test]
    fn parse_two_terminal_dc_v33() {
        let ttdc_v33_str: &'static str = "\"2           \",0,  25.25000,   656.30,  510.00,    1.00,  25.2500, 0.20000, 'R',    1.00,   30, 2.00000 
715600, 3,  27.500,  22.500,   1.31800,  32.45000,  330.00,  0.84260,  0.85470,  1.30000,  0.80000,  0.02250,      1121,      7777,      8888, '2 ',   1.00000 
715353, 3,  27.000,  27.000,   1.32700,  29.22000,  445.00,  0.57140,  0.86250,  1.31250,  0.81250,  0.02250,      1234,      5555,      9999, '2 ',   1.00000 ";
        let lines: Vec<&[u8]> = ttdc_v33_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<TwoTerminalDc> = parse_two_terminal_dc_line(&lines, 33);
        let ttdc_v33: Option<&TwoTerminalDc> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(ttdc_v33.unwrap().rec_from_bus, 7777);
        assert_eq!(ttdc_v33.unwrap().rec_to_bus, 8888);
        assert_eq!(ttdc_v33.unwrap().inv_from_bus, 5555);
        assert_eq!(ttdc_v33.unwrap().inv_to_bus, 9999);
        assert_eq!(ttdc_v33.unwrap().rec_comm_bus_id, 1121);
        assert_eq!(ttdc_v33.unwrap().inv_comm_bus_id, 1234);
        assert_eq!(ttdc_v33.unwrap().inv_comm_cap_x, 1.0);
        assert_eq!(ttdc_v33.unwrap().power_ctrl_mode, 0);
        assert_eq!(ttdc_v33.unwrap().acceleration_fac, 2.0);
        assert_eq!(ttdc_v33.unwrap().iteration_limit, 30);
    }
}