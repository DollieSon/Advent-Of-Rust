use std::{
    fs::File,
    io::{BufRead, BufReader},
};
struct sequence{
    start: u64,
    end: u64,
}

fn parse_file() -> Vec<sequence> {
    let mut res = Vec::new();

    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);

    for line in bfread.lines() {
        if let Ok(word) = line{
            let ranges :Vec<&str>= word.split(',').collect();
            for range in ranges{
                let vals:Vec<&str> = range.split('-').collect();
                res.push(
                    sequence { 
                        start: vals[0].parse().unwrap(),
                        end: vals[1].parse().unwrap()
                    }
                );
            }
        }
        // println!("{:?}", line.unwrap());
    }
    return res;
}

fn is_invalid(num:u64) -> bool{
    let thing = num.to_string();
    if thing.len()%2 == 0 {
        let len = thing.len();
        return thing[0..len/2] == thing[len/2..len];
    }
    // let ch = thing[..1].as_bytes();
    // let cmp = ch[0];
    // // println!("{}",cmp as char);
    // return thing.chars().into_iter().all(|x| x == cmp as char);
    return false;
}

fn main() {
    let mut total:u64 = 0;
    let seq = parse_file();
    for range in seq{
        for num in range.start..=range.end{
            if(is_invalid(num)){
                total+=num;
                println!("invalid = {}",num);
            }
        }
    }
    println!("{total}");
}
