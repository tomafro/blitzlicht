use crate::data::Line;

use std::collections::HashMap;

use ansi_term::Colour::RGB;

struct Palette {
    current: usize,
    colours: std::vec::Vec<ansi_term::Colour>,
    colour_roles: HashMap<String, ansi_term::Colour>
}

impl Palette {
    pub fn colour_for(&mut self, id: &str) -> &ansi_term::Colour {
        let roles = &mut self.colour_roles;
        let colour = match roles.contains_key(id) {
            true => None,
            false => {
                self.current = self.current + 1;
                if self.current >= self.colours.len() {
                    self.current = 0;
                }
                Some(self.colours[self.current])
            }
        };
        roles.entry(id.to_owned()).or_insert_with(|| colour.unwrap())
    }
}

pub trait Printer {
    fn unrecognised(&mut self, line: &String);
    fn unmatched(&mut self, line: &Line);
    fn matched(&mut self, line: &Line);
    fn matched_id(&mut self, line: &Line);
}

pub struct BasicPrinter {
    print_unmatched: bool,
    print_unrecognised: bool,
    palette: Palette
}

const PALETTE: [ansi_term::Colour; 6] = [
    RGB(255, 154, 162),
    RGB(255, 183, 178),
    RGB(255, 218, 193),
    RGB(226, 240, 203),
    RGB(181, 234, 215),
    RGB(199, 206, 234)
];

impl BasicPrinter {
    pub fn new() -> BasicPrinter {
        let palette = Palette { current: 0, colours: PALETTE.to_vec(), colour_roles: HashMap::new() };
        Self { palette: palette, print_unmatched: false, print_unrecognised: false }
    }
}

impl Printer for BasicPrinter {
    fn unrecognised(&mut self, line: &String) {
        if self.print_unrecognised {
            print!("{}", line);
        }
    }

    fn unmatched(&mut self, line: &Line) {
        if self.print_unmatched {
            print!("[{}:{:.5}] {}\n", line.context, line.id, line.rest);
        }
    }

    fn matched(&mut self, line: &Line) {
        print!("[{}] [{}] {}\n", line.context, self.palette.colour_for(&line.id).paint(line.id), line.rest);
    }

    fn matched_id(&mut self, line: &Line) {
        print!("[{}] [{}] {}\n", line.context, self.palette.colour_for(&line.id).paint(line.id), line.rest);
    }
}
