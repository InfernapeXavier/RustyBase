#![allow(dead_code)]
use crate::defs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Attribute {
    name: String,
    my_type: defs::DataType,
}
impl Attribute {
    pub fn new() -> Attribute {
        Attribute {
            name: String::from("name"),
            my_type: defs::DataType::INT,
        }
    }
}

pub struct Schema {
    // Attributes of schema
    num_atts: i64,
    my_atts: Vec<Attribute>,

    // physical location of binary file storing the relation
    file_name: String,
    // friend class Record;
}

// schema functions
impl Schema {
    pub fn new() -> Schema {
        Schema {
            num_atts: 0,
            my_atts: Vec::new(),
            file_name: String::from(""),
        }
    }

    fn find(&self, att_name: String) -> i64 {
        for x in 0..self.num_atts {
            let y = x as usize; // can't index using integer
            if att_name == self.my_atts[y].name {
                return x;
            }
        }
        -1
    }

    fn find_type(&self, att_name: String) -> &defs::DataType {
        for x in 0..self.num_atts {
            let y = x as usize; // can't index using integer
            if att_name == self.my_atts[y].name {
                return &self.my_atts[y].my_type;
            }
        }
        &defs::DataType::INT
    }

    pub fn get_num_atts(&self) -> i64 {
        self.num_atts
    }

    pub fn get_atts(&self) -> &Vec<Attribute> {
        &self.my_atts
    }

    pub fn build(&mut self, file_name: &str, rel_name: &str) -> &Schema {
        let file_ref = File::open(file_name).expect("Unable to open file"); // open file in read mode
        let reader = BufReader::new(file_ref);
        let mut scans: usize = 1;
        let mut is_schema: bool = false;
        let mut is_required: bool = false;
        self.num_atts = 0;

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
            } else if is_schema {
                // we found the start of a valid schema
                if !is_required {
                    // we haven't found the required schema yet
                    if line.trim() == rel_name {
                        // we have the required schema
                        is_required = true;
                    } else {
                        is_schema = false;
                    }
                } else {
                    let vec: Vec<&str> = line.split_whitespace().collect(); //splits the line at whitespace and collects the parts in a vector
                    if vec.len() == 1 {
                        if vec[0] == "END" {
                            // if at the end, set flags accordingly to stop
                            // TODO: Break if required schema is found
                            is_required = false;
                            is_schema = false;
                        } else {
                            // if the word isn't end then it's the name of record file
                            self.file_name = (vec[0]).to_string();
                        }
                    } else if vec.len() == 2 {
                        self.num_atts += 1; //increase count

                        // create a local instance of attribute
                        let mut my_attribute = Attribute::new();
                        my_attribute.name = (vec[0]).to_string();
                        if vec[1] == "Int" {
                            my_attribute.my_type = defs::DataType::INT;
                        } else if vec[1] == "Double" {
                            my_attribute.my_type = defs::DataType::DOUBLE;
                        } else if vec[1] == "String" {
                            my_attribute.my_type = defs::DataType::STRING;
                        } else {
                            panic!("Bad Attribute type for {:#?}", my_attribute.my_type)
                        }
                        
                        // push the local attribute to my_atts which is a vector of attributes
                        self.my_atts.push(my_attribute)

                    }
                }
            }
        }

        self
    }
}
