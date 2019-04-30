use crate::*;
use clap::{App, Arg};

pub struct Cli {
}

impl<'a, 'b> Cli {
    pub fn app() -> App<'a, 'b> {
        App::new(NAME)
            .version(VERSION)
            .author(AUTHORS)
            .about(DESCRIPTION)
                .display_order(0)
                .arg(Arg::with_name("short")
                    .long("short")
                    .help("Show only basic information about each request"))
                .arg(Arg::with_name("rainbow")
                    .long("rainbow")
                    .help("Colour request context and ids"))
                .arg(Arg::with_name("tail")
                    .long("tail")
                    .short("t")
                    .takes_value(false)
                    .help("Tail the given file"))
                .arg(Arg::with_name("file")
                    .long("file")
                    .short("f")
                    .takes_value(true)
                    .help("path to log file")
                    .default_value("./log/development.log"))
                .arg(Arg::with_name("pattern")
                    .value_name("PATTERNS")
                    .help("Patterns to match")
                    .takes_value(true)
                    .multiple(true))
    }

    pub fn from_cli() -> Cli {
        let app = Cli::app();
        println!("{:?}", app.get_matches());
        Cli {}
    }
}
