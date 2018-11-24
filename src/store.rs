//! Centralized data store for the app state.

use serde_json::Value;

#[derive(Default, Debug)]
pub struct Store {
    pub raw_sheet_data: Option<Value>,
    pub image_path: Option<String>,
    pub sheet_path: Option<String>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            ..::std::default::Default::default()
        }
    }
}
