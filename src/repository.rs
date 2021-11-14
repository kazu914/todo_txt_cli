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

    fn overwrite(&self, contents: &[&str]) {
        let _ = remove_file(&self.path);
        for content in contents {
            self.append(content);
        }
    }

    pub fn get_todo_with_key(&self, key: usize) -> String {
        let lines = self.read();
        let todo_string = lines.get(key);
        if todo_string.is_none() {
            println!("Error: Couldn't find todo with key: {}", key);
            process::exit(1);
        } else {
            return todo_string.unwrap().to_string();
        }
    }

    pub fn update_todo(&self, key: usize, content: &str) {
        let mut lines = self.read();
        lines[key] = content.to_string();
        self.overwrite(
            lines
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>()
                .as_ref(),
        );
    }

    pub fn remove(&self, key: usize) {
        let mut lines = self.read();
        lines.remove(key);
        self.overwrite(
            lines
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>()
                .as_ref(),
        );
    }

    pub fn check_if_exist(&self, key: usize) -> bool {
        let lines = self.read();
        let todo_string = lines.get(key);
        todo_string.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_all() {
        let path = "test.txt";
        let file = TodoFile::new(path);
        let contents = vec!["test1", "test2"];
        file.overwrite(&contents);
        assert!(Path::new(path).exists());
        assert_eq!(contents, file.read());
        let _ = remove_file(path);
    }
}
