use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "../tpch/nation.tbl";
    let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
    let reader = BufReader::new(file);
    let line = reader.lines().nth(25);
    match line {
        None => print!("EMPTY"),
        Some(x) => match x {
            Ok(o) => {
                let mut vec: Vec<&str> = o.split("|").collect();
                vec.pop();
                println!("{:#?}", vec);
            }
            Err(e) => print!("Error {:#?}", e),
        },
    }

    // for line in reader.lines().nth(26) {
    //     print!("HELLO");
    //     let line = line.expect("Unable to read line");
    //     let mut vec: Vec<&str> = line.split("|").collect();
    //     vec.pop();
    //     if vec.is_empty() {
    //         println!("EMPTY");
    //     } else {
    //         println!("{:#?}", 5);
    //     }
    // }
}
