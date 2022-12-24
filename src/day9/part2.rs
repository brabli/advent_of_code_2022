use advent_of_code_2022::parser::get_lines;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn from(line: &String) -> Self {
        let first_char = line.chars().next().unwrap();
        match first_char {
            'R' => Self::Right,
            'U' => Self::Up,
            'L' => Self::Left,
            'D' => Self::Down,
            _ => panic!("Couldn't determine a direction from '{}'", first_char),
        }
    }
}

pub fn solve() -> u32 {
    let lines = get_lines("input/day9.txt");
    let mut head_coords = (0, 0);
    let mut one_coords = (0, 0);
    let mut two_coords = (0, 0);
    let mut three_coords = (0, 0);
    let mut four_coords = (0, 0);
    let mut five_coords = (0, 0);
    let mut six_coords = (0, 0);
    let mut seven_coords = (0, 0);
    let mut eight_coords = (0, 0);
    let mut nine_coords = (0, 0);

    let mut all_nine_coords = Vec::new();
    for line in &lines {
        let dir = Direction::from(&line);
        let num = extract_number(&line);

        for _ in 0..num {
            move_coords(&mut head_coords, &dir);
            move_tail(&head_coords, &mut one_coords);
            move_tail(&one_coords, &mut two_coords);
            move_tail(&two_coords, &mut three_coords);
            move_tail(&three_coords, &mut four_coords);
            move_tail(&four_coords, &mut five_coords);
            move_tail(&five_coords, &mut six_coords);
            move_tail(&six_coords, &mut seven_coords);
            move_tail(&seven_coords, &mut eight_coords);
            move_tail(&eight_coords, &mut nine_coords);

            all_nine_coords.push(format!("{}.{}", nine_coords.0, nine_coords.1));
        }
    }

    all_nine_coords.sort_unstable();
    all_nine_coords.dedup();
    all_nine_coords.len() as u32
}

fn move_tail(head_coords: &(i32, i32), tail_coords: &mut (i32, i32)) {
    let hx = head_coords.0;
    let hy = head_coords.1;
    let tx = tail_coords.0;
    let ty = tail_coords.1;
    if !does_tail_need_to_move(&head_coords, &tail_coords) {
        return;
    }

    if is_out_by_two_or_more(hx, tx) && is_out(hy, ty)
        || is_out_by_two_or_more(hy, ty) && is_out(hx, tx)
    {
        let new_tail_x = move_towards(hx, tx);
        let new_tail_y = move_towards(hy, ty);
        tail_coords.0 = new_tail_x;
        tail_coords.1 = new_tail_y;
    } else if is_out_by_two_or_more(hx, tx) && !is_out(hy, ty) {
        tail_coords.0 = move_towards(hx, tx);
    } else if is_out_by_two_or_more(hy, ty) && !is_out(hx, tx) {
        tail_coords.1 = move_towards(hy, ty);
    }
}

fn is_out_by_two_or_more(head: i32, tail: i32) -> bool {
    (head - tail).abs() >= 2
}

fn is_out_by_one(head: i32, tail: i32) -> bool {
    (head - tail).abs() == 1
}

fn is_out(head: i32, tail: i32) -> bool {
    (head - tail).abs() != 0
}

fn move_towards(head: i32, tail: i32) -> i32 {
    if head > tail {
        return tail + 1;
    }

    return tail - 1;
}

fn does_tail_need_to_move(head_coords: &(i32, i32), tail_coords: &(i32, i32)) -> bool {
    return !((head_coords.0 - tail_coords.0).abs() <= 1
        && (head_coords.1 - tail_coords.1).abs() <= 1);
}

fn extract_number(line: &String) -> i32 {
    let s = line.split_whitespace().last().unwrap();
    s.parse::<i32>().unwrap()
}

fn move_coords(coords: &mut (i32, i32), direction: &Direction) {
    match direction {
        Direction::Left => coords.0 -= 1,
        Direction::Right => coords.0 += 1,
        Direction::Up => coords.1 += 1,
        Direction::Down => coords.1 -= 1,
    }
}
