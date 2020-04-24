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

// Custom imports
mod comparisionengine;
mod comparison;
mod defs;
mod file;
mod parsedefs;
mod record;
mod schema;

// LALRPOP Parser to parse inputs
lalrpop_mod!(pub parser);

fn main() {
    println!("\n\nExecuting Main.........");
    // print!("\n\nEnter in your CNF: ");
    // io::stdout().flush().unwrap();
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Can't parse your CNF");
    // input = input.trim().to_string();
    // println!("\n\nYou entered: {}", input);

    // // Creating Schema
    // let mut my_schema = schema::Schema::new();
    // let catalog = Path::new("src/scratch/catalog");
    // my_schema = my_schema.build(catalog, "nation");
    // let my_comparison = comparison::CNF::new();
    // my_comparison.grow_from_parse_tree();
}

#[test]
fn test1() {
    // Schema+Record Test
    let mut my_schema = schema::Schema::new();

    let catalog = Path::new("src/scratch/catalog");
    my_schema = my_schema.build(catalog, "nation");

    let record_file = Path::new("src/scratch/nation.tbl");

    let mut my_record = record::Record::new(record_file);
    my_record.suck_next_record(&my_schema);
    my_record.print(&my_schema);
    my_record.suck_next_record(&my_schema);
    my_record.print(&my_schema);

    // // Comparison Test
    let comparison = comparison::Comparison::new(
        defs::Target::Left,
        defs::Target::Right,
        4,
        5,
        defs::DataType::INT,
        defs::CompOperator::LessThan,
    );
    comparison.print();
}

#[test]
fn test2() {
    let parse_tree = parser::ParseTreeParser::new()
        .parse("(l_orderkey > 27) AND (l_orderkey < 45)")
        .unwrap();
    println!("{:#?}\n\n", parse_tree);
    // Creating Schema
    let mut my_schema = schema::Schema::new();
    let catalog = Path::new("src/scratch/catalog");
    my_schema = my_schema.build(catalog, "lineitem");
    let out_rec_file = File::create("sdafdsfFFDSDA").expect("Could not create record file");
    let out_rec_path = Path::new("sdafdsfFFDSDA");
    let mut literal = record::Record::new(out_rec_path);
    let mut my_comparison = comparison::CNF::new();
    my_comparison =
        my_comparison.grow_from_parse_tree(parse_tree, &out_rec_file, &my_schema, &mut literal);
    my_comparison.print();
    my_comparison.display();
}

#[test]
fn lalrtest() {
    let expression = parser::ParseTreeParser::new()
        .parse("(l_orderkey > 27) AND (l_orderkey < 'don't')")
        .unwrap();
    println!("{:#?}", expression);
}

#[test]
fn record_test() {
    let mut my_schema = schema::Schema::new();

    let catalog = Path::new("src/scratch/catalog");
    my_schema = my_schema.build(catalog, "nation");
    let path = Path::new("src/scratch/nation.tbl");

    let mut my_record = record::Record::new(path);
    my_record.suck_next_record(&my_schema);
    my_record.print(&my_schema);
    my_record.suck_next_record(&my_schema);
    my_record.print(&my_schema);
}
