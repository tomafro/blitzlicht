use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::printer::{ Printer, BasicPrinter };

use std::collections::HashSet;

fn main() -> Result<()> {
    let path = "/Users/tom/Work/basecamp/haystack/log/development.log";
    let reader = Reader::file(path, false)?;
    let matcher = Matcher::new();
    let printer = BasicPrinter::new();
    let mut matches = HashSet::new();

    for line in reader {
        if matcher.matches(&line) {
            match Line::parse(&line) {
                Some(l) => {
                    matches.insert(l.id.to_owned());
                    printer.matched(&l);
                },
                None    => printer.unrecognised(&line)
            }
        } else {
            match Line::parse(&line) {
                Some(l) => {
                    if matches.contains(l.id) {
                        printer.matched_id(&l);
                    }
                    else {
                        printer.unmatched(&l);
                    }
                }
                None    => printer.unrecognised(&line)
            }
        }
    }

    Ok(())
}
