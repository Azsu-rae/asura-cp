use std::cmp::max;
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut _n: i32 = s.trim().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let arr: Vec<i64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let (mut count, mut prev): (i64, i64) = (0, 0);
    for num in arr {
        let diff: i64 = (prev - num).max(0);
        count += diff;
        prev = num + diff;
    }

    println!("{}", count);
}
