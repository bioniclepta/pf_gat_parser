use std::str::from_utf8;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network voltage source converter (VSC) DC branch in the PSS/E .RAW case
pub struct VSCDc {
    /// ## NAME: Two-terminal DC line name
    /// ### Type: String
    /// ### Default: " "
    pub name: String,
    /// ## MDC: Control mode
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Out-of-service
    /// * 1 : In-service
    pub status: i8,
    /// ## RDC: DC line resistance
    /// ### Type: f64
    /// ### Units: Ohms
    /// ### Default: 0.0
    pub resistance: f64,
    /// ## O1: VSC DC Branch owner number 1
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner1: i16,
    /// ## O2: VSC DC Branch owner number 2
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner2: i16,
    /// ## O3: VSC DC Branch owner number 3
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner3: i16,
    /// ## O4: VSC DC Branch owner number 4
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner4: i16,
    /// ## F1: VSC DC Branch ownership percentage 1
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner1_percent: f64,
    /// ## F2: VSC DC Branch ownership percentage 2
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner2_percent: f64,
    /// ## F3: VSC DC Branch ownership percentage 3
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner3_percent: f64,
    /// ## F4: VSC DC Branch ownership percentage 4
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner4_percent: f64,

    // Line 2

    /// ## IBUS: Converter 1 Bus ID
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub converter_1_bus_id: i32,
    /// ## TYPE: Converter 1 code for DC control
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Converter 1 out-of-service
    /// * 1 : DC voltage control
    /// * 2 : MW control
    pub converter_1_dc_control: i8,
    /// ## MODE: Converter 1 code for AC control
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : AC voltage control
    /// * 2 : Fixed AC power
    pub converter_1_ac_control: i8,
    /// ## DCSET: Converter 1 DC setpoint
    /// ### Type: f64
    /// ### Units (TYPE = 1): kV
    /// ### Units (TYPE = 2): MW
    /// ### Default: 0.0
    pub dc_setpoint_1: f64,
    /// ## ACSET: Converter 1 AC setpoint
    /// ### Type: f64
    /// ### Units (TYPE = 1): voltage in p.u.
    /// ### Units (TYPE = 2): power factor
    /// ### Default: 0.0
    pub ac_setpoint_1: f64,
    /// ## ALOSS: Converter 1 coefficient used to calculate converter losses
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub cont_loss_coeff_1: f64,
    /// ## BLOSS: Converter 1 coefficient used to calculate converter losses
    /// ### Type: f64
    /// ### Units: kW/A
    /// ### Default: 0.0
    pub prop_loss_coeff_1: f64,
    /// ## MINLOSS: Converter 1 minimum losses
    /// ### Type: f64
    /// ### Units: kW
    /// ### Default: 0.0
    pub min_converter_loss_1: f64,
    /// ## SMAX: Converter 1 MVA rating
    /// ### Type: f64
    /// ### Units: MVA
    /// ### Default: 0.0
    pub mva_rating_1: f64,
    /// ## SMAX: Converter 1 AC current rating
    /// ### Type: f64
    /// ### Units: A
    /// ### Default: 0.0
    pub ac_rating_1: f64,
    /// ## PWF: Converter 1 power weighting factor fraction
    /// ### Type: f64
    /// ### Default: 1.0
    pub power_weighting_factor_1: f64,
    /// ## MAXQ: Converter 1 reactive power upper limit
    /// ### Type: f64
    /// ### Units: MVAR
    /// ### Default: 9999.0
    pub max_reac_1: f64,
    /// ## MINQ: Converter 1 reactive power lower limit
    /// ### Type: f64
    /// ### Units: MVAR
    /// ### Default: -9999.0
    pub min_reac_1: f64,
    /// ## VSREG: Remote bus ID for converter 1 to control the voltage of
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub control_bus_id_1: i32,
    /// ## NREG: Remote bus node number for converter 1 to control the voltage of
    /// ### Type: i32
    /// ### Default: 0
    pub control_node_1: i32,
    /// ## RMPCT: The percent of the total MVAR required to hold the voltage at the bus controlled by converter_1_bus_id (IBUS)
    /// ### Type: f64
    /// ### Default: 100.0
    pub control_bus_percent_mvar_1: f64,

    // Line 3

