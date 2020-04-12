#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::defs;
use crate::schema;

struct Record {
    bits: Vec<char>,
}

impl Record {
    // returns the bit contents of the vector
    fn get_bits(&self) -> &Vec<char> {
        &self.bits
    }

    fn set_bits(&mut self, bits: Vec<char>) {
        self.bits = bits;
    }

    fn copy_bits(&mut self, bits: Vec<char>, b_len: usize) {
        let mut temp: Vec<char> = Vec::with_capacity(b_len);
        temp.copy_from_slice(&bits);
        self.bits = temp;
    }

    fn consume(mut self, from_me: Record) {
        // move occurs here and the ownership changes
        // so from_me will be purged
        self.bits = from_me.bits
    }

    fn suck_next_record(mut self, my_schema: schema::Schema, file_name: &str, offset: usize) {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let space: Vec<char> = Vec::with_capacity(defs::PAGE_SIZE);
        let rec_space: Vec<char> = Vec::with_capacity(defs::PAGE_SIZE);

        // clearing out the current record
        self.bits = Vec::new();

        let n = my_schema.get_num_atts();
        let atts = my_schema.get_atts();

        let file = File::open(file_name).expect("Unable to open file"); // open file in read mode
        let reader = BufReader::new(file);

        let line = reader.lines().skip(1).next();
        let line = match line {
        None => String::from("No Line!"),
        Some(x) =>  x.expect("Unable to read line")
    };

    let mut vec: Vec<&str> = line.split("|").collect();
    vec.pop();
    println!("{:#?}", vec);
    }
}
