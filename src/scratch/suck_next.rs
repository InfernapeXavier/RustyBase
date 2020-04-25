#![allow(unused_variables)]
#[derive(Debug)]

pub enum DataType {
    INT,
    DOUBLE,
    STRING,
}

struct Record {
    bits: Vec<char>,
}

use std::fs::File;
use std::io::{BufRead, BufReader};

pub const PAGE_SIZE: usize = 131_072;

fn main() {
    let blank_schema = Schema::new();
    let my_schema = blank_schema.build("catalog", "nation");

    suck_next_record(&my_schema, &my_schema.file_name);
}
#[derive(Debug)]

pub struct Schema {
    // Attributes of schema
    num_atts: i64,
    my_atts: Vec<Attribute>,

    // physical location of binary file storing the relation
    file_name: String,
    // friend class Record;
}
#[derive(Debug)]

pub struct Attribute {
    name: String,
    my_type: DataType,
}

impl Attribute {
    // Rust cannot initialize empty structures
    pub fn new() -> Attribute {
        Attribute {
            name: String::from("name"),
            my_type: DataType::INT,
        }
    }
}

fn suck_next_record(my_schema: &Schema, file_name: &str) {
    let space: Vec<char> = Vec::with_capacity(PAGE_SIZE);
    let rec_space: Vec<char> = Vec::with_capacity(PAGE_SIZE);

    // clearing out the current record
    // self.bits = Vec::new();

    let n = my_schema.get_num_atts();
    let atts = my_schema.get_atts();

    let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut vec: Vec<&str> = line.split("|").collect();
        vec.pop();
        println!("{:#?}", vec)
    }
}

impl Schema {
    pub fn get_num_atts(&self) -> i64 {
        self.num_atts
    }

    pub fn get_atts(&self) -> &Vec<Attribute> {
        &self.my_atts
    }

    pub fn new() -> Schema {
        // Rust cannot initialize empty structures
        Schema {
            num_atts: 0,
            my_atts: Vec::new(),
            file_name: String::from(""),
        }
    }

    pub fn build(mut self, file_name: &str, rel_name: &str) -> Schema {
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
                if !is_required {
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
                            self.file_name = (vec[0]).to_string();
                        }
                    } else if vec.len() == 2 {
                        let index = self.num_atts as usize;
                        self.num_atts += 1;

                        let mut my_attribute = Attribute::new();
                        my_attribute.name = (vec[0]).to_string();

                        // self.my_atts[index].name = (vec[0]).to_string();

                        if vec[1] == "Int" {
                            my_attribute.my_type = DataType::INT;
                        } else if vec[1] == "Double" {
                            my_attribute.my_type = DataType::DOUBLE;
                        } else if vec[1] == "String" {
                            my_attribute.my_type = DataType::STRING;
                        } else {
                            panic!("Bad Attribute type for {:#?}", self.my_atts[index].name)
                        }

                        self.my_atts.push(my_attribute)
                    }
                }
            }
        }

        self
    }
}
