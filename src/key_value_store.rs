use std::collections::HashMap;
/*
* loading everything to memory seems like kind of a bad idea, but I shall solve this problem
* later for now loading the entire file
*/
pub struct Doc {
    file_name: &'static String,
    json: serde_json::Value,
}

/*
* Naive Implementation for now, but I plan to implement a b-tree implementation later
* */
pub struct DocumentStore {
    store_name: String,
    store: HashMap<Vec<u8>, Doc>,
}

impl DocumentStore {
    pub fn new(store_name: &str) -> Self {
        return DocumentStore {
            store_name: store_name.to_string(),
            store: HashMap::new(),
        };
    }
}
