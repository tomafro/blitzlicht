use blitzlicht::*;
use blitzlicht::io::Reader;

fn main() -> Result<()> {
    let reader = Reader::file("/Users/tom/Work/basecamp/haystack/log/development.log", true)?;

    for line in reader {
        print!("{}", line);
    }

    Ok(())
}
