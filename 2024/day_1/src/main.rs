use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (mut l1, mut l2) = parse_file();
    l1.sort();
    l2.sort();
    let differences = get_differences(l1, l2);
    let sum: u128 = differences.iter().sum();
    println!("{}", sum);
}
fn parse_file() -> (Vec<i128>, Vec<i128>) {
    let mut list1 = Vec::<i128>::new();
    let mut list2 = Vec::<i128>::new();
    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);
    for line in bfread.lines() {
        // println!("{:?}", line.unwrap());
        let holder = line.unwrap();
        let mut pair_string = holder.split_ascii_whitespace();
        let num1 = pair_string.next().unwrap().parse::<i128>().unwrap();
        let num2 = pair_string.next().unwrap().parse::<i128>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    (list1, list2)
}

fn get_differences(l1: Vec<i128>, l2: Vec<i128>) -> Vec<u128> {
    let mut difference = Vec::new();
    for (ind, num) in l1.iter().enumerate() {
        let num2 = *l2.get(ind).unwrap();
        let mut diff = 0;
        if (*num >= num2) {
            diff = *num - num2;
        } else {
            diff = num2 - *num;
        }

        difference.push(diff as u128);
    }
    difference
}
