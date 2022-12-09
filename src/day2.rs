// A - Rock
// B - Paper
// C - Scissors
// X - Rock
// Y - Paper
// Z - Scissors

use std::fs;

enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    pub fn from(symbol: char) -> Throw {
        match symbol {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("{} cannot be converted into a Throw.", symbol),
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn loses_against(&self) -> Throw {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn wins_against(&self) -> Throw {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn get_copy(&self) -> Throw {
        match self {
            Self::Rock => Self::Rock,
            Self::Paper => Self::Paper,
            Self::Scissors => Self::Scissors,
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn get_score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    pub fn get_value(&self) -> i32 {
        match self {
            Self::Win => 2,
            Self::Draw => 0,
            Self::Lose => 1,
        }
    }

    pub fn from(symbol: char) -> GameResult {
        match symbol {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("{} cannot be converted into a game result", symbol),
        }
    }
}

fn play_game(elf_throw: &Throw, my_throw: &Throw) -> GameResult {
    let mut game_result = elf_throw.get_score() - my_throw.get_score();
    if game_result < 0 {
        game_result += 3;
    }

    match game_result {
        0 => GameResult::Draw,
        1 => GameResult::Lose,
        2 => GameResult::Win,
        _ => panic!("{} is an invalid game result number!", game_result),
    }
}

fn calculate_game_score(elf_throw: &Throw, my_throw: &Throw) -> i32 {
    play_game(elf_throw, my_throw).get_score() + my_throw.get_score()
}

fn get_input_as_string(path: &str) -> String {
    let r = fs::read_to_string(path);
    match r {
        Ok(input) => return input,
        Err(_) => panic!("Guess it not found. {}", path),
    }
}

pub fn calculate_score(path: &str) -> i32 {
    let mut total_score = 0;
    let input = get_input_as_string(path);
    for pair in input.split('\n') {
        assert!(pair.len() == 3);
        let elf_throw = Throw::from(pair.chars().next().unwrap());
        let my_throw = Throw::from(pair.chars().last().unwrap());
        let score = calculate_game_score(&elf_throw, &my_throw);
        total_score += score;
    }

    return total_score;
}

pub fn calculate_score_2(path: &str) -> i32 {
    let mut total_score = 0;
    let input = get_input_as_string(path);
    for pair in input.split('\n') {
        assert!(pair.len() == 3);
        let elf_throw = Throw::from(pair.chars().next().unwrap());
        let required_result = GameResult::from(pair.chars().last().unwrap());
        let my_throw = figure_out_what_to_throw(&elf_throw, &required_result);
        let score = calculate_game_score(&elf_throw, &my_throw);
        total_score += score;
    }

    return total_score;
}

fn figure_out_what_to_throw(elf_throw: &Throw, required_result: &GameResult) -> Throw {
    match required_result {
        GameResult::Win => elf_throw.loses_against(),
        GameResult::Lose => elf_throw.wins_against(),
        _ => elf_throw.get_copy(),
    }
}
