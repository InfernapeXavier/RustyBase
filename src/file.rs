// STD Imports
use std::collections::LinkedList;
use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Custom imports
use crate::defs;
use crate::record;
use crate::schema;

#[derive(Debug)]
pub struct Page {
    pub my_recs: Vec<Vec<String>>,
    num_recs: usize,
    cur_size_in_bytes: usize,
}

impl Page {
    pub fn new() -> Page {
        Page {
            my_recs: Vec::new(),
            num_recs: 0,
            cur_size_in_bytes: 0,
        }
    }

    pub fn empty_it_out(&mut self) {
        self.my_recs.clear();
        self.num_recs = 0;
        self.cur_size_in_bytes = 0;
    }

    pub fn get_first(&mut self) -> Option<Vec<String>> {
        // Check if vec is empty
        if self.my_recs.is_empty() {
            None
        } else {
            let first_one: Vec<String>;
            first_one = self.my_recs.remove(0);
            Some(first_one)
        }
    }

    pub fn append(&mut self, add_me: &record::Record) -> bool {
        if self.cur_size_in_bytes > defs::PAGE_SIZE {
            false
        } else {
            {
                let mut temp_file = fs::File::create("lknlnhjFDFSASP")
                    .expect("Could not create temp file for page");
                for x in &add_me.bits {
                    temp_file
                        .write(format!("{}|", x).as_bytes())
                        .expect("Can't write to schema file");
                }
            }
            {
                let temp_path = Path::new("lknlnhjFDFSASP");
                let temp_file =
                    fs::File::open(temp_path).expect("Unable to open temp file for page");
                let reader = BufReader::new(temp_file);
                let line = reader.lines().next();
                let line = match line {
                    None => String::from("No Line!"),
                    Some(x) => x.expect("Unable to read line"),
                };
                let mut vec: Vec<String> = line.split('|').map(|s| s.to_string()).collect();
                vec.pop();
                self.my_recs.push(vec);
                self.num_recs += 1;
                fs::remove_file(temp_path).expect("Failed to remove temporary file for page");
                true
            }
        }
    }

    // pub fn to_binary(&self, bits: &mut Vec<Vec<String>>) {
    //     &bits.push(vec![self.num_recs.to_string()]);
    //     for x in &self.my_recs {
    //         bits.push(x.get_bits().to_vec());
    //     }
    // }
}

#[derive(Debug, Clone)]
pub struct File {
    my_file_des: usize,
    curr_length: usize,
}

impl File {
    pub fn new() -> File {
        File {
            my_file_des: 0,
            curr_length: 0,
        }
    }

    pub fn get_length(self) -> usize {
        self.curr_length
    }
}
