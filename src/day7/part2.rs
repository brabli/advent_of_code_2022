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

struct Dir {
    name: String,
    size: u32,
}

pub fn solve_part_two() -> u32 {
    let lines = get_lines("input/day7.txt");
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

    let mut dirs = Vec::new();

    for dir in directories {
        let mut directory_size = 0;
        for file in &files {
            if file.name.starts_with(dir.as_str()) {
                directory_size += file.size
            }
        }

        dirs.push(Dir {
            name: dir,
            size: directory_size,
        });
    }

    let mut filesystem_size = 0;

    for file in files {
        filesystem_size += file.size;
    }

    let total_size = 70_000_000;
    let unused_space = total_size - filesystem_size;
    let delete_dir_min_size = 30_000_000 - unused_space;
    println!("{}", delete_dir_min_size);

    let mut closest = 1_000_000_000;
    for dir in dirs {
        if dir.size >= delete_dir_min_size && dir.size < closest {
            closest = dir.size;
        }
    }

    closest
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
