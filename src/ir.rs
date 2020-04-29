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

    pub fn add_to_main(&mut self, what: &str) {
        self.main = format!("{}{}\n", self.main, what);
    }

    pub fn dump(&self) -> String {
        format!("; Header\n{}\n; Main\n{}}}", self.header, self.main)
    }
}
