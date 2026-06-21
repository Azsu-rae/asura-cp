use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n: i32 = s.trim().parse().unwrap();
    if n == 1 {
        println!("{}", 1);
        return;
    } else if n <= 3 {
        println!("NO SOLUTION");
        return;
    }

    // n=4
    // 2 4 1 3 difference btween the highest even number and the lowest odd...
    // 1 3 2 4 difference btween the highest odd number and the lowest even..
    //
    // n=5
    // 1 3 5 2 4
    // 2 4 1 3 5

    for num in (2..=n).step_by(2) {
        print!("{} ", num);
    }

    for num in (1..=n).step_by(2) {
        print!("{} ", num);
    }
}
