// STD Imports
use std::convert::TryInto;

// Custom Imports
use crate::comparison::{Comparison, CNF};
use crate::defs::{CompOperator, DataType, Target};
use crate::record::Record;

// Applies the given CNF to a single record and accepts or rejects the record
// This version is for unary operations
pub fn compare_unary(left: &Record, literal: &Record, my_comparison: &CNF) -> bool {
    for x in 0..my_comparison.num_ands {
        for y in 0..my_comparison.or_lens[x] {
            let result = run_one_record(left, literal, &my_comparison.or_list[x][y]);
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

// Applies the given CNF to a pair of records and accepts or rejects them
// This version is for binary operations
pub fn compare_binary(
    left: &Record,
    right: &Record,
    literal: &Record,
    my_comparison: &CNF,
) -> bool {
    for x in 0..my_comparison.num_ands {
        for y in 0..my_comparison.or_lens[x] {
            let result = run_two_records(left, right, literal, &my_comparison.or_list[x][y]);
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

// Internal Functions used by Compare

fn run_one_record(left: &Record, literal: &Record, c: &Comparison) -> bool {
    let left_bits = left.get_bits();
    let lit_bits = literal.get_bits();
    let index_one: usize = (c.which_att_one).try_into().unwrap();
    let index_two: usize = (c.which_att_two).try_into().unwrap();
    let val_one;
    let val_two;

    // Get first value
    match c.operand_one {
        Target::Left => val_one = &left_bits[index_one],
        _ => val_one = &lit_bits[index_one],
    }
    // Get second value
    match c.operand_two {
        Target::Left => val_two = &left_bits[index_two],
        _ => val_two = &lit_bits[index_two],
    }
    // Checking type and the comparison operator
    match c.att_type {
        DataType::INT => {
            let int_one = val_one.parse::<i64>().unwrap();
            let int_two = val_two.parse::<i64>().unwrap();
            match c.op {
                CompOperator::LessThan => int_one < int_two,
                CompOperator::GreaterThan => int_one > int_two,
                CompOperator::Equals => int_one == int_two,
            }
        }

        DataType::DOUBLE => {
            let double_one = val_one.parse::<f64>().unwrap();
            let double_two = val_two.parse::<f64>().unwrap();
            match c.op {
                CompOperator::LessThan => double_one < double_two,
                CompOperator::GreaterThan => double_one > double_two,
                CompOperator::Equals => double_one == double_two,
            }
        }

        DataType::STRING => match c.op {
            CompOperator::LessThan => val_one < val_two,
            CompOperator::GreaterThan => val_one > val_two,
            CompOperator::Equals => val_one == val_two,
        },
    }
}

fn run_two_records(left: &Record, right: &Record, literal: &Record, c: &Comparison) -> bool {
    let left_bits = left.get_bits();
    let right_bits = right.get_bits();
    let lit_bits = literal.get_bits();
    let index_one: usize = (c.which_att_one).try_into().unwrap();
    let index_two: usize = (c.which_att_two).try_into().unwrap();
    let val_one;
    let val_two;

    // Get first value
    match c.operand_one {
        Target::Left => val_one = &left_bits[index_one],
        Target::Right => val_one = &right_bits[index_one],
        Target::Literal => val_one = &lit_bits[index_one],
    }
    // Get second value
    match c.operand_two {
        Target::Left => val_two = &left_bits[index_two],
        Target::Right => val_two = &right_bits[index_two],
        Target::Literal => val_two = &lit_bits[index_two],
    }
    // Checking type and the comparison operator
    match c.att_type {
        DataType::INT => {
            let int_one = val_one.parse::<i64>().unwrap();
            let int_two = val_two.parse::<i64>().unwrap();
            match c.op {
                CompOperator::LessThan => int_one < int_two,
                CompOperator::GreaterThan => int_one > int_two,
                CompOperator::Equals => int_one == int_two,
            }
        }

        DataType::DOUBLE => {
            let double_one = val_one.parse::<f64>().unwrap();
            let double_two = val_two.parse::<f64>().unwrap();
            match c.op {
                CompOperator::LessThan => double_one < double_two,
                CompOperator::GreaterThan => double_one > double_two,
                CompOperator::Equals => double_one == double_two,
            }
        }

        DataType::STRING => match c.op {
            CompOperator::LessThan => val_one < val_two,
            CompOperator::GreaterThan => val_one > val_two,
            CompOperator::Equals => val_one == val_two,
        },
    }
}
