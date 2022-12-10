pub mod parser {
    use std::fs;

    pub fn get_input_as_string(path: &str) -> String {
        let r = fs::read_to_string(path);
        match r {
            Ok(input) => return input,
            Err(_) => panic!("Guess it not found. {}", path),
        }
    }
}
