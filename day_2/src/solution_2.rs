use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(PartialEq, Eq)]
enum Eval {
    DECREASING,
    INCREASING,
    INVALID,
}
impl Eval {
    pub fn to_decreasing(self) -> Result<bool, &'static str> {
        match self {
            Eval::DECREASING => Ok(true),
            Eval::INCREASING => Ok(false),
            _ => Err("Cannot Be Converted"),
        }
    }
}
pub fn solution_debuged() -> Vec<(bool, String)> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count = Vec::new();
    for line in reader.lines() {
        let line_p = line.unwrap();
        let res = parse_line(&line_p);
        count.push((res, line_p));
    }
    count
}
pub fn solution() -> i32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let res = parse_line(&line.unwrap());
        if res {
            count += 1;
        }
    }
    count
}
fn parse_line(line: &String) -> bool {
    let levels = line.split_ascii_whitespace();
    let mut prev_num: Option<i32> = None;
    let mut is_descending: Option<bool> = None;
    for str in levels {
        let num: i32 = str.parse().unwrap();
        if prev_num == None {
            prev_num = Some(num);
            continue;
        }
        let val = prev_num.unwrap();
        let diff_eval = evaluate(num, val);
        if diff_eval == Eval::INVALID {
            return false;
        }
        if is_descending == None {
            is_descending = Some(diff_eval.to_decreasing().unwrap());
        } else {
            if is_descending.unwrap() != diff_eval.to_decreasing().unwrap() {
                return false;
            }
        }
        prev_num = Some(num);
    }
    return true;
}
fn evaluate(curr: i32, prev: i32) -> Eval {
    let diff = curr - prev;
    if diff.abs() > 3 {
        Eval::INVALID
    } else if diff > 0 {
        Eval::DECREASING
    } else if diff == 0 {
        Eval::INVALID
    } else {
        Eval::INCREASING
    }
}
