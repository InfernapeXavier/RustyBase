// Inner Attributes to suppress Cargo Warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Importing the parser
#[macro_use]
extern crate lalrpop_util;

// STD Imports
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

// Declaring Modules
mod comparisionengine;
mod comparison;
mod defs;
mod file;
mod parsedefs;
mod record;
mod schema;

// Defining LALRPOP Parser to parse inputs
lalrpop_mod!(pub parser);

fn main() {
    println!("\n\nExecuted Main.........");
}

#[test]
fn main_test() {
    // EG Input: (l_orderkey > 27) AND (l_orderkey < 45)
    // EG Input: (l_orderkey = 33)

    // Getting Input
    print!("\n\nEnter in your CNF: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read your CNF");
    // Parsing the CNF
    let expression = parser::ParseTreeParser::new().parse(&input).unwrap();

    // Building the schema
    let catalog = Path::new("src/tpch/catalog");
    let mut lineitem = schema::Schema::new();
    lineitem = lineitem.build(catalog, "lineitem");

    // Need to create the file so that new doesn't fail for literal
    let out_rec_file = File::create("sdafdsfFFDSDA").expect("Could not create record file");
    let out_rec_path = Path::new("sdafdsfFFDSDA");

    // Building the literal record
    let mut literal = record::Record::new(out_rec_path);

    // Building the CNF
    let mut my_comparison = comparison::CNF::new();
    my_comparison =
        my_comparison.grow_from_parse_tree(expression, &out_rec_file, &lineitem, &mut literal);

    // Building the temp record
    let table_file = Path::new("src/tpch/lineitem.tbl");
    let mut temp = record::Record::new(table_file);

    // Building the schema
    let mut my_schema = schema::Schema::new();
    my_schema = my_schema.build(catalog, "lineitem");

    let mut counter = 0;
    while temp.suck_next_record(&my_schema) {
        counter = counter + 1;

        if counter % 10000 == 0 {
            println!("{}", counter);
        }

        if comparisionengine::compare(&temp, &literal, &my_comparison) {
            temp.print(&my_schema);
        }
    }
}

// #[test]
// fn page_test() {
//     // EG Input: (l_orderkey > 27) AND (l_orderkey < 45)

//     // Getting Input
//     print!("\n\nEnter in your CNF: ");
//     io::stdout().flush().unwrap();
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Can't read your CNF");
//     // Parsing the CNF
//     let expression = parser::ParseTreeParser::new().parse(&input).unwrap();

//     // Building the schema
//     let catalog = Path::new("src/tpch/catalog");

//     // Need to create the file so that new doesn't fail for literal
//     let out_rec_file = File::create("sdafdsfFFDSDA").expect("Could not create record file");
//     let out_rec_path = Path::new("sdafdsfFFDSDA");

//     // Building the temp record
//     let table_file = Path::new("src/tpch/lineitem.tbl");
//     let mut temp = record::Record::new(table_file);

//     // Building the schema
//     let mut my_schema = schema::Schema::new();
//     my_schema = my_schema.build(catalog, "lineitem");

//     // Building Page
//     let mut my_page = file::Page::new();
//     temp.suck_next_record(&my_schema);
//     my_page.append(temp);
//     println!("{:#?}", my_page);
// }
