extern crate elf;

use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SymbolClass {
    RustFunction,
    RustData,
    CFunction,
    CData,
    DataConst,
    DataStr,
    DataRef,
    PanicLoc,
    ExceptTable,
    VTable,
    SystemInfo,
    Other,
}

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    class: SymbolClass,
    size: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SectionClass {
    Code,
    Data,
    Debug,
    Metadata,
    Other,
}


fn symbol_class_from_name(name: &str, section: &SectionClass) -> SymbolClass {
    // We might need the section class here too, so we can separate
    // a function called "const1" with a constant called "const1"
    match name {
        x if x.starts_with("_ZN") => {
            if *section == SectionClass::Code {
                SymbolClass::RustFunction
            } else {
                SymbolClass::RustData
            }
        },
        x if x.starts_with("const") => SymbolClass::DataConst,
        x if x.starts_with("str") => SymbolClass::DataStr,
        x if x.starts_with("ref") => SymbolClass::DataRef,
        x if x.starts_with("GCC_except_table") => SymbolClass::ExceptTable,
        x if x.starts_with("panic_") => SymbolClass::PanicLoc,
        x if x.starts_with("vtable") => SymbolClass::VTable,
        x if x.starts_with("ref") => SymbolClass::Other,
        x if x.starts_with("__") => SymbolClass::SystemInfo,
        _ => {
            if *section == SectionClass::Code {
                SymbolClass::CFunction
            } else {
                SymbolClass::CData
            }
        },
    }

}




fn section_class_from_name(name: &str) -> SectionClass {
    match name {
        "" => SectionClass::Other,
        ".interp" => SectionClass::Metadata,
        x if x.starts_with(".note") => SectionClass::Other,
        x if x.starts_with(".gnu") => SectionClass::Other,
        ".dynsym" => SectionClass::Metadata,
        ".dynstr" => SectionClass::Metadata,
        x if x.starts_with(".rela") => SectionClass::Metadata,
        x if x.starts_with(".eh_frame") => SectionClass::Metadata,
        x if x.starts_with(".gcc_except") => SectionClass::Debug,
        x if x.starts_with(".init") => SectionClass::Code,
        x if x.starts_with(".plt") => SectionClass::Metadata,
        ".text" => SectionClass::Code,
        x if x.starts_with(".data") => SectionClass::Data,
        ".dynamic" => SectionClass::Metadata,
        x if x.starts_with(".got") => SectionClass::Metadata,
        ".bss" => SectionClass::Data,
        x if x.starts_with(".fini") => SectionClass::Code,
        ".rodata" => SectionClass::Data,
        ".symtab" => SectionClass::Metadata,
        ".strtab" => SectionClass::Data,
        x if x.starts_with(".debug") => SectionClass::Debug,
        _  => SectionClass::Other,
            
    }
}

// A convenient structure for doing the accounting we want to do.
// We have to rearrange things somewhat from how they are laid out
// in the ELF file, so.
#[derive(Debug, Clone)]
struct Section {
    name: String,
    class: SectionClass,
    symbols: Vec<Symbol>,
    size: u64,
}

impl Section {
    fn from_elf_file(file: &elf::File, section: &elf::types::SectionHeader) -> Section {
        Section {
            name: section.name.clone(),
            class: section_class_from_name(section.name.as_str()),
            symbols: Vec::new(),
            size: section.size,
        }
    }
}

fn find_section_class_sizes(sections: &Vec<Section>) -> HashMap<SectionClass, u64> {
    let mut map = HashMap::new();
    for s in sections {
        let entry: &mut u64 = map.entry(s.class).or_insert(0);
        *entry = *entry + s.size;
    }
    map
}

fn find_symbol_class_sizes(sections: &Vec<Section>) -> HashMap<SymbolClass, u64> {
    let mut map = HashMap::new();
    for section in sections {
        for symbol in &section.symbols {
            let entry: &mut u64 = map.entry(symbol.class).or_insert(0);
            *entry = *entry + symbol.size;
        }
    }
    map
}


/// Takes a Vec<Section> and goes through all symbols in all sections,
/// updating the symbol list for each Section.
fn resolve_symbols(file: elf::File, sections: &mut Vec<Section>) {
    for s in &file.sections {
        let symbols = file.get_symbols(s).unwrap();
        for sym in symbols {
            if sym.name == "" {
                continue;
            };
            let section_offset = sym.shndx as usize;
            if section_offset >= sections.len() {
                continue;
            }
            let ref mut our_section = sections[section_offset];
            let new_sym = Symbol {
                name: sym.name.clone(),
                class: symbol_class_from_name(sym.name.as_str(), &our_section.class),
                size: sym.size,
            };
            our_section.symbols.push(new_sym);
        }
    }
}

fn round_up_to_kb(num: u64) -> u64 {
    (num+1024) / 1024
}

fn summarize_sections(sections: &Vec<Section>) {
    println!("Symbol info:");
    let symbol_sizes = find_symbol_class_sizes(&sections);
    for (key, val) in symbol_sizes.iter() {
        println!("{:?} {} Kb", key, round_up_to_kb(*val));
    }
    println!("");

    println!("Section info:");
    let section_sizes = find_section_class_sizes(&sections);
    for (key, val) in section_sizes.iter() {
        println!("{:?} {} Kb", key, round_up_to_kb(*val));
    }
    println!("");

    let symbol_size = symbol_sizes.values()
        .fold(0, |x,y| x+y);
    println!("Size of all symbol info: {} Kb", round_up_to_kb(symbol_size));


    let total_size = sections.iter()
        .map(|section| section.size)
        .fold(0, |x,y| x+y);
    println!("Total size: {} Kb", round_up_to_kb(total_size));

}

fn analyze_file(file: elf::File) {
    let mut sections: Vec<Section> = file.sections.iter()
        .map(|section| Section::from_elf_file(&file, &section.shdr))
        .collect();
    resolve_symbols(file, &mut sections);
    // for section in &sections {
    //     println!("{}, class {:?}", section.name, section.class);
    //     for sym in &section.symbols {
    //         println!("  Symbol {}, class {:?}", sym.name, sym.class);
    //     }
    // }

    summarize_sections(&sections);
}

fn main() {
    let path = PathBuf::from("target/debug/rust-dietician");
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };
    
    analyze_file(file)
}
