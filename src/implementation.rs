use serde_json::Value;
use std::fs::File;

use crate::{fs, interface::*, store::Store};

pub struct StoreWrapper {
    emit: StoreWrapperEmitter,
    data: Store,
}

impl StoreWrapperTrait for StoreWrapper {
    fn new(emit: StoreWrapperEmitter) -> StoreWrapper {
        let maybe_sheet = std::env::args().nth(1);
        let maybe_image = std::env::args().nth(2);

        let mut wrapper = StoreWrapper {
            emit,
            data: Store::new(),
        };

        wrapper.set_sheet_path(
            maybe_sheet.map(|path| std::fs::canonicalize(path).unwrap().display().to_string()),
        );

        if let Some(path) = maybe_image {
            wrapper.set_image_path(Some(
                std::fs::canonicalize(path).unwrap().display().to_string(),
            ))
        }

        wrapper
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
        let sheet_path = value.map(|s| fs::qt_file_uri_to_path_buf(&s).display().to_string());
        if let Some(fp) = &sheet_path {
            let f = File::open(fp).unwrap();
            let data: Value = serde_json::from_reader(f).unwrap();
            let image_path = data["meta"]["image"].as_str().map(|s| s.to_string());
            self.set_image_path(image_path);
            self.data.raw_sheet_data = Some(data);
        }
        self.data.sheet_path = sheet_path;
        self.emit().sheet_path_changed();

        //        debug!("{:?}", self.data);
    }

    fn image_path(&self) -> Option<&str> {
        match self.data.image_path.as_ref() {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }

    fn set_image_path(&mut self, value: Option<String>) {
        let image_path = value.map(|s| fs::qt_file_uri_to_path_buf(&s).display().to_string());
        self.data.image_path = image_path;
        self.emit().image_path_changed();
        //        debug!("{:?}", self.data);
    }
}
