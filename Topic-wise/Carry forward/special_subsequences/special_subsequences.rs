//    You have given a string having Uppercase English letters.
//    You have to find the number of pairs (i, j)
//    such that A[i] = 'A', A[j] = 'G' and i < j.

use std::io;

struct DSA;

impl DSA {
    pub fn special_subsequences(string: String) -> i64 {
        let mut count: i64 = 0;
        let mut res: i64 = 0;
        for char in string.chars() {
            count += (char == 'A' as char) as i64;
            res += (char == 'G' as char) as i64 * count;
        }
        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let string: String = input.trim().to_string();
    println!(
        "String {} contains {} AG pairs",
        string,
        DSA::special_subsequences(string.clone())
    );
}
