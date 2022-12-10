use advent_of_code_2022::parser::get_input_as_string;

use self::supplies::{Stack, StackCollection};

mod supplies;

struct Instruction {
    pub amount: i32,
    pub from: i32,
    pub to: i32,
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        let line_split: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(line_split.len(), 6);

        Instruction {
            amount: line_split[1].parse::<i32>().unwrap(),
            from: line_split[3].parse::<i32>().unwrap(),
            to: line_split[5].parse::<i32>().unwrap(),
        }
    }
}

pub fn solve(path: &str) -> i32 {
    let input = get_input_as_string(path);
    let mut stack_collection = get_stack_collection();
    for line in input.split('\n') {
        let i = Instruction::from(line);
        stack_collection.move_between_stacks(i.amount, i.from, i.to);
    }

    println!("{}", stack_collection.get_top_row());
    0
}

fn get_stack_collection() -> StackCollection {
    let stacks = vec![
        Stack::from("NDMQBPZ"),
        Stack::from("CLZQMDHV"),
        Stack::from("QHRDVFZG"),
        Stack::from("HGDFN"),
        Stack::from("NFQ"),
        Stack::from("DQVZFBT"),
        Stack::from("QMTZDVSH"),
        Stack::from("MGFPNQ"),
        Stack::from("BWRM"),
    ];

    StackCollection::from(stacks)
}
