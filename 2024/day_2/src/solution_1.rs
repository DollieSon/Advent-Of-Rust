use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub fn solution() -> i32 {
    let mut count = 0;
    let parsed = parse_file();
    parsed.iter().for_each(|level| {
        if verify_level(level) {
            count += 1;
        }
    });
    return count;
}
pub fn solution_debuged() -> Vec<bool> {
    let mut count = Vec::new();
    let parsed = parse_file();
    parsed.iter().for_each(|line| {
        count.push(verify_level(line));
    });
    return count;
}
fn parse_file() -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        res.push(parse_line(&line.unwrap()));
    });
    res
}

pub fn parse_line(line: &String) -> Vec<i32> {
    let levels = line.split_ascii_whitespace();
    let mut level_vec = Vec::new();
    levels.for_each(|str| {
        level_vec.push(str.parse().unwrap());
    });
    level_vec
}

pub fn verify_level(level: &Vec<i32>) -> bool {
    let mut res = true;
    let mut is_decreasing: Option<bool> = None;
    level.windows(2).for_each(|wind| {
        let diff = (wind[0] - wind[1]);
        if diff.abs() > 3 || diff == 0 {
            res = false;
        }
        // increasing
        if diff < 0 {
            match is_decreasing {
                Some(val) => {
                    if val != false {
                        res = false
                    }
                }
                None => is_decreasing = Some(false),
            }
        } else {
            // decreasing
            match is_decreasing {
                Some(val) => {
                    if val != true {
                        res = false
                    }
                }
                None => is_decreasing = Some(true),
            }
        }
    });
    res
}
