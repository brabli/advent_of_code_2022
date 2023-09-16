use advent_of_code_2022::parser::get_lines;

use self::grid::Grid;

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

    let num_items = char_slice_to_int_slice(&char_items);

    let _grid = Grid {
        items: &num_items,
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

mod grid {
    #[derive(Debug)]
    pub struct Grid<'a, T>
    where
        T: Copy,
    {
        pub items: &'a [T],
        pub rows: usize,
        pub cols: usize,
    }

    impl<'a, T: Copy> Grid<'a, T> {
        pub fn get(&self, x: usize, y: usize) -> Option<T> {
            if x >= self.cols {
                return None;
            }

            if y >= self.rows {
                return None;
            }

            let y_inv = inverse(y, self.rows);
            let index = index(x, y_inv, self.cols);

            if let Some(val) = self.items.get(index) {
                return Some(*val);
            }

            None
        }

        pub fn find_index_coords(&self, index: usize) -> Option<(usize, usize)> {
            if index >= self.items.len() {
                return None;
            }

            let x = index % self.cols;
            let y = (index - x) / self.cols;
            let y_inv = inverse(y, self.rows);
            Some((x, y_inv))
        }
    }

    fn inverse(y: usize, rows: usize) -> usize {
        isize::abs(y as isize - (rows as isize - 1)) as usize
    }

    fn index(x: usize, y: usize, cols: usize) -> usize {
        x + cols * y
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn get_returns_value_at_inverted_coordinate() {
            let grid = setup_3_by_3_grid();
            assert_eq!(grid.get(0, 0), Some(7));
            assert_eq!(grid.get(0, 2), Some(1));
            assert_eq!(grid.get(1, 1), Some(5));
        }

        #[test]
        fn get_returns_none_if_x_out_of_range() {
            let grid = setup_3_by_3_grid();
            assert_eq!(grid.get(3, 0), None);
        }

        #[test]
        fn get_returns_none_if_y_out_of_range() {
            let grid = setup_3_by_3_grid();
            assert_eq!(grid.get(0, 3), None);
        }

        #[test]
        fn index_coords_are_returned() {
            let grid = setup_3_by_3_grid();
            assert_eq!(grid.find_index_coords(0), Some((0, 2)));
            assert_eq!(grid.find_index_coords(6), Some((0, 0)));
            assert_eq!(grid.find_index_coords(2), Some((2, 2)));
        }

        #[test]
        fn none_returned_if_index_out_of_range() {
            let grid = setup_3_by_3_grid();
            assert_eq!(grid.find_index_coords(9), None);
        }

        fn setup_3_by_3_grid() -> Grid<'static, u8> {
            Grid {
                rows: 3,
                cols: 3,
                items: &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            }
        }
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
        let result = replace_s_with_a(&['a', 'b', 'S']);
        assert_eq!(vec!['a', 'b', 'a'], result);
    }

    #[test]
    fn letter_e_is_replaced_with_z() {
        let result = replace_e_with_z(&['S', 'b', 'E']);
        assert_eq!(vec!['S', 'b', 'z'], result);
    }
}
