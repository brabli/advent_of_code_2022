pub struct Monkey {
    pub name: Name,
    held_items: Vec<usize>,
    pub calc_new_worry: Box<dyn Fn(usize) -> usize>,
    worry_divisor: usize,
    success_monkey: Name,
    failure_monkey: Name,
}

impl Monkey {
    pub fn check_worry(&self, worry: usize) -> bool {
        worry % self.worry_divisor == 0
    }
}

pub fn init_monkeys() -> [Monkey; 8] {
    let monkey_0 = Monkey {
        name: Name::Zero,
        held_items: vec![78, 53, 89, 51, 52, 59, 58, 85],
        calc_new_worry: Box::new(|old| old * 3),
        worry_divisor: 5,
        success_monkey: Name::Two,
        failure_monkey: Name::Seven,
    };

    let monkey_1 = Monkey {
        name: Name::One,
        held_items: vec![64],
        calc_new_worry: Box::new(|old| old + 7),
        worry_divisor: 2,
        success_monkey: Name::Three,
        failure_monkey: Name::Six,
    };

    let monkey_2 = Monkey {
        name: Name::Two,
        held_items: vec![71, 93, 65, 82],
        calc_new_worry: Box::new(|old| old + 5),
        worry_divisor: 13,
        success_monkey: Name::Five,
        failure_monkey: Name::Four,
    };

    let monkey_3 = Monkey {
        name: Name::Three,
        held_items: vec![67, 73, 95, 75, 56, 74],
        calc_new_worry: Box::new(|old| old + 8),
        worry_divisor: 19,
        success_monkey: Name::Six,
        failure_monkey: Name::Zero,
    };

    let monkey_4 = Monkey {
        name: Name::Four,
        held_items: vec![85, 91, 90],
        calc_new_worry: Box::new(|old| old + 4),
        worry_divisor: 11,
        success_monkey: Name::Three,
        failure_monkey: Name::One,
    };

    let monkey_5 = Monkey {
        name: Name::Five,
        held_items: vec![67, 96, 69, 55, 70, 83, 62],
        calc_new_worry: Box::new(|old| old * 2),
        worry_divisor: 3,
        success_monkey: Name::Four,
        failure_monkey: Name::One,
    };

    let monkey_6 = Monkey {
        name: Name::Six,
        held_items: vec![53, 86, 98, 70, 64],
        calc_new_worry: Box::new(|old| old + 6),
        worry_divisor: 7,
        success_monkey: Name::Seven,
        failure_monkey: Name::Zero,
    };

    let monkey_7 = Monkey {
        name: Name::Seven,
        held_items: vec![88, 64],
        calc_new_worry: Box::new(|old| old * old),
        worry_divisor: 17,
        success_monkey: Name::Two,
        failure_monkey: Name::Five,
    };

    [
        monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7,
    ]
}

#[derive(PartialEq, Debug)]
pub enum Name {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
