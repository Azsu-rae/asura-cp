use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let target: i32 = s.trim().parse().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for num_str in s.trim().split_whitespace() {
        nums.push(num_str.parse().unwrap());
    }

    let mut two_sum: Vec<i32> = vec![0, 0];
    'outer: for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                two_sum[0] = i as i32;
                two_sum[1] = j as i32;
                break 'outer;
            }
        }
    }

    println!("{:?}", two_sum);
}
