// Here, the imports are different to namespace them into this file for easier access
use crate::defs::{CompOperator, DataType, Target};
use crate::parsedefs;
use crate::parsedefs::{AndList, ComparisonOp, Operand, OrList};
use crate::record::Record;
use crate::schema::Schema;

// STD Imports for File
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// This stores the individual comparison that is part of the CNF
#[derive(Debug, Clone, Copy)]
pub struct Comparison {
    pub operand_one: Target,
    pub operand_two: Target,
    pub which_att_one: i64,
    pub which_att_two: i64,

    pub att_type: DataType,
    pub op: CompOperator,
}

impl Comparison {
    pub fn new(
        operand_one: Target,
        operand_two: Target,
        which_att_one: i64,
        which_att_two: i64,

        att_type: DataType,

        op: CompOperator,
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

        if self.operand_one == Target::Left {
            target_one = "left record";
        } else if self.operand_one == Target::Right {
            target_one = "right record";
        } else {
            target_one = "literal record";
        }

        if self.op == CompOperator::LessThan {
            operator = "<";
        } else if self.op == CompOperator::GreaterThan {
            operator = ">";
        } else {
            operator = "=";
        }

        if self.operand_two == Target::Left {
            target_two = "left record";
        } else if self.operand_two == Target::Right {
            target_two = "right record";
        } else {
            target_two = "literal record";
        }

        if self.att_type == DataType::INT {
            attribute_type = "Int";
        } else if self.att_type == DataType::DOUBLE {
            attribute_type = "Double";
        } else {
            attribute_type = "String";
        }

        print!(
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

// This encapsulates the sort order for records
#[derive(Debug, Clone)]
pub struct OrderMaker {
    pub num_atts: usize,
    pub which_atts: Vec<usize>,
    pub which_types: Vec<DataType>,
}

impl OrderMaker {
    pub fn new(num_atts: usize, which_atts: Vec<usize>, which_types: Vec<DataType>) -> OrderMaker {
        OrderMaker {
            num_atts,
            which_atts,
            which_types,
        }
    }

    pub fn print(self) {
        print!("Number of Atts = {:5}", self.num_atts);
        for x in 0..self.num_atts {
            print!("{:3}: {:5} ", x, self.which_atts[x]);
            if self.which_types[x] == DataType::INT {
                println!("INT");
            } else if self.which_types[x] == DataType::DOUBLE {
                println!("DOUBLE");
            } else {
                println!("STRING");
            }
        }
    }

    // Builds an OrderMaker that can be used to sort records based on ALL of their attributes
    pub fn build(mut self, my_schema: &Schema) {
        let n: usize = my_schema.get_num_atts().try_into().unwrap(); // can't index using non usize type, so need to convert
        let atts = my_schema.get_atts();

        // First for all INTs
        for x in 0..n {
            if atts[x].my_type == DataType::INT {
                self.which_atts.push(x);
                self.which_types.push(DataType::INT);
            }
        }
        // Then for all DOUBLEs
        for x in 0..n {
            if atts[x].my_type == DataType::DOUBLE {
                self.which_atts.push(x);
                self.which_types.push(DataType::DOUBLE);
            }
        }
        // Then for all STRINGs
        for x in 0..n {
            if atts[x].my_type == DataType::STRING {
                self.which_atts.push(x);
                self.which_types.push(DataType::STRING);
            }
        }

        self.num_atts = self.which_atts.len();
    }
}

// This structure stores a CNF expression that is to be evaluated during query execution
#[derive(Debug, Clone)]
pub struct CNF {
    pub or_list: Vec<Vec<Comparison>>,
    pub or_lens: Vec<usize>,
    pub num_ands: usize,
}

impl CNF {
    // Rust cannot initialize empty structures
    pub fn new() -> CNF {
        CNF {
            or_list: Vec::new(),
            or_lens: Vec::new(),
            num_ands: 0,
        }
    }

    pub fn print(&self) {
        for x in 0..self.num_ands {
            print!("( ");
            for y in 0..self.or_lens[x] {
                self.or_list[x][y].print();
                if y < self.or_lens[x] - 1 {
                    print!(" OR ");
                }
            }
            println!(") ");
            if x < self.num_ands - 1 {
                println!("AND");
            }
        }
    }

    // This takes a parse tree for a CNF and converts it into a 2-D matrix
    // This version is applicable to selections over a single relation
    pub fn grow_from_parse_tree_two(
        mut self,
        mut parse_tree: AndList,
        out_rec_file: &File,
        left_schema: &Schema,
        right_schema: &Schema,
        literal: &mut Record,
    ) -> CNF {
        // Building up the literal and schema in an external text file to read from later
        let mut out_schema_file =
            File::create("hkljdfgkSDFSDF").expect("Could not create schema file");
        out_schema_file
            .write_all(b"BEGIN\ntempSchema\nwherever\n")
            .expect("Can't write to schema file");

        // Tracks the size of literal record
        let mut num_fields_in_lit: i64 = 0;

        // Building the comparison structure
        let mut which_and = 0;

        loop {
            let mut comparisons: Vec<Comparison> = Vec::new();
            let mut my_or: OrList = *parse_tree.left;
            let mut which_or = 0;
            loop {
                let type_left: DataType;
                let type_right: DataType;

                // Required for building the Comparison structure locally
                let operand_one: Target;
                let operand_two: Target;
                let which_att_one: i64;
                let which_att_two: i64;
                let att_type: DataType;
                let op: CompOperator;

                // dealing with the left operand
                match my_or.left.left.code {
                    parsedefs::NAME => {
                        // Try to find the attribute in the left schema
                        if left_schema.find(&my_or.left.left.value) != 1 {
                            operand_one = Target::Left;
                            which_att_one = left_schema.find(&my_or.left.left.value);
                            type_left = left_schema.find_type(&my_or.left.left.value);
                        } else if left_schema.find(&my_or.left.left.value) != 1 {
                            //Try to find in the right schema
                            operand_one = Target::Right;
                            which_att_one = right_schema.find(&my_or.left.left.value);
                            type_left = right_schema.find_type(&my_or.left.left.value);
                        } else {
                            panic!(
                                "ERROR: Could not find attribute {} \n",
                                my_or.left.left.value
                            );
                        }
                    }
                    parsedefs::STRING => {
                        // It it's a string then add to the literal record
                        operand_one = Target::Literal;
                        which_att_one = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.left.value,
                            DataType::STRING,
                        );
                        type_left = DataType::STRING;
                    }

                    parsedefs::INT => {
                        // Same for Int
                        operand_one = Target::Literal;
                        which_att_one = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.left.value,
                            DataType::INT,
                        );
                        type_left = DataType::INT;
                    }

                    parsedefs::DOUBLE => {
                        // Same for Int
                        operand_one = Target::Literal;
                        which_att_one = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.left.value,
                            DataType::DOUBLE,
                        );
                        type_left = DataType::DOUBLE;
                    }

                    _ => panic!("Unrecognized operand type"),
                }

                // Dealing with the right operand
                match my_or.left.right.code {
                    parsedefs::NAME => {
                        // If it's an attribute name, look it up in the left schema
                        if left_schema.find(&my_or.left.right.value) != 1 {
                            operand_two = Target::Left;
                            which_att_two = left_schema.find(&my_or.left.right.value);
                            type_right = left_schema.find_type(&my_or.left.right.value);
                        } else if right_schema.find(&my_or.left.right.value) != 1 {
                            // Check if it's in the right schema
                            operand_two = Target::Right;
                            which_att_two = left_schema.find(&my_or.left.right.value);
                            type_right = left_schema.find_type(&my_or.left.right.value);
                        } else {
                            // There's an error in the query
                            panic!(
                                "ERROR: Could not find attribute {} \n",
                                my_or.left.right.value
                            );
                        }
                    }
                    parsedefs::STRING => {
                        // It it's a string then add to the literal record
                        operand_two = Target::Literal;
                        which_att_two = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.right.value,
                            DataType::STRING,
                        );
                        type_right = DataType::STRING;
                    }

                    parsedefs::INT => {
                        // Same for Int
                        operand_two = Target::Literal;
                        which_att_two = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.right.value,
                            DataType::INT,
                        );
                        type_right = DataType::INT;
                    }

                    parsedefs::DOUBLE => {
                        // Same for Int
                        operand_two = Target::Literal;
                        which_att_two = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.right.value,
                            DataType::DOUBLE,
                        );
                        type_right = DataType::DOUBLE;
                    }

                    _ => panic!("Unrecognized operand type"),
                }

