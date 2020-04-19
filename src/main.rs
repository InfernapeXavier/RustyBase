// Inner Attributes to suppress Cargo Warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// STD Imports
use std::path::Path;

// Custom imports
mod comparison;
mod defs;
mod file;
mod record;
mod schema;

fn main() {
    println!("\n\nmain.rs Executing.........\n\n");

    // // Schema+Record Test
    // let mut my_schema = schema::Schema::new();

    // let catalog = Path::new("src/scratch/catalog");
    // my_schema = my_schema.build(catalog, "nation");

    // let record_file = Path::new("src/scratch/nation.tbl");

    // let mut my_record = record::Record::new(record_file);
    // my_record.suck_next_record(&my_schema);
    // my_record.print(&my_schema);
    // my_record.suck_next_record(&my_schema);
    // my_record.print(&my_schema);

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
