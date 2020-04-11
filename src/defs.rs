use std::collections::LinkedList;

#[derive(Debug)]
pub enum DataType {
    INT,
    DOUBLE,
    STRING,
}

pub enum Target {
    Left,
    Right,
    Literal,
}

pub enum CompOperator {
    LessThan,
    GreaterThan,
    Equals,
}

pub const MAX_ANDS: usize = 20;
pub const MAX_ORS: usize = 20;
pub const PAGE_SIZE: usize = 131_072;

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
