use std::collections::HashMap;

pub trait ToMap {
    fn to_map(&self) -> HashMap<String, String>;
}

// {"name": "usashi", "age": "34"}
