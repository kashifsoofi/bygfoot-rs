use std::{env, fs};
use std::path::{Path, PathBuf};

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

fn find_support_file(filename: String) -> String {
    "".to_string()
}