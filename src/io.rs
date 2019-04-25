use crate::*;

use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::time::Duration;

pub struct Reader {
    input: Box<BufRead>,
    tail: bool,
}

impl Reader {
    pub fn file(path: &str, tail: bool) -> Result<Reader> {
        let file = open(path)?;
        let input = Box::new(BufReader::new(file));

        Ok(Reader { input, tail })
    }
}

static PAUSE: Duration = Duration::from_millis(100);

impl Iterator for Reader {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();

        loop {
            match self.input.read_line(&mut line) {
                Ok(0) => {
                    match self.tail {
                        true => std::thread::sleep(PAUSE),
                        false => return None
                    }
                },
                Ok(_) => return Some(Line { line }),
                Err(_) => return None
            }
        }
    }
}

fn open(path: &str) -> Result<File> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => Err(Error::UnableToOpenFile(path.to_string()))
    }
}
