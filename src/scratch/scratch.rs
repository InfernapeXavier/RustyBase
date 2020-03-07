use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("catalog").expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let split = line.split_whitespace();

        let vec: Vec<&str> = split.collect();
        println!("{:#?}, {}",vec, vec.len())
    }


}