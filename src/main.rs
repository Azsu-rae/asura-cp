use rand::Rng;

fn init_vec() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(5..10);
    let mut t: Vec<u8> = vec![0; n];

    for i in 0..t.len() {
        t[i] = rng.gen_range(1..100);
    }

    return t;
}

fn main() {
    let t = init_vec();
    println!("{:?}", t);
}
