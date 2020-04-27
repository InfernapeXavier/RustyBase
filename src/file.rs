// STD Imports
use std::collections::LinkedList;

// Custom imports
use crate::defs;
use crate::record;
use crate::schema;

#[derive(Debug)]
pub struct Page {
    pub my_recs: Vec<record::Record>,
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

    pub fn get_first(&mut self) -> Option<record::Record> {
        // Check if vec is empty
        if self.my_recs.is_empty() {
            None
        } else {
            let first_one: record::Record;
            first_one = self.my_recs.remove(0);
            Some(first_one)
        }
    }

    pub fn append(&mut self, add_me: record::Record) -> bool {
        if self.cur_size_in_bytes > defs::PAGE_SIZE {
            false
        } else {
            self.my_recs.push(add_me);
            self.num_recs += 1;
            true
        }
    }

    pub fn to_binary(&self, bits: &mut Vec<Vec<String>>) {
        &bits.push(vec![self.num_recs.to_string()]);
        for x in &self.my_recs {
            bits.push(x.get_bits().to_vec());
        }
    }
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
