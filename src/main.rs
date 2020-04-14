#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod defs;
mod record;
mod schema;

use std::path::Path;
use std::env;

fn main() {
    println!("\n\nmain.rs Executing.........\n\n\n");

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
