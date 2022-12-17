use advent_of_code_2022::parser::get_lines;

struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn from(file_line: &str, dir: &str) -> Self {
        let split: Vec<&str> = file_line.split_whitespace().collect();

        File {
            name: format!("{}{}", dir, split.last().unwrap().to_string()),
            size: split.first().unwrap().parse::<u32>().unwrap(),
        }
    }
}

pub fn solve() -> u32 {
    let lines = get_lines("../../input/day7.txt");
    let mut directory = String::from("/");

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<File> = Vec::new();

    for line in lines {
        if line.starts_with("$ cd") {
            change_directory(line.split_whitespace().last().unwrap(), &mut directory);
            directories.push(directory.clone());
        }
        if !line.starts_with("$") && !line.starts_with("dir ") {
            let file = File::from(line.as_str(), directory.clone().as_str());
            files.push(file);
        }
    }

    directories.sort_unstable();
    directories.dedup();

    let mut dirs_hundred_thousand = Vec::new();

    for dir in directories {
        let mut dir_size = 0;
        for file in &files {
            println!("{}", file.name);
            if file.name.starts_with(dir.as_str()) {
                dir_size += file.size
            }
        }
        if dir_size <= 100_000 {
            dirs_hundred_thousand.push(dir_size);
        }
    }

    let mut total = 0;
    for i in dirs_hundred_thousand {
        total += i;
    }

    total
}

fn change_directory(new_dir: &str, current_dir: &mut String) {
    match new_dir {
        "/" => *current_dir = String::from("/"),
        ".." => go_back_one_dir(current_dir),
        "" => panic!(
            "You are trying to cd to nowhere! Current dir: {}",
            current_dir
        ),
        _ => {
            current_dir.push_str(new_dir);
            current_dir.push('/');
        }
    }
}

fn go_back_one_dir(current_dir: &mut String) {
    assert_ne!(
        current_dir.as_str(),
        "/",
        "Can't go back a directory, already at root!"
    );

    let mut split_dirs: Vec<&str> = current_dir.trim_matches('/').split('/').collect();

    split_dirs.pop().expect("Failed to go back a directory!");

    let mut new_dir = String::new();

    for dir in split_dirs {
        new_dir.push('/');
        new_dir.push_str(dir);
    }

    new_dir.push('/');

    *current_dir = new_dir;
}
