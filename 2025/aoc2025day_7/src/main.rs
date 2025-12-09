use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_file() -> Vec<String> {
    let mut res = Vec::new();

    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);
    
    let mut important = true;

    for line in bfread.lines() {
        if let Ok(word) = line{
            if important{
                res.push(word);
                important = false;
            }else {
                important = true;
            }
        }
    }
    return res;
}
fn main() {
    let lines = parse_file();
    let mut keeper = vec![false;lines[0].len()];
    let mut counter = 0;
    for line in lines{
        let mut curr = vec![false;keeper.len()];
        for (ind,ch) in line.chars().enumerate(){
            match ch {
                'S' => {
                    curr[ind] = true;
                }
                '^' => {
                    if keeper[ind] == true{
                        counter+=1;
                        if let Some(x) = curr.get_mut(ind+1){
                            *x = true;
                        }
                        if let Some(x) = curr.get_mut(ind-1){
                            *x = true;
                        }
                    }
                }
                _ => {
                    if curr[ind] == false{
                        curr[ind] = keeper[ind];
                    }
                    // not my problem hehe
                }
            }
        }
        keeper = curr;
    }
    let res = keeper.iter().filter(|x| **x == true).count();
    println!("{res},{counter}");
}
