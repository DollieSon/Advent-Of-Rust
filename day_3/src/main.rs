use Solution_1::solution;

fn main() {
    println!("Hello, world!");
    solution();
}
// S1 -> find "Mult"
// S2 -> check if "(" is next to mult
// S3 -> store next characters till ")"
// S4 -> Check if S3's output is valid
pub mod Solution_1 {
    use std::{
        fs::{File, read_to_string},
        io::{BufRead, BufReader, Read},
    };

    use regex::Regex;

    enum Stage {
        MULTSEARCH,
        FINDOPEN,
        STORESTRING,
        VALIDATION,
    }
    pub fn solution() {
        let whole_string = read_to_string("input.txt").unwrap();
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut res = 0;
        for (line, [d1, d2]) in re.captures_iter(&whole_string).map(|cap| cap.extract()) {
            let num1: i128 = d1.parse().unwrap();
            let num2: i128 = d2.parse().unwrap();
            res += num1 * num2;
        }
        println!("{res}");
    }
}
