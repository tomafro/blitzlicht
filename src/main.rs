use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;
use blitzlicht::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::from_cli();
    println!("{:?}", cli);

    let runner = Runner::new(
        Reader::file(&cli.file, cli.tail)?,
        Matcher::new(cli.patterns)
    );

    let mut printer = BasicPrinter::new();
    runner.run(&mut printer)
}
