use std::collections::HashMap;

pub trait GetValue {
    fn get_str(&self, key: &str) -> Option<String>;
    fn get_int(&self, key: &str) -> Option<usize>;
    fn get_bool(&self, key: &str) -> Option<bool>;
}

pub trait SetValue {
    fn set_str(&mut self, key: &str, value: &str) -> Option<String>;
    fn set_int(&mut self, key: &str, value: usize) -> Option<String>;
    fn set_bool(&mut self, key: &str, value: bool) -> Option<String>;
}

pub struct LocalVars {
    pub store: HashMap<String, String>,
}

impl LocalVars {
    pub fn new() -> Self {
        LocalVars {
            store: HashMap::from([
                ("dummy_mode".to_owned(), "yes".to_owned()),
                ("suicide_mode".to_owned(), "no".to_owned()),
            ]),
        }
    }
}

impl GetValue for LocalVars {
    fn get_str(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    fn get_int(&self, key: &str) -> Option<usize> {
        self.store
            .get(key)
            .and_then(|value| value.parse::<usize>().ok())
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        self.store
            .get(key)
            .and_then(|value| match value.as_str() {
                "true"  | "yes" | "y" | "1" => Some(true),
                "false" | "no"  | "n" | "0" => Some(false),
                _ => None,
            })
    }
}

impl SetValue for LocalVars {
    fn set_str(&mut self, key: &str, value: &str) -> Option<String> {
        self.store.insert(key.to_owned(), value.to_owned())
    }

    fn set_int(&mut self, key: &str, value: usize) -> Option<String> {
        self.store.insert(key.to_owned(), value.to_string())
    }

    fn set_bool(&mut self, key: &str, value: bool) -> Option<String> {
        let value_str = if value { "true" } else { "false" };
        self.store.insert(key.to_owned(), value_str.to_owned())
    }
}
