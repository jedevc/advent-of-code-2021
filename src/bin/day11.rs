use advent_of_code_2021::solver::{solve_file, Solver};

struct Day11Solver {
    octopi: Octopi,
}

impl Solver<u64> for Day11Solver {
    fn new(problem: &str) -> Self {
        let data: Vec<Vec<i8>> = problem
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i8)
                    .collect()
            })
            .collect();
        let width = data.len();
        let height = data[0].len();

        let octopi = Octopi {
            data,
            width,
            height,
        };
        Self { octopi }
    }

    fn solve1(&self) -> Option<u64> {
        let mut octopi = self.octopi.clone();
        let mut flashes = 0;

        for _ in 0..100 {
            flashes += octopi.step();
        }
        Some(flashes)
    }

    fn solve2(&self) -> Option<u64> {
        let mut octopi = self.octopi.clone();

        let mut generations = 1;
        while octopi.step() as usize != octopi.width * octopi.height {
            generations += 1;
        }
        Some(generations)
    }
}

#[derive(Clone)]
struct Octopi {
    data: Vec<Vec<i8>>,
    width: usize,
    height: usize,
}

impl Octopi {
    fn step(&mut self) -> u64 {
        let mut flashes = 0;

        // increase all by 1
        for x in 0..self.width {
            for y in 0..self.height {
                self.data[x][y] += 1;
            }
        }

        // flashing
        loop {
            let mut have_flashed = false;

            for x in 0..self.width {
                for y in 0..self.height {
                    if self.data[x][y] > 9 {
                        self.data[x][y] = -1;
                        have_flashed = true;

                        for dx in -1..2 {
                            for dy in -1..2 {
                                if dx == 0 && dy == 0 {
                                    continue;
                                }

                                let (nx, ny) = (x as isize + dx, y as isize + dy);
                                if nx >= 0
                                    && nx < self.width as isize
                                    && ny >= 0
                                    && ny < self.height as isize
                                {
                                    let (nx, ny) = (nx as usize, ny as usize);
                                    if self.data[nx][ny] >= 0 && self.data[nx][ny] <= 9 {
                                        self.data[nx][ny] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !have_flashed {
                break;
            }
        }

        // count flashes
        for x in 0..self.width {
            for y in 0..self.height {
                if self.data[x][y] == -1 {
                    self.data[x][y] = 0;
                    flashes += 1;
                }
            }
        }

        flashes
    }
}

#[cfg(test)]
mod day11tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day11Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(1656));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day11Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(195));
    }
}

fn main() {
    solve_file::<Day11Solver, u64>("day11.txt");
}
