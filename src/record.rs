// STD Imports
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Custom Import
use crate::defs::DataType;
use crate::schema::Schema;
#[derive(Debug)]
pub struct Record {
    pub bits: Vec<String>,
    // Stores the value of the reader to make sure we always get the next record in order
    // reader: std::io::Lines<std::io::BufReader<std::fs::File>>,
    skipper: usize,
}

impl Record {
    // Rust cannot initialize empty structures
    pub fn new() -> Record {
        Record {
            bits: Vec::new(),
            skipper: 0,
        }
    }
    // returns the bit contents of the vector
    pub fn get_bits(&self) -> &Vec<String> {
        &self.bits
    }
    // sets the value
    fn set_bits(&mut self, bits: Vec<String>) {
        self.bits = bits;
    }
    // copies the bits from another record
    fn copy_bits(&mut self, bits: Vec<String>) {
        let temp: Vec<&str> = Vec::new();
        self.bits = bits;
    }
    // consumes bits from another record
    fn consume(mut self, from_me: Record) {
        // move occurs here and the ownership changes
        // so from_me will be purged
        self.bits = from_me.bits
    }

    pub fn print(&self, my_schema: &Schema) {
        let n: usize = my_schema.get_num_atts().try_into().unwrap();
        let atts = my_schema.get_atts();
        for x in 0..n {
            print!("{}: [", atts[x].name);

            if atts[x].my_type == DataType::INT {
                let value = self.bits[x].parse::<i32>().unwrap();
                print!("{}], ", value);
            } else if atts[x].my_type == DataType::DOUBLE {
                let value = self.bits[x].parse::<f32>().unwrap();
                print!("{}], ", value);
            } else if atts[x].my_type == DataType::STRING {
                let value = &self.bits[x];
                print!("{}], ", value);
            }
        }
        // println!("{:#?}", self.bits);
        println!("");
    }
    // reads the next record and returns false when there's no more data
    pub fn suck_next_record(&mut self, my_schema: &Schema, file_name: &std::path::Path) -> bool {
        // reads the next record
        let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
        let reader = BufReader::new(file);
        let line = reader.lines().nth(self.skipper);

        match line {
            None => false,
            Some(x) => match x {
                Ok(o) => {
                    let mut vec: Vec<&str> = o.split("|").collect();
                    vec.pop();
                    self.bits = Vec::new();
                    for x in &vec {
                        self.bits.push(x.to_string());
                    }
                    self.skipper = self.skipper + 1;
                    // println!("{:#?}", &vec);
                    true
                }
                Err(e) => {
                    panic!("Error {:#?}", e);
                }
            },
        }
    }
}
