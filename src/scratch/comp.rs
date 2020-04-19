#[derive(Debug, PartialEq)]

pub enum DataType {
    INT,
    DOUBLE,
    STRING,
}

fn main() {
    let att = DataType::INT;
    let att2 = DataType::INT;
    assert_eq!(att, att2);
    print!("A");
    print!("B");
    println!("C");
    print!("D");
}