    /// ## IBUS: Converter 2 Bus ID
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub converter_2_bus_id: i32,
    /// ## TYPE: Converter 2 code for DC control
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Converter 2 out-of-service
    /// * 1 : DC voltage control
    /// * 2 : MW control
    pub converter_2_dc_control: i8,
    /// ## MODE: Converter 2 code for AC control
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : AC voltage control
    /// * 2 : Fixed AC power
    pub converter_2_ac_control: i8,
    /// ## DCSET: Converter 2 DC setpoint
    /// ### Type: f64
    /// ### Units (TYPE = 1): kV
    /// ### Units (TYPE = 2): MW
    /// ### Default: 0.0
    pub dc_setpoint_2: f64,
    /// ## ACSET: Converter 2 AC setpoint
    /// ### Type: f64
    /// ### Units (TYPE = 1): voltage in p.u.
    /// ### Units (TYPE = 2): power factor
    /// ### Default: 0.0
    pub ac_setpoint_2: f64,
    /// ## ALOSS: Converter 2 coefficient used to calculate converter losses
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    pub cont_loss_coeff_2: f64,
    /// ## BLOSS: Converter 2 coefficient used to calculate converter losses
    /// ### Type: f64
    /// ### Units: kW/A
    /// ### Default: 0.0
    pub prop_loss_coeff_2: f64,
    /// ## MINLOSS: Converter 2 minimum losses
    /// ### Type: f64
    /// ### Units: kW
    /// ### Default: 0.0
    pub min_converter_loss_2: f64,
    /// ## SMAX: Converter 2 MVA rating
    /// ### Type: f64
    /// ### Units: MVA
    /// ### Default: 0.0
    pub mva_rating_2: f64,
    /// ## SMAX: Converter 2 AC current rating
    /// ### Type: f64
    /// ### Units: A
    /// ### Default: 0.0
    pub ac_rating_2: f64,
    /// ## PWF: Converter 2 power weighting factor fraction
    /// ### Type: f64
    /// ### Default: 2.0
    pub power_weighting_factor_2: f64,
    /// ## MAXQ: Converter 2 reactive power upper limit
    /// ### Type: f64
    /// ### Units: MVAR
    /// ### Default: 9999.0
    pub max_reac_2: f64,
    /// ## MINQ: Converter 2 reactive power lower limit
    /// ### Type: f64
    /// ### Units: MVAR
    /// ### Default: -9999.0
    pub min_reac_2: f64,
    /// ## VSREG: Remote bus ID for converter 2 to control the voltage of
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub control_bus_id_2: i32,
    /// ## NREG: Remote bus node number for converter 2 to control the voltage of
    /// ### Type: i32
    /// ### Default: 0
    pub control_node_2: i32,
    /// ## RMPCT: The percent of the total MVAR required to hold the voltage at the bus controlled by converter_2_bus_id (IBUS)
    /// ### Type: f64
    /// ### Default: 100.0
    pub control_bus_percent_mvar_2: f64,
}

