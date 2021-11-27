use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn generate_random_arr(length: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i32> = (1..length as i32).collect();
    nums.shuffle(&mut rng);

    nums
}


pub struct Logger {
    file: File,
    mirror_to_console: bool,
}

impl Logger {
    pub fn new(file_name: String, mirror_to_console: bool) -> Logger {
        let path = Path::new(&file_name);

        let file = match File::create(&path) {
            Err(err) => panic!("{}", err),
            Ok(file) => file,
        };

        Logger {
            file: file,
            mirror_to_console: mirror_to_console
        }
    }

    pub fn log(&mut self, mut text: String) {
        text = format!("{}\n", text);
        if self.mirror_to_console { print!("{}", text); };
        match self.file.write(text.as_bytes()) {
            Err(err) => panic!("Error writing to file, ERR: {}", err),
            Ok(_) => {},
        }
    }
}
