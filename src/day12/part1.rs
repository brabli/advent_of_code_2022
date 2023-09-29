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

    // let num_items = char_slice_to_int_slice(&char_items);

    let _grid = Grid {
        items: &char_items,
        rows,
        cols,
    };

    // dbg!(grid);

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

fn replace_s_with_a(chars: &[char]) -> Vec<char> {
    chars
        .iter()
        .map(|c| {
            if *c == 'S' {
                return 'a';
            }

            *c
        })
        .collect()
}

fn replace_e_with_z(chars: &[char]) -> Vec<char> {
    chars
        .iter()
        .map(|c| {
            if *c == 'E' {
                return 'z';
            }

            *c
        })
        .collect()
}

#[derive(Debug)]
enum SpecialChar {
    Start,
    End,
}

impl PartialEq<char> for SpecialChar {
    fn eq(&self, other: &char) -> bool {
        match self {
            SpecialChar::Start => 'S' == *other,
            SpecialChar::End => 'E' == *other,
        }
    }
}

type Function = Box<dyn Fn(&Grid<char>) -> (usize, usize)>;

fn find_special_char(special: SpecialChar) -> Function {
    Box::new(move |grid| {
        let index = grid
            .items
            .iter()
            .position(|i| special == *i)
            .unwrap_or_else(|| panic!("No special char {:?} found.", grid));

        grid.find_index_coords(index)
            .expect("Found starting value but failed to fins it's coords.")
    })
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
        let result = replace_s_with_a(&['a', 'b', 'S']);
        assert_eq!(vec!['a', 'b', 'a'], result);
    }

    #[test]
    fn letter_e_is_replaced_with_z() {
        let result = replace_e_with_z(&['S', 'b', 'E']);
        assert_eq!(vec!['S', 'b', 'z'], result);
    }

    #[test]
    fn returns_starting_coords() {
        let grid = create_grid();
        let find_starting_coords = find_special_char(SpecialChar::Start);
        let find_starting_coords = find_starting_coords.as_ref();

        assert_eq!(find_starting_coords(&grid), (0, 3));
    }

    #[test]
    fn returns_ending_coords() {
        let grid = create_grid();
        let find_end_coords = find_special_char(SpecialChar::End);
        let find_end_coords = find_end_coords.as_ref();

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
