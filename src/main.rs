use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;
use blitzlicht::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::from_cli();
    println!("{:?}", cli);

    let mut printer = BasicPrinter::new();
    let runner = Runner::new(
        Reader::file(&cli.file, cli.tail)?,
        Matcher::new(cli.patterns),
        &mut printer
    );

    runner.run()
}
