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

#[test]
fn test_read_lines() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let filepath: String = format!("{}/cases/IEEE 118 Bus.RAW", dir);
    let lines = read_file_lines(&filepath);
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        println!("{}", line);
    }
}