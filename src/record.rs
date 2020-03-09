#[allow(dead_code)]
use crate::defs;
struct Record {
    bits: Vec<char>,
}

impl Record {

    fn get_bits(&self) -> &Vec<char> {
        return &self.bits
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
        self.bits = from_me.bits   
    }




}