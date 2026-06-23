use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n: i64 = s.trim().parse().unwrap();
    if (n * (n + 1) / 2) % 2 == 1 {
        println!("NO");
        return;
    } else {
        println!("YES");
    }

    let mut set1: Vec<i64> = Vec::new();
    let mut set2: Vec<i64> = Vec::new();

    let mid = n / 2;
    for step in 1..=mid {
        if step <= (mid / 2) + n % 2 {
            set1.push(mid - step + 1);
            set1.push(mid + step);
        } else {
            set2.push(mid - step + 1);
            set2.push(mid + step);
        }
    }

    if n % 2 == 1 {
        set2.push(n);
    }

    println!("{}", set1.len());
    for num in set1 {
        print!("{} ", num);
    }

    println!("\n{}", set2.len());
    for num in set2 {
        print!("{} ", num);
    }
}

//
// 1 : 1
// 3 : 1 2
// 6 : 1 2 3
// 10: 1 2 3 4
//
//     2 3
//     4 1
//
// 15: 1 2 3 4 5
// 21: 1 2 3 4 5 6
//
//
// 28: 1 2 3 4 5 6 7
//
//     4 3 5 2
//     1 6 7
//
// 36: 1 2 3 4 5 6 7 8
//
//     4 5 6 3
//     2 7 8 1
//
//
//  /2
//
//  3
//
//  4
