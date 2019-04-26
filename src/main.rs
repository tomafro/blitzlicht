use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::printer::{ Printer, BasicPrinter };

use std::collections::HashSet;

struct Runner {
    reader: Reader,
    matcher: Matcher,
    printer: BasicPrinter,
    matches: HashSet<String>
}

fn matched_line(matches: &mut HashSet<String>, printer: &Printer, unparsed: &String) {
    match Line::parse(unparsed) {
        Some(line) => {
            matches.insert(line.id.to_owned());
            printer.matched(&line);
        },
        None    => printer.unrecognised(unparsed)
    }
}

fn unmatched_line(matches: &HashSet<String>, printer: &Printer, unparsed: &String) {
    match Line::parse(&unparsed) {
        Some(line) => {
            if matches.contains(line.id) {
                printer.matched_id(&line);
            }
            else {
                printer.unmatched(&line);
            }
        }
        None    => printer.unrecognised(&unparsed)
    }
}

impl Runner {
    pub fn run(mut self) -> Result<()> {
        for line in self.reader {
            if self.matcher.matches(&line) {
                matched_line(&mut self.matches, &self.printer, &line);
            } else {
                unmatched_line(&self.matches, &self.printer, &line);
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let path = "/Users/tom/Work/basecamp/haystack/log/development.log";

    let runner = Runner {
        reader:  Reader::file(path, false)?,
        matcher: Matcher::new(),
        printer: BasicPrinter::new(),
        matches: HashSet::new()
    };

    runner.run()
}
