use interface::*;

pub struct Store {
    emit: StoreEmitter,
    data_path: Option<String>,
    image_path: Option<String>,
}

impl StoreTrait for Store {
    fn new(emit: StoreEmitter) -> Store {
        Store {
            emit,
            data_path: None,
            image_path: None,
        }
    }
    fn emit(&mut self) -> &mut StoreEmitter {
        &mut self.emit
    }

    fn guess_image_path(&self, uri: String) -> bool {
        match ::fs::guess_image_path(&uri) {
            Ok(_) => true,
            Err(e) => {
                error!("{:?}", e);
                false
            }
        }
    }

    fn data_path(&self) -> Option<&str> {
        match self.data_path.as_ref() {
            Some(s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn image_path(&self) -> Option<&str> {
        match self.image_path.as_ref() {
            Some(s) => Some(s.as_str()),
            _ => None,
        }
    }
}
