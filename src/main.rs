use blitzlicht::*;
use blitzlicht::io::Reader;
use blitzlicht::matcher::Matcher;
use blitzlicht::runner::Runner;

fn main() -> Result<()> {
    let path = "/Users/tom/Work/basecamp/haystack/log/development.log";

    let runner = Runner::new(
        Reader::file(path, false)?,
        Matcher::new()
    );

    runner.run()
}
