use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::{ Printer, BasicPrinter };

use std::collections::HashSet;

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
