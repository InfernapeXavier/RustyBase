// use crate::defs::{DataType, CompOperator}

#[derive(Debug, Clone)]
pub struct Operand {
    pub code: usize,
    pub value: String,
}

impl Operand {
    pub fn new(code: usize, value: String) -> Operand {
        Operand { code, value }
    }
}

#[derive(Debug, Clone)]
pub struct ComparisonOp {
    pub code: usize,
    pub left: Operand,
    pub right: Operand,
}

impl ComparisonOp {
    pub fn new(code: usize, left: Operand, right: Operand) -> ComparisonOp {
        ComparisonOp { code, left, right }
    }
}

#[derive(Debug, Clone)]
pub struct OrList {
    pub left: ComparisonOp,
    pub right_or: Option<Box<OrList>>,
}

impl OrList {
    pub fn new(left: ComparisonOp, right_or: Option<Box<OrList>>) -> OrList {
        OrList { left, right_or }
    }
}

#[derive(Debug, Clone)]
pub struct AndList {
    pub left: Box<OrList>,
    pub right_and: Option<Box<AndList>>,
}

impl AndList {
    pub fn new(left: Box<OrList>, right_and: Option<Box<AndList>>) -> AndList {
        AndList { left, right_and }
    }
}

pub const LESS_THAN: usize = 1;
pub const GREATER_THAN: usize = 2;
pub const EQUALS: usize = 3;

pub const DOUBLE: usize = 1;
pub const INT: usize = 2;
pub const STRING: usize = 3;
pub const NAME: usize = 4;

fn main() {
    println!("Hello")
}
