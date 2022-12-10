use advent_of_code_2022::parser;

struct Elf {
    start: i32,
    end: i32,
}

impl Elf {
    pub fn get_sections(&self) -> Vec<i32> {
        (self.start..=self.end).collect()
    }
}

pub fn solve(path: &str) -> i32 {
    let input = parser::get_input_as_string(path);
    let lines: Vec<&str> = input.split('\n').collect();
    // let mut full_overlaps = 0;
    let mut overlaps = 0;
    for line in lines {
        let ranges = get_ranges_from_line(line);
        let first_elf = get_elf_from_range(ranges.0);
        let second_elf = get_elf_from_range(ranges.1);
        // if fully_contains(&first_elf, &second_elf) {
        //     full_overlaps += 1;
        // }

        if overlaps_at_all(&first_elf, &second_elf) {
            overlaps += 1;
        }
    }

    return overlaps;
}

fn get_ranges_from_line(line: &str) -> (&str, &str) {
    let unparsed_ranges: Vec<&str> = line.split(',').collect();
    assert!(unparsed_ranges.len() == 2);
    (unparsed_ranges[0], unparsed_ranges[1])
}

fn get_elf_from_range(range: &str) -> Elf {
    let values: Vec<&str> = range.split('-').collect();

    Elf {
        start: values[0].parse::<i32>().unwrap(),
        end: values[1].parse::<i32>().unwrap(),
    }
}

fn fully_contains(e1: &Elf, e2: &Elf) -> bool {
    (e1.start <= e2.start && e1.end >= e2.end) || (e2.start <= e1.start && e2.end >= e1.end)
}

fn overlaps_at_all(e1: &Elf, e2: &Elf) -> bool {
    let elf_one: Vec<i32> = (e1.start..=e1.end).collect();
    let elf_two_sectors = e2.start..=e2.end;
    for sector in elf_two_sectors {
        if elf_one.contains(&sector) {
            return true;
        }
    }

    false
}

mod test {
    use super::*;

    #[test]
    fn true_if_all_sections_same() {
        assert!(fully_contains(&elf(1, 1), &elf(1, 1)));
    }

    #[test]
    fn false_if_range_overlaps() {
        assert!(!fully_contains(&elf(1, 2), &elf(2, 3)));
    }

    #[test]
    fn true_if_range_contained() {
        assert!(fully_contains(&elf(1, 2), &elf(1, 1)));
        assert!(fully_contains(&elf(1, 2), &elf(1, 2)));
        assert!(fully_contains(&elf(4, 8), &elf(2, 9)));
    }

    #[test]
    fn false_if_range_doesnt_overlap() {
        assert!(!overlaps_at_all(&elf(1, 2), &elf(3, 4)));
    }

    #[test]
    fn true_if_fully_contains() {
        assert!(overlaps_at_all(&elf(1, 4), &elf(2, 3)));
        assert!(overlaps_at_all(&elf(2, 3), &elf(1, 4)));
    }

    #[test]
    fn true_if_overlaps() {
        assert!(overlaps_at_all(&elf(1, 2), &elf(2, 3)));
        assert!(overlaps_at_all(&elf(2, 3), &elf(1, 2)));
    }

    fn elf(s: i32, e: i32) -> Elf {
        Elf { start: s, end: e }
    }
}
