pub mod parser {
    use std::fs;

    pub fn get_lines(input_path: &str) -> Vec<String> {
        let input = get_input_as_string(input_path);
        let x: Vec<&str> = input.split('\n').collect();
        x.iter().map(|s| -> String { String::from(*s) }).collect()
    }

    fn get_input_as_string(input_path: &str) -> String {
        let r = fs::read_to_string(input_path);
        match r {
            Ok(input) => input,
            Err(_) => panic!("Input file was not found at {}", input_path),
        }
    }
}
