use crate::defs;

pub struct Comparison {
    operand_one: defs::Target,
    operand_two: defs::Target,
    which_att_one: i64,
    which_att_two: i64,

    att_type: defs::DataType,

    op: defs::CompOperator,
}

impl Comparison {
    pub fn new(
        operand_one: defs::Target,
        operand_two: defs::Target,
        which_att_one: i64,
        which_att_two: i64,

        att_type: defs::DataType,

        op: defs::CompOperator,
    ) -> Comparison {
        Comparison {
            operand_one: operand_one,
            operand_two: operand_two,
            which_att_one: which_att_one,
            which_att_two: which_att_two,
            att_type: att_type,
            op: op,
        }
    }

    pub fn print(self) {
        let target_one;
        let operator;
        let target_two;
        let attribute_type;

        if self.operand_one == defs::Target::Left {
            target_one = "left record";
        } else if self.operand_one == defs::Target::Right {
            target_one = "right record";
        } else {
            target_one = "literal record";
        }

        if self.op == defs::CompOperator::LessThan {
            operator = "<";
        } else if self.op == defs::CompOperator::GreaterThan {
            operator = ">";
        } else {
            operator = "=";
        }

        if self.operand_two == defs::Target::Left {
            target_two = "left record";
        } else if self.operand_two == defs::Target::Right {
            target_two = "right record";
        } else {
            target_two = "literal record";
        }

        if self.att_type == defs::DataType::INT {
            attribute_type = "Int";
        } else if self.att_type == defs::DataType::DOUBLE {
            attribute_type = "Double";
        } else {
            attribute_type = "String";
        }

        println!(
            "Attribute {} from {} {} Attribute {} from {} ({})",
            self.which_att_one,
            target_one,
            operator,
            self.which_att_two,
            target_two,
            attribute_type
        );
    }
}

pub struct OrderMaker {
    num_atts: usize,

    which_atts: Vec<usize>,
    which_types: defs::DataType,
}

impl OrderMaker {
    pub fn new(num_atts: usize, which_atts: Vec<usize>, which_types: defs::DataType) -> OrderMaker {
        OrderMaker {
            num_atts: num_atts,
            which_atts: which_atts,
            which_types: which_types,
        }
    }
}

pub struct CNF {
    or_list: Comparison,

    or_lens: Vec<usize>,
    num_ands: usize,
}

impl CNF {
    pub fn new(or_list: Comparison, or_lens: Vec<usize>, num_ands: usize) -> CNF {
        CNF {
            or_list: or_list,
            or_lens: or_lens,
            num_ands: num_ands,
        }
    }

    fn print() {}
    fn grow_from_parse_tree() {}
}
