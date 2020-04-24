// STD Imports
use std::convert::TryInto;

use crate::comparison::{Comparison, CNF};
use crate::defs::{CompOperator, DataType, Target};
use crate::record::Record;

struct ComparisonEngine {}

impl ComparisonEngine {
    pub fn compare(left: &mut Record, literal: &mut Record, my_comparison: &CNF) -> bool {
        for x in 0..my_comparison.num_ands {
            for y in 0..my_comparison.or_lens[x] {
                let result = run(left, literal, &my_comparison.or_list[x][y]);
                if result {
                    break;
                }
                if y == (my_comparison.or_lens[x] - 1) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn run(left: &mut Record, literal: &mut Record, c: &Comparison) -> bool {
    let left_bits = left.get_bits();
    let lit_bits = literal.get_bits();
    let index_one: usize = (c.which_att_one + 1).try_into().unwrap();
    let index_two: usize = (c.which_att_two + 1).try_into().unwrap();
    let val_one;
    let val_two;

    // Get first value
    if c.operand_one == Target::Left {
        val_one = &left_bits[index_one];
    } else {
        val_one = &lit_bits[index_one];
    }
    // Get second value
    if c.operand_two == Target::Left {
        val_two = &left_bits[index_two];
    } else {
        val_two = &lit_bits[index_two];
    }

    match c.op {
        CompOperator::LessThan => val_one < val_two,
        CompOperator::GreaterThan => val_one > val_two,
        CompOperator::Equals => val_one == val_two,
    }
}
