use std::fmt::Display;
use std::fs;
use std::path::Path;

pub trait Solver<T> {
    fn new(problem: &str) -> Self;
    fn solve1(&self) -> Option<T>;
    fn solve2(&self) -> Option<T>;
}

pub fn solve<S: Solver<T>, T: Display>(problem: &str) {
    let solver = S::new(problem);
    if let Some(result1) = solver.solve1() {
        println!("part 1: {}", result1);
    }
    if let Some(result2) = solver.solve2() {
        println!("part 2: {}", result2);
    }
}

pub fn solve_file<S: Solver<T>, T: Display>(filename: &str) {
    let filename = Path::new("resources").join(filename);
    let problem = fs::read_to_string(filename).unwrap();
    let problem = problem.trim();
    solve::<S, T>(&problem);
}
