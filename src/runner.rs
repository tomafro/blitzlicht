
use crate::*;
use crate::io::Reader;
use crate::matcher::Matcher;
use crate::printer::{ Printer, BasicPrinter };

use std::collections::{ HashSet, VecDeque };

pub struct Buffer {
    capacity: usize,
    buffer: VecDeque<String>
}

impl Buffer {
    pub fn new(capacity: usize) -> Buffer {
        let buffer = VecDeque::with_capacity(capacity);
        Buffer {
            capacity,
            buffer
        }
    }

    pub fn append(&mut self, line: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(line);
    }

    pub fn lines(&self) -> &VecDeque<String> {
        &self.buffer
    }
}

impl IntoIterator for Buffer {
    type Item = String;
    type IntoIter = std::collections::vec_deque::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

pub struct Runner<'a> {
    reader: Reader,
    matcher: Matcher,
    printer: &'a mut Printer,
    matches: HashSet<String>,
    buffer: Buffer,
}

impl<'a> Runner<'a> {
    pub fn new(reader: Reader, matcher: Matcher, printer: &mut Printer) -> Runner {
        Runner { reader, matcher, printer, matches: HashSet::new(), buffer: Buffer::new(1000) }
    }

    pub fn run(mut self) -> Result<()> {
        for line in self.reader {
            match self.matcher.matches(&line) {
                true  => Self::matched_line(&mut self.matches, &self.buffer, self.printer, &line),
                false => Self::unmatched_line(&self.matches, self.printer, &line)
            }
            self.buffer.append(line);
        }

        Ok(())
    }

    fn matched_line(matches: &mut HashSet<String>, buffer: &Buffer, printer: &mut Printer, unparsed: &String) {
        match Line::parse(unparsed) {
            Some(matched) => {
                if matches.insert(matched.id.to_owned()) {
                    for previous in buffer.lines() {
                        match Line::parse(&previous) {
                            Some(line) => {
                                if line.id == matched.id {
                                    printer.matched_id(&line);
                                }
                            },
                            None => ()
                        }
                    }
                }
                printer.matched(&matched);
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
}
