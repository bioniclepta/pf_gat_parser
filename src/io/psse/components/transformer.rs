use std::str::from_utf8;


#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network transformer in the PSS/E .RAW case
pub struct Transformer {
    /// ## I: Bus number the transformer leaves from
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub from_bus: i32,
    /// ## J: Bus number the transformer goes to
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub to_bus: i32,
    /// ## K: Tertiary Bus number the transformer goes to (0 if 2-winding transformer)
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    /// ## Parameters
    /// * 0 : Two Winding Transformer
    /// * \> 0 : Three Winding Transformer
    pub tertiary_bus: i32,
    /// ## CKT: Transformer circuit identifier
    /// ### Type: String
    /// ### Default: '1'
    pub circuit: String,
    /// ## CW: Transformer winding data I/O code where the turns ratio are specified
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : Off-nominal turns ratio in p.u. of the winding bus base voltage
    /// * 2 : Winding voltage in kV
    /// * 3 : Off-nominal turns ratio in p.u. of nominal winding voltage, NOMV1, NOMV2, NOMV3
    pub cw: i8,
    /// ## CZ: Transformer impedance data I/O code where the units of winding impedances are specified
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : Resistance and Reactance in p.u. on a system MVA base and winding voltage base
    /// * 2 : Resistance and Reactance in p.u. on a specified MVA base and winding voltage base
    /// * 3 : Transformer load loss in watts and impedance magnitude in p.u. on a specified MVA and winding voltage base
    pub cz: i8,
    /// ## CM: Transformer magnetizing admittance data I/O code where the units of MAG1 and MAG2 are specified
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : Complex admittance in p.u. on the system MVA base and winding 1 base voltage
    /// * 2 : No-load loss in watts and exciting current in p.u. and exciting current in p.u.
    pub cm: i8,
    /// ## MAG1: Transformer magnetizing admittance connected to the ground bus I
    /// ### Type: f64
    /// ### Default: 0.0
    pub mag1: f64,
    /// ## MAG2: Transformer magnetizing admittance connected to the ground bus I
    /// ### Type: f64
    /// ### Default: 0.0
    pub mag2: f64,
    /// ## NMETER: Transformer nonmetered end
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 1 : From Bus (Winding 2 bus)
    /// * 2 : To Bus (Winding 2 bus)
    /// * 3 : Tertiary Bus (Winding 3 bus)
    pub metered_end: i8,
    /// ## NAME: Transformer name
    /// ### Type: String
    /// ### Default: " "
    pub name: String,
    /// ## ST: Transformer Status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Transformer out-of-service
    /// * 1 : Transformer in service
    /// * 2 : Winding 2 out-of-service
    /// * 3 : Winding 3 out-of-service
    /// * 4 : Winding 1 out-of-service
    pub status: i8,
    /// ## O1: Transformer owner number 1
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner1: i16,
    /// ## O2: Transformer owner number 2
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner2: i16,
    /// ## O3: Transformer owner number 3
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner3: i16,
    /// ## O4: Transformer owner number 4
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner4: i16,
    /// ## F1: Transformer ownership percentage 1
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner1_percent: f64,
    /// ## F2: Transformer ownership percentage 2
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner2_percent: f64,
    /// ## F3: Transformer ownership percentage 3
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner3_percent: f64,
    /// ## F4: Transformer ownership percentage 4
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner4_percent: f64,
    /// ## VECGRP: Transformer vector group based on winding connection and phase angles (Informational only)
    /// ### Type: String
    /// ### Default: " "
    pub vector_group: String,
    /// ## ZCOD: Method to derive actual transformer impedances in applying transformer impedance adjustment tables
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Apply impedance adjustment factors to winding impedances
    /// * 1 : Apply impedance adjustment factors to bus-to-bus impedances
    pub zcod: i8,

    //Line 2

    /// ## R1-2: Resistance of the transformer between windings 1 and 2
    /// ### Type: f64
    /// ### Default: 0.0
    pub r12: f64,
    /// ## X1-2: Reactance of the transformer between windings 1 and 2
    /// ### Type: f64
    /// ### Default: 0.0
    pub x12: f64,
    /// ## SBASE1-2: Base MVA of the transformer for windings 1 and 2
    /// ### Type: f64
    /// ### Default: 100.0
    pub sbase12: f64,
    /// ## R2-3: Resistance of the transformer between windings 2 and 3
    /// ### Type: f64
    /// ### Default: 0.0
    pub r23: f64,
    /// ## X2-3: Reactance of the transformer between windings 2 and 3
    /// ### Type: f64
    /// ### Default: 0.0
    pub x23: f64,
    /// ## SBASE2-3: Base MVA of the transformer for windings 2 and 3
    /// ### Type: f64
    /// ### Default: 100.0
    pub sbase23: f64,
    /// ## R3-1: Resistance of the transformer between windings 3 and 1
    /// ### Type: f64
    /// ### Default: 0.0
    pub r31: f64,
    /// ## X3-1: Reactance of the transformer between windings 3 and 1
    /// ### Type: f64
    /// ### Default: 0.0
    pub x31: f64,
    /// ## SBASE3-1: Base MVA of the transformer for windings 3 and 1
    /// ### Type: f64
    /// ### Default: 100.0
    pub sbase31: f64,
    /// ## VMSTAR: Voltage magnitude at the hidden star point bus
    /// ### Type: f64
    /// ### Units: p.u.
    /// ### Default: 1.0
    pub star_vm: f64,
    /// ## ANSTAR: Voltage angle at the hidden star point bus
    /// ### Type: f64
    /// ### Units: degrees
    /// ### Default: 0.0
    pub star_ang: f64,

