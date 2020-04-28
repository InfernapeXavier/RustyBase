use std::mem;

fn main() {
    let test: u8 = 0;
    println!("{:#?}", mem::size_of_val(&test));
}
