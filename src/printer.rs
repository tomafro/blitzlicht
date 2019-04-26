use crate::*;

pub trait Printer {
    fn unrecognised(&self, line: &String);
    fn line(&self, line: &Line);
}

pub struct BasicPrinter {
}

impl BasicPrinter {
    pub fn new() -> BasicPrinter {
        Self {}
    }
}

impl Printer for BasicPrinter {
    fn unrecognised(&self, line: &String) {
        print!("{}", line);
    }

    fn line(&self, line: &Line) {
        print!("[{}:{:.5}] {}\n", line.context, line.id, line.rest);
    }
}
