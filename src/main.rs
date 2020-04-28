use std::env;
use yara::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please enter a file");
        std::process::exit(1);
    }

    let mut compiler = Compiler::new().unwrap();
    compiler.add_rules_file("rules.yara").unwrap();
    let rules = compiler.compile_rules().unwrap();
    let results = rules.scan_file(&args[1], 5).unwrap();

    for i in results.iter() {
        for s in i.strings.iter() {
            for m in s.matches.iter() {
                match std::str::from_utf8(&m.data) {
                    Ok(d) => println!("{}", d),
                    Err(e) => panic!("Invalid UTF8 sequence: {}", e),
                }
            }
        }
    }
}
