use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    let n = 5;
    file.write_all(format!("{} {}\n", "Hello, world!", n).as_bytes())?;
    file.write_all(b"Hello\nworld!")?;
    file = File::open("foo.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
