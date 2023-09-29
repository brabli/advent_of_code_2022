mod grid;

use self::grid::Grid;
use advent_of_code_2022::parser::get_lines;

pub fn solve() -> i32 {
    let lines = get_lines("input/day12.txt");
    let cols = lines.first().unwrap().len();
    let rows = lines.len();

    let char_items: Vec<char> = lines
        .into_iter()
        .reduce(|line, acc| format!("{}{}", line, acc))
        .unwrap()
        .chars()
        .collect();

    let grid = Grid {
        items: &char_items,
        rows,
        cols,
    };

    let start = find_special(Special::Start)(&grid);
    let end = find_special(Special::End)(&grid);

    let grid_without_specials = replace_with('e')('z')(&char_items);
    // let num_items = char_slice_to_int_slice(&char_items);

    0
}

fn char_slice_to_int_slice(chars: &[char]) -> Vec<u8> {
    chars.iter().map(|c| char_to_int(*c)).collect()
}

fn char_to_int(c: char) -> u8 {
    if !c.is_ascii_lowercase() {
        panic!("Non lowercase ascii char passed");
    }

    c as u8 - 96
}

type ReplaceFunction = Box<dyn Fn(char) -> Box<dyn Fn(&[char]) -> Vec<char>>>;
fn replace_with(target: char) -> ReplaceFunction {
    Box::new(move |replacement: char| {
        Box::new(move |chars: &[char]| {
            chars
                .iter()
                .map(|c| if *c == target { replacement } else { *c })
                .collect()
        })
    })
}

#[derive(Debug)]
enum Special {
    Start,
    End,
}

impl PartialEq<char> for Special {
    fn eq(&self, other: &char) -> bool {
        match self {
            Special::Start => 'S' == *other,
            Special::End => 'E' == *other,
        }
    }
}

fn find_special(special: Special) -> impl Fn(&Grid<char>) -> (usize, usize) {
    move |grid| {
        let index = grid
            .items
            .iter()
            .position(|i| special == *i)
            .unwrap_or_else(|| panic!("No special char {:?} found.", grid));

        grid.find_index_coords(index)
            .expect("Found starting value but failed to fins it's coords.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected_nums() {
        assert_eq!(char_to_int('a'), 1);
        assert_eq!(char_to_int('b'), 2);
        assert_eq!(char_to_int('z'), 26);
    }

    #[test]
    #[should_panic(expected = "Non lowercase ascii char passed")]
    fn panics_on_uc_letter() {
        char_to_int('A');
    }

    #[test]
    #[should_panic(expected = "Non lowercase ascii char passed")]
    fn panics_on_number() {
        char_to_int('7');
    }

    #[test]
    fn returns_u8_vec() {
        let chars = ['a', 'b', 'c'];
        assert_eq!(char_slice_to_int_slice(&chars), vec![1, 2, 3]);
    }

    #[test]
    fn letter_s_is_replaced_with_a() {
        let chars = ['a', 'b', 'S'];
        let res = replace_with('S')('a')(&chars);
        assert_eq!(vec!['a', 'b', 'a'], res);
    }

    #[test]
    fn letter_e_is_replaced_with_z() {
        let chars = ['S', 'b', 'E'];
        let res = replace_with('E')('z')(&chars);
        assert_eq!(vec!['S', 'b', 'z'], res);
    }

    #[test]
    fn returns_starting_coords() {
        let grid = create_grid();
        let find_starting_coords = find_special(Special::Start);

        assert_eq!(find_starting_coords(&grid), (0, 3));
    }

    #[test]
    fn returns_ending_coords() {
        let grid = create_grid();
        let find_end_coords = find_special(Special::End);

        assert_eq!(find_end_coords(&grid), (1, 0));
    }

    fn create_grid() -> Grid<'static, char> {
        Grid {
            items: &['S', 'b', 'c', 'd', 'e', 'f', 'g', 'E'],
            rows: 4,
            cols: 2,
        }
    }
}
