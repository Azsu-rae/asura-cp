use std::collections::HashMap;
use std::io::{self, Read};

// fn naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut two_sum: Vec<i32> = vec![0, 0];
//     for i in 0..nums.len() {
//         for j in (i + 1)..nums.len() {
//             if nums[i] + nums[j] == target {
//                 return vec![j as i32, i as i32];
//             }
//         }
//     }
//
//     unreachable!()
// }

fn idiomatic(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }

        seen.insert(num, i);
    }

    unreachable!()
}

fn main() {
    let mut scan = Scanner::new();

    let target: i32 = scan.next();
    let n: usize = scan.next();

    let nums: Vec<i32> = (0..n).map(|_| scan.next()).collect();

    // println!("{target}");
    // println!("{nums:?}");

    println!("{:?}", idiomatic(nums, target));
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
