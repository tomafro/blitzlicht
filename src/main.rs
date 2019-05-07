use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;
use blitzlicht::cli::from_cli;

fn main() -> Result<()> {
    let config = from_cli();
    println!("{:?}", config);

    let runner = Runner::new(
        Reader::file(&config.file, config.tail)?,
        Matcher::new(config.patterns)
    );

    let mut printer = BasicPrinter::new();
    runner.run(&mut printer)
}
