use std::{fmt::format, fs};

fn get_input_as_string(path: &str) -> String {
    let r = fs::read_to_string(path);
    match r {
        Ok(input) => return input,
        Err(_) => panic!("Guess it not found. {}", path),
    }
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: String,
    second_compartment: String,
    all_items: String,
}

struct ElfGroup<'a> {
    first_rucksack: &'a Rucksack,
    second_rucksack: &'a Rucksack,
    third_rucksack: &'a Rucksack,
}

impl<'a> ElfGroup<'a> {
    pub fn from(r1: &'a Rucksack, r2: &'a Rucksack, r3: &'a Rucksack) -> ElfGroup<'a> {
        ElfGroup {
            first_rucksack: r1,
            second_rucksack: r2,
            third_rucksack: r3,
        }
    }

    pub fn get_all_items(&self) -> String {
        let s1 = self.first_rucksack.get_all_items();
        let s2 = self.second_rucksack.get_all_items();
        let s3 = self.third_rucksack.get_all_items();
        format!(
            "{}{}{}",
            remove_duplicate_chars(s1),
            remove_duplicate_chars(s2),
            remove_duplicate_chars(s3)
        )
    }
}

impl Rucksack {
    pub fn from(contents: &str) -> Self {
        if contents.len() % 2 == 1 {
            panic!(
                "Contents must contain an even number of items! {} items found.",
                contents.len()
            );
        }

        let midpoint = contents.len() / 2;

        Rucksack {
            first_compartment: String::from(&contents[0..midpoint]),
            second_compartment: String::from(&contents[midpoint..contents.len()]),
            all_items: String::from(contents),
        }
    }

    pub fn get_all_items(&self) -> &String {
        &self.all_items
    }

    pub fn find_common_letter(&self) -> Option<char> {
        for item in self.first_compartment.chars() {
            if self.second_compartment.contains(item) {
                return Some(item);
            }
        }

        None
    }
}

pub fn solve(path: &str) -> i32 {
    let input = get_input_as_string(path);
    let mut group = Vec::new();
    let mut running_total = 0;
    for rucksack_items in input.split('\n') {
        let rucksack = Rucksack::from(rucksack_items);
        group.push(rucksack);
        if group.len() == 3 {
            let elf_group = ElfGroup::from(&group[0], &group[1], &group[2]);
            let all_items = elf_group.get_all_items();
            for letter in all_items.chars() {
                let mut counter = 0;
                for letter_sub in all_items.chars() {
                    if letter == letter_sub {
                        counter += 1;
                    }
                }
                if counter == 3 {
                    running_total += letter_value(letter);
                    break;
                }
            }
            group = Vec::new();
        }
    }

    return running_total;
}

fn letter_value(letter: char) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result = alphabet.find(letter);
    match result {
        Some(i) => i as i32 + 1,
        None => panic!("{} was not found in the alphabet!", letter),
    }
}

fn remove_duplicate_chars(s: &String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        if !result.contains(c) {
            result.push(c);
        }
    }

    result
}