                // End of Match
                // Now we check for type mismatch between left and right operand

                if type_left != type_right {
                    panic!(
                        "ERROR: Type mismatch in CNF. {} and {} don't match.\n",
                        &my_or.left.left.value, &my_or.left.right.value
                    )
                }
                // Set attribute type for the comparison
                att_type = type_right;

                // Setting up the comparison operator
                match my_or.left.code {
                    parsedefs::LESS_THAN => op = CompOperator::LessThan,
                    parsedefs::GREATER_THAN => op = CompOperator::GreaterThan,
                    parsedefs::EQUALS => op = CompOperator::Equals,
                    _ => panic!("BAD: Found a comparison operator that is not recognized"),
                }

                let temp_comparison = Comparison::new(
                    operand_one,
                    operand_two,
                    which_att_one,
                    which_att_two,
                    att_type,
                    op,
                );
                comparisons.push(temp_comparison);

                which_or += 1;
                my_or = match my_or.right_or {
                    None => {
                        self.or_lens.push(which_or);
                        break;
                    }
                    Some(x) => *x,
                }
            }
            self.or_list.push(comparisons);
            which_and += 1;
            parse_tree = match parse_tree.right_and {
                None => {
                    self.num_ands = which_and;
                    break;
                }
                Some(x) => *x,
            }
        }

        out_schema_file
            .write_all(b"END\n")
            .expect("Can't write to schema file");
        let out_schema_path = Path::new("hkljdfgkSDFSDF");
        let out_rec_path = Path::new("sdafdsfFFDSDA");
        let mut out_schema: Schema = Schema::new();
        out_schema = out_schema.build(out_schema_path, "tempSchema");
        literal.suck_next_record(&out_schema);
        fs::remove_file(out_schema_path).expect("Failed to remove temporary schema file");
        fs::remove_file(out_rec_path).expect("Failed to remove temporary record file");
        self
    }

    // This takes a parse tree for a CNF and converts it into a 2-D matrix
    // This version is applicable to selections over a single relation
    pub fn grow_from_parse_tree_single(
        mut self,
        mut parse_tree: AndList,
        out_rec_file: &File,
        my_schema: &Schema,
        literal: &mut Record,
    ) -> CNF {
        // Building up the literal and schema in an external text file to read from later
        let mut out_schema_file =
            File::create("hkljdfgkSDFSDF").expect("Could not create schema file");
        out_schema_file
            .write_all(b"BEGIN\ntempSchema\nwherever\n")
            .expect("Can't write to schema file");

        // Tracks the size of literal record
        let mut num_fields_in_lit: i64 = 0;

        // Building the comparison structure
        let mut which_and = 0;
        loop {
            let mut comparisons: Vec<Comparison> = Vec::new();
            let mut my_or: OrList = *parse_tree.left;
            let mut which_or = 0;
            loop {
                let type_left: DataType;
                let type_right: DataType;

                // Required for building the Comparison structure locally
                let operand_one: Target;
                let operand_two: Target;
                let which_att_one: i64;
                let which_att_two: i64;
                let att_type: DataType;
                let op: CompOperator;

                // dealing with the left operand
                match my_or.left.left.code {
                    parsedefs::NAME => {
                        // If it's an attribute name, look it up in the schema
                        if my_schema.find(&my_or.left.left.value) != 1 {
                            operand_one = Target::Left;
                            which_att_one = my_schema.find(&my_or.left.left.value);
                            type_left = my_schema.find_type(&my_or.left.left.value);
                        } else {
                            panic!(
                                "ERROR: Could not find attribute {} \n",
                                my_or.left.left.value
                            );
                        }
                    }
                    parsedefs::STRING => {
                        // It it's a string then add to the literal record
                        operand_one = Target::Literal;
                        which_att_one = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.left.value,
                            DataType::STRING,
                        );
                        type_left = DataType::STRING;
                    }

                    parsedefs::INT => {
                        // Same for Int
                        operand_one = Target::Literal;
                        which_att_one = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.left.value,
                            DataType::INT,
                        );
                        type_left = DataType::INT;
                    }

                    parsedefs::DOUBLE => {
                        // Same for Int
                        operand_one = Target::Literal;
                        which_att_one = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.left.value,
                            DataType::DOUBLE,
                        );
                        type_left = DataType::DOUBLE;
                    }

                    _ => panic!("Unrecognized operand type"),
                }

                // Dealing with the right operand
                match my_or.left.right.code {
                    parsedefs::NAME => {
                        // If it's an attribute name, look it up in the schema
                        if my_schema.find(&my_or.left.right.value) != 1 {
                            operand_two = Target::Left;
                            which_att_two = my_schema.find(&my_or.left.right.value);
                            type_right = my_schema.find_type(&my_or.left.right.value);
                        } else {
                            // There's an error in the query
                            panic!(
                                "ERROR: Could not find attribute {} \n",
                                my_or.left.right.value
                            );
                        }
                    }
                    parsedefs::STRING => {
                        // It it's a string then add to the literal record
                        operand_two = Target::Literal;
                        which_att_two = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.right.value,
                            DataType::STRING,
                        );
                        type_right = DataType::STRING;
                    }

                    parsedefs::INT => {
                        // Same for Int
                        operand_two = Target::Literal;
                        which_att_two = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.right.value,
                            DataType::INT,
                        );
                        type_right = DataType::INT;
                    }

                    parsedefs::DOUBLE => {
                        // Same for Int
                        operand_two = Target::Literal;
                        which_att_two = num_fields_in_lit;
                        num_fields_in_lit = add_lit_to_file(
                            num_fields_in_lit,
                            &out_rec_file,
                            &out_schema_file,
                            &my_or.left.right.value,
                            DataType::DOUBLE,
                        );
                        type_right = DataType::DOUBLE;
                    }

                    _ => panic!("Unrecognized operand type"),
                }

                // End of Match
                // Now we check for type mismatch between left and right operand

                if type_left != type_right {
                    panic!(
                        "ERROR: Type mismatch in CNF. {} and {} don't match.\n",
                        &my_or.left.left.value, &my_or.left.right.value
                    )
                }
                // Set attribute type for the comparison
                att_type = type_right;

                // Setting up the comparison operator
                match my_or.left.code {
                    parsedefs::LESS_THAN => op = CompOperator::LessThan,
                    parsedefs::GREATER_THAN => op = CompOperator::GreaterThan,
                    parsedefs::EQUALS => op = CompOperator::Equals,
                    _ => panic!("BAD: Found a comparison operator that is not recognized"),
                }

                let temp_comparison = Comparison::new(
                    operand_one,
                    operand_two,
                    which_att_one,
                    which_att_two,
                    att_type,
                    op,
                );
                comparisons.push(temp_comparison);

                which_or += 1;
                my_or = match my_or.right_or {
                    None => {
                        self.or_lens.push(which_or);
                        break;
                    }
                    Some(x) => *x,
                }
            }
            self.or_list.push(comparisons);
            which_and += 1;
            parse_tree = match parse_tree.right_and {
                None => {
                    self.num_ands = which_and;
                    break;
                }
                Some(x) => *x,
            }
        }

        out_schema_file
            .write_all(b"END\n")
            .expect("Can't write to schema file");
        let out_schema_path = Path::new("hkljdfgkSDFSDF");
        let out_rec_path = Path::new("sdafdsfFFDSDA");
        let mut out_schema: Schema = Schema::new();
        out_schema = out_schema.build(out_schema_path, "tempSchema");
        literal.suck_next_record(&out_schema);
        fs::remove_file(out_schema_path).expect("Failed to remove temporary schema file");
        fs::remove_file(out_rec_path).expect("Failed to remove temporary record file");
        self
    }

    pub fn get_sort_order(self, left: &mut OrderMaker, right: &mut OrderMaker) -> usize {
        // Looping through all disjunctions in the CNG to find those that are acceptable for use in a sort ordering
        for x in 0..self.num_ands {
            // If there's no disjunction of length one then it can't be used
            if self.or_lens[x] != 1 {
                continue;
            }

            // Check for equality
            if self.or_list[x][0].op != CompOperator::Equals {
                continue;
            }

            // Verify that it operates over attributes from both tables
            if !((self.or_list[x][0].operand_one == Target::Left
                && self.or_list[x][0].operand_two == Target::Right)
                || (self.or_list[x][0].operand_two == Target::Left
                    && self.or_list[x][0].operand_one == Target::Right))
            {
                continue;
            }

            // If we reach here, we have a join attribute!
            // Now we need to add the comparison information into the relevant structure
            match self.or_list[x][0].operand_one {
                Target::Left => {
                    left.which_atts
                        .push(self.or_list[x][0].which_att_one.try_into().unwrap());
                    left.which_types.push(self.or_list[x][0].att_type);
                }

                Target::Right => {
                    right
                        .which_atts
                        .push(self.or_list[x][0].which_att_one.try_into().unwrap());
                    right.which_types.push(self.or_list[x][0].att_type);
                }

                _ => panic!("Something went wrong while creating the sort order!"),
            }
            match self.or_list[x][0].operand_two {
                Target::Left => {
                    left.which_atts
                        .push(self.or_list[x][0].which_att_two.try_into().unwrap());
                    left.which_types.push(self.or_list[x][0].att_type);
                }

                Target::Right => {
                    right
                        .which_atts
                        .push(self.or_list[x][0].which_att_two.try_into().unwrap());
                    right.which_types.push(self.or_list[x][0].att_type);
                }

                _ => panic!("Something went wrong while creating the sort order!"),
            }
        }

        // Set number of attributes for both OrderMakers
        left.num_atts = left.which_atts.len();
        right.num_atts = right.which_atts.len();
        left.num_atts
    }
}

// This is a helper function used by CNF
fn add_lit_to_file(
    num_fields_in_lit: i64,
    mut out_rec_file: &File,
    mut out_schema_file: &File,
    value: &str,
    my_type: DataType,
) -> i64 {
    out_rec_file
        .write_all(format!("{}|", value).as_bytes())
        .expect("Could not write to file");
    match my_type {
        a if a == DataType::INT => out_schema_file
            .write_all(format!("att{} Int\n", num_fields_in_lit).as_bytes())
            .expect("Could not write to file"),
        a if a == DataType::DOUBLE => out_schema_file
            .write_all(format!("att{} Double\n", num_fields_in_lit).as_bytes())
            .expect("Could not write to file"),
        a if a == DataType::STRING => out_schema_file
            .write_all(format!("att{} String\n", num_fields_in_lit).as_bytes())
            .expect("Could not write to file"),
        _ => panic!("I don't know that type!\n"),
    }
    num_fields_in_lit + 1
}
