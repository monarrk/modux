mod ir;

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use yara::Compiler;
use clap::{Arg, App};

#[cfg(target_os = "linux")]
static OS: &str = "linux";

#[cfg(target_os = "windows")]
static OS: &str = "windows";

#[cfg(target_os = "macos")]
static OS: &str = "macos";

#[cfg(target_os = "freebsd")]
static OS: &str = "freebsd";

#[cfg(target_arch = "x86_64")]
static ARCH: &str = "x86_64";

#[cfg(target_arch = "x86")]
static ARCH: &str = "x86";

#[cfg(target_vendor = "unknown")]
static VENDOR: &str = "unknown";

#[cfg(target_vendor = "apple")]
static VENDOR: &str = "apple";

#[cfg(target_vendor = "pc")]
static VENDOR: &str = "pc";

fn main() {
    let matches = App::new("Modux")
        .arg(Arg::with_name("rules")
             .short("r")
             .long("rules")
             .value_name("FILE")
             .help("Sets a custom rules file (default is `./rules.yara`)")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE")
             .help("Sets a custom output file")
             .takes_value(true))
        .arg(Arg::with_name("INPUT")
             .index(1)
             .takes_value(true)
             .value_name("INPUT")
             .required(true)
             .help("The file to build"))
        .get_matches();
    
    let input = matches.value_of("INPUT").unwrap_or("main.mx");

    let rules = matches.value_of("rules").unwrap_or("rules.yara");
    let mut compiler = Compiler::new().expect("Failed to initialize compiler");
    match compiler.add_rules_file(rules) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error whule parsing rules: {}", e);
            exit(1);
        }
    };

    let rules = match compiler.compile_rules() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error while compiling rules: {}", e);
            exit(1);
        }
    };
    let results = match rules.scan_file(input, 5) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error while scanning file {}: {}", input, e);
            exit(1);
        },
    };

    let triple = format!("{}-{}-{}", ARCH, VENDOR, OS);
    let mut ir = ir::Ir::new(triple);

    // Create target file
    let path_str = match matches.value_of("output") {
        Some(s) => String::from(s),
        None => {
            let path_slice = input.split(".").collect::<Vec<&str>>();
            let path_str = path_slice[0].to_owned();
            path_str + ".ll"
        }
    };
    let path = Path::new(&path_str);
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            exit(1);
        }
    };

    // Sort matches
    let mut ordr = Vec::new();
    let mut ordm = Vec::new();
    for i in results.iter() {
        for s in i.strings.iter() {
            for m in s.matches.iter() {
                ordr.push(i);
                ordm.push(m);
            }
        }
    }
    if ordm.len() != ordr.len() {
        panic!("This should be unreachable. Something is very wrong.");
    }

    let len = ordm.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if ordm[right].offset < ordm[smallest].offset {
                smallest = right;
            }
        }
        ordm.swap(smallest, left);
        ordr.swap(smallest, left);
    }
   
    for (i, m) in ordm.iter().enumerate() {
                match ordr[i].metadatas[0].value {
                    yara::MetadataValue::String(s) => {
                        if ordr[i].metadatas.len() > 2 {
                            // Location of the IR
                            let loc = match ordr[i].metadatas[1].value {
                                yara::MetadataValue::String(i) => i,
                                _ => panic!("Value must be a String!"),
                            };
                            // Starting delimeters
                            let start = match ordr[i].metadatas[2].value {
                                yara::MetadataValue::String(i) => i,
                                _ => panic!("Value must be a String!"),
                            };
                            // Ending delimeters
                            let end = match ordr[i].metadatas[3].value {
                                yara::MetadataValue::String(i) => i,
                                _ => panic!("Value must be a String!"),
                            };
                            
                            match loc {
                                "main" => ir.add_to_main(s, std::str::from_utf8(&m.data).expect("Failed to decode data"), start, end),
                                "header" => ir.add_to_header(s, std::str::from_utf8(&m.data).expect("Failed to decode data"), start, end),
                                _ => panic!("Invalid location '{}'", loc),
                            };
                        } else {
                            // Add the IR without any substitution
                            let loc = match ordr[i].metadatas[1].value {
                                yara::MetadataValue::String(i) => i,
                                _ => panic!("Value must be a String!"),
                            };

                            match loc {
                                "main" => ir.add_raw_to_main(s),
                                "header" => ir.add_raw_to_header(s),
                                _ => panic!("Invalid location: '{}'", loc),
                            };
                        }
                    },
                    _ => panic!("Value must be a string!"),
                }
    }

    file.write_all(ir.dump().as_bytes()).expect("Failed to write to file");
}
