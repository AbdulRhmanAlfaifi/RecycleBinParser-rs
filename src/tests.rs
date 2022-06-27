use crate::RecycleBinParser;
use glob::glob;
use std::fs::File;

#[test]
fn parse_samples() {
    for entry in glob("samples/$I*").expect("Failed to read glob pattren") {
        let mut f = File::open(entry.unwrap()).unwrap();
        let data = RecycleBinParser::from_reader(&mut f).unwrap();
        println!("{:?}", data);
    }
}

#[test]
fn main() {
    let mut f = File::open("samples/$IK9O3HW.txt").expect("Unable to open file");
    let data = RecycleBinParser::from_reader(&mut f).expect("Unable to parse the file");
    println!("{:?}", data);
}
