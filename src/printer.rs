use crate::*;

pub trait Printer {
    fn raw(&self, line: &String);
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
    fn raw(&self, line: &String) {
        print!("{}", line);
    }

    fn line(&self, line: &Line) {
        print!("{}", line);
    }
}
