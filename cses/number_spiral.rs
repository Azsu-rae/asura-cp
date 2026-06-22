use std::cmp::max;
use std::io;
use std::io::Read;

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let mut input = s.trim().split_whitespace();

    let n: i64 = input.next().unwrap().parse().unwrap();

    for _i in 0..n {
        let x: i64 = input.next().unwrap().parse().unwrap();
        let y: i64 = input.next().unwrap().parse().unwrap();

        let mx: i64 = max(x, y);
        let square = mx.pow(2);

        println!(
            "{}",
            if square % 2 == 1 {
                if y == mx {
                    square - (x - 1)
                } else {
                    (square - (mx - 1)) - (mx - y)
                }
            } else {
                if x == mx {
                    square - (y - 1)
                } else {
                    (square - (mx - 1)) - (mx - x)
                }
            }
        )
    }
}
