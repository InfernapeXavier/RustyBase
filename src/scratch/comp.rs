#[derive(Debug, PartialEq)]

pub enum DataType {
    INT,
    DOUBLE,
    STRING,
}

fn main() {
    let att = DataType::INT;
    let att2 = DataType::STRING;
    assert_eq!(att, att2);
}
