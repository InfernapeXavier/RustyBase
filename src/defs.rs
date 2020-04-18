use std::collections::LinkedList;

// derive debug lets us use the inbuilt formatter for printing
// derive PartialEq implements a default equality operator
// these can't be defined as outer because of the functions in this file
#[derive(Debug, PartialEq)]
pub enum DataType {
    INT,
    DOUBLE,
    STRING,
}

#[derive(Debug, PartialEq)]
pub enum Target {
    Left,
    Right,
    Literal,
}

#[derive(Debug, PartialEq)]
pub enum CompOperator {
    LessThan,
    GreaterThan,
    Equals,
}

// Defining all the Constants/Globals we'll be requiring
pub const MAX_ANDS: usize = 20;
pub const MAX_ORS: usize = 20;
pub const PAGE_SIZE: usize = 131_072;

// Defining custom methods for manipulation of Linked Lists
// inserts data at the 'at'th position
pub fn insert(at: usize, data: DataType, two_way_list: &mut LinkedList<DataType>) {
    let mut rest = two_way_list.split_off(at - 1);
    two_way_list.push_back(data);
    two_way_list.append(&mut rest);
}
// removes data at the 'at'th position
pub fn remove(at: usize, two_way_list: &mut LinkedList<DataType>) {
    let mut rest = two_way_list.split_off(at);
    two_way_list.pop_back();
    two_way_list.append(&mut rest);
}
// returns the number of elements to the right of current position
pub fn right_length(at: usize, two_way_list: LinkedList<DataType>) -> usize {
    two_way_list.len() - at
}
