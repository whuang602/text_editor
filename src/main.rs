use std::io;
use std::io::{stdin, Write, stdout};
use termion::color;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
fn main() {
    println!("Editor start...\n");
    // let mut input = String::new();
    print!("{}", termion::clear::All);
    println!("{}", color::Fg(color::Red));
    let mut stdout = stdout().into_raw_mode().unwrap();
    let input = stdin();
    for c in input.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => {
                break;
          }
          _=> {}
        }
        stdout.flush().unwrap();
    }
    // while io::stdin().read_line(&mut input).expect("Failed to accept input") > 0 {
    //     println!("{}",input);
    //     input.clear();
    // }
    ()
}
