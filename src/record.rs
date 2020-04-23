// STD Imports
use std::fs::File;
use std::io::{BufRead, BufReader};

// Custom Import
use crate::schema::Schema;

pub struct Record {
    bits: Vec<String>,
    reader: std::io::Lines<std::io::BufReader<std::fs::File>>,
}

impl Record {
    pub fn new(file_name: &std::path::Path) -> Record {
        let file = File::open(file_name).expect("Unable to open Data File"); // open file in read mode
        let reader = BufReader::new(file);
        let line = reader.lines();
        Record {
            bits: Vec::new(),
            reader: line,
        }
    }
    // returns the bit contents of the vector
    fn get_bits(&self) -> &Vec<String> {
        &self.bits
    }

    fn set_bits(&mut self, bits: Vec<String>) {
        self.bits = bits;
    }

    fn copy_bits(&mut self, bits: Vec<String>, b_len: usize) {
        let temp: Vec<&str> = Vec::with_capacity(b_len);
        // temp.copy_from_slice(&bits);
        self.bits = bits;
    }

    fn consume(mut self, from_me: Record) {
        // move occurs here and the ownership changes
        // so from_me will be purged
        self.bits = from_me.bits
    }

    pub fn print(&self, my_schema: &Schema) {
        let n = my_schema.get_num_atts();
        println!("{:#?}", self.bits);
    }

    pub fn suck_next_record(&mut self, my_schema: &Schema) -> usize {
        // // clearing out the current record
        // self.bits = Vec::new();
        let newline = self.reader.next();
        let newline = match newline {
            None => String::from("No Line!"),
            Some(x) => x.expect("Unable to read line"),
        };

        let mut vec: Vec<&str> = newline.split('|').collect();
        vec.pop();
        if vec.is_empty() {
            0
        } else {
            self.bits = Vec::new();
            for x in vec {
                self.bits.push(x.to_string());
            }
            1
        }
    }
}