    //Line 3

    /// ## WINDV1: Transformer Winding 1 voltage
    /// ### Type: f64
    /// ### Default: 1.0
    pub winding_1_volt: f64,
    /// ## NOMV1: Transformer Nominal Winding 1 voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    /// ## Parameters
    /// * 0.0 : Use bus I base voltage
    /// * \> 0.0 : Use value input for base voltage
    pub nominal_volt1: f64,
    /// ## ANG1: Transformer winding 1 phase shift angle
    /// ### Type: f64
    /// ### Units: degrees
    /// ### Default: 0.0
    pub angle1: f64,
    /// ## RATE1: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate1: f64,
    /// ## RATE2: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate2: f64,
    /// ## RATE3: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate3: f64,
    /// ## RATE4: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate4: f64,
    /// ## RATE5: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate5: f64,
    /// ## RATE6: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate6: f64,
    /// ## RATE7: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate7: f64,
    /// ## RATE8: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate8: f64,
    /// ## RATE9: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate9: f64,
    /// ## RATE10: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate10: f64,
    /// ## RATE11: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate11: f64,
    /// ## RATE12: Transformer Winding 1 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w1_rate12: f64,
    /// ## COD1: Transformer control mode for automatic adjustments of the winding 1 tap
    /// ### Type: i32
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Fixed tap and phase shift
    /// * ± 1 : Voltage control
    /// * ± 2 : Reactive power flow control
    /// * ± 3 : Active power flow control
    /// * ± 4 : Control of a DC line quantity
    /// * ± 5 : Asymmetric active power flow control
    pub control_mode_1: i32,
    /// ## CONT1: Bus number that is controlled by the transformer turns ratio adjustment option
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub controlled_bus_id_1: i32,
    /// ## RMA1: Upper limit of winding 1
    /// ### Type: f64
    /// ### Default: 1.1
    pub rma1: f64,
    /// ## RMI1: Lower limit of winding 1
    /// ### Type: f64
    /// ### Default: 0.9
    pub rmi1: f64,
    /// ## VMA1: Upper limit of winding 1
    /// ### Type: f64
    /// ### Default: 1.1
    pub vma1: f64,
    /// ## VMI1: Lower limit of winding 1
    /// ### Type: f64
    /// ### Default: 0.9
    pub vmi1: f64,
    /// ## NTP: The number of tap positions available for Winding 1
    /// ### Type: i32
    /// ### Default: 33
    /// ## Range
    /// * 2 - 9999
    pub tap_positions_1: i32,
    /// ## TAB1: The impedance table correction number to use for winding 1
    /// ### Type: i32
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : No impedance correction table to be used
    /// * \> 0 : Impedance correction table number to use
    pub impedance_correction_table_1: i32,
    /// ## CR1: Load drop compensation resistance of winding 1
    /// ### Type: f64
    /// ### Units: p.u. (system MVA base)
    /// ### Default: 0.0
    pub load_drop_comp_r1: f64,
    /// ## CR1: Load drop compensation reactance of winding 1
    /// ### Type: f64
    /// ### Units: p.u. (system MVA base)
    /// ### Default: 0.0
    pub load_drop_comp_x1: f64,
    /// ## CNXA1: Transformer winding 1 connection angle
    /// ### Type: f64
    /// ### degrees
    /// ### Default: 0.0
    pub connection_ang_1: f64,

    //Line 4

