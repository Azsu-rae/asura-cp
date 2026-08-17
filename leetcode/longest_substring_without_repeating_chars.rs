use std::cmp::max;
use std::io::{self, Read};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ascii: [i32; 128] = [-1; 128];
    let mut last_repetition = -1;

    let mut mx = 0;

    for (i, c) in s.bytes().enumerate() {
        let c = c as usize;
        let i = i as i32;
        last_repetition = last_repetition.max(ascii[c]);
        ascii[c] = i;
        mx = max(i - last_repetition, mx);
    }

    // dbg!(&buf);
    // dbg!(last_repetition);
    mx as i32
}

fn main() {
    dbg!(length_of_longest_substring(Scanner::new().next()));
}

struct Scanner {
    input: Vec<String>,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();

        Self {
            input: input.split_whitespace().rev().map(String::from).collect(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.input.pop().unwrap().parse().ok().unwrap()
    }
}
