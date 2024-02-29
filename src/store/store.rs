use std::fs::read_dir;
use std::{env, fs};
use std::path::{Path, PathBuf};

use crate::domain::option::{Option, OptionsList};

use super::help_store::FileHelpStore;
use super::hints_store::FileHintsStore;

extern crate dirs;

static PACKAGE_DATA_DIR: &str = "../..";
static PACKAGE_LOCALE_DIR: &str = ".";
static HOME_DIR_NAME: &str = ".bygfoot";

static mut support_directories: Vec<String> = vec![];
static root_definitions_directories: Vec<String> = vec![];
static definitions_directories: Vec<String> = vec![];

#[derive(Clone, Copy)]
pub struct FileStore {
    hints_store: FileHintsStore,
    help_store: FileHelpStore,
}

impl FileStore {
    pub fn new() -> FileStore {
        let base_dir = get_current_dir();
        let base_dir = base_dir.join(PACKAGE_DATA_DIR);
        let support_files_directory = base_dir.join("support_files");
        add_support_directories_recursive(support_files_directory);
        let home_directory = home_dir();
        let home_directory = home_directory.join(HOME_DIR_NAME);
        add_support_directories_recursive(home_directory);

        Self {
            hints_store: FileHintsStore::new(),
            help_store: FileHelpStore::new(),
        }
    }

    pub fn hints_store(self) -> FileHintsStore {
        self.hints_store.clone()
    }

    pub fn help_store(self) -> FileHelpStore {
        self.help_store.clone()
    }
}

fn add_definitions_directory(_directory: PathBuf) {

}

fn add_support_directories_recursive(directory: PathBuf) {
    if !exists(directory.to_owned()) {
        return;
    }

    let directory_str = directory.to_str().unwrap();
    if directory_str.contains(".svn") || directory_str.contains(".git") {
        return;
    }

    add_definitions_directory(directory.to_owned());
    // add_pixmap_directory(directory);
    unsafe { support_directories.push(directory.to_str().unwrap().to_string()); }

    let entries = read_dir(directory).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        if meta.is_dir() {
            add_support_directories_recursive(entry.path());
        }
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
    let path = PathBuf::from(filename);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    
    unsafe {
    for directory in support_directories.iter() {
        let file_path = PathBuf::from(directory);
        let file_path = file_path.join(file_name);
        if exists(file_path.clone()) {
            return Some(file_path);
        }
    }
    }

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
        Some(index) => line[0..index].to_string(),
        None => line,
    };

    if line.is_empty() {
        return None;
    }

    let (name, value) = line.split_once(" ").unwrap();
    Some((name.to_string(), value.to_string()))
}