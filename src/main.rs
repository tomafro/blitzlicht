use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;

fn main() -> Result<()> {
    let path = "/Users/tom/Work/basecamp/haystack/log/development.log";

    let mut printer = BasicPrinter::new();
    let runner = Runner::new(
        Reader::file(path, false)?,
        Matcher::new(),
        &mut printer
    );

    runner.run()
}
