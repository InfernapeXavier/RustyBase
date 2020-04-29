// Inner Attributes to suppress Cargo Warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Importing the parser
#[macro_use]
extern crate lalrpop_util;

// STD Imports
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

// For Benchmarking
use std::thread::sleep;
use std::time::{Duration, Instant};

// Declaring Modules
mod comparisionengine;
mod comparison;
mod defs;
mod file;
mod parsedefs;
mod record;
mod schema;

// Defining LALRPOP Parsers to parse inputs
lalrpop_mod!(pub parser); // Parses a basic CNF
lalrpop_mod!(pub parserfunc); // Parses Functional Expressions
lalrpop_mod!(pub sqlparser); // Parses SQL
                             // The parsers are mostly incremental
fn main() {
    println!("\n\nExecuted Main.........");
}

// This test simulates the main function in P1
// Takes in a simple CNF and returns the records that are satisfied by it
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

    // Create a time instance
    let instant = Instant::now();

    // Parsing the CNF
    let expression = parser::ParseTreeParser::new().parse(&input).unwrap();

    // Building the schema
    let catalog = Path::new("tpch/catalog");
    let mut lineitem = schema::Schema::new();
    lineitem = lineitem.build(catalog, "lineitem");

    // Need to create the file so that new doesn't fail for literal
    let out_rec_file = fs::File::create("sdafdsfFFDSDA").expect("Could not create record file");
    let out_rec_path = Path::new("sdafdsfFFDSDA");

    // Building the literal record
    let mut literal = record::Record::new(out_rec_path);

    // Building the CNF
    let mut my_comparison = comparison::CNF::new();
    my_comparison = my_comparison.grow_from_parse_tree_single(
        expression,
        &out_rec_file,
        &lineitem,
        &mut literal,
    );

    // Building the temp record
    let table_file = Path::new("tpch/lineitem.tbl");
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

        if comparisionengine::compare_unary(&temp, &literal, &my_comparison) {
            // temp.print(&my_schema);
        }
    }

    println!("\n\nTime Taken: {:#?}", instant.elapsed());
}

// This test creates a page with all records that match the CNF
#[test]
fn page_test() {
    // EG Input: (l_orderkey > 27) AND (l_orderkey < 45)
    // EG Input: (l_orderkey = 33)

    // Getting Input
    print!("\n\nEnter in your CNF: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read your CNF");

    // Create a time instance
    let instant = Instant::now();

    // Parsing the CNF
    let expression = parser::ParseTreeParser::new().parse(&input).unwrap();

    // Building the schema
    let catalog = Path::new("tpch/catalog");
    let mut lineitem = schema::Schema::new();
    lineitem = lineitem.build(catalog, "lineitem");

    // Need to create the file so that new doesn't fail for literal
    let out_rec_file = fs::File::create("sdafdsfFFDSDA").expect("Could not create record file");
    let out_rec_path = Path::new("sdafdsfFFDSDA");

    // Building the literal record
    let mut literal = record::Record::new(out_rec_path);

    // Building the CNF
    let mut my_comparison = comparison::CNF::new();
    my_comparison = my_comparison.grow_from_parse_tree_single(
        expression,
        &out_rec_file,
        &lineitem,
        &mut literal,
    );

    // Building the temp record
    let table_file = Path::new("tpch/lineitem.tbl");
    let mut temp = record::Record::new(table_file);

    // Building the schema
    let mut my_schema = schema::Schema::new();
    my_schema = my_schema.build(catalog, "lineitem");

    let mut my_page = file::Page::new();
    let mut counter = 0;
    while temp.suck_next_record(&my_schema) {
        counter = counter + 1;

        if counter % 10000 == 0 {
            println!("{}", counter);
        }

        if comparisionengine::compare_unary(&temp, &literal, &my_comparison) {
            // temp.print(&my_schema);
            my_page.append(&temp);
        }
    }

    println!("This is what the page looks like:\n{:#?}", my_page);
    let mut bin = Vec::new();
    my_page.to_binary(&mut bin);
    println!("This is what to_binary does:\n{:#?}", bin);
    let mut test_page = file::Page::new();
    test_page.from_binary(bin);
    println!("Result of from_binary:\n{:#?}", test_page);

    // File test
    let mut test_file = file::File::new();
    let temp_page = Path::new("tempFile");
    test_file.add_page(&test_page, 7, temp_page, 0);
    let mut test_page2 = file::Page::new();
    println!("This is what a file looks like:\n{:#?}", test_file);
    test_file.get_page(&mut test_page2, 7, temp_page);
    println!("This page has been read from the file:\n{:#?}", test_page2);
    // If you comment the following line you can see the actual file being created
    fs::remove_file(temp_page).expect("Unable to remove temp file");

    println!("\n\nTime Taken: {:#?}", instant.elapsed());
}

