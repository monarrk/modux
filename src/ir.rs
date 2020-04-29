pub struct Ir {
    header: String,
    main: String
}

impl Ir {
    pub fn new(triple: String) -> Ir {
        Ir {
            header: format!("target triple = \"{}\"\ntarget datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"", triple),
            main: String::from("define dso_local i32 @main() #0 {\n"),
        }
    }

    pub fn add_raw_to_main(&mut self, what: &str) {
        self.main = format!("{}{}\n", self.main, what);
    }

    // TODO this is an insane hack
    pub fn add_to_main(&mut self, emit: &str, what: &str, start: &str, end: &str) {
        let starts: Vec<&str> = start.split(":").collect();
        let ends: Vec<&str> = end.split(":").collect();

        let mut emit_mut: Vec<char> = emit.chars().collect();
        let mut arg = 0usize;

        let mut skip = false;
        for (i, c) in emit.chars().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            if c == '#' && emit.len() > i {
                if emit.chars().nth(i + 1).unwrap() != '#' {
                    println!("i @ : '{}'", emit_mut[i]);
                    emit_mut.remove(i);
                    let mut first = emit_mut.clone();
                    let last = first.split_off(i);
                    //println!("first: {}\nlast: {}", first.iter().collect::<String>(), last.iter().collect::<String>());
                    let mut vec: Vec<char> = Vec::new();
                    println!("{}", &what[starts[arg].parse::<usize>().expect("Failed to parse string")..ends[arg].parse::<usize>().expect("Failed to parse string")]);
                    for c in (&what[starts[arg].parse::<usize>().expect("Failed to parse string")..ends[arg].parse::<usize>().expect("Failed to parse string")]).chars() {
                        vec.push(c);
                    }
                    first = [first, vec[..].to_vec()].concat();
                    emit_mut = [first, last.to_vec()].concat();
                    arg += 1;
                    skip = true;
                } else {
                    emit_mut.remove(i);
                }
            }
        }

        let to_emit: String = emit_mut.into_iter().collect();
        println!("to_emit: {}", to_emit);
        self.main = format!("{}\t{}\n", self.main, to_emit);
    }

    pub fn dump(&self) -> String {
        format!("; Header\n{}\n; Main\n{}\n\tret i32 0\n}}", self.header, self.main)
    }
}
