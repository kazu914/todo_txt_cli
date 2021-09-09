use std::{
    fs::{remove_file, OpenOptions},
    io::{prelude::*, BufReader},
    process,
};

pub struct TodoFile {
    pub path: String,
}

impl TodoFile {
    pub fn new(path: &str) -> TodoFile {
        TodoFile {
            path: path.to_string(),
        }
    }

    pub fn append(&self, content: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap_or_else(|_| {
                println!("Error: Couldn't open the file: {}", &self.path);
                process::exit(1)
            });
        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Error: Couldn't write to file: {}", e)
        };
    }

    pub fn read(&self) -> Vec<String> {
        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .unwrap_or_else(|_| {
                println!("Error: Couldn't open the file: {}", &self.path);
                process::exit(1)
            });
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|l| l.expect("Error: Couldn't parse line"))
            .collect()
    }

    pub fn overwrite(&self, contents: &Vec<&str>) {
        let _ = remove_file(&self.path);
        for content in contents {
            self.append(content);
        }
    }
}
}
