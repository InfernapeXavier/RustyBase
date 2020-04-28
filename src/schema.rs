use crate::defs::DataType;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub my_type: DataType,
}
impl Attribute {
    // Rust cannot initialize empty structures
    pub fn new() -> Attribute {
        Attribute {
            name: String::from("name"),
            my_type: DataType::INT,
        }
    }

    // A constructor that can take values
    pub fn init(name: String, my_type: DataType) -> Attribute {
        Attribute { name, my_type }
    }
}

pub struct Schema {
    // Attributes of schema
    num_atts: i64,
    my_atts: Vec<Attribute>,

    // File storing the relation
    file_name: String,
}

// schema functions
impl Schema {
    // Rust cannot initialize empty structures
    pub fn new() -> Schema {
        Schema {
            num_atts: 0,
            my_atts: Vec::new(),
            file_name: String::from(""),
        }
    }

    // Returns position of required attribute
    pub fn find(&self, att_name: &String) -> i64 {
        for x in 0..self.num_atts {
            let y = x as usize; // can't index using integer
            if *att_name == self.my_atts[y].name {
                return x;
            }
        }
        -1
    }

    // Returns type of given attribute
    pub fn find_type(&self, att_name: &String) -> DataType {
        for x in 0..self.num_atts {
            let y = x as usize; // can't index using integer
            if *att_name == self.my_atts[y].name {
                return self.my_atts[y].my_type;
            }
        }
        DataType::INT
    }

    // Returns number of attributes
    pub fn get_num_atts(&self) -> i64 {
        self.num_atts
    }

    // Returns all attributes
    pub fn get_atts(&self) -> &Vec<Attribute> {
        &self.my_atts
    }

    // Builds the actual schema structure from a schema file
    pub fn build(mut self, file_name: &std::path::Path, rel_name: &str) -> Schema {
        let file_ref = File::open(file_name).expect("Unable to open Schema File"); // open file in read mode
        let reader = BufReader::new(file_ref);
        let mut scans: usize = 1;
        let mut is_schema: bool = false;
        let mut is_required: bool = false;
        self.num_atts = 0;

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            // if the file is invalid, cause Rust to panic
            if scans == 1 {
                if line.trim() != "BEGIN" {
                    panic!("Unfortunately, this does not seem to be a schema file.");
                    // exit(1);
                }
                scans += 1;
            }

            if line.trim() == "BEGIN" {
                is_schema = true;
            } else if is_schema {
                // we found the start of a valid schema
                if !is_required {
                    // we haven't found the required schema yet
                    if line.trim() == rel_name {
                        // we have the required schema
                        is_required = true;
                    } else {
                        is_schema = false;
                    }
                } else {
                    let vec: Vec<&str> = line.split_whitespace().collect(); //splits the line at whitespace and collects the parts in a vector
                    if vec.len() == 1 {
                        if vec[0] == "END" {
                            // if at the end, set flags accordingly to stop
                            // TODO: Break if required schema is found
                            is_required = false;
                            is_schema = false;
                        } else {
                            // if the word isn't end then it's the name of record file
                            self.file_name = (vec[0]).to_string();
                        }
                    } else if vec.len() == 2 {
                        self.num_atts += 1; //increase count

                        // create a local instance of attribute
                        let mut my_attribute = Attribute::new();
                        my_attribute.name = (vec[0]).to_string();
                        if vec[1] == "Int" {
                            my_attribute.my_type = DataType::INT;
                        } else if vec[1] == "Double" {
                            my_attribute.my_type = DataType::DOUBLE;
                        } else if vec[1] == "String" {
                            my_attribute.my_type = DataType::STRING;
                        } else {
                            panic!("Bad Attribute type for {:#?}", my_attribute.my_type)
                        }

                        // push the local attribute to my_atts which is a vector of attributes
                        self.my_atts.push(my_attribute)
                    }
                }
            }
        }
        self
    }

    // Build schema from a list of attributes
    pub fn build_from_atts(mut self, file_name: &str, num_atts: i64, atts: Vec<Attribute>) {
        self.file_name = file_name.to_string();
        self.num_atts = num_atts;
        self.my_atts = Vec::new();

        for x in atts {
            let temp_att = Attribute::init(x.name, x.my_type);
            self.my_atts.push(temp_att);
        }
    }
}
