extern crate elf;

use std::env;
use std::path::PathBuf;

fn main() {
    
    println!("Hello, world!");
    let path = PathBuf::from("target/debug/rust-dietician");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };
    
    let text_scn = match file.get_section(".text") {
        Some(s) => println!("{}", s),
        None => panic!("Failed to look up .text section"),
    };
}
