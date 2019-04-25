use crate::*;

pub trait Printer {
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
    fn line(&self, line: &Line) {
        print!("{}", line);
    }
}
