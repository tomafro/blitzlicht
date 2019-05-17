use crate::*;
use std::str::FromStr;
use clap::{App, Arg, Shell};
use std::str;

pub fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
            .display_order(0)
            .arg(Arg::with_name("short")
                .long("short")
                .help("Show only basic information about each request"))
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

pub fn completions(name: &str) -> String {
    let shell = Shell::from_str(name).unwrap();
    let mut cli = cli();
    let mut output: Vec<u8> = Vec::new();
    cli.gen_completions_to(NAME, shell, &mut output);
    str::from_utf8(&output).unwrap().to_string()
}

impl<'a, 'b> From<App<'a, 'b>> for Config {
    fn from(cli: App) -> Self {
        let matches = cli.get_matches();
        let file = matches.value_of("file").unwrap().to_owned();
        let tail = matches.is_present("tail");
        let short = matches.is_present("short");
        let patterns: Option<Vec<_>> = match matches.values_of("pattern") {
            Some(values) => Some(values.map(|p| p.to_owned()).collect()),
            None => None
        };
        Config { file, tail, short, patterns }
    }
}
