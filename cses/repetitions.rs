use std::cmp::max;
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut curr: char = 'O';
    let (mut longest, mut curr_len) = (0, 0);
    for c in s.chars() {
        if c != curr {
            curr = c;
            longest = max(longest, curr_len);
            curr_len = 1;
        } else {
            curr_len += 1;
        }
    }

    println!("{}", longest);
}
