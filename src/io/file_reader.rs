use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_file_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    lines
}