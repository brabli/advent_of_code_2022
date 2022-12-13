use advent_of_code_2022::parser::get_input_as_string;

pub fn solve(path: &str) -> u32 {
    let input = get_input_as_string(path);
    let lines: Vec<&str> = input.split('\n').collect();

    let mut directory = String::from("/");

    for line in lines {
        if line.starts_with("$ cd") {
            change_directory(line.split_whitespace().last().unwrap(), &mut directory);
            println!("{}", directory);
        }
        if !line.starts_with("$") && !line.starts_with("dir ") {
            let file_iter: Vec<&str> = line.split_whitespace().collect();
            println!(
                "{}{} ({})",
                directory,
                file_iter.last().unwrap(),
                file_iter.first().unwrap(),
            );
        }
    }

    0
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
