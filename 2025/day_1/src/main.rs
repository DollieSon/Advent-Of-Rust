use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Direction{
    Left,
    Right
}
struct Instruction{
    pub dir: Direction,
    pub angle: i64
}

fn parse_file() -> Vec<Instruction> {
    let mut res = Vec::new();

    let file = File::open("input.txt").unwrap();
    let bfread = BufReader::new(file);

    for line in bfread.lines() {
        if let Ok(word) = line{
            let iter = word.as_str();
            let number = &iter[1..].parse::<i64>().unwrap();
            let dir_c = &iter[..=0];
            let mut dir = Direction::Right;
            if dir_c.contains('L'){
                dir = Direction::Left;
            }
            res.push(Instruction {
                dir: dir,
                angle: *number
            });
        }
        // println!("{:?}", line.unwrap());
    }
    return res;
}

fn main() {
    let mut angle:i64 = 50;
    let mut counter = 0;
    let ins = parse_file();
    for x in ins {
        println!("d -{:?} ,a - {}",x.dir,x.angle);
        match x.dir{
            Direction::Left => {
                angle += x.angle;
            }
            Direction::Right => {
                angle -= x.angle;
            }
        }
        println!("angle = {} , {}",angle, angle%100);
        angle%=100;
        if angle == 0 {
            counter += 1;
        }
    }
    println!("counter={}",counter);
}
