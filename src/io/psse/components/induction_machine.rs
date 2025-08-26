
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InductionMachine {
    pub bus_id: i32,    //Bus ID the induction machine is at
    pub id: String, // Induction machine id
    pub status: i8, //Induction machine status (ST) {0: Out of service, 1: In service}
    pub standard_code: i8, //Induction machine standard code (SC) {1: NEMA, 2: IEC}
    pub design_code: i8, //Induction machine design code (DC) {0: Custom, 1: NEMA A, 2: NEMA B / IEC N, 3: NEMA C / IEC H, 4: NEMA D, 5: NEMA E}
    pub area: i32,  //Induction machine area number (AREA)
    pub zone: i32, //Induction machine zone number (ZONE)
    pub owner: i32, //Induction machine owner number (OWNER)
    pub torque_code: i8, // Induction machine load torque variation (TC) {1: Simple, 2: WECC}
    pub base_power_code: i8, //Induction machine base power code (BC) {1: Mechanical MW output, 2: Electrical MVA input}
    pub sbase: f64, //Nominal power base (MBASE)
    pub rated_voltage: f64, //Induction machine rated voltage (RATEKV)
    pub power_code: i32,    //Couldn't find documentation on this (PC)
    pub sched_active_power: f64,    //Scheduled active power (PSET)
    pub inertia: f64,   //Induction machine inertia (H)
    pub torque_params: Vec<f64>,    //List of induction machine parameters (A, B, D, E)
    pub arm_resistance: f64,    //IM armature resistance (RA)
    pub arm_reactance: f64,    //IM armature reactance (XA)
    pub mag_reactance: f64,    //IM armature magnetizing reactance (XM)
    pub windng1_resist: f64,    //Resistance of IM rotor winding 1 (R1)
    pub windng1_react: f64,    //Reactance of IM rotor winding 1 (R1)
    pub windng2_resist: f64,    //Resistance of IM rotor winding 2 (R1)
    pub windng2_react: f64,    //Reactance of IM rotor winding 2 (R1)
    pub windng3_react: f64,    //Reactance of IM rotor winding 3 (R1)
    pub volt_point_1: f64, // Open circuit voltage for IM terminal 1 (E1)
    pub v_sat1: f64,    //Saturation factor for volt_point_1 (SE1)
    pub volt_point_2: f64, // Open circuit voltage for IM terminal 2 (E2)
    pub v_sat2: f64,    //Saturation factor for volt_point_2 (SE2)
    pub stator_current_1: f64,  //Stator current of IM (IA1)
    pub stator_current_2: f64,  //Stator current of IM (IA2)
    pub saturation_mult: f32 //Multiplier value for the saturated value (XAMULT) {values between 0 - 1}


}

impl From<Vec<String>> for InductionMachine {

    fn from(values: Vec<String>) -> Self {

        InductionMachine {
            bus_id: values.get(0).and_then(|s| s.parse().ok()).unwrap_or(0),
            id: values.get(1).and_then(|s| s.parse().ok()).unwrap_or("1".to_string()),
            status: values.get(2).and_then(|s| s.parse().ok()).unwrap_or(1),
            standard_code: values.get(3).and_then(|s| s.parse().ok()).unwrap_or(1),
            design_code: values.get(4).and_then(|s| s.parse().ok()).unwrap_or(2),
            area: values.get(5).and_then(|s| s.parse().ok()).unwrap_or(0),
            zone: values.get(6).and_then(|s| s.parse().ok()).unwrap_or(0),
            owner: values.get(7).and_then(|s| s.parse().ok()).unwrap_or(0),
            torque_code: values.get(8).and_then(|s| s.parse().ok()).unwrap_or(1),
            base_power_code: values.get(9).and_then(|s| s.parse().ok()).unwrap_or(1),
            sbase: values.get(10).and_then(|s| s.parse().ok()).unwrap_or(100.0),
            rated_voltage: values.get(11).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            power_code: values.get(12).and_then(|s| s.parse().ok()).unwrap_or(1),
            sched_active_power: values.get(13).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            inertia: values.get(14).and_then(|s| s.parse().ok()).unwrap_or(1.0),
            torque_params: vec![
                values.get(15).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                values.get(16).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                values.get(17).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                values.get(18).and_then(|s| s.parse().ok()).unwrap_or(1.0),
            ],arm_resistance: values.get(19).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            arm_reactance: values.get(20).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            mag_reactance: values.get(21).and_then(|s| s.parse().ok()).unwrap_or(2.5),
            windng1_resist: values.get(22).and_then(|s| s.parse().ok()).unwrap_or(999.0),
            windng1_react: values.get(23).and_then(|s| s.parse().ok()).unwrap_or(999.0),
            windng2_resist: values.get(24).and_then(|s| s.parse().ok()).unwrap_or(999.0),
            windng2_react: values.get(25).and_then(|s| s.parse().ok()).unwrap_or(999.0),
            windng3_react: values.get(26).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            volt_point_1: values.get(27).and_then(|s| s.parse().ok()).unwrap_or(1.0),
            v_sat1: values.get(28).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            volt_point_2: values.get(29).and_then(|s| s.parse().ok()).unwrap_or(1.2),
            v_sat2: values.get(30).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            stator_current_1: values.get(31).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            stator_current_2: values.get(32).and_then(|s| s.parse().ok()).unwrap_or(0.0),
            saturation_mult: values.get(33).and_then(|s| s.parse().ok()).unwrap_or(1.0),
        }
    }
}