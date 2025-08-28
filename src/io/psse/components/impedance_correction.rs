use std::str::from_utf8;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each impedance correction table entry in the impedance table correction series
pub struct ImpedanceCorrectionEntry {
    /// ## Ti: Off nominal turns ratio or the phase shift angle in degrees
    /// ### Type: f64
    pub tap: f64,
    /// ## Rei: Complex scaling factor (Real)
    /// ### Type: f64
    pub real_complex_scaling_factor: f64,
    /// ## Imi: Complex scaling factor (Imaginary)
    /// ### Type: f64
    pub imag_complex_scaling_factor: f64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// A struct containing the information for each impedance correction table entry series in the PSS/E .RAW case
pub struct ImpedanceCorrectionTable {
    /// ## I: Impedance correction table number
    /// ### Type: i32
    pub index: i32,
    /// ## I: Vec of impedance correction table entries
    /// ### Type: ImpedanceCorrectionEntry
    pub correction_table_entries: Vec<ImpedanceCorrectionEntry>
}

impl ImpedanceCorrectionEntry {
    fn is_end_of_table(&self) -> bool {
        (self.tap == 0.0) &
        (self.real_complex_scaling_factor == 0.0) &
        (self.imag_complex_scaling_factor == 0.0)
    }
}

pub fn parse_impedance_correction_table(lines: &[&[u8]], psse_version: i8) -> Vec<ImpedanceCorrectionTable> {
    //Create a new vec of correction tables to push into
    let mut correction_tables: Vec<ImpedanceCorrectionTable> = Vec::new();
    //Define while loop variables
    let mut line_position: usize = 0;
    let mut entry_position: usize = 0;
    //Define a flag to check if this is a new entry
    let mut begin_entry: bool = true;
    //Define a new correction table
    let mut correction_table: ImpedanceCorrectionTable = ImpedanceCorrectionTable::default();

    //Loop over each line of the correction table
    while line_position < lines.len() {
        //Read the entry line
        if let Ok(line1_str) = from_utf8(lines[line_position]) {
            //Split the entry line by commas and collect into a vec
            let parts1: Vec<&str> = line1_str.split(',').map(|s| s.trim()).collect();
            //Checks if this is the begining of a new table entry
            if begin_entry {
                //Creates a new correction table
                correction_table = ImpedanceCorrectionTable {
                    //Sets the correction index from the first value of the entry
                    index: parts1[0].parse::<i32>().unwrap_or(0),
                    //Creates an empty vec to store the table entries
                    correction_table_entries: Vec::new(),
                };
                //Increase the entry position by one to move past the index
                entry_position += 1;
                //Change the entry flag
                begin_entry = false;
            }

            //Loop over each entry position
            while entry_position < parts1.len() {
                //Create a new table entry, reading 3 parts at a time since each entry will always be 3 long
                let table_entry: ImpedanceCorrectionEntry = ImpedanceCorrectionEntry {
                    tap: parts1[entry_position].parse::<f64>().unwrap_or(0.0),
                    real_complex_scaling_factor: parts1.get(entry_position + 1).map_or("0.0", |v| v).parse::<f64>().unwrap_or(0.0),
                    imag_complex_scaling_factor: parts1.get(entry_position + 2).map_or("0.0", |v| v).parse::<f64>().unwrap_or(0.0),
                };
                //Checks if the entry is all 0.0 indicating the end of this table
                if table_entry.is_end_of_table() {
                    //Push the correction table
                    //Don't push the blank table entry
                    correction_tables.push(correction_table.clone());
                    //Set the entry flag to read the index
                    begin_entry = true;
                    //Break out from reading the entries and start a new line
                    break;
                } else {
                    //If there are still more entries to read, keep going
                    //Push the table entry to the main table
                    correction_table.correction_table_entries.push(table_entry);
                    //Increase the position to read the next 3
                    entry_position += 3;                    
                }
            }
            if psse_version <= 33 {
                //Push the correction table
                correction_tables.push(correction_table.clone());
                //Set the entry flag to read the index
                begin_entry = true;
            }
            //Change the entry position back to 0
            entry_position = 0;
            //Increase the line position to read the next line
            line_position += 1;
        }        
    }
    //Return the vec of collection tables
    correction_tables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correction_table_1_line() {
        let correction_table_str: &'static str = " 1,  80.00000,  1.00000,  0.00000,  -36.00000,  0.53800,  0.00000,  -54.40000,  0.49200,  0.00000,  -15.80000,  0.02400,  0.00000,   -20.30000,  0.09400,  0.00000,    0.00000,  0.00000,  0.00000";
        let lines: Vec<&[u8]> = correction_table_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<ImpedanceCorrectionTable> = parse_impedance_correction_table(&lines, 35);
        let correction_table: Option<&ImpedanceCorrectionTable> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(correction_table.unwrap().index, 1);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().tap, 80.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().real_complex_scaling_factor, 1.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().imag_complex_scaling_factor, 0.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(1).unwrap().tap, -36.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(1).unwrap().real_complex_scaling_factor, 0.538);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(1).unwrap().imag_complex_scaling_factor, 0.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().tap, -20.3);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().real_complex_scaling_factor, 0.09400);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().imag_complex_scaling_factor, 0.0);
    }

    #[test]
    fn parse_correction_table_2_line() {
        let correction_table_str: &'static str = " 1,  80.00000,  1.00000,  0.00000,  -36.00000,  0.53800,  0.00000,  -54.40000,  0.49200,  0.00000,  -15.80000,  0.02400,  0.00000,   -20.30000,  0.09400,  0.00000,    0.00000,  0.11000,  0.00000
       10.30000,  0.02300,  0.00000,   15.40000,  0.01400,  0.00000,   84.40000,  0.29200,  0.00000,   66.00000,  0.46800,  0.00000,   20.00000,  1.00000,  0.00000,    0.00000,  0.00000,  0.00000";
        let lines: Vec<&[u8]> = correction_table_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<ImpedanceCorrectionTable> = parse_impedance_correction_table(&lines, 35);
        let correction_table: Option<&ImpedanceCorrectionTable> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(correction_table.unwrap().index, 1);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().tap, 80.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().real_complex_scaling_factor, 1.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().imag_complex_scaling_factor, 0.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(2).unwrap().tap, -54.4);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(2).unwrap().real_complex_scaling_factor, 0.492);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(2).unwrap().imag_complex_scaling_factor, 0.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().tap, 20.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().real_complex_scaling_factor, 1.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().imag_complex_scaling_factor, 0.0);
    }

    #[test]
    fn parse_correction_table_3_line() {
        let correction_table_str: &'static str = " 1,  80.00000,  1.00000,  0.00000,  -36.00000,  0.53800,  0.00000,  -54.40000,  0.49200,  0.00000,  -15.80000,  0.02400,  0.00000,   -20.30000,  0.09400,  0.00000,    0.00000,  0.11000,  0.00000
        10.30000,  0.02300,  0.00000,   15.40000,  0.01400,  0.00000,   84.40000,  0.29200,  0.00000,   66.00000,  0.46800,  0.00000,   20.00000,  1.00000,  0.00000,    -15.80000,  0.02400,  0.00000
       10.30000,  0.02300,  0.00000,   15.40000,  0.01400,  0.00000,   84.40000,  0.29200,  0.00000,   66.00000,  0.46800,  0.00000,   98.12300,  5.92800,  2.10000,    0.00000,  0.00000,  0.00000";
        let lines: Vec<&[u8]> = correction_table_str.lines().map(|line| line.as_bytes()).collect();
        let binding: Vec<ImpedanceCorrectionTable> = parse_impedance_correction_table(&lines, 35);
        let correction_table: Option<&ImpedanceCorrectionTable> = binding.get(0);
        //To avoid checking everything, check bits and pieces to make sure things are in the right place
        //If any values were distorted on the lines from the adder, it would show here
        assert_eq!(correction_table.unwrap().index, 1);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().tap, 80.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().real_complex_scaling_factor, 1.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.first().unwrap().imag_complex_scaling_factor, 0.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(3).unwrap().tap, -15.8);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(3).unwrap().real_complex_scaling_factor, 0.02400);
        assert_eq!(correction_table.unwrap().correction_table_entries.get(3).unwrap().imag_complex_scaling_factor, 0.0);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().tap, 98.123);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().real_complex_scaling_factor, 5.928);
        assert_eq!(correction_table.unwrap().correction_table_entries.last().unwrap().imag_complex_scaling_factor, 2.1);
    }
}