pub fn parse_vsc_dc_line(lines: &[&[u8]], psse_version: i8) -> Vec<VSCDc> {
    let mut vsc_dc_lines: Vec<VSCDc> = Vec::new();
    let mut i: usize = 0;
    // Version 34 added NREG
    let parse_adder: usize = (psse_version >= 34) as usize;

    while i < lines.len() - 1 {
        if let (Ok(line1_str), Ok(line2_str), Ok(line3_str)) = (
            from_utf8(lines[i]), from_utf8(lines[i + 1]), from_utf8(lines[i + 2])
        ) {
            let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
            let parts2: Vec<&str> = line2_str.split(',').map(|s| s.trim()).collect();
            let parts3: Vec<&str> = line3_str.split(',').map(|s| s.trim()).collect();

            let vsc_dc: VSCDc = VSCDc {
                name: parts1[0].replace("'", "").trim().to_string(),
                status: parts1[1].parse().unwrap_or(1),
                resistance: parts1[2].parse().unwrap_or(0.0),
                owner1: parts1[3].parse().unwrap_or(1),
                owner1_percent: parts1[4].parse().unwrap_or(1.0),
                // Sometimes they won't list multiple owners and just leave the fields blank, get solves this
                owner2: parts1.get(5).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner2_percent: parts1.get(6).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner3: parts1.get(7).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner3_percent: parts1.get(8).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner4: parts1.get(9).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner4_percent: parts1.get(10).and_then(|s| s.parse().ok()).unwrap_or(1.0),

                // Line 2

                converter_1_bus_id: parts2[0].parse().unwrap_or(0),
                converter_1_dc_control: parts2[1].parse().unwrap_or(0),
                converter_1_ac_control: parts2[2].parse().unwrap_or(1),
                dc_setpoint_1: parts2[3].parse().unwrap_or(0.0),
                ac_setpoint_1: parts2[4].parse().unwrap_or(0.0),
                cont_loss_coeff_1: parts2[5].parse().unwrap_or(0.0),
                prop_loss_coeff_1: parts2[6].parse().unwrap_or(0.0),
                min_converter_loss_1: parts2[7].parse().unwrap_or(0.0),
                mva_rating_1: parts2[8].parse().unwrap_or(0.0),
                ac_rating_1: parts2[9].parse().unwrap_or(0.0),
                power_weighting_factor_1: parts2[10].parse().unwrap_or(1.0),
                max_reac_1: parts2[11].parse().unwrap_or(9999.0),
                min_reac_1: parts2[12].parse().unwrap_or(-9999.0),
                control_bus_id_1: parts2[13].parse().unwrap_or(0),
                control_node_1: parts2[14].parse().unwrap_or(0) * (parse_adder as i32),
                control_bus_percent_mvar_1: parts2[14 + parse_adder].parse().unwrap_or(100.0),

                // Line 3

                converter_2_bus_id: parts3[0].parse().unwrap_or(0),
                converter_2_dc_control: parts3[1].parse().unwrap_or(0),
                converter_2_ac_control: parts3[2].parse().unwrap_or(1),
                dc_setpoint_2: parts3[3].parse().unwrap_or(0.0),
                ac_setpoint_2: parts3[4].parse().unwrap_or(0.0),
                cont_loss_coeff_2: parts3[5].parse().unwrap_or(0.0),
                prop_loss_coeff_2: parts3[6].parse().unwrap_or(0.0),
                min_converter_loss_2: parts3[7].parse().unwrap_or(0.0),
                mva_rating_2: parts3[8].parse().unwrap_or(0.0),
                ac_rating_2: parts3[9].parse().unwrap_or(0.0),
                power_weighting_factor_2: parts3[10].parse().unwrap_or(1.0),
                max_reac_2: parts3[11].parse().unwrap_or(9999.0),
                min_reac_2: parts3[12].parse().unwrap_or(-9999.0),
                control_bus_id_2: parts3[13].parse().unwrap_or(0),
                control_node_2: parts3[14].parse().unwrap_or(0) * (parse_adder as i32),
                control_bus_percent_mvar_2: parts3[14 + parse_adder].parse().unwrap_or(100.0),
            };
            vsc_dc_lines.push(vsc_dc);

            i += 3;
        }        
    }
    vsc_dc_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vsc_dc_v35() {
        let vsdc_v35_str: &'static str = "'TST-VSC     ', 1,     1.5800,  28,  1.0000
99890, 2, 1,      96.00, 1.00000,  1800.000,     3.400,  1110.000,    360.00,   1032.00,0.50000,    -20.00,    -70.00, 128880,   0,  100.0
22050, 1, 1,     590.00, 1.08080,  2200.000,     3.400,  1110.000,    360.00,   1085.00,0.50000,    -11.00,    -70.00, 123636,   2,  90.0";
        let lines: Vec<&[u8]> = vsdc_v35_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<VSCDc> = parse_vsc_dc_line(&lines, 35);
        let vsdc_v35_str: Option<&VSCDc> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(vsdc_v35_str.unwrap().name, "TST-VSC".to_string());
        assert_eq!(vsdc_v35_str.unwrap().converter_1_bus_id, 99890);
        assert_eq!(vsdc_v35_str.unwrap().converter_2_bus_id, 22050);
        assert_eq!(vsdc_v35_str.unwrap().control_bus_percent_mvar_1, 100.0);
        assert_eq!(vsdc_v35_str.unwrap().control_bus_percent_mvar_2, 90.0);
        assert_eq!(vsdc_v35_str.unwrap().converter_1_dc_control, 2);
        assert_eq!(vsdc_v35_str.unwrap().dc_setpoint_2, 590.0);
        assert_eq!(vsdc_v35_str.unwrap().control_node_2, 2);
        assert_eq!(vsdc_v35_str.unwrap().ac_setpoint_1, 1.0);
        assert_eq!(vsdc_v35_str.unwrap().cont_loss_coeff_2, 2200.0);
    }

    #[test]
    fn parse_vsc_dc_v33() {
        let vsdc_v35_str: &'static str = "'TST-VSC     ', 1,     1.5800,  28,  1.0000
99890, 2, 1,      96.00, 1.00000,  1800.000,     3.400,  1110.000,    360.00,   1032.00,0.50000,    -20.00,    -70.00, 128880,  100.0
22050, 1, 1,     590.00, 1.08080,  2200.000,     3.400,  1110.000,    360.00,   1085.00,0.50000,    -11.00,    -70.00, 123636,  90.0";
        let lines: Vec<&[u8]> = vsdc_v35_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<VSCDc> = parse_vsc_dc_line(&lines, 33);
        let vsdc_v35_str: Option<&VSCDc> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(vsdc_v35_str.unwrap().name, "TST-VSC".to_string());
        assert_eq!(vsdc_v35_str.unwrap().converter_1_bus_id, 99890);
        assert_eq!(vsdc_v35_str.unwrap().converter_2_bus_id, 22050);
        assert_eq!(vsdc_v35_str.unwrap().control_bus_percent_mvar_1, 100.0);
        assert_eq!(vsdc_v35_str.unwrap().control_bus_percent_mvar_2, 90.0);
        assert_eq!(vsdc_v35_str.unwrap().converter_1_dc_control, 2);
        assert_eq!(vsdc_v35_str.unwrap().dc_setpoint_2, 590.0);
        assert_eq!(vsdc_v35_str.unwrap().control_node_2, 0);
        assert_eq!(vsdc_v35_str.unwrap().ac_setpoint_1, 1.0);
        assert_eq!(vsdc_v35_str.unwrap().cont_loss_coeff_2, 2200.0);
    }
}