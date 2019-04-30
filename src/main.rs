use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;
use blitzlicht::printer::BasicPrinter;
use blitzlicht::cli::Cli;

use std::str::FromStr;
use clap::Shell;

fn main() -> Result<()> {
    let path = "/Users/tom/Work/basecamp/haystack/log/development.log";

    let cli = Cli::from_cli();
    let shell = Shell::from_str("fish").unwrap();
    let mut app = Cli::app();
    app.gen_completions_to("blitzlicht", shell, &mut std::io::stdout());


    // let mut printer = BasicPrinter::new();
    // let runner = Runner::new(
    //     Reader::file(path, false)?,
    //     Matcher::new(),
    //     &mut printer
    // );

    // runner.run()
    Ok(())
}

// let outdir = ::std::env::var_os("OUT_DIR").expect("OUT_DIR not found.");
//   let mut app = cli::Cli::app();
//   for shell in &Shell::variants() {
//     let shell = Shell::from_str(*shell)?;
//     app.gen_completions(env!("CARGO_PKG_NAME"), shell, &outdir);
//   }
//   Ok(())
