use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

use indoc::indoc;

use advent_of_code_2021::solver::{solve_file, Solver};

struct Day23Solver {
    problem: String,
}

static TARGET: &'static str = indoc!(
    "
    #############
    #...........#
    ###A#B#C#D###
      #A#B#C#D#
      #########
"
);

static TARGET2: &'static str = indoc!(
    "
    #############
    #...........#
    ###A#B#C#D###
      #A#B#C#D#
      #A#B#C#D#
      #A#B#C#D#
      #########
"
);

impl Solver<u64> for Day23Solver {
    fn new(problem: &str) -> Self {
        Self {
            problem: problem.to_owned(),
        }
    }

    fn solve1(&self) -> Option<u64> {
        let target = Map::parse(TARGET);
        let map = Map::parse(&self.problem);

        // let state = State::new(map, &target);
        // let state = &state.expand()[0];
        // for i in state.expand() {
        //     println!("{}", i.map);
        // }
        // None

        let state = self.solve(&map, &target).unwrap();
        Some(state.cost as u64)
    }

    fn solve2(&self) -> Option<u64> {
        let target = Map::parse(TARGET2);

        let mut lines: Vec<&str> = self.problem.lines().collect();
        lines.insert(3, "  #D#C#B#A#");
        lines.insert(4, "  #D#B#A#C#");
        let problem = lines.join("\n");

        let map = Map::parse(&problem);

        let state = self.solve(&map, &target).unwrap();
        Some(state.cost as u64)
    }
}

impl Day23Solver {
    fn solve<'a>(&self, map: &'a Map, target: &'a Map) -> Option<State<'a>> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();
        candidates.push(State::new(map.clone(), target));

        while let Some(state) = candidates.pop() {
            if visited.contains(&state.map) {
                continue;
            }

            if &state.map == target {
                return Some(state);
            }

            for next in state.expand() {
                candidates.push(next);
            }
            visited.insert(state.map);
        }

        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Pod {
    A,
    B,
    C,
    D,
    Wall,
    Empty,
}

impl Pod {
    fn is_pod(&self) -> bool {
        match self {
            Pod::A => true,
            Pod::B => true,
            Pod::C => true,
            Pod::D => true,
            _ => false,
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Pod::A => 1,
            Pod::B => 10,
            Pod::C => 100,
            Pod::D => 1000,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Pod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pod::A => write!(f, "A"),
            Pod::B => write!(f, "B"),
            Pod::C => write!(f, "C"),
            Pod::D => write!(f, "D"),
            Pod::Wall => write!(f, "#"),
            Pod::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    width: usize,
    height: usize,
    basis: Vec<Pod>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                write!(f, "{}", self.get(j, i))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn parse(data: &str) -> Self {
        let data = data.trim();

        let width = data.find('\n').unwrap();
        let height = data.chars().filter(|ch| *ch == '\n').count() + 1;

        let data: Vec<_> = data
            .split("\n")
            .flat_map(|line| {
                let mut line: Vec<Pod> = line
                    .chars()
                    .map(|ch| match ch {
                        'A' => Pod::A,
                        'B' => Pod::B,
                        'C' => Pod::C,
                        'D' => Pod::D,
                        '#' => Pod::Wall,
                        ' ' | '.' => Pod::Empty,
                        _ => unreachable!(),
                    })
                    .collect();
                while line.len() < width {
                    line.push(Pod::Empty);
                }
                line
            })
            .collect();

        Self {
            basis: data,
            width,
            height,
        }
    }

    fn get(&self, y: usize, x: usize) -> Pod {
        let y = y * self.width;
        self.basis[x + y]
    }

    fn set(&mut self, y: usize, x: usize, pod: Pod) {
        let y = y * self.width;
        self.basis[x + y] = pod;
    }

    fn empty(&mut self, y: usize, x: usize) {
        self.set(y, x, Pod::Empty);
    }
}

#[derive(PartialEq, Eq)]
struct State<'a> {
    map: Map,
    cost: u32,
    target: &'a Map,
}

impl<'a> State<'a> {
    fn new(map: Map, target: &'a Map) -> State<'a> {
        Self {
            map,
            target,
            cost: 0,
        }
    }

    fn expand(&self) -> Vec<State<'a>> {
        let mut states = vec![];

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let pod = self.map.get(y, x);
                if pod.is_pod() {
                    // find all reachable locations
                    let mut visited = HashMap::new();
                    let mut stack = vec![(y, x, 0)];
                    while stack.len() > 0 {
                        let (j, i, dist) = stack.pop().unwrap();
                        if let Some(old_dist) = visited.get(&(j, i)) {
                            if *old_dist < dist {
                                continue;
                            }
                        }
                        visited.insert((j, i), dist);
                        if self.map.get(j + 1, i) == Pod::Empty {
                            stack.push((j + 1, i, dist + 1));
                        }
                        if self.map.get(j - 1, i) == Pod::Empty {
                            stack.push((j - 1, i, dist + 1));
                        }
                        if self.map.get(j, i + 1) == Pod::Empty {
                            stack.push((j, i + 1, dist + 1));
                        }
                        if self.map.get(j, i - 1) == Pod::Empty {
                            stack.push((j, i - 1, dist + 1));
                        }
                    }
                    visited.remove(&(y, x));

                    for ((j, i), dist) in visited {
                        if self.map.get(y - 1, x) == Pod::Wall {
                            // pod in the corridor...
                            if pod != self.target.get(j, i) {
                                // ...cannot end up in not it's room
                                continue;
                            }
                        } else if self.target.get(j, i) != Pod::Empty {
                            // all other pods going into a room...
                            if pod != self.target.get(j, i) {
                                // ...cannot end up in not it's room
                                continue;
                            }
                        }

                        // pods cannot share types
                        if self.map.get(j + 1, i).is_pod() && pod != self.map.get(j + 1, i) {
                            continue;
                        }

                        // pod cannot end up in room entrance
                        if self.map.get(j - 1, i) == Pod::Wall
                            && self.map.get(j + 1, i) != Pod::Wall
                        {
                            continue;
                        }

                        let mut map = self.map.clone();
                        map.set(j, i, pod);
                        map.empty(y, x);
                        let cost = self.cost + dist * pod.cost();
                        states.push(State {
                            map,
                            cost,
                            target: self.target,
                        })
                    }
                }
            }
        }
        states
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod day23tests {
    use super::*;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
        "
    );

    #[test]
    fn test_parse() {
        let map = Map::parse(EXAMPLE_DATA.trim());
        assert_eq!(map.get(2, 3), Pod::B);
        assert_eq!(map.get(3, 3), Pod::A);
        assert_eq!(map.get(2, 5), Pod::C);
        assert_eq!(map.get(3, 5), Pod::D);
        assert_eq!(map.get(2, 7), Pod::B);
        assert_eq!(map.get(3, 7), Pod::C);
        assert_eq!(map.get(2, 9), Pod::D);
        assert_eq!(map.get(3, 9), Pod::A);
    }

    #[test]
    fn test_example_part1() {
        let solver = Day23Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(12521));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day23Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(44169));
    }
}

fn main() {
    solve_file::<Day23Solver, u64>("day23.txt");
}
