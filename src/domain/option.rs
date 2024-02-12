
enum Value {
    StringValue(String),
    Value(i32),
}

pub struct Option {
    pub name: String,
    pub string_value: String,
    pub value: i32,
}

impl Option {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            string_value: "".to_string(),
            value: -1,
        }
    }
}

pub type OptionsList = Vec<Option>;