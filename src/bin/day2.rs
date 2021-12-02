use advent_of_code_2021::solver::{solve_file, Solver};

struct Day2Solver {
    commands: Vec<Command>,
}

#[derive(Clone, Copy)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Solver<i64> for Day2Solver {
    fn new(problem: &str) -> Self {
        Day2Solver {
            commands: problem
                .split("\n")
                .map(|line| {
                    let (command, amount) = line.split_once(" ").unwrap();
                    let amount = amount.parse::<i64>().unwrap();
                    match command {
                        "forward" => Command::Forward(amount),
                        "down" => Command::Down(amount),
                        "up" => Command::Up(amount),
                        _ => panic!("unexpected command"),
                    }
                })
                .collect(),
        }
    }

    fn solve1(&self) -> Option<i64> {
        let (mut x, mut y) = (0, 0);

        for command in &self.commands {
            match command {
                Command::Forward(n) => x += n,
                Command::Down(n) => y += n,
                Command::Up(n) => y -= n,
            }
        }

        Some(x * y)
    }

    fn solve2(&self) -> Option<i64> {
        let (mut x, mut y) = (0, 0);
        let mut aim = 0;

        for command in &self.commands {
            match command {
                Command::Forward(n) => {
                    x += n;
                    y += aim * n;
                }
                Command::Down(n) => aim += n,
                Command::Up(n) => aim -= n,
            }
        }

        Some(x * y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test_example_part1() {
        let solver = Day2Solver {
            commands: EXAMPLE.to_vec(),
        };
        assert_eq!(solver.solve1(), Some(150));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day2Solver {
            commands: EXAMPLE.to_vec(),
        };
        assert_eq!(solver.solve2(), Some(900));
    }
}

fn main() {
    solve_file::<Day2Solver, i64>("day2.txt");
}
