use std::path::{Path, PathBuf};
use std::{env, fs};

use super::store::get_bygfoot_dir;

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
        vec!(
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "11".to_string(),
            "12".to_string(),
            "13".to_string(),
            "14".to_string(),
        )
    }
}
