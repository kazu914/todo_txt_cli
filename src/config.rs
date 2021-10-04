use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use shellexpand;
use std::{fmt, fs, fs::OpenOptions, io::BufReader, io::Error, process};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_file_name: String,
}

impl Config {
    pub fn new() -> Config {
        let project_dirs = ProjectDirs::from("", "", "todo_txt").unwrap();
        let _ = fs::create_dir_all(project_dirs.config_dir());
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
        let mut config: Config = serde_json::from_reader(reader).unwrap_or_else(|e| {
            println!("Error: {:?} is not valid config json file.", config_path);
            println!("  {}", e);
            process::exit(1)
        });
        config.default_file_name = shellexpand::full(&config.default_file_name)
            .unwrap_or_else(|e| {
                println!("Error: {:?} is not valid config json file.", config_path);
                println!("  {}", e);
                process::exit(1)
            })
            .to_string();
        Ok(config)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "default_file_name: {}", self.default_file_name)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}
