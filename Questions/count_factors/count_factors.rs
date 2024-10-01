use std::io;


struct DSA;

impl DSA {
    pub fn count_factors(a: i64) -> i64 {
        let root = (a as f64).sqrt() as i64 + 1;
        let mut res = 1;
        for i in 2..root {
            if a % i == 0 {
                res += 1;
            }
        }
        res * 2 - ((root - 1) * (root - 1) == a) as i64
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let input: i64 = input.trim().parse()
        .expect("Please type a number!");
    println!("Number of factors: {}", DSA::count_factors(input));
}
