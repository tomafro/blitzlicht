use crate::*;

use std::collections::HashMap;

use ansi_term::Colour::RGB;

pub trait Printer {
    fn unrecognised(&mut self, line: &String);
    fn unmatched(&mut self, line: &Line);
    fn matched(&mut self, line: &Line);
    fn matched_id(&mut self, line: &Line);
}

pub struct BasicPrinter {
    print_unmatched: bool,
    print_unrecognised: bool,
    colours: HashMap<String, ansi_term::Colour>
}

impl BasicPrinter {
    pub fn new() -> BasicPrinter {
        Self { colours: HashMap::new(), print_unmatched: false, print_unrecognised: false }
    }

    fn colour_for(&mut self, id: &str) -> &ansi_term::Colour {
        self.colours.entry(id.to_owned()).or_insert_with(|| RGB(rand::random(), rand::random(), rand::random()))
    }
}

impl Printer for BasicPrinter {
    fn unrecognised(&mut self, line: &String) {
        if self.print_unrecognised {} {
            print!("{}", line);
        }
    }

    fn unmatched(&mut self, line: &Line) {
        if self.print_unmatched {
            print!("[{}:{:.5}] {}\n", line.context, line.id, line.rest);
        }
    }

    fn matched(&mut self, line: &Line) {
        print!("{}{}:{} {}\n", self.colour_for(&line.id).paint("░"), line.context, self.colour_for(&line.id).paint(line.id), line.rest);
    }

    fn matched_id(&mut self, line: &Line) {
        print!("{}{}:{} {}\n", self.colour_for(&line.id).paint("░"), line.context, self.colour_for(&line.id).paint(line.id), line.rest);
    }
}
