use std::io;

struct DSA;

impl DSA {
    pub fn is_prime(a: i64) -> bool {
        let root = (a as f64).sqrt() as i64 + 1;
        for i in 2..root {
            if a % i == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i64 = input.trim().parse().expect("Please type a number!");
    println!("Is prime number: {}", DSA::is_prime(input));
}
