use std::fs::File;
use fs;
use interface::*;
use serde_json::Value;
use store::Store;

pub struct StoreWrapper {
    emit: StoreWrapperEmitter,
    data: Store,
}

impl StoreWrapperTrait for StoreWrapper {
    fn new(emit: StoreWrapperEmitter) -> StoreWrapper {
        StoreWrapper {
            emit,
            data: Store::new(),
        }
    }

    fn emit(&mut self) -> &mut StoreWrapperEmitter {
        &mut self.emit
    }

    fn sheet_path(&self) -> Option<&str> {
        match self.data.sheet_path.as_ref() {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }

    fn set_sheet_path(&mut self, value: Option<String>) {
        self.data.sheet_path = value.map(|s| fs::qt_file_uri_to_path_buf(&s).display().to_string());
        if let Some(fp) = &self.data.sheet_path {
            let f = File::open(fp).unwrap();
            self.data.raw_sheet_data = Some(serde_json::from_reader(f).unwrap());
        }
        debug!("{:?}", self.data);
    }

    fn image_path(&self) -> Option<&str> {
        match self.data.image_path.as_ref() {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }

    fn set_image_path(&mut self, value: Option<String>) {
        self.data.image_path = value.map(|s| fs::qt_file_uri_to_path_buf(&s).display().to_string());
        debug!("{:?}", self.data);
    }
}
