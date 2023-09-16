pub fn solve() -> i32 {
    0
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
