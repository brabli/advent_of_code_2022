use monkey::*;

mod monkey;

pub fn solve() {
    let mut monkies = init_monkeys();
    let mut mods: Vec<MonkeyMod> = vec![];
    let product = get_divisors().into_iter().product::<usize>();

    for i in 0..10000 {
        if i % 100 == 0 {
            println!("{}", i);
        }
        for monkey in &mut monkies {
            for m in &mut mods {
                if m.name == monkey.name {
                    monkey.held_items.push(m.item);
                }
            }

            mods.retain(|m| m.name != monkey.name);

            // dbg!(mods.len());
            for item in &monkey.held_items {
                // println!("{}", item);
                let mut increased_worry = monkey.increase_worry_level_closure.as_ref()(*item);

                increased_worry %= product;

                monkey.inspections += 1;
                let worry_check = check_worry_is_divisible(monkey.worry_divisor, increased_worry);
                let throwing_to_monkey_name = monkey.throwing_to_monkey(worry_check);

                mods.push(MonkeyMod {
                    name: throwing_to_monkey_name.clone(),
                    item: increased_worry,
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

    let a = first_largest * second_largest;
    println!("The answer is {}", a);
}

fn find_largest(nums: &mut [usize]) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    if nums.len() == 1 {
        return Some(nums[0]);
    }

    nums.sort_by(|a, b| b.cmp(a));

    let largest = nums[0];
    Some(largest)
}

#[derive(Debug)]
struct MonkeyMod {
    name: Name,
    item: usize,
}
