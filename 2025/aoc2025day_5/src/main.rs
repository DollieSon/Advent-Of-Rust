/// THIS IS A DISGUSTING SOLVE, SAVED BY THE SPEED OF RUST
/// WTF
/// PLEASE IMPLEMENT A Binary Search Or Clean the Range


use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug)]
struct IDRange{
    start:u128,
    end:u128
}

fn parse_file() -> (Vec<IDRange>,Vec<u128>) {
    let mut res = Vec::new();
    let mut res2 = Vec::new();

    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);

    let mut on_range_read = true;

    for line in bfread.lines() {
        if let Ok(word) = line{
            if word.is_empty(){
                on_range_read =false;
                continue;
            }
            if on_range_read == true{
                let splitted:Vec<&str> = word.split('-').collect();
                // println!("{:?}",splitted);
                res.push(IDRange { 
                    start: splitted[0].parse().unwrap(), 
                    end: splitted[1].parse().unwrap() 
                });
            }else{
                res2.push(word.parse().unwrap());
            }
        }
    }
    res.sort_by_key(|x| x.start);
    return (res,res2);
}

fn clean_range(range:Vec<IDRange>) -> Vec<IDRange>{
    let mut res_range = Vec::new();
    let mut prev_range:Option<IDRange> = None;
    for cur_range in range {
        match prev_range.take() {
            None => {
                prev_range = Some(cur_range);
            }
            Some(r) => {
                if r.end > cur_range.start{
                    prev_range = Some(IDRange { 
                        start: r.start, 
                        end: {
                            if r.end > cur_range.end {
                                r.end
                            }else {
                                cur_range.end
                            }
                        }
                     });
                }else{
                    res_range.push(r);
                    prev_range = Some(cur_range);
                }
            }
        }
    }
    match prev_range.take() {
        Some(x) => {
            res_range.push(x);
        }   
        _ => {
            // Not my problem
        }
    }
    return res_range;   
}

enum CompRes{
    LESS,
    INSIDE,
    GREATER,
}

impl IDRange{
    pub fn compare(&self,num:u128)-> CompRes{
        if num >= self.start && num <= self.end{
            return CompRes::INSIDE
        }else if num < self.start {
            return CompRes::LESS
        }else {
            return CompRes::GREATER
        }
    }
}
fn main() {
    let (mut ranges,IDs) = parse_file();
    println!("{:?}",ranges.len());
    ranges = clean_range(ranges);
    println!("{:?}",ranges.len());
    // println!("{:?}",IDs);

    let mut counter = 0;
    for Id in IDs{
        for range in &ranges{
            match range.compare(Id){
                CompRes::INSIDE => {
                    counter+=1;
                    break;
                }
                CompRes::LESS => {
                    break;
                }
                CompRes::GREATER => {
                    // not my problem just move
                }
            }
        }
    }
    println!("{counter}");
}
