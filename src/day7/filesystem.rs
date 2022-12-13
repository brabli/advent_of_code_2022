use core::panic;

use super::dir::Dir;

pub struct Filesystem {
    root: Dir,
    location: String,
}

impl Filesystem {
    pub fn new() -> Self {
        Filesystem {
            root: Dir::new("/"),
            location: String::from("/"),
        }
    }

    pub fn cd(&mut self, dir: &str) {
        if dir == "/" {
            self.go_to_root();
        } else if dir == ".." {
            self.go_back_one_dir();
        } else {
            // let d = self.get_current_dir();
        }
    }

    // fn get_current_dir(&self) -> &Dir {
    //     let location = self.location;
    //     for dir in self.root.get_dirs() {
    //         if dir.get_name().as_str() == location {
    //             return dir;
    //         }
    //     }
    //     panic!("aaa");
    // }

    fn go_to_root(&mut self) {
        self.location = String::from("/");
    }

    fn go_back_one_dir(&mut self) {
        let mut location: Vec<&str> = self.location.split("/").collect();
        let popped_dir = location.pop();
        match popped_dir {
            Some(_) => {
                self.location = location.join("/");
                self.location.insert(0, '/');
            }
            None => panic!("Tried to go back a directory, but we cannot go back anymore!"),
        }
    }
}
