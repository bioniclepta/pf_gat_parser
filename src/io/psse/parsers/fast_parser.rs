use std::{collections::HashMap, fs::File, io, str::from_utf8};

use memmap2::Mmap;

use crate::io::psse::{components::{area::parse_areas, branch::parse_lines, bus::parse_buses, facts::parse_facts, fixed_shunt::parse_fixedshunts, generator::parse_generators, header::HeaderInfo, impedance_correction::parse_impedance_correction_table, interarea_transfer::parse_area_transfers, load::parse_loads, multisection_line::parse_multisection_lines, multiterminal_dc_line::parse_multiterminal_dc_line, owner::parse_owners, system_switching_device::parse_system_switching_device, transformer::parse_transformers, two_terminal_dc::parse_two_terminal_dc_line, vsc_dc::parse_vsc_dc_line, zone::parse_zones}, pssedata::PSSEData};

/*
******* Section Encodings *******
0: Header
1: Bus
2: Load
3: Fixed Shunt
4: Generator
5: Branch (Lines) (Non-Transformer Branch)
6: Transformer
7: Area (Area Interchange)
*/

pub fn parse_fast(filepath: &str) -> Result<PSSEData, io::Error>  {
    let file: File = File::open(filepath)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let mut psse_data: PSSEData = PSSEData::default();

    //Finds section boundaries
    let lines: Vec<&[u8]> = mmap.split(|&c| c == b'\n').collect();
    let mut section_starts: HashMap<usize, usize> = HashMap::new();
    let mut section_ends: HashMap<usize, usize> = HashMap::new();
    let mut section_number: usize = 0;

    for (i, line_bytes) in lines.iter().enumerate() {
        if i > 5 { break; }
        if let Ok(line) = from_utf8(line_bytes) {
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() && !trimmed_line.starts_with("/") && !trimmed_line.starts_with("@") {
                let parts: Vec<&str> = trimmed_line.split("/").nth(0).unwrap().split(",").map(|s| s.trim()).collect();
                if parts.len() >= 4 && parts.len() <= 7 {
                    psse_data.header = HeaderInfo {
                        ic: parts[0].parse().unwrap_or(0),
                        sbase: parts[1].parse().unwrap_or(100.0),
                        revision: parts[2].parse().unwrap_or(33),
                        transformer_rating_code: parts[3].parse().unwrap_or(0),
                        branch_rating_code: parts[4].parse().unwrap_or(0),
                        system_frequency: parts[5].parse().unwrap_or(60.0),
                    };
                    // Since V34+ have the leading /0 to indicate buses and also contain other junk at the start,
                    // Skip the attempt to locate the bus section
                    if psse_data.header.revision >= 34 {
                        // Increase the section number to get to start the bus section
                        section_starts.insert(section_number, i + 1);
                        section_number += 1;
                        break;
                    }
                    section_starts.insert(section_number, i + 1);
                    section_number += 1;
                // Specifically check for the bus section by looking for sections longer than 10 and starting with a numeric character
                } else if (parts.len() > 10) & (trimmed_line.chars().next().unwrap().is_numeric()) {
                    section_starts.insert(section_number, i + 1);
                    section_number += 1;
                    break;
                }
            }
        }
    }
    let mut found_section_start: bool = false;
    for (i, line_bytes) in lines.iter().enumerate() {
        if let Ok(line) = from_utf8(line_bytes) {
            let trimmed_line = line.trim();
            // Check if the line is a section start
            if trimmed_line.starts_with("0 /") {
                // If a section start has already been found, set the start and end to the same value
                if found_section_start {
                    section_starts.insert(section_number, i );
                    section_number += 1;
                }
                // Sets the section end to the previous section and flags that a new section has been found
                found_section_start = true;
                section_ends.insert(section_number - 1, i);
                continue;
            }
            // If a section start has been found, and the line is not a comment or another line start, add it to the section start
            if found_section_start & !trimmed_line.starts_with("@") & !trimmed_line.starts_with("0 /") {
                // Look specifically for lines that contain data after the section start
                // then add it to the section start and revert the found_section_start variable
                section_starts.insert(section_number, i );
                section_number += 1;
                found_section_start = false;
            }
        }
    }

    let parse_adder: usize = (psse_data.header.revision >= 34) as usize;
    //Parse Buses
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&1), section_ends.get(&1)) {
        //let end_index = section_starts.get(&2).cloned().unwrap_or(lines.len())-1;
        //This fixes some issues since buses are parsed first, and in v33, they don't have a leading '/0'
        if start_index == end_index {
            psse_data.buses = parse_buses(std::slice::from_ref(&lines[start_index-1]));
        } else {
            psse_data.buses = parse_buses(&lines[start_index..end_index]);
        }
    }
    //Parse Loads
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&2), section_ends.get(&2)) {
        psse_data.loads = parse_loads(&lines[start_index..end_index]);
    }
    //Parse Fixed Shunts
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&3), section_ends.get(&3)) {
        psse_data.fixed_shunts = parse_fixedshunts(&lines[start_index..end_index]);
    }
    //Parse Generators
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&4), section_ends.get(&4)) {
        psse_data.generators = parse_generators(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Parse Branches
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&5), section_ends.get(&5)) {
        psse_data.branches = parse_lines(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Parse System Switching Devices (V34+ Only)
    if psse_data.header.revision >= 34 {
        if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&6), section_ends.get(&6)) {
            psse_data.switching_devices = parse_system_switching_device(&lines[start_index..end_index]);
        }
    } else {
        psse_data.switching_devices = Vec::new();
    }
    //Parse Transformers
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(6 + parse_adder)), section_ends.get(&(6 + parse_adder))) {
        psse_data.transformers = parse_transformers(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Parse Areas
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(7 + parse_adder)), section_ends.get(&(7 + parse_adder))) {
        psse_data.areas = parse_areas(&lines[start_index..end_index]);
    }
    //Parse Two Terminal DC
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(8 + parse_adder)), section_ends.get(&(8 + parse_adder))) {
        psse_data.two_terminal_dc = parse_two_terminal_dc_line(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Parse VSC DC line
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(9 + parse_adder)), section_ends.get(&(9 + parse_adder))) {
        psse_data.vsc_dc = parse_vsc_dc_line(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Parse Impedance Correction tables
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(10 + parse_adder)), section_ends.get(&(10 + parse_adder))) {
        psse_data.impedance_correction = parse_impedance_correction_table(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Parse Multi-Terminal DC line
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(11 + parse_adder)), section_ends.get(&(11 + parse_adder))) {
        psse_data.multi_terminal_line = parse_multiterminal_dc_line(&lines[start_index..end_index]);
    }
    //Parse Multi-Terminal DC line
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(12 + parse_adder)), section_ends.get(&(12 + parse_adder))) {
        psse_data.multi_section_line = parse_multisection_lines(&lines[start_index..end_index]);
    }
    //Parse Zones
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(13 + parse_adder)), section_ends.get(&(13 + parse_adder))) {
        psse_data.zones = parse_zones(&lines[start_index..end_index]);
    }
    //Parse Inter-Area Transfer
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(14 + parse_adder)), section_ends.get(&(14 + parse_adder))) {
        psse_data.inter_area_transfer = parse_area_transfers(&lines[start_index..end_index]);
    }
    //Parse Owners
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(15 + parse_adder)), section_ends.get(&(15 + parse_adder))) {
        psse_data.owners = parse_owners(&lines[start_index..end_index]);
    }
    //Parse FACTS
    if let (Some(&start_index), Some(&end_index)) = (section_starts.get(&(16 + parse_adder)), section_ends.get(&(16 + parse_adder))) {
        psse_data.facts = parse_facts(&lines[start_index..end_index], psse_data.header.revision);
    }
    //Return the completed PSS/E data struct
    Ok(psse_data)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_speed() {
        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath: String = format!("{}/cases/v33.raw", dir);
        // let filepath: String = format!("{}/cases/IEEE 118 Bus.RAW", dir);
        let start = Instant::now();
        let _ = parse_fast(&filepath);
        println!("Time to read and parse file: {:?}", start.elapsed());
    }

    #[test]
    fn test_parse_v33() {
        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let filepath: String = format!("{}/test-cases/TEST_parse_psse_v33.RAW", dir);
        let data: PSSEData = parse_fast(&filepath).unwrap();
        //Test Header
        assert_eq!(data.header, HeaderInfo { ic: 0, sbase: 923.45, revision: 33, transformer_rating_code: 0, branch_rating_code: 3, system_frequency: 45.67 });
        //Test Bus
        assert_eq!(data.buses[0].id, 347);
        assert_eq!(data.buses[0].evlo, 0.7654);
        //Test Load
        assert_eq!(data.loads[0].bus_id, 347);
        assert_eq!(data.loads[0].interruptable, 3);
        assert_eq!(data.loads[0].dgen_mw, 0.0);
        // Test Fixed Shunt
        assert_eq!(data.generators[0].bus_id, 24568);
        assert_eq!(data.generators[0].base_loaded, 2);
        assert_eq!(data.generators[0].machine_mode, 0);
        //Test Branches
        assert_eq!(data.branches[0].from_bus, 347);
        assert_eq!(data.branches[0].rate2, 345.67);
        assert_eq!(data.branches[0].owner1, 2);
        //Test System Switching Devices
        assert_eq!(data.switching_devices, Vec::new());
        //Test Parse Transformer
        assert_eq!(data.transformers[0].from_bus, 24568);
        assert_eq!(data.transformers[0].tertiary_bus, 0);
        assert_eq!(data.transformers[0].rma1, 1.23456);
        //Test Parse Transformer
        assert_eq!(data.areas[0].area_id, 2);
        assert_eq!(data.areas[0].swing_bus_id, 1);
        assert_eq!(data.areas[0].name, "CHANGE1".to_string());
        //Test Parse Two Terminal DC line
        assert_eq!(data.two_terminal_dc[0].rec_xfmr_ratio, 0.87654);
        assert_eq!(data.two_terminal_dc[0].inv_comm_cap_x, 0.12345);
        assert_eq!(data.two_terminal_dc[0].name, "TEST2DC".to_string());
        //Test Parse VSC DC line
        assert_eq!(data.vsc_dc[0].owner1, 68);
        assert_eq!(data.vsc_dc[0].control_bus_percent_mvar_2, 90.0);
        assert_eq!(data.vsc_dc[0].name, "TST-VSC".to_string());
        //Test Parse Impedance Correction Tables
        assert_eq!(data.impedance_correction[0].index, 2);
        assert_eq!(data.impedance_correction[0].correction_table_entries[0].imag_complex_scaling_factor, -45.6789);
        //Test Parse Multi-Terminal DC line
        // --- I don't have any examples of this entry in any cases I've found, so I don't have a practical format reference
        assert_eq!(data.multi_terminal_line, Vec::new());
        //Test Parse Multi-Section line
        assert_eq!(data.multi_section_line[0].from_bus, 234567);
        assert_eq!(data.multi_section_line[0].dummy_bus_ids, vec![234569, 234570, 234571, 0, 0, 0, 0, 0, 0]);
        //Test Parse Zones
        assert_eq!(data.zones[0].zone_id, 2);
        assert_eq!(data.zones[0].zone_name, "TEST123".to_string());
        //Test Parse Inter-Area Transfer
        // --- I don't have any examples of this entry in any cases I've found, so I don't have a practical format reference
        assert_eq!(data.inter_area_transfer, Vec::new());
        //Test Parse Owners
        assert_eq!(data.owners[0].owner_id, 2);
        assert_eq!(data.owners[0].owner_name, "OWNER21".to_string());
        //Test Parse FACTS
        assert_eq!(data.owners[0].owner_id, 2);
        assert_eq!(data.owners[0].owner_name, "OWNER21".to_string());
    }
}
