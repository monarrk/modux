use rand::{thread_rng, Rng};

pub struct Ir {
    header: String,
    main: String,
    number: usize,
    numbers: Vec<String>,
}

impl Ir {
    pub fn new(triple: String) -> Ir {
        Ir {
            header: format!("target triple = \"{}\"\ntarget datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n\n", triple),
            main: String::from("define dso_local i32 @main() #0 {\n"),
            number: 0,
            numbers: Vec::new(),
        }
    }

    /// Generate random strings for variable names
    fn rand_string() -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const LEN: usize = 8;
        let mut rng = thread_rng();

        (0..LEN).map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        }).collect()
    }

    /// Add raw IR (no substitution) to the main function
    pub fn add_raw_to_main(&mut self, what: &str) {
        self.main = format!("{}{}\n", self.main, what);
    }

    /// self.add_raw_to_main() except for the header
    pub fn add_raw_to_header(&mut self, what: &str) {
        self.header = format!("{}{}\n", self.header, what);
    }

    /// Parse IR and input modux code to perform substitutions if need be
    fn parse(&mut self, emit: &str, what: &str, start: &str, end: &str) -> String {
        let starts: Vec<&str> = start.split(":").collect();
        let ends: Vec<&str> = end.split(":").collect();

        let mut emit_mut: Vec<char> = emit.chars().collect();
        let mut arg = 0usize;

        let mut skip = false;
        let mut skips = 0;
        
        // Iterate over every character in the IR that will be emitted
        for (i, c) in emit.chars().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            if c == '#' && emit.chars().nth(i + 1) == Some('#') {
                skip = true;
                emit_mut.remove(i);
            } else if c == '#' {
                // substitute # for a string in the IR
                
                // Remove extra characters
                emit_mut.remove(i + skips);
                let mut first = emit_mut.clone();

                // Skip some chars in case there's more than one substitution
                let last = first.split_off(i + skips);
                let mut vec: Vec<char> = Vec::new();

                // Iterate over the modux input and parse out the bits to be substituted into IR
                'outer: for (j, a) in what.chars().enumerate() {
                    // Detected a starting delimeter
                    if a == starts[arg].chars().collect::<Vec<char>>()[0] {
                        for b in what[j + 1..].chars() {
                            if b == ends[arg].chars().collect::<Vec<char>>()[0] {
                                skips -= 1;
                                break 'outer;
                            }
                            skips += 1;
                            vec.push(b);
                        }
                    };
                }

                // Concatenate vectors
                first = [first, vec[..].to_vec()].concat();
                emit_mut = [first, last.to_vec()].concat();
                arg += 1;
            } else if c == '$' {
                // Replace with the next number (for placeholder variables)
                emit_mut.remove(i + skips);
                let mut first = emit_mut.clone();

                let last = first.split_off(i + skips);
               
                let rstr = Ir::rand_string();
                // TODO: this makes me feel dirty
                self.numbers.push(format!("{}", rstr));
                for i in rstr.chars() {
                    first.push(i);
                    skips += 1;
                }
                skips -= 1;
                emit_mut = [first, last.to_vec()].concat();
                self.number += 1;
            } else if c == '^' {
                // Replace with the last number (for placeholder variables)
                let num = (match emit_mut[i + skips + 1].to_digit(10) {
                    Some(n) => n,
                    None => {
                        eprintln!("ERROR parsing ^ expression from YARA ({}): '{}' is not a valid number", i, emit_mut[i + skips + 1]);
                        std::process::exit(1);
                    }
                }) as usize;

                emit_mut.remove(i + skips);
                emit_mut.remove(i + skips);

                let mut first = emit_mut.clone();

                let last = first.split_off(i + skips);
                
                for i in self.numbers[self.number - num].chars() {
                    first.push(i);
                    skips += 1;
                }
                skips -= 2;
                emit_mut = [first, last.to_vec()].concat();
            }
        }

        let to_emit: String = emit_mut.into_iter().collect();
        format!("{}\n", to_emit)
    }

    /// Add IR to the main function with substitutions
    pub fn add_to_main(&mut self, emit: &str, what: &str, start: &str, end: &str) {
        let parsed = self.parse(emit, what, start, end);
        self.main = format!("{}\t{}", self.main, parsed);
    }

    /// Same as self.add_to_main() but for the header
    pub fn add_to_header(&mut self, emit: &str, what: &str, start: &str, end: &str) {
        let parsed = self.parse(emit, what, start, end);
        self.header = format!("{}{}", self.header, parsed);
    }

    /// Dump the generated IR into a String so it can be written to a file
    pub fn dump(&self) -> String {
        format!("; Header\n{}\n; Main\n{}\n\tret i32 0\n}}", self.header, self.main)
    }
}
