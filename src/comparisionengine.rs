use crate::comparison::Comparison;
use crate::defs::Target;
use crate::record::Record;

struct ComparisonEngine {}

impl ComparisonEngine {
    pub fn run(left: &Record, literal: &Record, c: &Comparison) {
        let left_bits = left.get_bits();
        let lit_bits = literal.get_bits();

        // let val_one = match c.operand_one {
        //     a if a == Target::Left => left_bits + (c.which_att_one +1),
        //     - =>
        // };
    }
}
