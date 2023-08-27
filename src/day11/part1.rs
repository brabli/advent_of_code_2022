use self::monkey::{init_monkeys, Monkey, Name};

mod monkey;

pub fn solve() -> i32 {
    let m = init_monkeys();
    let new_worry = m.first().unwrap().calc_new_worry.as_ref()(12);
    dbg!(new_worry);
    0
}

fn monkey_turn(monkey: Monkey) {}

fn find_monkey_by_name(name: Name, monkies: &[Monkey; 8]) -> &Monkey {
    for monkey in monkies {
        if monkey.name == name {
            return monkey;
        }
    }

    panic!("Could not find a monkey with name {:?}", name);
}
