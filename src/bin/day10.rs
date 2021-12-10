use advent_of_code_2021::solver::{solve_file, Solver};

struct Day10Solver {
    lines: Vec<Line>,
}

impl Solver<u64> for Day10Solver {
    fn new(problem: &str) -> Self {
        let lines = problem.split("\n").map(Line::new).collect();
        Self { lines }
    }

    fn solve1(&self) -> Option<u64> {
        let score = self
            .lines
            .iter()
            .map(|line| {
                if let Some((_, chunk)) = line.find_error() {
                    chunk.score_error()
                } else {
                    0
                }
            })
            .sum();
        Some(score)
    }

    fn solve2(&self) -> Option<u64> {
        let mut scores: Vec<u64> = self
            .lines
            .iter()
            .map(|line| {
                if let Some(completions) = line.autocomplete() {
                    completions
                        .iter()
                        .fold(0, |acc, chunk| acc * 5 + chunk.score_complete())
                } else {
                    0
                }
            })
            .filter(|score| *score != 0)
            .collect();
        scores.sort();
        Some(scores[scores.len() / 2])
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ChunkType {
    Round,
    Square,
    Curly,
    Angle,
}

impl ChunkType {
    fn score_error(&self) -> u64 {
        match self {
            ChunkType::Round => 3,
            ChunkType::Square => 57,
            ChunkType::Curly => 1197,
            ChunkType::Angle => 25137,
        }
    }

    fn score_complete(&self) -> u64 {
        match self {
            ChunkType::Round => 1,
            ChunkType::Square => 2,
            ChunkType::Curly => 3,
            ChunkType::Angle => 4,
        }
    }
}

#[derive(Clone, Copy)]
enum Chunk {
    Open(ChunkType),
    Close(ChunkType),
}

struct Line {
    chunks: Vec<Chunk>,
}

impl Line {
    fn new(line: &str) -> Self {
        let chunks = line
            .chars()
            .map(|ch| match ch {
                '(' => Chunk::Open(ChunkType::Round),
                ')' => Chunk::Close(ChunkType::Round),
                '[' => Chunk::Open(ChunkType::Square),
                ']' => Chunk::Close(ChunkType::Square),
                '{' => Chunk::Open(ChunkType::Curly),
                '}' => Chunk::Close(ChunkType::Curly),
                '<' => Chunk::Open(ChunkType::Angle),
                '>' => Chunk::Close(ChunkType::Angle),
                _ => panic!("invalid chunk"),
            })
            .collect();
        Self { chunks }
    }

    fn find_error(&self) -> Option<(usize, ChunkType)> {
        let mut stack = vec![];

        for (i, chunk) in self.chunks.iter().enumerate() {
            match chunk {
                Chunk::Open(chunk_type) => stack.push(*chunk_type),
                Chunk::Close(chunk_type) => {
                    if stack.pop() != Some(*chunk_type) {
                        return Some((i, *chunk_type));
                    }
                }
            }
        }

        None
    }

    fn autocomplete(&self) -> Option<Vec<ChunkType>> {
        let mut stack = vec![];

        for chunk in self.chunks.iter() {
            match chunk {
                Chunk::Open(chunk_type) => stack.push(*chunk_type),
                Chunk::Close(chunk_type) => {
                    if stack.pop() != Some(*chunk_type) {
                        return None;
                    }
                }
            }
        }
        stack.reverse();
        Some(stack)
    }
}

#[cfg(test)]
mod day10tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_DATA: &'static str = indoc!(
        "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "
    );

    #[test]
    fn test_example_part1() {
        let solver = Day10Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve1(), Some(26397));
    }

    #[test]
    fn test_example_part2() {
        let solver = Day10Solver::new(EXAMPLE_DATA.trim());
        assert_eq!(solver.solve2(), Some(288957));
    }
}

fn main() {
    solve_file::<Day10Solver, u64>("day10.txt");
}
