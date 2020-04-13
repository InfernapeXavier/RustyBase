#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::defs;
use crate::schema;

struct Record {
    bits: Vec<String>,
}

impl Record {
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

    fn suck_next_record(
        mut self,
        my_schema: schema::Schema,
        file_name: &str,
        offset: usize,
    ) -> usize {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let space: Vec<&str> = Vec::with_capacity(defs::PAGE_SIZE);
        let rec_space: Vec<&str> = Vec::with_capacity(defs::PAGE_SIZE);

        // clearing out the current record
        self.bits = Vec::new();

        let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
        let reader = BufReader::new(file);

        let mut line = reader.lines();
        let newline = line.next();
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
