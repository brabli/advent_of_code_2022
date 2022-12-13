pub struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn new(name: &str, size: u32) -> Self {
        File {
            name: String::from(name),
            size,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
