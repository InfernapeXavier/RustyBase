use std::mem;

fn main() {
    // let a = ['a', 'b', 'c', 'd'];

    // let mut iter = a.iter().enumerate();
    // match iter.next() {
    //     Some(x) => println!("{:#?}, {}", x.0, x.1),
    //     None => print!("Error"),
    // }

    // print!("{:#?}", mem::size_of_val(&a));

    let a: String = String::from("223");
    let b = String::from("123");
    println!("{:#?}", a > b);
}
