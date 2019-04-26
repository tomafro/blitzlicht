use crate::*;

pub trait Printer {
    fn unrecognised(&self, line: &String);
    fn unmatched(&self, line: &Line);
    fn matched(&self, line: &Line);
    fn matched_id(&self, line: &Line);
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
        //print!("{}", line);
    }

    fn unmatched(&self, line: &Line) {
        //print!("[{}:{:.5}] {}\n", line.context, line.id, line.rest);
    }

    fn matched(&self, line: &Line) {
        print!("MATCH! [{}:{:.5}] {}\n", line.context, line.id, line.rest);
    }

    fn matched_id(&self, line: &Line) {
        print!("ID! [{}:{:.5}] {}\n", line.context, line.id, line.rest);
    }
}