    /// ## WINDV2: Transformer Winding 2 voltage
    /// ### Type: f64
    /// ### Default: 1.0
    pub winding_2_volt: f64,
    /// ## NOMV2: Transformer Nominal Winding 2 voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    /// ## Parameters
    /// * 0.0 : Use bus I base voltage
    /// * \> 0.0 : Use value input for base voltage
    pub nominal_volt2: f64,
    /// ## ANG2: Transformer winding 2 phase shift angle
    /// ### Type: f64
    /// ### Units: degrees
    /// ### Default: 0.0
    pub angle2: f64,
    /// ## RATE2: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate1: f64,
    /// ## RATE2: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate2: f64,
    /// ## RATE3: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate3: f64,
    /// ## RATE4: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate4: f64,
    /// ## RATE5: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate5: f64,
    /// ## RATE6: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate6: f64,
    /// ## RATE7: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate7: f64,
    /// ## RATE8: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate8: f64,
    /// ## RATE9: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate9: f64,
    /// ## RATE20: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate10: f64,
    /// ## RATE11: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate11: f64,
    /// ## RATE12: Transformer Winding 2 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w2_rate12: f64,
    /// ## COD2: Transformer control mode for automatic adjustments of the winding 2 tap
    /// ### Type: i32
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Fixed tap and phase shift
    /// * ± 1 : Voltage control
    /// * ± 2 : Reactive power flow control
    /// * ± 3 : Active power flow control
    /// * ± 4 : Control of a DC line quantity
    /// * ± 5 : Asymmetric active power flow control
    pub control_mode_2: i32,
    /// ## CONT2: Bus number that is controlled by the transformer turns ratio adjustment option
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub controlled_bus_id_2: i32,
    /// ## RMA2: Upper limit of winding 2
    /// ### Type: f64
    /// ### Default: 1.1
    pub rma2: f64,
    /// ## RMI2: Lower limit of winding 2
    /// ### Type: f64
    /// ### Default: 0.9
    pub rmi2: f64,
    /// ## VMA2: Upper limit of winding 2
    /// ### Type: f64
    /// ### Default: 1.1
    pub vma2: f64,
    /// ## VMI2: Lower limit of winding 2
    /// ### Type: f64
    /// ### Default: 0.9
    pub vmi2: f64,
    /// ## NTP: The number of tap positions available for Winding 2
    /// ### Type: i32
    /// ### Default: 33
    /// ## Range
    /// * 2 - 9999
    pub tap_positions_2: i32,
    /// ## TAB2: The impedance table correction number to use for winding 2
    /// ### Type: i32
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : No impedance correction table to be used
    /// * \> 0 : Impedance correction table number to use
    pub impedance_correction_table_2: i32,
    /// ## CR2: Load drop compensation resistance of winding 2
    /// ### Type: f64
    /// ### Units: p.u. (system MVA base)
    /// ### Default: 0.0
    pub load_drop_comp_r2: f64,
    /// ## CX2: Load drop compensation reactance of winding 2
    /// ### Type: f64
    /// ### Units: p.u. (system MVA base)
    /// ### Default: 0.0
    pub load_drop_comp_x2: f64,
    /// ## CNXA2: Transformer winding 2 connection angle
    /// ### Type: f64
    /// ### degrees
    /// ### Default: 0.0
    pub connection_ang_2: f64,

    //Line 5

    /// ## WINDV3: Transformer Winding 3 voltage
    /// ### Type: f64
    /// ### Default: 1.0
    pub winding_3_volt: f64,
    /// ## NOMV3: Transformer Nominal Winding 3 voltage
    /// ### Type: f64
    /// ### Units: kV
    /// ### Default: 0.0
    /// ## Parameters
    /// * 0.0 : Use bus I base voltage
    /// * \> 0.0 : Use value input for base voltage
    pub nominal_volt3: f64,
    /// ## ANG3: Transformer winding 3 phase shift angle
    /// ### Type: f64
    /// ### Units: degrees
    /// ### Default: 0.0
    pub angle3: f64,
    /// ## RATE3: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate1: f64,
    /// ## RATE2: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate2: f64,
    /// ## RATE3: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate3: f64,
    /// ## RATE4: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate4: f64,
    /// ## RATE5: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate5: f64,
    /// ## RATE6: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate6: f64,
    /// ## RATE7: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate7: f64,
    /// ## RATE8: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate8: f64,
    /// ## RATE9: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate9: f64,
    /// ## RATE10: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate10: f64,
    /// ## RATE11: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate11: f64,
    /// ## RATE12: Transformer Winding 3 MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub w3_rate12: f64,
    /// ## COD3: Transformer control mode for automatic adjustments of the winding 3 tap
    /// ### Type: i32
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : Fixed tap and phase shift
    /// * ± 1 : Voltage control
    /// * ± 2 : Reactive power flow control
    /// * ± 3 : Active power flow control
    /// * ± 4 : Control of a DC line quantity
    /// * ± 5 : Asymmetric active power flow control
    pub control_mode_3: i32,
    /// ## CONT3: Bus number that is controlled by the transformer turns ratio adjustment option
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub controlled_bus_id_3: i32,
    /// ## RMA3: Upper limit of winding 3
    /// ### Type: f64
    /// ### Default: 1.1
    pub rma3: f64,
    /// ## RMI3: Lower limit of winding 3
    /// ### Type: f64
    /// ### Default: 0.9
    pub rmi3: f64,
    /// ## VMA3: Upper limit of winding 3
    /// ### Type: f64
    /// ### Default: 1.1
    pub vma3: f64,
    /// ## VMI3: Lower limit of winding 3
    /// ### Type: f64
    /// ### Default: 0.9
    pub vmi3: f64,
    /// ## NTP: The number of tap positions available for Winding 3
    /// ### Type: i32
    /// ### Default: 33
    /// ## Range
    /// * 2 - 9999
    pub tap_positions_3: i32,
    /// ## TAB3: The impedance table correction number to use for winding 3
    /// ### Type: i32
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : No impedance correction table to be used
    /// * \> 0 : Impedance correction table number to use
    pub impedance_correction_table_3: i32,
    /// ## CR3: Load drop compensation resistance of winding 3
    /// ### Type: f64
    /// ### Units: p.u. (system MVA base)
    /// ### Default: 0.0
    pub load_drop_comp_r3: f64,
    /// ## CR3: Load drop compensation reactance of winding 3
    /// ### Type: f64
    /// ### Units: p.u. (system MVA base)
    /// ### Default: 0.0
    pub load_drop_comp_x3: f64,
    /// ## CNXA3: Transformer winding 3 connection angle
    /// ### Type: f64
    /// ### degrees
    /// ### Default: 0.0
    pub connection_ang_3: f64,
}

