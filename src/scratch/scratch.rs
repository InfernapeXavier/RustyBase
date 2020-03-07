pub enum DataType {
    INT,
    DOUBLE,
    STRING,
}

fn main() {

  schema("catalog", "nation");

}


fn schema(file_name: &str, rel_name: &str) {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let foo = File::open(file_name).expect("Unable to open file"); // open file in read mode
    let reader = BufReader::new(foo);
    let mut scans: usize = 1;
    let mut is_schema: bool = false;
    let mut is_required: bool = false;
    let mut num_atts = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        // if the file is invalid, cause Rust to panic
        if scans == 1 {
            if line.trim() != "BEGIN" {
                panic!("Unfortunately, this does not seem to be a schema file.");
                // exit(1);
            }
            scans += 1;
        }

        if line.trim() == "BEGIN" {
            is_schema = true;
        } else {
            if is_schema == true {
                if is_required == false {
                    // we haven't found the required schema yet
                    if line.trim() == rel_name {
                        // we have the required schema
                        is_required = true;
                    } else {
                        is_schema = false;
                    }
                } else {
                    let split = line.split_whitespace();
                    let vec: Vec<&str> = split.collect();
                    if vec.len() == 1 {
                        if vec[0] == "END" {
                            is_required = false;
                            is_schema = false;
                        } else {
                             println!("File Name: {:#?}",vec[0])
                        }
                    } else if vec.len() == 2 {
                        num_atts += 1;
                        print!("Attribute Name: {:#?}",vec[0]);
                        if vec[1] == "Int" {
                            println!(" Type: INT");
                        } else if vec[1] == "Double" {
                            println!(" Type: DOUBLE");
                        } else if vec[1] == "String" {
                            println!(" Type: STRING");
                        } else {
                            panic! ("Bad Attribute type for {:#?}",  vec[0] )
                        }
                    }
                }
            }
        }
    }

    println!("Num of Atts: {}", num_atts);

}