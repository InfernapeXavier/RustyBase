// Inner Attributes to suppress Cargo Warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// STD Imports
use std::io;
use std::io::prelude::*;
use std::path::Path;

// Custom imports
mod comparison;
mod defs;
mod file;
mod record;
mod schema;

fn main() {
    print!("\n\nEnter in your CNF: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't parse your CNF");

    println!("{}", input);

    // Creating Schema
    let mut my_schema = schema::Schema::new();
    let catalog = Path::new("src/scratch/catalog");
    my_schema = my_schema.build(catalog, "nation");
    let my_comparison = comparison::CNF::new();
    my_comparison.grow_from_parse_tree();
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
