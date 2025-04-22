use std::{fs, io};

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("File contents: {}", contents);
}
