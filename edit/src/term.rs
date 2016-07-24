use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::io::Stdout;

const ESC: &'static str = "\x1b";
const CSI: &'static str = "\x1b[";
const CLEAR: &'static str = "\x1b[2J";

pub struct Screen {
    row: u32,
    col: u32,
    out: BufWriter<Stdout>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen { row: 0, col: 0, out: BufWriter::new(io::stdout()) }
    }

    fn write(&mut self, text: &str) {
        self.out.write(text.as_bytes());
    }

    pub fn to_col(&mut self, col: u32) {
        self.col = col;
        self.write(CSI);
        self.write(&(col + 1).to_string());
        self.write("G");
    }

    pub fn to_pos(&mut self, row: u32, col: u32) {
        self.row = row;
        self.col = col;
        self.write(CSI);
        self.write(&(row + 1).to_string());
        self.write(";");
        self.write(&(col + 1).to_string());
        self.write("H");
    }

    pub fn clear(&mut self) {
        self.row = 0;
        self.col = 0;
        self.write(CLEAR);
    }

    pub fn print(&mut self, text: &str) {
        self.row += text.len() as u32;
        self.out.write(text.as_bytes());
    }

}

