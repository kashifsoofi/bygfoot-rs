use std::path::{Path, PathBuf};
use std::{env, fs};

use super::store::{get_bygfoot_dir, find_support_file, load_options_file};
use crate::domain::option::{self, OptionsList};

#[derive(Clone, Copy)]
pub struct FileHintsStore;

impl FileHintsStore {
    pub fn new() -> FileHintsStore {
        Self {}
    }

    pub fn load_hint_number(self) -> i32 {
        let mut path = get_bygfoot_dir();
        path.push("hint_num");
    
        if !Path::new(&path).exists() {
            return 0;
        }
    
        let data = fs::read_to_string(path).expect("unable to read file");
        data.parse::<i32>().unwrap()
    }

    pub fn save_hint_number(self, hint_num: i32) {
        let mut path = get_bygfoot_dir();
        path.push("hint_num");
        
        fs::write(path, &hint_num.to_string()).expect("write failed");
    }

    pub fn get_hints(self) -> Vec<String> {
        let mut hints = Vec::new();

        let option_list = load_hints_file();
        for option in option_list.iter() {
            hints.push(option.string_value.clone())
        }
        hints
    }

}

fn get_language_code() -> String {
    return "en".to_string()
}

fn load_hints_file() -> OptionsList {
    let mut hints_filename = format!("bygfoot_hints_{})", get_language_code());
    let hints_filepath = find_support_file(hints_filename.clone(), false);
    if hints_filepath == None {
        hints_filename = "bygfoot_hints_en".to_string()
    }

    load_options_file(hints_filename, false)
}
