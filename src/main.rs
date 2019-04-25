use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::printer::{ Printer, BasicPrinter };

fn main() -> Result<()> {
    let path = "/Users/tom/Work/basecamp/haystack/log/development.log";
    let reader = Reader::file(path, false)?;
    let printer = BasicPrinter::new();

    for line in reader {
        let l = Line::new(line);
        printer.line(&l);
    }

    Ok(())
}