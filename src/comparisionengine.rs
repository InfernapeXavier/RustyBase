// STD Imports
use std::convert::TryInto;

// Custom Imports
use crate::comparison::{Comparison, OrderMaker, CNF};
use crate::defs::{CompOperator, DataType, Target};
use crate::record::Record;

// This version of Compare is for sorting. The OrderMaker struct encapsulates the specification for a sort order.
// For example, say you are joining two tables on R.att1 = S.att2, and so you want to sort R using att1.
// The OrderMaker struct specifies this sort ordering on att1.
// Compare returns a negative number, 0, or a positive number if left is less than, equal to, or greater than right.
// This particular version of Compare is used when both of the records come from the SAME RELATION

pub fn compare_sort_same(left: &Record, right: &Record, order_us: &OrderMaker) -> i64 {
    let left_bits = left.get_bits();
    let right_bits = right.get_bits();
    let mut val_one;
    let mut val_two;

    for x in 0..order_us.num_atts {
        let index: usize = (order_us.which_atts[x]).try_into().unwrap();
        val_one = &left_bits[index];
        val_two = &right_bits[index];

        // Check the type and do the comparison
        match order_us.which_types[x] {
            DataType::INT => {
                let int_one = val_one.parse::<i64>().unwrap();
                let int_two = val_two.parse::<i64>().unwrap();
                if int_one < int_two {
                    return -1;
                } else if int_one > int_two {
                    return 1;
                }
            }
            DataType::DOUBLE => {
                let double_one = val_one.parse::<f64>().unwrap();
                let double_two = val_two.parse::<f64>().unwrap();
                if double_one < double_two {
                    return -1;
                } else if double_one > double_two {
                    return 1;
                }
            }
            DataType::STRING => {
                if val_one < val_two {
                    return -1;
                } else if val_one > val_two {
                    return 1;
                }
            }
        }
    }
    0
}

// Similar to the last function, except that this one works in the
// case where the two records come from different input relations.
// It is used to do sorts for a sort-merge join

pub fn compare_sort_different(
    left: &Record,
    order_left: &OrderMaker,
    right: &Record,
    order_right: &OrderMaker,
) -> i64 {
    let left_bits = left.get_bits();
    let right_bits = right.get_bits();
    let mut val_one;
    let mut val_two;

    for x in 0..order_left.num_atts {
        let index_left: usize = (order_left.which_atts[x]).try_into().unwrap();
        let index_right: usize = (order_right.which_atts[x]).try_into().unwrap();

        val_one = &left_bits[index_left];
        val_two = &right_bits[index_right];

        // Check the type and do the comparison
        match order_left.which_types[x] {
            DataType::INT => {
                let int_one = val_one.parse::<i64>().unwrap();
                let int_two = val_two.parse::<i64>().unwrap();
                if int_one < int_two {
                    return -1;
                } else if int_one > int_two {
                    return 1;
                }
            }
            DataType::DOUBLE => {
                let double_one = val_one.parse::<f64>().unwrap();
                let double_two = val_two.parse::<f64>().unwrap();
                if double_one < double_two {
                    return -1;
                } else if double_one > double_two {
                    return 1;
                }
            }
            DataType::STRING => {
                if val_one < val_two {
                    return -1;
                } else if val_one > val_two {
                    return 1;
                }
            }
        }
    }
    0
}

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
