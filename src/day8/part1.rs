use core::panic;

use advent_of_code_2022::parser::get_lines;
use grid::Grid;

pub fn solve_part_one() -> u32 {
    let lines = get_lines("input/day8.txt");
    let grid = convert_lines_to_u8_grid(&lines);

    let mut visible_trees = 0;

    // let coords_to_check = Vec::new();

    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if is_tree_visible(r, c, &grid) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

// If col is 0 or last col CAN SEE
// if row is 0 or last row CAN SEE
// if num is equal or greater CANNOT SEE
fn is_tree_visible(row: usize, col: usize, grid: &Grid<u8>) -> bool {
    if is_tree_on_edge(row, col, grid) {
        return true;
    }

    let tree_height = grid.get(row, col).unwrap();

    let mut blocked_counter = 0;

    for r in (0..row).rev() {
        if is_tree_blocked_by_other_tree(*tree_height, r, col, &grid) {
            blocked_counter += 1;
            break;
        }
    }
    for r in row + 1..grid.rows() {
        if is_tree_blocked_by_other_tree(*tree_height, r, col, &grid) {
            blocked_counter += 1;
            break;
        }
    }
    for c in (0..col).rev() {
        if is_tree_blocked_by_other_tree(*tree_height, row, c, &grid) {
            blocked_counter += 1;
            break;
        }
    }
    for c in col + 1..grid.cols() {
        if is_tree_blocked_by_other_tree(*tree_height, row, c, &grid) {
            blocked_counter += 1;
            break;
        }
    }

    blocked_counter != 4
}

fn is_tree_blocked_by_other_tree(tree: u8, row: usize, col: usize, grid: &Grid<u8>) -> bool {
    let other_tree = grid.get(row, col);
    match other_tree {
        Some(other_tree_size) => {
            if other_tree_size >= &tree {
                return true;
            }
            false
        }
        _ => panic!(
            "Panicked with coords Row: {} Col: {}, Tree size: {}",
            row, col, tree
        ),
    }
}

fn is_tree_on_edge(row: usize, col: usize, grid: &Grid<u8>) -> bool {
    0 == row || 0 == col || row == grid.rows() - 1 || col == grid.cols() - 1
}

fn convert_lines_to_u8_grid(lines: &Vec<String>) -> Grid<u8> {
    let line_len = get_line_len(&lines);
    let string_of_chars = lines.join("");
    let vec_of_u8 = parse_line_to_u8_vec(string_of_chars);

    Grid::from_vec(vec_of_u8, line_len)
}

fn get_line_len(lines: &Vec<String>) -> usize {
    let first_line = lines.first();
    match first_line {
        Some(line) => line.len(),
        None => panic!("Can't find individual line length as there are no lines!"),
    }
}

fn convert_to_grid_rows(lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut parsed_lines = Vec::new();
    for line in lines {
        let u8_vec = parse_line_to_u8_vec(line);
        parsed_lines.push(u8_vec);
    }

    parsed_lines
}

fn parse_line_to_u8_vec(line: String) -> Vec<u8> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}
