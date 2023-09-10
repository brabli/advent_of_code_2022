use monkey::*;

mod monkey;

// #[allow(unused)]
pub fn solve() -> i32 {
    let mut monkies = init_monkeys();
    let mut mods: Vec<MonkeyMod> = vec![];

    for _i in 0..20 {
        for monkey in &mut monkies {
            if monkey.name == Name::Zero {
                println!("{}", monkey.inspections);
            }

            for m in &mut mods {
                if m.name == monkey.name {
                    monkey.held_items.push(m.item);
                }
            }

            mods.retain(|m| m.name != monkey.name);

            println!("Number of items held: {}", monkey.held_items.len());
            for item in &monkey.held_items {
                let increased_worry = monkey.increase_worry_level_closure.as_ref()(*item);
                monkey.inspections += 1;
                let reduced_worry = reduce_worry_level(increased_worry);
                let worry_check = check_worry_is_divisible(monkey.worry_divisor, reduced_worry);
                let throwing_to_monkey_name = monkey.throwing_to_monkey(worry_check);

                mods.push(MonkeyMod {
                    name: throwing_to_monkey_name.clone(),
                    item: reduced_worry,
                });
            }

            monkey.held_items.clear();
        }
    }

    let mut nums = monkies
        .into_iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();

    nums.sort_by(|a, b| b.cmp(a));

    let first_largest = nums.first().unwrap();
    let second_largest = nums.get(1).unwrap();

    println!(
        "First largest: {}\nSecond largest: {}",
        first_largest, second_largest
    );

    (first_largest * second_largest).try_into().unwrap()
}

#[derive(Debug)]
struct MonkeyMod {
    name: Name,
    item: usize,
}
