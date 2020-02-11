// Creating the TwoWayList
#[derive(Clone)]
struct Node {
    // data: Type,
    next: Box<Node>,
    previous: Box<Node>,
    // The Box smart pointer is required for self-referential data

}
// Methods for TwoWayList
// impl Node {
//     fn new() -> Node {
//         Node {
//             // data: 0,
//             next: Box::new(0),
//             previous: Box::new(0),
//         }
//     }
// }
#[derive(Clone)]
// Header
struct Header {
    first: Box<Node>,
    last: Box<Node>,
    current: Box<Node>,
    left_size: i64,
    right_size: i64,
}
#[derive(Clone)]
struct TwoWayList {
    list: Box<Header>,
}

impl TwoWayList {

}

fn main() {
    
}