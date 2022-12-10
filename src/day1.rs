use std::fs;

#[allow(dead_code)]
pub fn find_most_calories(input_path: &str) -> i32 {
    let raw_input = get_input_as_string(input_path);
    let elf_calorie_strings = get_individual_elf_calorie_strings(&raw_input);
    let mut vec_of_calories: Vec<i32> = Vec::new();
    for calorie_string in elf_calorie_strings {
        let calories_string_vec: Vec<&str> = calorie_string.split("\n").collect();
        let mut total_cals = 0;
        for calories in calories_string_vec {
            let calories_as_int = calories.parse::<i32>();
            match calories_as_int {
                Ok(int) => total_cals += int,
                Err(_) => {
                    println!("Couldn't parse number: {}", calories);
                    total_cals += 0;
                }
            }
        }
        vec_of_calories.push(total_cals);
    }

    vec_of_calories.sort();
    for i in vec_of_calories {
        println!("{}", i);
    }

    return 0;
}

fn get_input_as_string(path: &str) -> String {
    let r = fs::read_to_string(path);
    match r {
        Ok(input) => return input,
        Err(_) => panic!("Guess it not found. {}", path),
    }
}

fn get_individual_elf_calorie_strings(raw_input: &String) -> Vec<&str> {
    raw_input.split("\n\n").collect()
}
