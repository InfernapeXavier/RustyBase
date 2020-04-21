pub struct Operand {
    code: usize,
    value: String,
}

pub struct ComparisonOp {
    code: usize,
    left: Operand,
    right: Operand,
}

pub struct OrList {
    left: ComparisonOp,
    right_or: Box<OrList>,
}

pub struct AndList {
    lift: Box<OrList>,
    right_and: Box<AndList>,
}

pub const LESS_THAN: usize = 1;
pub const GREATER_THAN: usize = 2;
pub const EQUALS: usize = 3;

pub const DOUBLE: usize = 1;
pub const INT: usize = 2;
pub const STRING: usize = 3;
pub const NAME: usize = 4;
