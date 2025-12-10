use std::{collections::HashMap, fs, io::Write};

use serde_json::Value;
use uuid::Uuid;

use crate::json::validation::Validator;

/*
* loading everything to memory seems like kind of a bad idea, but I shall solve this problem
* later for now loading the entire file
*/
pub struct Doc {
    file_name: String,
    json: serde_json::Value,
}

impl Doc {
    pub fn new(json_string: String, object_id: &Uuid) -> Self {
        let file_name = format!("{}.json", object_id.to_string());

        let mut file = fs::File::create(&file_name).expect("failed to create file");
        file.write_all(json_string.as_bytes())
            .expect("failed to write to file");

        let json: Value = serde_json::from_str(&json_string).unwrap();

        return Doc { file_name, json };
    }
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

    pub fn insert(&mut self, json_string: String) -> Result<Vec<u8>, String> {
        match Validator::validate_json(&json_string) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => {
                let object_id = Uuid::new_v4();

                self.store.insert(
                    object_id.into_bytes().to_vec(),
                    Doc::new(json_string, &object_id),
                );

                return Ok(object_id.into_bytes().to_vec());
            }
        }
    }

    pub fn get(self, object_id: Vec<u8>) -> Option<String> {
        return self.store.get(&object_id).map(|doc| {
            return doc.json.to_string();
        });
    }

    pub fn get_all(self) -> Vec<String> {
        return self.store.values().map(|v| v.json.to_string()).collect();
    }
}
