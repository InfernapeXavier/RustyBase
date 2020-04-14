
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let file_name = "nation.tbl";
    let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
    let reader = BufReader::new(file);


    // for line in reader.lines().nth(3){
    //     let line = line.expect("Unable to read line");
    //     let mut vec: Vec<&str> = line.split("|").collect();
    //     vec.pop();
    //     println!("{:#?}", vec);
    //     break;
    // }
    
    let mut line = reader.lines();
    for _ in 0..5 {
        let newline = line.next();
        println!("{:#?}", newline);

        let newline = match newline {
            None => String::from("No Line!"),
            Some(x) =>  x.expect("Unable to read line")
        };

        let mut vec: Vec<&str> = newline.split("|").collect();
        vec.pop();
        if vec.is_empty() {
            println!("EOF", );
        } else {
            let mut v = Vec::new();
            for x in vec {
                v.push(x.to_string());
            }
            println!("{:#?}", v);
        }
    }


}