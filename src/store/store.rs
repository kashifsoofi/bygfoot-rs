use std::{env, fs};
use std::path::{Path, PathBuf};

use crate::domain::option::{Option, OptionsList};

use super::hints_store::FileHintsStore;

extern crate dirs;

static HOME_DIR_NAME: &str = ".bygfoot";

pub struct FileStore {
    hints_store: FileHintsStore,
}

impl FileStore {
    pub fn new() -> FileStore {
        Self {
            hints_store: FileHintsStore::new(),
        }
    }

    pub fn hints_store(self) -> FileHintsStore {
        self.hints_store.clone()
    }
}

fn exists(path: PathBuf) -> bool {
    Path::new(&path).exists()
}

fn home_dir() -> PathBuf {
    dirs::home_dir().expect("get home failed")
}

fn get_current_dir() -> PathBuf {
    let mut dir = env::current_exe().expect("get current path failed");
    dir.pop();
    dir
}

fn os_is_unix() -> bool {
    env::consts::FAMILY == "unix"
}

pub fn get_bygfoot_dir() -> PathBuf {
    if os_is_unix() {
        let mut home = dirs::home_dir().expect("get home failed");
        home.push(".bygfoot");
        return home;
    }

    get_current_dir()
}

pub fn find_support_file(filename: String, warning: bool) -> std::option::Option<PathBuf> {
    None
}

pub fn load_options_file(filename: String, warning: bool) -> OptionsList {
    // let support_file = find_support_file(filename, warning);
    // let support_file = support_file.unwrap();
    let current_dir = get_current_dir();
    let support_file = PathBuf::from(filename);
    let content = fs::read_to_string(support_file).unwrap();

    let mut options_list = OptionsList::new();
    for line in content.lines() {
        match parse_option_line(line.to_string()) {
            None => continue,
            Some((name, value)) => {
                let mut option = Option::new(name.clone());
                if name.starts_with("string_") {
                    option.string_value = value.clone();
                } else {
                    let int_value = value.parse::<i32>().unwrap();
                    option.value = int_value;
                }
                options_list.push(option);

                if name.ends_with("_unix") && os_is_unix() {
                    let name = name.replace("_unix", "");
                    let mut option = Option::new(name);
                    option.string_value = value.clone();
                    options_list.push(option);
                } else if name.ends_with("_win32") && !os_is_unix() {
                    let name = name.replace("_win32", "");
                    let mut option = Option::new(name);
                    option.string_value = value.clone();
                    options_list.push(option);
                }
            }
        }
    }

    options_list
}

fn parse_option_line(line: String) -> std::option::Option<(String, String)> {
    let line = match line.find('#') {
        Some(index) => line[index..].to_string(),
        None => line,
    };

    if line.is_empty() {
        return None;
    }

    let (name, value) = line.split_once(" ").unwrap();
    Some((name.to_string(), value.to_string()))
}