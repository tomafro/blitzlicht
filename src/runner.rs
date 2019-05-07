
use crate::*;
use crate::data::Line;
use crate::io::Reader;
use crate::matcher::Matcher;
use crate::printer::Printer;

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

    pub fn iter(&self) -> std::collections::vec_deque::Iter<String> {
        self.buffer.iter()
    }
}

pub struct Runner {
    reader: Reader,
    matcher: Matcher,
    matches: HashSet<String>,
    buffer: Buffer,
}

impl Runner {
    fn new(reader: Reader, matcher: Matcher) -> Runner {
        Runner { reader, matcher, matches: HashSet::new(), buffer: Buffer::new(1000) }
    }

    pub fn configure<C>(config: C) -> Result<Self>
    where C: Into<Config> {
        let config = config.into();
        let reader = Reader::file(&config.file, config.tail)?;
        let matcher = Matcher::new(config.patterns);
        let runner = Self::new(reader, matcher);
        Ok(runner)
    }

    pub fn run(mut self, printer: &mut Printer) -> Result<()> {
        for line in self.reader {
            match self.matcher.matches(&line) {
                true  => Self::matched_line(&mut self.matches, &self.buffer, printer, &line),
                false => Self::unmatched_line(&self.matches, printer, &line)
            }
            self.buffer.append(line);
        }

        Ok(())
    }

    fn matched_line(matches: &mut HashSet<String>, buffer: &Buffer, printer: &mut Printer, unparsed: &String) {
        match Line::parse(unparsed) {
            Some(matched) => {
                if matches.insert(matched.id.to_owned()) {
                    for previous in buffer.iter() {
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
