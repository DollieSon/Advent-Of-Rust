const ANSWER: i32 = 472;
pub mod solution_1;
pub mod solution_2;
//answer 472
#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::ANSWER;
    use crate::solution_1::solution as sol1;
    use crate::solution_2::solution as sol2;
    #[test]
    fn solution_1() {
        println!("Starting solution 1");
        let start = Instant::now();
        assert_eq!(sol1(), ANSWER);
        let dur = start.elapsed();
        println!("Elapsed S1:{:.2?}", dur);
    }
    #[test]
    fn solution_2() {
        println!("Starting solution 2");
        let start = Instant::now();
        println!("Asnwer for sol 2: {}", sol2());
        assert_eq!(sol2(), ANSWER);
        let dur = start.elapsed();
        println!("Elapsed S2:{:.2?}", dur);
    }
}
fn main() {
    use crate::solution_1::solution_debuged as sol1;
    use crate::solution_2::solution_debuged as sol2;
    let res1 = sol1();
    let res2 = sol2();
    for (ind, item) in res1.iter().enumerate() {
        let item2 = res2.get(ind).unwrap();
        if *item != (*item2).0 {
            println!("{}, {} : {}", ind, (*item2).0, (*item2).1);
        }
    }
}
