use super::store::{get_bygfoot_dir, find_support_file, load_options_file};

#[derive(Clone, Copy)]
pub struct FileHelpStore;

const HELP_FILE_NAME: &str = "bygfoot_help";

impl FileHelpStore {
    pub fn new() -> FileHelpStore {
        Self {}
    }

    pub fn get_contributors(self) -> Vec<Contributor> {
        let mut contributors: Vec<Contributor> = Vec::new();

        let help_file_path = find_support_file(HELP_FILE_NAME.to_string(), false);
        if help_file_path == None {
            return contributors;
        }

        let help_file_path = help_file_path.unwrap().to_str().unwrap().to_string();
        let option_list = load_options_file(help_file_path, false);
        let mut i = 0;
        while i < option_list.len() {
            if option_list[i].name == "string_contrib_title" {
                contributors.push(Contributor { title: option_list[i].string_value.clone(), entries: vec![] });
            } else {
                let index = contributors.len() - 1;
                contributors[index].entries.push(option_list[i].string_value.clone());
            }
            i += 1
        }

        contributors
    }
}

pub struct Contributor {
    pub title: String,
    pub entries: Vec<String>,
}