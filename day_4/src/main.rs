use Solution_1::solution;

fn main() {
    println!("Hello, world!");
    solution();
}

pub mod Solution_1 {
    use std::{
        fs::read_to_string,
        io::{self, Write},
        os::windows,
    };

    fn str_to_vec(str: &String) -> Vec<Vec<char>> {
        str.lines().map(|line| line.chars().collect()).collect()
    }
    fn checked_man(opp1: usize, opp2: i32, bound: usize) -> Option<usize> {
        if opp2.is_negative() {
            return opp1.checked_sub(opp2.abs() as usize);
        } else {
            let res = opp1.checked_add(opp2 as usize);
            match res {
                Some(opp_res) => {
                    if opp_res >= bound {
                        return None;
                    } else {
                        return Some(opp_res);
                    }
                }
                None => return None,
            }
        }
    }
    fn is_mas(vec: &Vec<Vec<char>>, dir: [(i32, i32); 3], coords: (usize, usize)) -> bool {
        let letter = "SAM";
        let bounds: (usize, usize) = (vec.get(0).unwrap().len(), vec.len());
        for (ind, (temp_x, temp_y)) in dir.iter().enumerate() {
            //check if not negative
            let (x, y) = (
                checked_man(coords.0, *temp_x, bounds.0),
                checked_man(coords.1, *temp_y, bounds.1),
            );
            match (x, y) {
                // check
                (Some(x1), Some(y1)) => {
                    // println!("checking {x1} {y1}");
                    let ch = vec.get(y1).unwrap().get(x1).unwrap();
                    if ch.eq_ignore_ascii_case(&letter.chars().nth(ind).unwrap()) == false {
                        // println!("Wrong Letter");
                        return false;
                    }
                }
                (_, _) => {
                    // println!("Out_Of_Bounds");
                    return false;
                }
            }
            //check if
        }
        return true;
    }
    fn count_xmas(index: (usize, usize), vec: &Vec<Vec<char>>) -> u32 {
        //first check is if index == "X";
        if let Some(v1) = vec.get(index.1) {
            if let Some(ch) = v1.get(index.0) {
                if *ch != 'X' {
                    return 0;
                }
            }
        } else {
            return 0;
        }
        //searching by SAM
        let directions: [[(i32, i32); 3]; 8] = [
            [(-3, 3), (-2, 2), (-1, 1)],
            [(0, 3), (0, 2), (0, 1)],
            [(3, 3), (2, 2), (1, 1)],
            [(-3, 0), (-2, 0), (-1, 0)],
            [(-3, -3), (-2, -2), (-1, -1)],
            [(0, -3), (0, -2), (0, -1)],
            [(3, -3), (2, -2), (1, -1)],
            [(3, 0), (2, 0), (1, 0)],
        ];
        let dir_name = ["UL", "U", "UR", "L", "DL", "D", "DR", "R"];
        let mut count: u32 = 0;
        for (ind, coords) in directions.iter().enumerate() {
            // println!("Checking {} {:?}", dir_name[ind], coords);
            if is_mas(vec, *coords, index) {
                count += 1;
                println!("Found At: {} {:?}", dir_name[ind], coords);
                // println!("CORRECT!!");
            }
        }
        return count;
    }

    pub fn solution() {
        let input_string = read_to_string("input2.txt").unwrap();
        let input_arr = str_to_vec(&input_string);
        let height = input_arr.len();
        let width = input_arr.get(0).unwrap().len();
        let mut count: u32 = 0;
        for y in 0..height {
            // println!("On Line {y} ");
            for x in 0..width {
                println!(
                    "Checking {x} {y},{}",
                    input_arr.get(y).unwrap().get(x).unwrap()
                );
                count += count_xmas((x, y), &input_arr);
            }
            // print!("\n");
            // io::stdout().flush().unwrap();
        }
        println!("Xmases: {count}");
    }
}
