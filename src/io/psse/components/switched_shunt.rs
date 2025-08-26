use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network switched shunt in the PSS/E .RAW case
pub struct SwitchedShunt {
    /// ## I: Switched shunt Number
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub bus_id: i32,
    /// ## ID: Switched shunt identifier
    /// ### Type: String
    /// ### Default: '1'
    pub id: String,
    /// ## MODSW: Switched shunt control mode
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Locked
    /// * 1 : Discrete adjustment, controlling the voltage locally or at the remote bus
    /// * 2 : Continuous adjustment, controlling the voltage locally or at the remote bus
    /// * 3 : Discrete adjustment, controlling the reactive power of the plant at the remote bus
    /// * 4 : Discrete adjustment, controlling the reactive power at VSC-DC line at the remote bus
    /// * 5 : Discrete adjustment, controlling the admittance setting of the switched shunt at the remote bus
    /// * 6 : Discrete adjustment, controlling the reactive power of the FACTS device at the remote bus
    pub control_mode: i8,
    /// ## ADJM: Adjustment method
    /// ### Type: i8
    /// ### Default: 0
    /// ## Parameters
    /// * 0 : steps and blocks are switched on in input order, and off in reverse
    /// * 1 : steps and blocks are switched on in the order of the next highest or lowest admittance
    pub adjust_method: i8,
    /// ## STAT: Switched shunt Status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Switched shunt out-of-service
    /// * 1 : Switched shunt in-service
    pub status: i8,
    /// ## VSWHI: Switched shunt maximum controlled voltage or reactive power upper limit
    /// ### Type: f64
    /// ### Default: 1.0
    pub upper_limit: f64,
    /// ## VSWLO: Switched shunt minimum controlled voltage or reactive power lower limit
    /// ### Type: f64
    /// ### Default: 1.0
    pub x_lower_limit: f64,
    /// ## SWREG: Bus ID controlled by the switched shunt
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub controlled_bus_id: i32,
    /// ## NREG: Bus node controlled by the switched shunt
    /// ### Type: i32
    /// ### Default: 0
    pub controlled_bus_node: i32,
    /// ## RMPCT: The percent of the total MVAR required to hold the voltage at the bus controlled by the switched shunt
    /// ### Type: f64
    /// ### Default: 100.0
    pub mvar_contribution_perc: f64,
    /// ## RMIDNT: The name of the element controlled by the switched shunt
    /// ### Type: String
    /// ### Default: " "
    pub controlled_bus_name: String, // Name of bus SS controls the voltage of (Only used when MODSW is 4) (RMIDNT)
    /// ## BINIT: Initial switched shunt admittance
    /// ### Type: f64
    /// ### Units: Mvar (at unity voltage)
    /// ### Default: 0.0
    pub b_init: f64,
    /// ## Si: Switched shunt block status (1-8)
    /// ### Type: Vec\<i8>   
    /// ### Default: Vec\<1; 8>
    /// ## Parameters
    /// * 0 : Switched shunt out-of-service
    /// * 1 : Switched shunt in-service
    pub block_status: Vec<i8>,
    /// ## Ni: Switched shunt block steps (1-8)
    /// ### Type: Vec\<i32>   
    /// ### Default: Vec\<0; 8>
    pub steps: Vec<i32>,
    /// ## Si: Switched shunt block admittance increment (1-8)
    /// ### Type: Vec\<f64>
    /// ### Units: Mvar (at unity voltage) 
    /// ### Default: Vec\<0.0; 8>
    pub b_increment: Vec<f64>   //Increment for each step for each block
}