// This test tests the methods of record
#[test]
fn record_test() {
    // EG Input: (l_orderkey > 27) AND (l_orderkey < 45)
    // EG Input: (l_orderkey = 33)

    // Getting Input
    print!("\n\nEnter in your CNF: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read your CNF");

    // Create a time instance
    let instant = Instant::now();

    // Parsing the CNF
    let expression = parser::ParseTreeParser::new().parse(&input).unwrap();

    // Building the schema
    let catalog = Path::new("tpch/catalog");
    let mut lineitem = schema::Schema::new();
    lineitem = lineitem.build(catalog, "lineitem");

    // Need to create the file so that new doesn't fail for literal
    let out_rec_file = fs::File::create("sdafdsfFFDSDA").expect("Could not create record file");
    let out_rec_path = Path::new("sdafdsfFFDSDA");

    // Building the literal record
    let mut literal = record::Record::new(out_rec_path);

    // Building the CNF
    let mut my_comparison = comparison::CNF::new();
    my_comparison = my_comparison.grow_from_parse_tree_single(
        expression,
        &out_rec_file,
        &lineitem,
        &mut literal,
    );

    // Building the temp record
    let table_file = Path::new("tpch/lineitem.tbl");
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

        if comparisionengine::compare_unary(&temp, &literal, &my_comparison) {
            // temp.print(&my_schema);
        }
    }

    println!("\n\nThis is what a record looks like:\n{:#?}", temp.bits);
    println!(
        "This is what a literal record looks like:\n{:#?}\n\n",
        literal.bits
    );
    let project_list = vec![
        true, true, true, true, true, true, true, true, true, false, false, false, false, false,
        false, false,
    ];
    let mut test_record = record::Record::new(table_file);
    test_record.merge_records(temp, literal, project_list);
    // Merge Uses Project so this checks both methods at once
    println!(
        "This is what a merged record looks like:\n{:#?}",
        test_record.bits
    );

    println!("\n\nTime Taken: {:#?}", instant.elapsed());
}

// This test prints out the Comparison structure that is generated from the CNF. It uses the first (basic) parser
#[test]
fn cnf_test() {
    // EG Input: (l_orderkey > 27) AND (l_orderkey < 45)
    // EG Input: (l_orderkey = 33)

    // Getting Input
    print!("\n\nEnter in your CNF: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read your CNF");

    // Create a time instance
    let instant = Instant::now();

    // Parsing the CNF
    let expression = parser::ParseTreeParser::new().parse(&input).unwrap();
    println!("The parsed expression is: {:#?}", expression);

    // Building the schema
    let catalog = Path::new("tpch/catalog");
    let mut lineitem = schema::Schema::new();
    lineitem = lineitem.build(catalog, "lineitem");

    // Need to create the file so that new doesn't fail for literal
    let out_rec_file = fs::File::create("sdafdsfFFDSDA").expect("Could not create record file");
    let out_rec_path = Path::new("sdafdsfFFDSDA");

    // Building the literal record
    let mut literal = record::Record::new(out_rec_path);

    // Building the CNF
    let mut my_comparison = comparison::CNF::new();
    my_comparison = my_comparison.grow_from_parse_tree_single(
        expression,
        &out_rec_file,
        &lineitem,
        &mut literal,
    );
    println!("The final CNF looks like this:\n{:#?}", my_comparison);

    println!("\n\nTime Taken: {:#?}", instant.elapsed());
}

// This prints out the parse tree that is generated from the extended parser that handles functional expressions
// #[test]
// fn extparser_test() {
//     // EG Input: 1+(2*3)

//     print!("\n\nEnter in your Expression: ");
//     io::stdout().flush().unwrap();
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Can't read your CNF");
//     // Parsing the CNF
//     let expression = parserfunc::CompoundExpParser::new().parse(&input).unwrap();
//     println!("This is the parsed expression:\n{:#?}", expression);
// }

#[test]
fn sql_test() {
    // EG Input: SELECT SUM DISTINCT (a.b + b), d.g FROM a AS b WHERE ('foo' > this.that OR 2 = 3) AND (12 > 5) GROUP BY a.f, c.d, g.f
    print!("\n\nEnter in your Expression: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can't read your CNF");
    // Parsing the CNF
    let expression = sqlparser::SQLParser::new().parse(&input).unwrap();
    println!("{:#?}", expression);
}
