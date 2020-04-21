use crate::defs;
#[derive(Clone, Copy)]
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
            operand_one,
            operand_two,
            which_att_one,
            which_att_two,
            att_type,
            op,
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
            num_atts,
            which_atts,
            which_types,
        }
    }
}

pub struct CNF {
    or_list: Vec<Vec<Comparison>>,

    or_lens: Vec<usize>,
    num_ands: usize,
}

impl CNF {
    pub fn new() -> CNF {
        CNF {
            or_list: Vec::new(),
            or_lens: Vec::new(),
            num_ands: 0,
        }
    }

    fn print(&self) {
        for x in 0..self.num_ands {
            print!("( ");
            for y in 0..self.or_lens[x] {
                self.or_list[x][y].print();
                if y < self.or_lens[x] - 1 {
                    print!(" OR ");
                }
            }
            print!(") ");
            if x < self.num_ands - 1 {
                println!(" AND");
            }
        }
    }

    pub fn grow_from_parse_tree(self) {
        println!("Comparison Tree");
    }
}

// fn add_lit_to_file(num_fields_in_lit: usize, file_name: String, value: char, my_type: defs:DataType) {

// }
