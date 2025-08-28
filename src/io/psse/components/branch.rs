use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network branch in the PSS/E .RAW case
pub struct Branch {
    /// ## I: Bus number the branch leaves from
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub from_bus: i32,
    /// ## J: Bus number the branch goes to
    /// ### Type: i32
    /// ### Default: 0
    /// ## Range
    /// * 1 - 999997
    pub to_bus: i32,
    /// ## CKT: Branch circuit identifier
    /// ### Type: String
    /// ### Default: '1'
    pub circuit: String,
    /// ## R: Branch resistance
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub r: f64,
    /// ## X: Branch reactance
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub x: f64,
    /// ## B: Branch charging susceptance
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub b: f64,
    /// ## NAME: Branch name
    /// ### Type: String
    /// ### Default: " "
    pub name: String,
    /// ## RATE1: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate1: f64,
    /// ## RATE2: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate2: f64,
    /// ## RATE3: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate3: f64,
    /// ## RATE4: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate4: f64,
    /// ## RATE5: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate5: f64,
    /// ## RATE6: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate6: f64,
    /// ## RATE7: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate7: f64,
    /// ## RATE8: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate8: f64,
    /// ## RATE9: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate9: f64,
    /// ## RATE10: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate10: f64,
    /// ## RATE11: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate11: f64,
    /// ## RATE12: Branch MVA rating
    /// ### Type: f64
    /// ### Unit: MVA
    /// ### Default: 0.0
    pub rate12: f64,
    /// ## GI: Complex admittance of the line shunt at from bus
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub gi: f64,
    /// ## BI: Complex admittance of the line shunt at from bus
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub bi: f64,
    /// ## GJ: Complex admittance of the line shunt at to bus
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub gj: f64,
    /// ## BJ: Complex admittance of the line shunt at to bus
    /// ### Type: f64
    /// ### Unit: p.u.
    /// ### Default: 0.0
    pub bj: f64,
    /// ## ST: Branch Status
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * 0 : Line out-of-service
    /// * 1 : Line in-service
    pub status: i8,
    /// ## MET: Branch metered end flag
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * <= 1 : From bus (I) is the metered end of the branch
    /// * \>=2 : To bus (J) is the metered end of the branch
    pub meter_end: i8,
    /// ## LEN: Line length
    /// ### Type: f64
    /// ### Default: 0.0
    pub length: f64,        // Branch length (LEN)
    /// ## O1: Branch owner number 1
    /// ### Type: i16
    /// ### Default: 1
    /// ## Range
    /// * 1 - 9999
    pub owner1: i16,
    /// ## O2: Branch owner number 2
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner2: i16,
    /// ## O3: Branch owner number 3
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner3: i16,
    /// ## O4: Branch owner number 4
    /// ### Type: i16
    /// ### Default: 0
    /// ## Range
    /// * 1 - 9999
    pub owner4: i16,
    /// ## F1: Branch ownership percentage 1
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner1_percent: f64,
    /// ## F2: Branch ownership percentage 2
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner2_percent: f64,
    /// ## F3: Branch ownership percentage 3
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner3_percent: f64,
    /// ## F4: Branch ownership percentage 4
    /// ### Type: f64
    /// ### Default: 1.0
    pub owner4_percent: f64,
}

pub fn parse_lines(lines: &[&[u8]], psse_version: i8) -> Vec<Branch> {
    // a bool to int to add to the line parsing since V34 introduced new variables in the middle of everything
    let parse_adder: usize = (psse_version >= 34) as usize;
    let rating_adder: usize = parse_adder * 10;
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(Branch{
                from_bus: parts[0].parse().unwrap_or(0),
                to_bus: parts[1].parse().unwrap_or(0),
                circuit: parts[2].replace("'", "").trim().to_string(),
                r: parts[3].parse().unwrap_or(0.0),
                x: parts[4].parse().unwrap_or(0.0001),
                b: parts[5].parse().unwrap_or(0.0),
                name: (psse_version >= 34).then_some(parts[6].replace("'", "").trim().to_string()).unwrap_or(" ".to_string()),
                rate1: parts[6 + parse_adder].parse().unwrap_or(0.0),
                rate2: parts[7 + parse_adder].parse().unwrap_or(0.0),
                rate3: parts[8 + parse_adder].parse().unwrap_or(0.0),
                rate4: (psse_version >= 34).then_some(parts[9 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate5: (psse_version >= 34).then_some(parts[10 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate6: (psse_version >= 34).then_some(parts[11 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate7: (psse_version >= 34).then_some(parts[12 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate8: (psse_version >= 34).then_some(parts[13 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate9: (psse_version >= 34).then_some(parts[14 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate10: (psse_version >= 34).then_some(parts[15 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate11: (psse_version >= 34).then_some(parts[16 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                rate12: (psse_version >= 34).then_some(parts[17 + parse_adder].parse().unwrap_or(0.0)).unwrap_or(0.0),
                gi: parts[9 + rating_adder].parse().unwrap_or(0.0),
                bi: parts[10 + rating_adder].parse().unwrap_or(0.0),
                gj: parts[11 + rating_adder].parse().unwrap_or(0.0),
                bj: parts[12 + rating_adder].parse().unwrap_or(0.0),
                status: parts[13 + rating_adder].parse().unwrap_or(1),
                meter_end: parts[14 + rating_adder].parse().unwrap_or(0),
                length: parts[15 + rating_adder].parse().unwrap_or(0.0),
                owner1: parts[16 + rating_adder].parse().unwrap_or(1),
                owner1_percent: parts[17 + rating_adder].parse().unwrap_or(1.0),
                owner2: parts.get(18 + rating_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner2_percent: parts.get(19 + rating_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner3: parts.get(20 + rating_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner3_percent: parts.get(21 + rating_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
                owner4: parts.get(22 + rating_adder).and_then(|s| s.parse().ok()).unwrap_or(0),
                owner4_percent: parts.get(23 + rating_adder).and_then(|s| s.parse().ok()).unwrap_or(1.0),
            })
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_branch_v35() {
        let branch_str: &'static str = "1111,   2222,'5 ',9.087000E-03,8.765000E-03,   1.22295,'                                        ',   550.00,   890.00,     700.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,     0.00,   0.00000,   0.00000,   0.00000,   0.00000, 2,3,   5.20000, 863,  1.0000,";
        let lines: Vec<&[u8]> = branch_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<Branch> = parse_lines(&lines, 35);
        let branch: Option<&Branch> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(branch.unwrap().from_bus, 1111);
        assert_eq!(branch.unwrap().to_bus, 2222);
        assert_eq!(branch.unwrap().circuit, "5".to_string());
        assert_eq!(branch.unwrap().status, 2);
        assert_eq!(branch.unwrap().meter_end, 3);
        assert_eq!(branch.unwrap().length, 5.2);
    }

    #[test]
    fn parse_branch_v33() {
        let branch_str: &'static str = " 1111,   2222,'5 ',9.087000E-03,8.765000E-03,   1.22295,   550.00,   890.00,     700.00,   0.00000,   0.00000,   0.00000,   0.00000, 2,3,   5.20000, 863,  1.0000,";
        let lines: Vec<&[u8]> = branch_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<Branch> = parse_lines(&lines, 33);
        let branch: Option<&Branch> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(branch.unwrap().from_bus, 1111);
        assert_eq!(branch.unwrap().to_bus, 2222);
        assert_eq!(branch.unwrap().circuit, "5".to_string());
        assert_eq!(branch.unwrap().status, 2);
        assert_eq!(branch.unwrap().meter_end, 3);
        assert_eq!(branch.unwrap().length, 5.2);
    }
}