pub fn parse_transformers(lines: &[&[u8]], psse_version: i8) -> Vec<Transformer> {
    let mut transformers: Vec<Transformer> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if let (Ok(line1_str), Ok(line2_str), Ok(line3_str), Ok(line4_str)) = (
            from_utf8(lines[i]), from_utf8(lines[i + 1]), from_utf8(lines[i + 2]), from_utf8(lines[i + 3])
        ) {
            let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
            let two_winding: bool = parts1[2].parse::<i32>().unwrap_or(0) == 0;
            if two_winding {
                // Two winding transformer (4 lines)
                let two_wnd_xfmr: Transformer = parse_two_winding_transformer(parts1, line2_str, line3_str, line4_str, psse_version);
                transformers.push(two_wnd_xfmr);
                i += 4;
            } else {
                // Three winding transformer (5 lines)
                let line5_str = from_utf8(lines[i + 4]).unwrap();
                //let three_wnd_xfmr: Transformer = parse_three_winding_transformer(parts1, line2_str, line3_str, line4_str, line5_str, psse_version);
                //transformers.push(three_wnd_xfmr);
                i += 5;
            }
        }        
    }
    transformers
}

fn parse_two_winding_transformer(parts1: Vec<&str>, line2_str: &str, line3_str: &str, line4_str: &str, psse_version: i8) -> Transformer {
    // let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
    let parts2: Vec<&str> = line2_str.split(',').map(|s| s.trim()).collect();
    let parts3: Vec<&str> = line3_str.split(',').map(|s| s.trim()).collect();
    let parts4: Vec<&str> = line4_str.split(',').map(|s| s.trim()).collect();

    let parse_adder: usize = ((psse_version >= 34) as usize) * 9;
    println!("{:?}", parts1);
    Transformer {
        from_bus: parts1[0].parse().unwrap_or(0),
        to_bus: parts1[1].parse().unwrap_or(0),
        tertiary_bus: 0,
        circuit: parts1[3].replace("'", "").trim().to_string(),
        cw: parts1[4].parse().unwrap_or(1),
        cz: parts1[5].parse().unwrap_or(1),
        cm: parts1[6].parse().unwrap_or(1),
        mag1: parts1[7].parse().unwrap_or(0.0),
        mag2: parts1[8].parse().unwrap_or(0.0),
        metered_end: parts1[9].parse().unwrap_or(1),
        name: parts1[10].replace("'", "").trim().to_string(),
        status: parts1[11].parse().unwrap_or(1),
        owner1: parts1[12].parse().unwrap_or(1),
        owner1_percent: parts1[13].parse().unwrap_or(1.0),
        owner2: parts1[14].parse().unwrap_or(0),
        owner2_percent: parts1[15].parse().unwrap_or(1.0),
        owner3: parts1[16].parse().unwrap_or(0),
        owner3_percent: parts1[17].parse().unwrap_or(1.0),
        owner4: parts1[18].parse().unwrap_or(0),
        owner4_percent: parts1[19].parse().unwrap_or(1.0),
        vector_group: parts1[20].replace("'", "").trim().to_string(),
        zcod: 0,

        // Line 2

        r12: parts2[0].parse().unwrap_or(0.0),
        x12: parts2[1].parse().unwrap_or(0.0),
        sbase12: parts2[2].parse().unwrap_or(100.0),
        r23: 0.0,
        x23: 0.0,
        sbase23: 0.0,
        r31: 0.0,
        x31: 0.0,
        sbase31: 0.0,
        star_vm: 0.0,
        star_ang: 0.0,

        // Line 3

        winding_1_volt: parts3[0].parse().unwrap_or(1.0),
        nominal_volt1: parts3[1].parse().unwrap_or(0.0),
        angle1: parts3[2].parse().unwrap_or(0.0),
        w1_rate1: parts3[3].parse().unwrap_or(0.0),
        w1_rate2: parts3[4].parse().unwrap_or(0.0),
        w1_rate3: parts3[5].parse().unwrap_or(0.0),
        w1_rate4: (psse_version >= 34).then_some(parts3[6].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate5: (psse_version >= 34).then_some(parts3[7].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate6: (psse_version >= 34).then_some(parts3[8].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate7: (psse_version >= 34).then_some(parts3[9].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate8: (psse_version >= 34).then_some(parts3[10].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate9: (psse_version >= 34).then_some(parts3[11].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate10: (psse_version >= 34).then_some(parts3[12].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate11: (psse_version >= 34).then_some(parts3[13].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate12: (psse_version >= 34).then_some(parts3[14].parse().unwrap_or(0.0)).unwrap_or(0.0),
        control_mode_1: parts3[6 + parse_adder].parse().unwrap_or(0),
        controlled_bus_id_1: parts3[7 + parse_adder].parse().unwrap_or(0),
        rma1: parts3[8 + parse_adder].parse().unwrap_or(1.1),
        rmi1: parts3[9 + parse_adder].parse().unwrap_or(0.9),
        vma1: parts3[10 + parse_adder].parse().unwrap_or(1.1),
        vmi1: parts3[11 + parse_adder].parse().unwrap_or(0.9),
        tap_positions_1: parts3[12 + parse_adder].parse().unwrap_or(33),
        impedance_correction_table_1: parts3[13 + parse_adder].parse().unwrap_or(0),
        load_drop_comp_r1: parts3[14 + parse_adder].parse().unwrap_or(0.0),
        load_drop_comp_x1: parts3[15 + parse_adder].parse().unwrap_or(0.0),
        connection_ang_1: parts3[16 + parse_adder].parse().unwrap_or(0.0),

        // Line 4

        winding_2_volt: parts4[0].parse().unwrap_or(1.0),
        nominal_volt2: parts4[0].parse().unwrap_or(1.0),
        angle2: 0.0,
        w2_rate1: 0.0,
        w2_rate2: 0.0,
        w2_rate3: 0.0,
        w2_rate4: 0.0,
        w2_rate5: 0.0,
        w2_rate6: 0.0,
        w2_rate7: 0.0,
        w2_rate8: 0.0,
        w2_rate9: 0.0,
        w2_rate10: 0.0,
        w2_rate11: 0.0,
        w2_rate12: 0.0,
        control_mode_2: 0,
        controlled_bus_id_2: 0,
        rma2: 0.0,
        rmi2: 0.0,
        vma2: 0.0,
        vmi2: 0.0,
        tap_positions_2: 0,
        impedance_correction_table_2: 0,
        load_drop_comp_r2: 0.0,
        load_drop_comp_x2: 0.0,
        connection_ang_2: 0.0,

        // Line 5

        winding_3_volt: 0.0,
        nominal_volt3: 0.0,
        angle3: 0.0,
        w3_rate1: 0.0,
        w3_rate2: 0.0,
        w3_rate3: 0.0,
        w3_rate4: 0.0,
        w3_rate5: 0.0,
        w3_rate6: 0.0,
        w3_rate7: 0.0,
        w3_rate8: 0.0,
        w3_rate9: 0.0,
        w3_rate10: 0.0,
        w3_rate11: 0.0,
        w3_rate12: 0.0,
        control_mode_3: 0,
        controlled_bus_id_3: 0,
        rma3: 0.0,
        rmi3: 0.0,
        vma3: 0.0,
        vmi3: 0.0,
        tap_positions_3: 0,
        impedance_correction_table_3: 0,
        load_drop_comp_r3: 0.0,
        load_drop_comp_x3: 0.0,
        connection_ang_3: 0.0,
    }
}

fn parse_three_winding_transformer(parts1: Vec<&str>, line2_str: &str, line3_str: &str, line4_str: &str, line5_str: &str, psse_version: i8) -> Transformer {
    // let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
    let parts2: Vec<&str> = line2_str.split(',').map(|s| s.trim()).collect();
    let parts3: Vec<&str> = line3_str.split(',').map(|s| s.trim()).collect();
    let parts4: Vec<&str> = line4_str.split(',').map(|s| s.trim()).collect();
    let parts5: Vec<&str> = line5_str.split(',').map(|s| s.trim()).collect();

    let parse_adder: usize = ((psse_version >= 34) as usize) * 9;

    Transformer {
        from_bus: parts1[0].parse().unwrap_or(0),
        to_bus: parts1[1].parse().unwrap_or(0),
        tertiary_bus: parts1[2].parse().unwrap_or(0),
        circuit: parts1[3].replace("'", "").trim().to_string(),
        cw: parts1[4].parse().unwrap_or(1),
        cz: parts1[5].parse().unwrap_or(1),
        cm: parts1[6].parse().unwrap_or(1),
        mag1: parts1[7].parse().unwrap_or(0.0),
        mag2: parts1[8].parse().unwrap_or(0.0),
        metered_end: parts1[9].parse().unwrap_or(1),
        name: parts1[10].replace("'", "").trim().to_string(),
        status: parts1[11].parse().unwrap_or(1),
        owner1: parts1[12].parse().unwrap_or(1),
        owner1_percent: parts1[13].parse().unwrap_or(1.0),
        owner2: parts1[14].parse().unwrap_or(0),
        owner2_percent: parts1[15].parse().unwrap_or(1.0),
        owner3: parts1[16].parse().unwrap_or(0),
        owner3_percent: parts1[17].parse().unwrap_or(1.0),
        owner4: parts1[18].parse().unwrap_or(0),
        owner4_percent: parts1[19].parse().unwrap_or(1.0),
        vector_group: parts1[20].replace("'", "").trim().to_string(),
        zcod: parts1.get(21).and_then(|s| s.parse().ok()).unwrap_or(0),
        
        // Line 2

        r12: parts2[0].parse().unwrap_or(0.0),
        x12: parts2[1].parse().unwrap_or(0.0),
        sbase12: parts2[2].parse().unwrap_or(100.0),
        r23: parts2[3].parse().unwrap_or(0.0),
        x23: parts2[4].parse().unwrap_or(0.0),
        sbase23: parts2[5].parse().unwrap_or(100.0),
        r31: parts2[6].parse().unwrap_or(0.0),
        x31: parts2[7].parse().unwrap_or(0.0),
        sbase31: parts2[8].parse().unwrap_or(100.0),
        star_vm: parts2[9].parse().unwrap_or(1.0),
        star_ang: parts2[10].parse().unwrap_or(0.0),

        // Line 3

        winding_1_volt: parts3[0].parse().unwrap_or(1.0),
        nominal_volt1: parts3[1].parse().unwrap_or(0.0),
        angle1: parts3[2].parse().unwrap_or(0.0),
        w1_rate1: parts3[3].parse().unwrap_or(0.0),
        w1_rate2: parts3[4].parse().unwrap_or(0.0),
        w1_rate3: parts3[5].parse().unwrap_or(0.0),
        w1_rate4: (psse_version >= 34).then_some(parts3[6].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate5: (psse_version >= 34).then_some(parts3[7].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate6: (psse_version >= 34).then_some(parts3[8].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate7: (psse_version >= 34).then_some(parts3[9].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate8: (psse_version >= 34).then_some(parts3[10].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate9: (psse_version >= 34).then_some(parts3[11].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate10: (psse_version >= 34).then_some(parts3[12].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate11: (psse_version >= 34).then_some(parts3[13].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w1_rate12: (psse_version >= 34).then_some(parts3[14].parse().unwrap_or(0.0)).unwrap_or(0.0),
        control_mode_1: parts3[6 + parse_adder].parse().unwrap_or(0),
        controlled_bus_id_1: parts3[7 + parse_adder].parse().unwrap_or(0),
        rma1: parts3[8 + parse_adder].parse().unwrap_or(1.1),
        rmi1: parts3[9 + parse_adder].parse().unwrap_or(0.9),
        vma1: parts3[10 + parse_adder].parse().unwrap_or(1.1),
        vmi1: parts3[11 + parse_adder].parse().unwrap_or(0.9),
        tap_positions_1: parts3[12 + parse_adder].parse().unwrap_or(33),
        impedance_correction_table_1: parts3[13 + parse_adder].parse().unwrap_or(0),
        load_drop_comp_r1: parts3[14 + parse_adder].parse().unwrap_or(0.0),
        load_drop_comp_x1: parts3[15 + parse_adder].parse().unwrap_or(0.0),
        connection_ang_1: parts3[16 + parse_adder].parse().unwrap_or(0.0),

        // Line 4

        winding_2_volt: parts4[0].parse().unwrap_or(1.0),
        nominal_volt2: parts4[1].parse().unwrap_or(0.0),
        angle2: parts4[2].parse().unwrap_or(0.0),
        w2_rate1: parts4[3].parse().unwrap_or(0.0),
        w2_rate2: parts4[4].parse().unwrap_or(0.0),
        w2_rate3: parts4[5].parse().unwrap_or(0.0),
        w2_rate4: (psse_version >= 34).then_some(parts4[6].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate5: (psse_version >= 34).then_some(parts4[7].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate6: (psse_version >= 34).then_some(parts4[8].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate7: (psse_version >= 34).then_some(parts4[9].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate8: (psse_version >= 34).then_some(parts4[10].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate9: (psse_version >= 34).then_some(parts4[11].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate10: (psse_version >= 34).then_some(parts4[12].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate11: (psse_version >= 34).then_some(parts4[13].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w2_rate12: (psse_version >= 34).then_some(parts4[14].parse().unwrap_or(0.0)).unwrap_or(0.0),
        control_mode_2: parts4[6 + parse_adder].parse().unwrap_or(0),
        controlled_bus_id_2: parts4[7 + parse_adder].parse().unwrap_or(0),
        rma2: parts4[8 + parse_adder].parse().unwrap_or(1.1),
        rmi2: parts4[9 + parse_adder].parse().unwrap_or(0.9),
        vma2: parts4[10 + parse_adder].parse().unwrap_or(1.1),
        vmi2: parts4[11 + parse_adder].parse().unwrap_or(0.9),
        tap_positions_2: parts4[12 + parse_adder].parse().unwrap_or(33),
        impedance_correction_table_2: parts4[13 + parse_adder].parse().unwrap_or(0),
        load_drop_comp_r2: parts4[14 + parse_adder].parse().unwrap_or(0.0),
        load_drop_comp_x2: parts4[15 + parse_adder].parse().unwrap_or(0.0),
        connection_ang_2: parts4[16 + parse_adder].parse().unwrap_or(0.0),

        // Line 5
        
        winding_3_volt: parts5[0].parse().unwrap_or(1.0),
        nominal_volt3: parts5[1].parse().unwrap_or(0.0),
        angle3: parts5[2].parse().unwrap_or(0.0),
        w3_rate1: parts5[3].parse().unwrap_or(0.0),
        w3_rate2: parts5[4].parse().unwrap_or(0.0),
        w3_rate3: parts5[5].parse().unwrap_or(0.0),
        w3_rate4: (psse_version >= 34).then_some(parts5[6].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate5: (psse_version >= 34).then_some(parts5[7].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate6: (psse_version >= 34).then_some(parts5[8].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate7: (psse_version >= 34).then_some(parts5[9].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate8: (psse_version >= 34).then_some(parts5[10].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate9: (psse_version >= 34).then_some(parts5[11].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate10: (psse_version >= 34).then_some(parts5[12].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate11: (psse_version >= 34).then_some(parts5[13].parse().unwrap_or(0.0)).unwrap_or(0.0),
        w3_rate12: (psse_version >= 34).then_some(parts5[14].parse().unwrap_or(0.0)).unwrap_or(0.0),
        control_mode_3: parts5[6 + parse_adder].parse().unwrap_or(0),
        controlled_bus_id_3: parts5[7 + parse_adder].parse().unwrap_or(0),
        rma3: parts5[8 + parse_adder].parse().unwrap_or(1.1),
        rmi3: parts5[9 + parse_adder].parse().unwrap_or(0.9),
        vma3: parts5[10 + parse_adder].parse().unwrap_or(1.1),
        vmi3: parts5[11 + parse_adder].parse().unwrap_or(0.9),
        tap_positions_3: parts5[12 + parse_adder].parse().unwrap_or(33),
        impedance_correction_table_3: parts5[13 + parse_adder].parse().unwrap_or(0),
        load_drop_comp_r3: parts5[14 + parse_adder].parse().unwrap_or(0.0),
        load_drop_comp_x3: parts5[15 + parse_adder].parse().unwrap_or(0.0),
        connection_ang_3: parts5[16 + parse_adder].parse().unwrap_or(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_3_winding_v35() {
        let three_winding_v35_str: &'static str = "1234, 5678, 91011, '2 ', 1, 1, 1, 0.00000E+00, 0.00000E+00, 2, 'TESTXFMR                                ', 1,  190, 1.0000,    0, 1.0000,    0, 1.0000,    0, 1.0000, '                                        '
 1.09000E-03, 5.12700E-02,   100.00, 2.11200E-02, 2.05040E-01,   100.00, 0.89800E-02, 2.03950E-01,   100.00,   1.010001894, 105.342819992,
    0.95967,    0.000,    0.000,   288.00,   288.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,  1,        0,   0,    1.10000,    0.90000,    1.10000,    0.90000,   33,    0,    0.00000,    0.00000,      0.000
    1.00000,    0.000,    0.000,   288.00,   288.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,  0,        0,   0,    1.10000,    0.90000,    1.10000,    0.90000,   33,    0,    0.00000,    0.00000,      0.000
    1.00000,    0.000,  -30.000,    10.00,    10.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,  0,        0,   0,    1.10000,    0.90000,    1.10000,    0.90000,   33,    0,    0.00000,    0.00000,      0.000
4444, 5555, 6908, '1 ', 1, 1, 2, 1.05300E+04, 9.86000E-03, 2, 'NEET12                                  ', 1,  191, 1.0000,    0, 1.0000,    0, 1.0000,    0, 1.0000, '                                        '";
        let lines: Vec<&[u8]> = three_winding_v35_str.lines().map(|line| line.as_bytes()).collect();
        let binding = parse_transformers(&lines, 35);
        let three_winding_v35: Option<&Transformer> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(three_winding_v35.unwrap().from_bus, 1234);
        assert_eq!(three_winding_v35.unwrap().to_bus, 5678);
        assert_eq!(three_winding_v35.unwrap().tertiary_bus, 91011);
        assert_eq!(three_winding_v35.unwrap().circuit, "2".to_string());
        assert_eq!(three_winding_v35.unwrap().control_mode_1, 1);
        assert_eq!(three_winding_v35.unwrap().r12, 0.00109);
        assert_eq!(three_winding_v35.unwrap().r23, 0.02112);
        assert_eq!(three_winding_v35.unwrap().r31, 0.00898);
        assert_eq!(three_winding_v35.unwrap().w1_rate1, 288.0);
        assert_eq!(three_winding_v35.unwrap().w2_rate1, 288.0);
        assert_eq!(three_winding_v35.unwrap().w3_rate1, 10.0);
    }

    #[test]
    fn parse_2_winding_v35() {
        let two_winding_v35_str: &'static str = "1234, 5678,      0, '1 ', 1, 1, 1, 0.00000E+00, 0.00000E+00, 2, 'TESTXFMR                             ', 1,    1, 1.0000,    0, 1.0000,    0, 1.0000,    0, 1.0000, '                                        '
 2.60000E-04, 2.36000E-02,   100.00,
    0.98040,    0.000,    0.000,   420.00,   480.00,   530.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,  1,        0,   0,    1.50000,    0.51000,    1.50000,    0.51000,  159,    0,    0.00000,    0.00000,      0.000
    1.00000,    0.000,";
        let lines: Vec<&[u8]> = two_winding_v35_str.lines().map(|line| line.as_bytes()).collect();
        let binding = parse_transformers(&lines, 35);
        let two_winding_v35: Option<&Transformer> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(two_winding_v35.unwrap().from_bus, 1234);
        assert_eq!(two_winding_v35.unwrap().to_bus, 5678);
        assert_eq!(two_winding_v35.unwrap().tertiary_bus, 0);
        assert_eq!(two_winding_v35.unwrap().circuit, "1".to_string());
        assert_eq!(two_winding_v35.unwrap().control_mode_1, 1);
        assert_eq!(two_winding_v35.unwrap().r12, 0.00026);
        assert_eq!(two_winding_v35.unwrap().r23, 0.0);
        assert_eq!(two_winding_v35.unwrap().r31, 0.0);
        assert_eq!(two_winding_v35.unwrap().w1_rate1, 420.0);
        assert_eq!(two_winding_v35.unwrap().w2_rate1, 0.0);
        assert_eq!(two_winding_v35.unwrap().w3_rate1, 0.0);
    }

    #[test]
    fn parse_3_winding_v33() {
        let three_winding_v33_str: &'static str = "1234, 5678, 91011, '2 ', 1, 1, 1, 0.00000E+00, 0.00000E+00, 2, 'TESTXFMR                                ', 1,  190, 1.0000,    0, 1.0000,    0, 1.0000,    0, 1.0000, '                                        '
 1.09000E-03, 5.12700E-02,   100.00, 2.11200E-02, 2.05040E-01,   100.00, 0.89800E-02, 2.03950E-01,   100.00,   1.010001894, 105.342819992,
    0.95967,    0.000,    0.000,   288.00,   288.00,     0.00,  1,        0,   0,    1.10000,    0.90000,    1.10000,    0.90000,   33,    0,    0.00000,    0.00000,      0.000
    1.00000,    0.000,    0.000,   288.00,   288.00,     0.00,  0,        0,   0,    1.10000,    0.90000,    1.10000,    0.90000,   33,    0,    0.00000,    0.00000,      0.000
    1.00000,    0.000,  -30.000,    10.00,    10.00,     0.00,  0,        0,   0,    1.10000,    0.90000,    1.10000,    0.90000,   33,    0,    0.00000,    0.00000,      0.000
4444, 5555, 6908, '1 ', 1, 1, 2, 1.05300E+04, 9.86000E-03, 2, 'NEET12                                  ', 1,  191, 1.0000,    0, 1.0000,    0, 1.0000,    0, 1.0000, '                                        '";
        let lines: Vec<&[u8]> = three_winding_v33_str.lines().map(|line| line.as_bytes()).collect();
        let binding = parse_transformers(&lines, 33);
        let three_winding_v35: Option<&Transformer> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(three_winding_v35.unwrap().from_bus, 1234);
        assert_eq!(three_winding_v35.unwrap().to_bus, 5678);
        assert_eq!(three_winding_v35.unwrap().tertiary_bus, 91011);
        assert_eq!(three_winding_v35.unwrap().circuit, "2".to_string());
        assert_eq!(three_winding_v35.unwrap().control_mode_1, 1);
        assert_eq!(three_winding_v35.unwrap().r12, 0.00109);
        assert_eq!(three_winding_v35.unwrap().r23, 0.02112);
        assert_eq!(three_winding_v35.unwrap().r31, 0.00898);
        assert_eq!(three_winding_v35.unwrap().w1_rate1, 288.0);
        assert_eq!(three_winding_v35.unwrap().w2_rate1, 288.0);
        assert_eq!(three_winding_v35.unwrap().w3_rate1, 10.0);
    }

    #[test]
    fn parse_2_winding_v33() {
        let two_winding_v33_str: &'static str = "1234, 5678,      0, '1 ', 1, 1, 1, 0.00000E+00, 0.00000E+00, 2, 'TESTXFMR                             ', 1,    1, 1.0000,    0, 1.0000,    0, 1.0000,    0, 1.0000, '                                        '
 2.60000E-04, 2.36000E-02,   100.00,
    0.98040,    0.000,    0.000,   420.00,   480.00,   530.00,  1,        0,   0,    1.50000,    0.51000,    1.50000,    0.51000,  159,    0,    0.00000,    0.00000,      0.000
    1.00000,    0.000,";
        let lines: Vec<&[u8]> = two_winding_v33_str.lines().map(|line| line.as_bytes()).collect();
        let binding = parse_transformers(&lines, 33);
        let two_winding_v35: Option<&Transformer> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(two_winding_v35.unwrap().from_bus, 1234);
        assert_eq!(two_winding_v35.unwrap().to_bus, 5678);
        assert_eq!(two_winding_v35.unwrap().tertiary_bus, 0);
        assert_eq!(two_winding_v35.unwrap().circuit, "1".to_string());
        assert_eq!(two_winding_v35.unwrap().control_mode_1, 1);
        assert_eq!(two_winding_v35.unwrap().r12, 0.00026);
        assert_eq!(two_winding_v35.unwrap().r23, 0.0);
        assert_eq!(two_winding_v35.unwrap().r31, 0.0);
        assert_eq!(two_winding_v35.unwrap().w1_rate1, 420.0);
        assert_eq!(two_winding_v35.unwrap().w2_rate1, 0.0);
        assert_eq!(two_winding_v35.unwrap().w3_rate1, 0.0);
    }
}
