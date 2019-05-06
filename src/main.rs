use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;
use blitzlicht::cli::Cli;

use std::str::FromStr;
use clap::Shell;

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
    //Ok(())
}

fn completion() -> Result<()> {
    let shell = Shell::from_str("fish").unwrap();
    let mut app = Cli::app();
    app.gen_completions_to("blitzlicht", shell, &mut std::io::stdout());
    Ok(())
}
