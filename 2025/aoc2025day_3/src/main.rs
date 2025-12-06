use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_file() -> Vec<String> {
    let mut res = Vec::new();

    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);

    for line in bfread.lines() {
        if let Ok(word) = line{
            res.push(word);
        }
    }
    return res;
}

// num & relative_pos
fn get_max(arr:&[i32]) -> (i32,usize){
    let mut max = (arr[0],0);
    for (ind,item) in arr.iter().enumerate(){
        if max.0 < *item{
            max = (*item,ind)
        }
    }
    return max;
}

fn main() {
    let battery = parse_file();
    let mut sum = 0;
    for line in battery{
        let nums:Vec<i32> = line.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
        let mut first_max = get_max(&nums);
        let mut second_max:(i32,usize) = (0,0);
        if first_max.1 == nums.len()-1{
            let temp_max = get_max(&nums[..(first_max.1)]);
            second_max = first_max;
            first_max = temp_max;
        }else {
            second_max = get_max(&nums[(first_max.1+1)..]);
        }
        let num = first_max.0 * 10 + second_max.0;
        println!("{num}");
        sum += num;
    }
    println!("{sum}");
}
