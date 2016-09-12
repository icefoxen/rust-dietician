extern crate elf;

use std::env;
use std::path::PathBuf;

fn analyze_file(file: elf::File) {
    for section in &file.sections {
        println!("{}\n", section);

        let symbols = file.get_symbols(section).unwrap();
        for symbol in symbols {
            println!("{}", symbol);
        }
        println!("");
    }
    /*
    match file.get_section(".text") {
        Some(s) => {
            let symbols = file.get_symbols(s).unwrap();
            println!("Symbols: {}", symbols.len());

        }
        None => panic!("Failed to look up .text section"),
    };

    match file.get_section(".dynsym") {
        Some(s) => {
            println!("Section .text: {}\n", s);
            let symbols = file.get_symbols(s).unwrap();
            println!("Symbols: {}", symbols.len());
            for symbol in symbols {
                println!("Symbol: {}", symbol);
            }
        }
        None => panic!("Failed to look up .text section"),
    };
*/

}

fn main() {
    
    println!("Hello, world!");
    let path = PathBuf::from("target/debug/rust-dietician");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };
    
    analyze_file(file)
}
