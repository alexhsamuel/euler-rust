extern crate edit;

use edit::term::Screen;

fn main() {
    let mut screen = Screen::new();
    screen.clear();
    screen.to_pos(0, 0);
    screen.print("The quick brown fox jumped over the lazy dogs.");
}

