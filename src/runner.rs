
use crate::*;
use crate::io::Reader;
use crate::matcher::Matcher;
use crate::printer::{ Printer, BasicPrinter };

use std::collections::HashSet;

pub struct Runner {
    pub reader: Reader,
    pub matcher: Matcher,
    pub printer: BasicPrinter,
    pub matches: HashSet<String>
}

fn matched_line(matches: &mut HashSet<String>, printer: &mut Printer, unparsed: &String) {
    match Line::parse(unparsed) {
        Some(line) => {
            matches.insert(line.id.to_owned());
            printer.matched(&line);
        },
        None    => printer.unrecognised(unparsed)
    }
}

fn unmatched_line(matches: &HashSet<String>, printer: &mut Printer, unparsed: &String) {
    match Line::parse(&unparsed) {
        Some(line) => {
            match matches.contains(line.id) {
                true  => printer.matched_id(&line),
                false => printer.unmatched(&line)
            }
        }
        None    => printer.unrecognised(&unparsed)
    }
}

impl Runner {
    pub fn run(mut self) -> Result<()> {
        for line in self.reader {
            match self.matcher.matches(&line) {
                true  => matched_line(&mut self.matches, &mut self.printer, &line),
                false => unmatched_line(&self.matches, &mut self.printer, &line)
            }
        }

        Ok(())
    }
}
