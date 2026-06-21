use std::io;
use std::io::Read;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut curr: char = 'O';
    for c in s {
        if c != curr {}
    }
}
