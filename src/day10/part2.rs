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

    pub fn execute(&self, x: &mut i32, cycles: &mut i32) {
        match self {
            Self::NOOP => {
                run_cycle(self.cycles(), cycles, x);
            }

            Self::ADD(num) => {
                run_cycle(self.cycles(), cycles, x);
                *x += num;
            }
        }
    }
}

pub fn solve() -> i32 {
    let lines = get_lines("input/day10.txt");

    let mut x = 1;
    let mut cycle = 0;

    for line in lines {
        let instruction = Instruction::from(line.as_str());
        instruction.execute(&mut x, &mut cycle);
    }

    0
}

fn run_cycle(cycles_to_run: u8, cycles: &mut i32, x: &mut i32) {
    for _ in 0..cycles_to_run {
        draw_pixel(*cycles, *x);
        *cycles += 1;
    }
}

fn draw_pixel(cycle: i32, x: i32) {
    let horizontal_position = cycle % 40;

    if 0 == horizontal_position {
        print!("\n");
    }

    if x - 1 == horizontal_position || x == horizontal_position || x + 1 == horizontal_position {
        print!("#");
    } else {
        print!(".");
    }
}
