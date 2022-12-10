use advent_of_code_2022::parser::get_input_as_string;

pub fn solve(path: &str) -> i32 {
    let input = get_input_as_string(path);
    for i in 0..input.len() {
        let group = &input[i..i + 14];
        println!("{}", group);
        if are_chars_unique(group) {
            return i as i32 + 14;
        }
    }

    0
}

fn are_chars_unique(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    chars
        .iter()
        .all(|c| chars.iter().filter(|&d| d == c).count() == 1)
}
