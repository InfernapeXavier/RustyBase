mod defs;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Attribute {
    name: String,
    my_type: defs::DataType,
}

struct Schema {
    // Attributes of schema
    num_atts: i64,
    my_atts: Vec<Attribute>,

    // physical location of binary file storing the relation
    file_name: String,
    // friend class Record;
}

// schema functions
impl Schema {
    fn find(&self, att_name: String) -> i64 {
        for x in 0..self.num_atts {
            let y = x as usize; // can't index using integer
            if att_name == self.my_atts[y].name {
                return x;
            }
        }
        return -1;
    }

    fn find_type(&self, att_name: String) -> &defs::DataType {
        for x in 0..self.num_atts {
            let y = x as usize; // can't index using integer
            if att_name == self.my_atts[y].name {
                return &self.my_atts[y].my_type;
            }
        }
        return &defs::DataType::INT;
    }

    fn get_num_atts(&self) -> i64 {
        self.num_atts
    }

    fn get_atts(&self) -> &Vec<Attribute> {
        &self.my_atts
    }

    #[allow(unused_variables)]
    fn schema(&mut self, file_name: String, rel_name: String) -> std::io::Result<()> {
        let foo = File::open(file_name)?; // open file in read mode
        let reader = BufReader::new(foo);
        let mut scans: usize = 1;
        let mut is_schema: bool = false;
        let mut is_required: bool = false;
        let mut rel_file: String;
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
            } else {
                if is_schema == true {
                    if is_required == false {
                        // we haven't found the required schema yet
                        if line.trim() == rel_name {
                            // we have the required schema
                            is_required = true;
                        } else {
                            is_schema = false;
                        }
                    } else {
                        let split = line.split_whitespace();
                        let vec: Vec<&str> = split.collect();
                        if vec.len() == 1 {
                            if vec[0] == "END" {
                                is_required = false;
                                is_schema = false;
                            } else {
                                self.file_name = (&vec[0]).to_string();
                            }
                        } else if vec.len() == 2 {
                            self.num_atts += 1;
                            let index = self.num_atts as usize;
                            self.my_atts[index].name = (&vec[0]).to_string();
                            if vec[1] == "Int" {
                                self.my_atts[index].my_type = defs::DataType::INT;
                            } else if vec[1] == "Double" {
                                self.my_atts[index].my_type = defs::DataType::DOUBLE;
                            } else if vec[1] == "String" {
                                self.my_atts[index].my_type = defs::DataType::STRING;
                            } else {
                                panic! ("Bad Attribute type for {:#?}", self.my_atts[index].name )
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
