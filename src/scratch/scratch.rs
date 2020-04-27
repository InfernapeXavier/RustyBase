use std::mem;

fn main() {
    let a = ['a', 'b', 'c', 'd'];

    let mut iter = a.iter();
    iter.next();
    match iter.next() {
        Some(x) => println!("{}", x),
        None => print!("Error"),
    }
}
