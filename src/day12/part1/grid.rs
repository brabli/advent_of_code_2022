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
