
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
    
    let mut line = reader.lines();
    for i in 0..5 {
        let mut newline = line.next();
        let newline = match newline {
            None => String::from("No Line!"),
            Some(x) =>  x.expect("Unable to read line")
        };

        let mut vec: Vec<&str> = newline.split("|").collect();
        vec.pop();
        if vec.is_empty() {
            println!("EOF", );
        } else {
            println!("{:#?}", vec);
        }
    }


}