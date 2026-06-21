use std::io;

fn main() {
    // read the line
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n: i64 = s.trim().parse().unwrap();

    s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut sum: i64 = 0;
    for a in s.split(" ") {
        sum += a.trim().parse::<i64>().unwrap();
    }

    println!("{}", n * (n + 1) / 2 - sum);
}
