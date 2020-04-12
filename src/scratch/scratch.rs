#[derive(Debug)]
enum Bits {
    INT(isize),
    DOUBLE(i128),
    STRING(String)
}

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let mut vec:Vec<String> = Vec::new();
    // vec.push("Hello".to_string());
    // // vec.push(bits::INT(1));
    // // vec.push(bits::DOUBLE(18.00));
    // // vec.push(bits::STRING(String::from("Hello")));
    // println!("{:#?}", vec);
    let file_name = "nation.tbl";
    let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
    let reader = BufReader::new(file);
    let line = reader.lines().next();
    // let mut vec: Vec<&str> = line.split("|").collect();
        println!("{:#?}", line)
    // for line in reader.lines().skip(1) {
    //     let line = line.expect("Unable to read line");
    //     let mut vec: Vec<&str> = line.split("|").collect();
    //     vec.pop();
    //     println!("{:#?}", vec)

    // }

    
}