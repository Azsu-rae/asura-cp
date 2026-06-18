use std::io;

fn main() {
    // read the line
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let _ = s.split_whitespace();
}
