use std::collections::{HashMap, HashSet};

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day20Solver {
    table: Vec<Pixel>,
    image: Image,
}

#[derive(PartialEq, Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

impl Solver<u64> for Day20Solver {
    fn new(problem: &str) -> Self {
        let (table, image) = problem.split_once("\n\n").unwrap();
        let table = table
            .chars()
            .filter_map(|ch| match ch {
                '#' => Some(Pixel::Light),
                '.' => Some(Pixel::Dark),
                '\n' => None, // ignore new lines in table
                _ => unreachable!(),
            })
            .collect();
        let image = Image::parse(image);
        Self { table, image }
    }

    fn solve1(&self) -> Option<u64> {
        let image = (0..2).fold(self.image.clone(), |image, _| image.transform(&self.table));
        Some(image.pixels.len() as u64)
    }

    fn solve2(&self) -> Option<u64> {
        let image = (0..50).fold(self.image.clone(), |image, _| image.transform(&self.table));
        Some(image.pixels.len() as u64)
    }
}

#[derive(Clone)]
struct Image {
    pixels: HashSet<(isize, isize)>,
    pixel_type: Pixel,
    top_left: (isize, isize),
    bottom_right: (isize, isize),
}

impl Image {
    fn parse(data: &str) -> Self {
        let lines: Vec<_> = data.split("\n").collect();
        let pixels = lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(j, ch)| match ch {
                        '#' => Some((i as isize, j as isize)),
                        '.' => None,
                        _ => unreachable!(),
                    })
            })
            .collect();

        Self {
            pixels,
            pixel_type: Pixel::Light,
            top_left: (0, 0),
            bottom_right: (lines.len() as isize - 1, lines[0].len() as isize - 1),
        }
    }

    fn transform(&self, table: &[Pixel]) -> Self {
        let mut next = HashSet::new();
        let next_type = if self.pixel_type == Pixel::Light && table[0] == Pixel::Light {
            Pixel::Dark
        } else if self.pixel_type == Pixel::Dark && table[table.len() - 1] == Pixel::Dark {
            Pixel::Light
        } else {
            self.pixel_type
        };

        let (x1, y1) = self.top_left;
        let (x2, y2) = self.bottom_right;
        for x in x1 - 2..=x2 + 2 {
            for y in y1 - 2..=y2 + 2 {
                let mut key = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        let is_light = match self.pixel_type {
                            Pixel::Light => self.pixels.contains(&(x + i, y + j)),
                            Pixel::Dark => !self.pixels.contains(&(x + i, y + j)),
                        };

                        key <<= 1;
                        if is_light {
                            key |= 1;
                        }
                    }
                }

                if table[key] == next_type {
                    next.insert((x, y));
                }
            }
        }

        Self {
            pixels: next,
            pixel_type: next_type,
            top_left: (x1 - 2, y1 - 2),
            bottom_right: (x2 + 2, y2 + 2),
        }
    }
}

#[cfg(test)]
mod day20tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        
        #..#.
        #....
        ##..#
        ..#..
        ..###
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day20Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(35));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day20Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(3351));
    }
}

fn main() {
    solve_file::<Day20Solver, u64>("day20.txt");
}
