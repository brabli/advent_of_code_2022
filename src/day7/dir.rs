use super::file::File;

pub struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Dir {
            name: String::from(name),
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    pub fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(dir);
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn get_dirs(&self) -> &Vec<Dir> {
        &self.dirs
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_size(&self) -> u32 {
        let mut size: u32 = 0;

        for dir in &self.dirs {
            size += dir.get_size();
        }

        for file in &self.files {
            size += file.get_size();
        }

        size
    }
}

#[cfg(test)]
mod dir_test {
    use crate::day7::file::File;

    use super::Dir;

    #[test]
    fn test_get_filesize() {
        let d = Dir::new("test_dir");
        assert_eq!(d.get_size(), 0);
    }

    #[test]
    fn test_get_filesize_with_files() {
        let mut d = Dir::new("test_dir");
        let f1 = File::new("file", 1000);
        let f2 = File::new("file", 2000);
        d.add_file(f1);
        d.add_file(f2);
        assert_eq!(d.get_size(), 3000);
    }

    #[test]
    fn test_get_size_with_nested_dirs() {
        let mut d1 = Dir::new("test_dir");
        let mut d2 = Dir::new("nested_dir");
        let f1 = File::new("file", 500);
        let f2 = File::new("file", 400);
        d1.add_file(f1);
        d2.add_file(f2);
        d1.add_dir(d2);

        assert_eq!(d1.get_size(), 900);
    }
}
