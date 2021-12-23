use std::fmt;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};

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
        Self { problem: problem.to_owned() }
    }

    fn solve1(&self) -> Option<u64> {
        let target = Map::parse(TARGET);
        let map = Map::parse(&self.problem);

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
            println!("{}", state.cost);

            if &state.map == target {
                return Some(state)
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
    data: Vec<Vec<Pod>>,
}

impl Map {
    fn parse(data: &str) -> Self {
        let data = data
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        'A' => Pod::A,
                        'B' => Pod::B,
                        'C' => Pod::C,
                        'D' => Pod::D,
                        '#' => Pod::Wall,
                        ' ' | '.' => Pod::Empty,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Self { data }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.data {
            for pod in line {
                write!(f, "{}", pod)?;
            }
            writeln!(f)?;
        }
        Ok(())
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
        for x in 0..self.map.data.len() {
            for y in 0..self.map.data[x].len() {
                if self.map.data[x][y].is_pod() {
                    // find all reachable locations
                    let mut visited = HashMap::new();
                    let mut stack = vec![((x, y), 0)];
                    while stack.len() > 0 {
                        let ((i, j), dist) = stack.pop().unwrap();
                        if let Some(old_dist) = visited.get(&(i, j)) {
                            if *old_dist < dist {
                                continue;
                            }
                        }
                        visited.insert((i, j), dist);
                        if self.map.data[i + 1][j] == Pod::Empty {
                            stack.push(((i + 1, j), dist + 1));
                        }
                        if self.map.data[i - 1][j] == Pod::Empty {
                            stack.push(((i - 1, j), dist + 1));
                        }
                        if self.map.data[i][j + 1] == Pod::Empty {
                            stack.push(((i, j + 1), dist + 1));
                        }
                        if self.map.data[i][j - 1] == Pod::Empty {
                            stack.push(((i, j - 1), dist + 1));
                        }
                    }
                    visited.remove(&(x, y));

                    for ((i, j), dist) in visited {
                        if self.map.data[x - 1][y] == Pod::Wall {
                            // pod in the corridor...
                            if self.map.data[x][y] != self.target.data[i][j] {
                                // ...cannot end up in not it's room
                                continue;
                            }
                        } else if self.target.data[i][j] != Pod::Empty {
                            // all other pods going into a room...
                            if self.map.data[x][y] != self.target.data[i][j] {
                                // ...cannot end up in not it's room
                                continue;
                            }
                        }

                        // pods cannot share types
                        if self.map.data[i + 1][j].is_pod() && self.map.data[x][y] != self.map.data[i + 1][j] {
                            continue;
                        }

                        // pod cannot end up in room entrance
                        if self.map.data[i - 1][j] == Pod::Wall
                            && self.map.data[i + 1][j] != Pod::Wall
                        {
                            continue;
                        }

                        let mut map = self.map.clone();
                        map.data[i][j] = self.map.data[x][y];
                        map.data[x][y] = Pod::Empty;
                        let cost = self.cost + dist * map.data[i][j].cost();
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
        assert_eq!(map.data[2][3], Pod::B);
        assert_eq!(map.data[3][3], Pod::A);
        assert_eq!(map.data[2][5], Pod::C);
        assert_eq!(map.data[3][5], Pod::D);
        assert_eq!(map.data[2][7], Pod::B);
        assert_eq!(map.data[3][7], Pod::C);
        assert_eq!(map.data[2][9], Pod::D);
        assert_eq!(map.data[3][9], Pod::A);
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
