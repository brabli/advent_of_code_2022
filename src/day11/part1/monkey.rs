pub struct Monkey {
    name: MonkeyName,
    held_items: Vec<Item>,
}

impl Monkey {
    pub fn get_name(&self) -> &MonkeyName {
        &self.name
    }

    pub fn throw_item(item: Item, to_monkey: &mut Monkey) {
        to_monkey.catch_item(item);
    }

    pub fn catch_item(&mut self, item: Item) {
        self.held_items.push(item);
    }
}

pub struct Item {
    worry_level: i32,
}

impl Item {
    pub fn get_worry_level(&self) -> i32 {
        self.worry_level
    }
}

pub enum MonkeyName {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}
