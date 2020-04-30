pub struct Ir {
    header: String,
    main: String
}

impl Ir {
    pub fn new(triple: String) -> Ir {
        Ir {
            header: format!("target triple = \"{}\"\ntarget datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n\n", triple),
            main: String::from("define dso_local i32 @main() #0 {\n"),
        }
    }

    pub fn add_raw_to_main(&mut self, what: &str) {
        self.main = format!("{}{}\n", self.main, what);
    }

    // Perform substitutions if need be
    fn parse(emit: &str, what: &str, start: &str, end: &str) -> String{
        let starts: Vec<&str> = start.split(":").collect();
        let ends: Vec<&str> = end.split(":").collect();

        let mut emit_mut: Vec<char> = emit.chars().collect();
        let mut arg = 0usize;

        let mut skip = false;
        let mut skips = 0;
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
                let last = first.split_off(i + skips);
                let mut vec: Vec<char> = Vec::new();
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

                first = [first, vec[..].to_vec()].concat();
                emit_mut = [first, last.to_vec()].concat();
                arg += 1;
            }
        }

        let to_emit: String = emit_mut.into_iter().collect();
        format!("\t{}\n", to_emit)
    }

    pub fn add_to_main(&mut self, emit: &str, what: &str, start: &str, end: &str) {
        self.main = format!("{}{}", self.main, Ir::parse(emit, what, start, end));
    }

    pub fn add_to_header(&mut self, emit: &str, what: &str, start: &str, end: &str) {
        self.header = format!("{}{}", self.header, Ir::parse(emit, what, start, end));
    }

    pub fn dump(&self) -> String {
        format!("; Header\n{}\n; Main\n{}\n\tret i32 0\n}}", self.header, self.main)
    }
}
