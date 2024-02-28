
#[derive(Clone, Copy)]
pub struct FileHelpStore;

impl FileHelpStore {
    pub fn new() -> FileHelpStore {
        Self {}
    }

    pub fn get_contributors(self) -> Vec<String> {
        let mut contributors = Vec::new();

        contributors.push("1".to_string());
        contributors.push("2".to_string());
        contributors.push("3".to_string());

        contributors
    }
}