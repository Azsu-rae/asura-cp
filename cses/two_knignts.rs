use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n: i64 = s.trim().parse().unwrap();

    for k in 1..=n {
        println!(
            "{}",
            if k == 1 {
                0
            } else if k == 2 {
                6
            } else if k == 3 {
                (k.pow(2) * (k.pow(2) - 1) - 8 - 8) / 2
            } else if k == 4 {
                (k.pow(2) * (k.pow(2) - 1) - 8 - 24 - 16) / 2
            } else {
                (k.pow(2) * (k.pow(2) - 1)
                    - 8
                    - 24
                    - 16
                    - (k - 4) * 4 * 4
                    - (k - 4) * 6 * 4
                    - (k - 4).pow(2) * 8)
                    / 2
            }
        )
    }
}
