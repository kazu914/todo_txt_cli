use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::BufReader, io::Error, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_file_name: String,
}

impl Config {
    pub fn new() -> Config {
        let project_dirs = ProjectDirs::from("", "", "todo_txt").unwrap();
        Config::read_config(&project_dirs).unwrap_or(Config {
            default_file_name: project_dirs
                .config_dir()
                .join("todo.txt")
                .into_os_string()
                .into_string()
                .unwrap(),
        })
    }

    fn read_config(project_dirs: &ProjectDirs) -> Result<Config, Error> {
        let config_path = project_dirs.config_dir().join("config.json");
        let config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(config_path.clone())?;
        let reader = BufReader::new(config_file);
        let config = serde_json::from_reader(reader).unwrap_or_else(|e| {
            println!("Error: {:?} is not valid config json file.", config_path);
            println!("  {}", e);
            process::exit(1)
        });
        Ok(config)
    }
}
