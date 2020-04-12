
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let file_name = "nation.tbl";
    let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
    let reader = BufReader::new(file);


    // for line in reader.lines().skip(1){
    //     let line = line.expect("Unable to read line");
    //     let mut vec: Vec<&str> = line.split("|").collect();
    //     vec.pop();
    //     println!("{:#?}", vec);
    //     break;
    // }
    
    let line = reader.lines().skip(1).next();
    let line = match line {
        None => String::from("No Line!"),
        Some(x) =>  x.expect("Unable to read line")
    };

    let mut vec: Vec<&str> = line.split("|").collect();
    vec.pop();
    println!("{:#?}", vec);

    // println!("{:#?}", match line {
    //     None => String::from("No Line!"),
    //     Some(x) =>  x.expect("Unable to read line")
    // }
    // )
}