use std::io;

const MOD: u64 = 1_000_000_007;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n: u128 = s.trim().parse().unwrap();
    let mut result = 1;
    for _ in 1..=n {
        result = (result * 2) % MOD;
    }

    println!("{}", result);
}