pub fn parse_switched_shunts(lines: &[&[u8]], psse_version: i8) -> Vec<SwitchedShunt> {
    let parse_adder_1: usize = (psse_version >= 34) as usize;
    let parse_adder_2: usize = parse_adder_1 * 2;
    
    let v33_status: Vec<i8> = vec![1, 1, 1, 1, 1, 1, 1, 1];
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(SwitchedShunt{
                bus_id: parts[0].parse().unwrap_or(0),
                id: parts[1].replace("'", "").trim().to_string(),
                control_mode: parts[1 + parse_adder_1].parse().unwrap_or(1),
                adjust_method: parts[2 + parse_adder_1].parse().unwrap_or(0),
                status: parts[3 + parse_adder_1].parse().unwrap_or(1),
                upper_limit: parts[4 + parse_adder_1].parse().unwrap_or(1.0),
                x_lower_limit: parts[5 + parse_adder_1].parse().unwrap_or(1.0),
                controlled_bus_id: parts[6 + parse_adder_1].parse().unwrap_or(0),
                controlled_bus_node: parts[7 + parse_adder_1].parse().unwrap_or(0),
                mvar_contribution_perc: parts[7 + parse_adder_2].parse().unwrap_or(100.0),
                controlled_bus_name: parts[8 + parse_adder_2].replace("'", "").trim().to_string(),
                b_init: parts[9 + parse_adder_2].parse().unwrap_or(0.0),
                block_status: (psse_version >= 34).then_some(
                    vec![
                    parts.get(12).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(15).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(18).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(21).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(24).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(27).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(30).and_then(|s| s.parse().ok()).unwrap_or(1),
                    parts.get(33).and_then(|s| s.parse().ok()).unwrap_or(1),
                    ]
                ).unwrap_or(v33_status.clone()),
                steps: vec![
                    parts.get(10 + (parse_adder_1 * 3)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(12 + (parse_adder_1 * 4)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(14 + (parse_adder_1 * 5)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(16 + (parse_adder_1 * 6)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(18 + (parse_adder_1 * 7)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(20 + (parse_adder_1 * 8)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(22 + (parse_adder_1 * 9)).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(24 + (parse_adder_1 * 10)).and_then(|s| s.parse().ok()).unwrap_or(0),
                ],
                b_increment: vec![
                    parts.get(11 + (parse_adder_1 * 3)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(13 + (parse_adder_1 * 4)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(15 + (parse_adder_1 * 5)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(17 + (parse_adder_1 * 6)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(19 + (parse_adder_1 * 7)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(21 + (parse_adder_1 * 8)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(23 + (parse_adder_1 * 9)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    parts.get(25 + (parse_adder_1 * 10)).and_then(|s| s.parse().ok()).unwrap_or(0.0),
                ],
            })
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ss_1_entry_v35() {
        let ss_str: &'static str = "  55555, '8 ',  1,0,1,    1.04000,   1.00000,      1907,   1708,  90.0,'SSTEST       ',  225.000,  0,  4,   75.000,";
        let lines: Vec<&[u8]> = ss_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<SwitchedShunt> = parse_switched_shunts(&lines, 35);
        let switched_shunt: Option<&SwitchedShunt> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(switched_shunt.unwrap().bus_id, 55555);
        assert_eq!(switched_shunt.unwrap().id, "8".to_string());
        assert_eq!(switched_shunt.unwrap().control_mode, 1);
        assert_eq!(switched_shunt.unwrap().adjust_method, 0);
        assert_eq!(switched_shunt.unwrap().status, 1);
        assert_eq!(switched_shunt.unwrap().upper_limit, 1.04);
        assert_eq!(switched_shunt.unwrap().x_lower_limit, 1.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_id, 1907);
        assert_eq!(switched_shunt.unwrap().controlled_bus_node, 1708);
        assert_eq!(switched_shunt.unwrap().mvar_contribution_perc, 90.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_name, "SSTEST".to_string());
        assert_eq!(switched_shunt.unwrap().b_init, 225.0);
        assert_eq!(switched_shunt.unwrap().block_status, vec![0, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(switched_shunt.unwrap().steps, vec![4, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(switched_shunt.unwrap().b_increment, vec![75.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn parse_ss_7_entry_v35() {
        let ss_str: &'static str = "  55555, '8 ',  1,0,1,    1.04000,   1.00000,      1907,   1708,  90.0,'SSTEST       ',  225.000,  1,  2,   75.000, 2,  3,   80.000, 3,  4,   85.000, 4,  5,   90.000, 5,  6,   95.000, 6,  7,   100.000, 7,  8,   105.000,";
        let lines: Vec<&[u8]> = ss_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<SwitchedShunt> = parse_switched_shunts(&lines, 35);
        let switched_shunt: Option<&SwitchedShunt> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(switched_shunt.unwrap().bus_id, 55555);
        assert_eq!(switched_shunt.unwrap().id, "8".to_string());
        assert_eq!(switched_shunt.unwrap().control_mode, 1);
        assert_eq!(switched_shunt.unwrap().adjust_method, 0);
        assert_eq!(switched_shunt.unwrap().status, 1);
        assert_eq!(switched_shunt.unwrap().upper_limit, 1.04);
        assert_eq!(switched_shunt.unwrap().x_lower_limit, 1.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_id, 1907);
        assert_eq!(switched_shunt.unwrap().controlled_bus_node, 1708);
        assert_eq!(switched_shunt.unwrap().mvar_contribution_perc, 90.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_name, "SSTEST".to_string());
        assert_eq!(switched_shunt.unwrap().b_init, 225.0);
        assert_eq!(switched_shunt.unwrap().block_status, vec![1, 2, 3, 4, 5, 6, 7, 1]);
        assert_eq!(switched_shunt.unwrap().steps, vec![2, 3, 4, 5, 6, 7, 8, 0]);
        assert_eq!(switched_shunt.unwrap().b_increment, vec![75.0, 80.0, 85.0, 90.0, 95.0, 100.0, 105.0, 0.0]);
    }

    #[test]
    fn parse_ss_1_entry_v33() {
        let ss_str: &'static str = "  55555,  1,0,1,    1.04000,   1.00000,      1907,  90.0,'SSTEST       ',  225.000,  4,   75.000,";
        let lines: Vec<&[u8]> = ss_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<SwitchedShunt> = parse_switched_shunts(&lines, 33);
        let switched_shunt: Option<&SwitchedShunt> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(switched_shunt.unwrap().bus_id, 55555);
        assert_eq!(switched_shunt.unwrap().id, "1".to_string());
        assert_eq!(switched_shunt.unwrap().control_mode, 1);
        assert_eq!(switched_shunt.unwrap().adjust_method, 0);
        assert_eq!(switched_shunt.unwrap().status, 1);
        assert_eq!(switched_shunt.unwrap().upper_limit, 1.04);
        assert_eq!(switched_shunt.unwrap().x_lower_limit, 1.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_id, 1907);
        assert_eq!(switched_shunt.unwrap().controlled_bus_node, 0);
        assert_eq!(switched_shunt.unwrap().mvar_contribution_perc, 90.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_name, "SSTEST".to_string());
        assert_eq!(switched_shunt.unwrap().b_init, 225.0);
        assert_eq!(switched_shunt.unwrap().block_status, vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(switched_shunt.unwrap().steps, vec![4, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(switched_shunt.unwrap().b_increment, vec![75.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn parse_ss_7_entry_v33() {
        let ss_str: &'static str = "  55555,  1,0,1,    1.04000,   1.00000,      1907,  90.0,'SSTEST       ',  225.000,  2,   75.000,  3,   80.000,  4,   85.000,  5,   90.000,  6,   95.000,  7,   100.000,  8,   105.000,";
        let lines: Vec<&[u8]> = ss_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<SwitchedShunt> = parse_switched_shunts(&lines, 33);
        let switched_shunt: Option<&SwitchedShunt> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(switched_shunt.unwrap().bus_id, 55555);
        assert_eq!(switched_shunt.unwrap().id, "1".to_string());
        assert_eq!(switched_shunt.unwrap().control_mode, 1);
        assert_eq!(switched_shunt.unwrap().adjust_method, 0);
        assert_eq!(switched_shunt.unwrap().status, 1);
        assert_eq!(switched_shunt.unwrap().upper_limit, 1.04);
        assert_eq!(switched_shunt.unwrap().x_lower_limit, 1.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_id, 1907);
        assert_eq!(switched_shunt.unwrap().controlled_bus_node, 0);
        assert_eq!(switched_shunt.unwrap().mvar_contribution_perc, 90.0);
        assert_eq!(switched_shunt.unwrap().controlled_bus_name, "SSTEST".to_string());
        assert_eq!(switched_shunt.unwrap().b_init, 225.0);
        assert_eq!(switched_shunt.unwrap().block_status, vec![1; 8]);
        assert_eq!(switched_shunt.unwrap().steps, vec![2, 3, 4, 5, 6, 7, 8, 0]);
        assert_eq!(switched_shunt.unwrap().b_increment, vec![75.0, 80.0, 85.0, 90.0, 95.0, 100.0, 105.0, 0.0]);
    }
}