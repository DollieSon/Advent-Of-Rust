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

fn populate_directions(arr: &mut Vec<Vec<i32>>,ind:(usize,usize)){
    if let Some(x)= arr.get_mut(ind.0){
        if let Some(inside) = x.get_mut(ind.1){
            *inside +=1;
        }
    }
}


fn main() {
    let directions = [
        (-1,-1),
        (-1,0),
        (-1,1),
        (1,1),
        (1,0),
        (1,-1),
        (0,-1),
        (0,1),
    ];
    let paper = parse_file();
    let inside_len = paper[0].len();
    let mut grid = vec![vec![0;inside_len];paper.len()];
    for (ind_x,lane) in paper.iter().enumerate(){
        for (ind_y,ch) in lane.chars().enumerate(){
            if ch == '@' {
                for dir in directions{
                    let op = ind_x as i32 + dir.0;
                    if op < 0 {
                        continue;
                    }
                    let op_2 = ind_y as i32 + dir.1;
                    if op_2 < 0 {
                        continue;
                    }
                    populate_directions(&mut grid,(op as usize,op_2 as usize));
                }
            }
        }
    }

    //print
    // for lane in grid{
    //     println!("{:?}",lane);
    // }
    let mut count = 0;
    for (ind_x,lane) in paper.iter().enumerate(){
        for (ind_y,ch) in lane.chars().enumerate(){
            let thing =grid.get(ind_x).unwrap().get(ind_y).unwrap();
            if ch == '@' && (*thing < 4){
                count+=1;
            }
        }
    }
    println!("{count}");
}
