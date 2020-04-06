#![allow(unused_imports)]
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

    fn suck_next_record(self) {

    }




}