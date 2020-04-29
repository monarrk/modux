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
    pub fn add_to_main(&mut self, emit: &str, what: &str, start: usize, end: usize) {
        let mut start_index = 0usize;
        let mut state = State::Start;
        let mut string = String::new();

        for (i, c) in emit.chars().enumerate() {
            match state {
                State::Start => match c {
                    '{' => {
                        if start_index < i {
                            //string += &emit[start_index..i];
                        }
                        state = State::Open;
                    },
                    '}' => {
                        if start_index < i {
                            string += &emit[start_index..i];
                        }
                        state = State::Close;
                    },
                    _ => {},
                },
                State::Open => match c {
                    '{' => {
                        state = State::Start;
                        string += &emit[start_index..i];
                        start_index = i;
                    },
                    '}' => {
                        string += &what[start..end];
                        state = State::Start;
                        start_index = i + 1;
                    },
                    _ => panic!("Uknown format character: {}", c),
                },
                State::Close => match c {
                    '}' => {
                        state = State::Start;
                        start_index = i;
                    },
                    _ => panic!("Single '}' encountered"),
                },
            }
        }

        self.main = format!("{}{}\n", self.main, emit.replace("{}", &string));
    }

    pub fn dump(&self) -> String {
        format!("; Header\n{}\n; Main\n{}\nret i32 0\n}}", self.header, self.main)
    }
}

// for Ir.add_to_main()
enum State {
    Start,
    Open,
    Close
}
