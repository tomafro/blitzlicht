use crate::*;

use std::collections::HashMap;

use ansi_term::Colour::{Red, Black, Yellow, RGB};

pub trait Printer {
    fn unrecognised(&mut self, line: &String);
    fn unmatched(&mut self, line: &Line);
    fn matched(&mut self, line: &Line);
    fn matched_id(&mut self, line: &Line);
}

pub struct BasicPrinter {
    colours: HashMap<String, ansi_term::Colour>
}

impl BasicPrinter {
    pub fn new() -> BasicPrinter {
        Self { colours: HashMap::new() }
    }

    fn colour_for(&mut self, id: &str) -> &ansi_term::Colour {
        self.colours.entry(id.to_owned()).or_insert_with(|| RGB(rand::random(), rand::random(), rand::random()))
    }
}

impl Printer for BasicPrinter {
    fn unrecognised(&mut self, line: &String) {
        //print!("{}", line);
    }

    fn unmatched(&mut self, line: &Line) {
        //print!("[{}:{:.5}] {}\n", line.context, line.id, line.rest);
    }

    fn matched(&mut self, line: &Line) {
        print!("{}[{}:{}] {}\n", self.colour_for(&line.id).paint("█"), line.context, self.colour_for(&line.id).paint(line.id), line.rest);
    }

    fn matched_id(&mut self, line: &Line) {
        print!("{}[{}:{}] {}\n", self.colour_for(&line.id).paint("█"), line.context, self.colour_for(&line.id).paint(line.id), line.rest);
    }
}
