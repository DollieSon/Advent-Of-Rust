use std::{
    fs::File,
    io::{BufRead, BufReader, Split},
};

use num_bigint::BigUint;
#[derive(Debug)]
enum Op{
    MULT,
    ADD
}

fn parse_file() -> (Vec<Vec<u64>>,Vec<Op>) {
    let mut res = Vec::new();
    let mut res2 = Vec::new();

    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);

    for line in bfread.lines() {
        let mut holder = Vec::new();
        if let Ok(word) = line{
            let split:Vec<&str> = word.split(" ").collect();
            for ch in split {
                match ch {
                    "*" => {
                        res2.push(Op::MULT);
                    }
                    "+" => {
                        res2.push(Op::ADD);
                    }
                    "" => {
                        // not my prob just catching
                    }
                    _ => {
                        // println!("_`{ch}_");
                        holder.push(ch.parse().unwrap());
                    }
                }
            }
        }
        if holder.len() > 0 {
            res.push(holder);
        }
    }
    return (res,res2);
}


fn main() {
    let (nums,ops) = parse_file();
    let mut sums:Vec<BigUint> = Vec::new();

    for (ind,op) in ops.iter().enumerate(){
        for thing in &nums {
            let second:BigUint = thing[ind].into();
            if sums.len() < (ind + 1) {
                sums.push(second);
                continue;
            }
            match op {
                Op::ADD => {
                    sums[ind]+= second;
                }
                Op::MULT => {
                    sums[ind] *= second;
                }
            }
        }
    }
    let sum:BigUint = sums.iter().sum();
    println!("{:?}",sum);
}
