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
