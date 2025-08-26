use std::str::from_utf8;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each network multisection branch in the PSS/E .RAW case
pub struct MultiSectionLine {
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
    /// ## MET: Branch metered end flag
    /// ### Type: i8
    /// ### Default: 1
    /// ## Parameters
    /// * <= 1 : From bus (I) is the metered end of the branch
    /// * \>=2 : To bus (J) is the metered end of the branch
    pub meter_end: i8,
    /// ## MET: Branch metered end flag
    /// ### Type: Vec<i32>
    /// ### Default: Vec<0, 0, 0, 0, 0, 0, 0, 0, 0>
    /// ## Range
    /// * 1 - 999997
    pub dummy_bus_ids: Vec<i32>,
}

pub fn parse_multisection_lines(lines: &[&[u8]]) -> Vec<MultiSectionLine> {
    lines.par_iter().filter_map(|line_bytes| {
        from_utf8(line_bytes).ok().and_then(|line| {
            let parts: Vec<&str> = line.split(",").map(|s| s.trim()).collect();
            Some(MultiSectionLine{
                from_bus: parts[0].parse().unwrap_or(0),
                to_bus: parts[1].parse().unwrap_or(0),
                circuit: parts[2].replace("'", "").trim().to_string(),
                meter_end: parts[3].parse().unwrap_or(1),
                dummy_bus_ids: vec![
                    parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(8).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(9).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(10).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(11).and_then(|s| s.parse().ok()).unwrap_or(0),
                    parts.get(12).and_then(|s| s.parse().ok()).unwrap_or(0),
                ]
            })
        })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_multiterminal_0_dummies() {
        let multiterminal_str: &'static str = "1997,  1898, '&1', 1,";
        let lines: Vec<&[u8]> = multiterminal_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<MultiSectionLine> = parse_multisection_lines(&lines);
        let multiterminal: Option<&MultiSectionLine> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(multiterminal.unwrap().from_bus, 1997);
        assert_eq!(multiterminal.unwrap().to_bus, 1898);
        assert_eq!(multiterminal.unwrap().circuit, "&1".to_string());
        assert_eq!(multiterminal.unwrap().meter_end, 1);
        assert_eq!(multiterminal.unwrap().dummy_bus_ids, vec![0; 9]);
    }

    #[test]
    fn parse_multiterminal_2_dummies() {
        let multiterminal_str: &'static str = "1997,  1898, '&1', 1, 1111,  2222,";
        let lines: Vec<&[u8]> = multiterminal_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<MultiSectionLine> = parse_multisection_lines(&lines);
        let multiterminal: Option<&MultiSectionLine> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(multiterminal.unwrap().from_bus, 1997);
        assert_eq!(multiterminal.unwrap().to_bus, 1898);
        assert_eq!(multiterminal.unwrap().circuit, "&1".to_string());
        assert_eq!(multiterminal.unwrap().meter_end, 1);
        assert_eq!(multiterminal.unwrap().dummy_bus_ids, vec![1111, 2222, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn parse_multiterminal_9_dummies() {
        let multiterminal_str: &'static str = "1997,  1898, '&1', 1, 1111,  2222, 3333, 4444, 5555, 66, 7, 88888, 9909";
        let lines: Vec<&[u8]> = multiterminal_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<MultiSectionLine> = parse_multisection_lines(&lines);
        let multiterminal: Option<&MultiSectionLine> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(multiterminal.unwrap().from_bus, 1997);
        assert_eq!(multiterminal.unwrap().to_bus, 1898);
        assert_eq!(multiterminal.unwrap().circuit, "&1".to_string());
        assert_eq!(multiterminal.unwrap().meter_end, 1);
        assert_eq!(multiterminal.unwrap().dummy_bus_ids, vec![1111, 2222, 3333, 4444, 5555, 66, 7, 88888, 9909]);
    }
}