use advent_of_code_2022::parser::get_lines;

enum Instruction {
    NOOP,
    ADD(i32),
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        match line {
            "noop" => Self::NOOP,
            _ => Self::ADD(
                line.split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            ),
        }
    }

    fn cycles(&self) -> u8 {
        match self {
            Self::ADD(_) => 2,
            Self::NOOP => 1,
        }
    }

    pub fn execute(&self, x: &mut i32, cycles: &mut i32, signal_str: &mut i32) {
        match self {
            Self::NOOP => {
                for _ in 0..self.cycles() {
                    *cycles += 1;
                    calc_new_strength_if_cycles_match(*cycles, *x, signal_str);
                }
            }
            Self::ADD(num) => {
                for _ in 0..self.cycles() {
                    *cycles += 1;
                    calc_new_strength_if_cycles_match(*cycles, *x, signal_str);
                }

                *x += num;
            }
        }
    }
}

pub fn solve() -> i32 {
    let lines = get_lines("input/day10.txt");

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_str = 0;

    for line in lines {
        let instruction = Instruction::from(line.as_str());
        instruction.execute(&mut x, &mut cycle, &mut signal_str);
    }
    signal_str
}

fn calc_signal_strength(cycle: i32, x: i32) -> i32 {
    cycle * x
}

fn calc_new_strength_if_cycles_match(cycle_num: i32, x: i32, signal_strength: &mut i32) {
    let magic_cycles = [20, 60, 100, 140, 180, 220];
    if magic_cycles.contains(&cycle_num) {
        *signal_strength += calc_signal_strength(cycle_num, x);
    }
}
