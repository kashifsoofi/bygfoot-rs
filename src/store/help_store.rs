
#[derive(Clone, Copy)]
pub struct FileHelpStore;

impl FileHelpStore {
    pub fn new() -> FileHelpStore {
        Self {}
    }

    pub fn get_contributors(self) -> Vec<Contributor> {
        let mut contributors: Vec<Contributor> = Vec::new();

        contributors.push(Contributor { title: "title 1".to_string(), entries: vec!["entry 1".to_string(), "entry 2".to_string()]});
        contributors.push(Contributor { title: "title 2".to_string(), entries: vec!["entry 1".to_string(), "entry 2".to_string()]});

        contributors
    }
}

pub struct Contributor {
    pub title: String,
    pub entries: Vec<String>,
}