use advent_of_code_2021::solver::{solve_file, Solver};

struct Day24Solver {
    evaluator: Evaluator,
}

impl Solver<u64> for Day24Solver {
    fn new(problem: &str) -> Self {
        Self { evaluator: Evaluator::new(problem) }
    }

    fn solve1(&self) -> Option<u64> {
        for n in (0..99999999999999).rev() {
            let ctx = self.evaluator.eval(&n.to_string());
            if ctx.z == 0 {
                return Some(n)
            }
        }
        None
    }

    fn solve2(&self) -> Option<u64> {
        None
    }
}

#[derive(Clone, Copy)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn parse(data: &str) -> Self {
        match data {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => unreachable!(),
        }
    }
}

enum Value {
    Read(Register),
    Literal(i64),
}

impl Value {
    fn parse(data: &str) -> Self {
        if let Ok(n) = data.parse::<i64>() {
            Value::Literal(n)
        } else {
            Value::Read(Register::parse(data))
        }
    }
}

enum Instruction {
    Input(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Equal(Register, Value),
}

impl Instruction {
    fn parse(data: &str) -> Self {
        let (instruction, operands) = data.split_once(" ").unwrap();
        let mut operands = operands.split(" ");

        match instruction {
            "inp" => Instruction::Input(Register::parse(operands.next().unwrap())),
            "add" => Instruction::Add(
                Register::parse(operands.next().unwrap()),
                Value::parse(operands.next().unwrap()),
            ),
            "mul" => Instruction::Mul(
                Register::parse(operands.next().unwrap()),
                Value::parse(operands.next().unwrap()),
            ),
            "div" => Instruction::Div(
                Register::parse(operands.next().unwrap()),
                Value::parse(operands.next().unwrap()),
            ),
            "mod" => Instruction::Mod(
                Register::parse(operands.next().unwrap()),
                Value::parse(operands.next().unwrap()),
            ),
            "eql" => Instruction::Equal(
                Register::parse(operands.next().unwrap()),
                Value::parse(operands.next().unwrap()),
            ),
            _ => unreachable!(),
        }
    }
}

struct Context {
    input: Vec<i8>,
    input_idx: usize,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Context {
    fn new(input: Vec<i8>) -> Self {
        Context {
            input,
            input_idx: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn eval(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Input(reg) => {
                self.write(reg, self.input[self.input_idx] as i64);
                self.input_idx += 1;
            }
            Instruction::Add(reg, value) => {
                self.write(reg, self.read(&Value::Read(*reg)) + self.read(value));
            }
            Instruction::Mul(reg, value) => {
                self.write(reg, self.read(&Value::Read(*reg)) * self.read(value));
            }
            Instruction::Div(reg, value) => {
                self.write(reg, self.read(&Value::Read(*reg)) / self.read(value));
            }
            Instruction::Mod(reg, value) => {
                self.write(reg, self.read(&Value::Read(*reg)) % self.read(value));
            }
            Instruction::Equal(reg, value) => {
                self.write(
                    reg,
                    (self.read(&Value::Read(*reg)) == self.read(value)) as i64,
                );
            }
        }
    }

    fn read(&self, val: &Value) -> i64 {
        match val {
            Value::Literal(n) => *n,
            Value::Read(Register::W) => self.w,
            Value::Read(Register::X) => self.x,
            Value::Read(Register::Y) => self.y,
            Value::Read(Register::Z) => self.z,
        }
    }

    fn write(&mut self, reg: &Register, value: i64) {
        match reg {
            Register::W => self.w = value,
            Register::X => self.x = value,
            Register::Y => self.y = value,
            Register::Z => self.z = value,
        }
    }
}

struct Evaluator {
    instructions: Vec<Instruction>,
}

impl Evaluator {
    fn new(program: &str) -> Self {
        let instructions = program.trim().lines().map(Instruction::parse).collect();
        Self { instructions }
    }

    fn eval(&self, input: &str) -> Context {
        let input = input
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i8)
            .collect();

        let mut ctx = Context::new(input);
        for instruction in &self.instructions {
            ctx.eval(instruction);
        }
        ctx
    }
}

#[cfg(test)]
mod day24tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_negate() {
        let eval = Evaluator::new(indoc!(
            "
            inp x
            mul x -1
        "
        ));
        assert_eq!(eval.eval("1").x, -1);
        assert_eq!(eval.eval("5").x, -5);
    }
}

fn main() {
    solve_file::<Day24Solver, u64>("day24.txt");
}
