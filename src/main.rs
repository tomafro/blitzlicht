use blitzlicht::*;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;
use blitzlicht::cli::cli;

fn main() -> Result<()> {
    let runner = Runner::configure(cli())?;
    let mut printer = BasicPrinter::configure(cli())?;
    runner.run(&mut printer)
}
