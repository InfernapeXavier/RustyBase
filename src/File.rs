// STD Imports
use std::collections::LinkedList;

// Custom imports
use crate::defs;
use crate::record;
use crate::schema;

pub struct Page {
    my_recs: LinkedList<record::Record>,
    num_recs: usize,
    cur_size_in_bytes: usize,
}

pub struct File {
    my_file_des: usize,
    curr_length: usize,
}
