fn main() {
    let s = String::from("Hello");
    print!("{:#?}", s[2..s.len()].to_string());
}
