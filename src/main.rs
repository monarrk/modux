mod ir;

use std::env;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use yara::Compiler;

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please enter a file");
        exit(1);
    }

    let mut compiler = Compiler::new().expect("Failed to initialize compiler");
    match compiler.add_rules_file("rules.yara") {
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
    let results = rules.scan_file(&args[1], 5).expect("Failed to scan file");

    let triple = format!("{}-{}-{}", ARCH, VENDOR, OS);
    let mut ir = ir::Ir::new(triple);

    // Create target file
    //
    // TODO make this less...like this
    let path_slice = args[1].split(".").collect::<Vec<&str>>();
    let path_str = path_slice[0].to_owned();
    let path_str_full = path_str + ".ll";
    let path = Path::new(&path_str_full);
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            exit(1);
        }
    };

    // Generate IR
    for i in results.iter() {
        for s in i.strings.iter() {
            for m in s.matches.iter() {
                match i.metadatas[0].value {
                    yara::MetadataValue::String(s) => {
                        if i.metadatas.len() > 1 {
                            let start = match i.metadatas[2].value {
                                yara::MetadataValue::String(i) => i,
                                _ => panic!("Value must be a String!"),
                            };
                            let end = match i.metadatas[3].value {
                                yara::MetadataValue::String(i) => i,
                                _ => panic!("Value must be a String!"),
                            };

                            ir.add_to_main(s, std::str::from_utf8(&m.data).expect("Failed to decode data"), start, end);
                        } else {
                            ir.add_raw_to_main(s);
                        }
                    },
                    _ => panic!("Value must be a string!"),
                }
            }
        }
    }

    file.write_all(ir.dump().as_bytes()).expect("Failed to write to file");
}
