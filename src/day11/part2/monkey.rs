use std::rc::Rc;

#[derive(Clone)]
pub struct Monkey {
    pub name: Name,
    pub held_items: Vec<usize>,
    pub increase_worry_level_closure: Rc<dyn Fn(usize) -> usize>,
    pub worry_divisor: usize,
    pub success_monkey: Name,
    pub failure_monkey: Name,
    pub inspections: usize,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("name", &self.name)
            .field("held_items", &self.held_items)
            .field("worry_divisor", &self.worry_divisor)
            .field("success_monkey", &self.success_monkey)
            .field("failure_monkey", &self.failure_monkey)
            .field("inspections", &self.inspections)
            .finish()
    }
}

pub fn check_worry_is_divisible(divisor: usize, worry: usize) -> bool {
    worry % divisor == 0
}

pub fn find_monkey<'a>(name: &Name, monkies: &'a [Monkey; 8]) -> &'a Monkey {
    for m in monkies {
        if m.name == *name {
            return m;
        }
    }

    panic!();
}

impl Monkey {
    pub fn throwing_to_monkey(&self, check_result: bool) -> &Name {
        if check_result {
            &self.success_monkey
        } else {
            &self.failure_monkey
        }
    }
}

pub fn get_divisors() -> [usize; 8] {
    init_monkeys().map(|m| m.worry_divisor)
}

pub fn init_monkeys() -> [Monkey; 8] {
    let monkey_0 = Monkey {
        name: Name::Zero,
        held_items: vec![78, 53, 89, 51, 52, 59, 58, 85],
        increase_worry_level_closure: Rc::new(|old| old * 3),
        worry_divisor: 5,
        success_monkey: Name::Two,
        failure_monkey: Name::Seven,
        inspections: 0,
    };

    let monkey_1 = Monkey {
        name: Name::One,
        held_items: vec![64],
        increase_worry_level_closure: Rc::new(|old| old + 7),
        worry_divisor: 2,
        success_monkey: Name::Three,
        failure_monkey: Name::Six,
        inspections: 0,
    };

    let monkey_2 = Monkey {
        name: Name::Two,
        held_items: vec![71, 93, 65, 82],
        increase_worry_level_closure: Rc::new(|old| old + 5),
        worry_divisor: 13,
        success_monkey: Name::Five,
        failure_monkey: Name::Four,
        inspections: 0,
    };

    let monkey_3 = Monkey {
        name: Name::Three,
        held_items: vec![67, 73, 95, 75, 56, 74],
        increase_worry_level_closure: Rc::new(|old| old + 8),
        worry_divisor: 19,
        success_monkey: Name::Six,
        failure_monkey: Name::Zero,
        inspections: 0,
    };

    let monkey_4 = Monkey {
        name: Name::Four,
        held_items: vec![85, 91, 90],
        increase_worry_level_closure: Rc::new(|old| old + 4),
        worry_divisor: 11,
        success_monkey: Name::Three,
        failure_monkey: Name::One,
        inspections: 0,
    };

    let monkey_5 = Monkey {
        name: Name::Five,
        held_items: vec![67, 96, 69, 55, 70, 83, 62],
        increase_worry_level_closure: Rc::new(|old| old * 2),
        worry_divisor: 3,
        success_monkey: Name::Four,
        failure_monkey: Name::One,
        inspections: 0,
    };

    let monkey_6 = Monkey {
        name: Name::Six,
        held_items: vec![53, 86, 98, 70, 64],
        increase_worry_level_closure: Rc::new(|old| old + 6),
        worry_divisor: 7,
        success_monkey: Name::Seven,
        failure_monkey: Name::Zero,
        inspections: 0,
    };

    let monkey_7 = Monkey {
        name: Name::Seven,
        held_items: vec![88, 64],
        increase_worry_level_closure: Rc::new(|old| old * old),
        worry_divisor: 17,
        success_monkey: Name::Two,
        failure_monkey: Name::Five,
        inspections: 0,
    };

    [
        monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7,
    ]
}

#[derive(PartialEq, Debug, Clone)]
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
