use failure::Error;
use serde_json::{self, Value};
use std::fs::File;
use std::path::PathBuf;

#[cfg(not(windows))]
static PROTOCOL_PREFIX: &'static str = "file://";
#[cfg(windows)]
static PROTOCOL_PREFIX: &'static str = "file:///";

pub fn qt_file_uri_to_path_buf(uri: &str) -> PathBuf {
    let pb = PathBuf::from(uri.replace(PROTOCOL_PREFIX, ""));
    debug!("stripped protocol: `{}`", pb.display());
    pb
}

pub fn guess_image_path(uri: &str) -> Result<PathBuf, Error> {
    let fp = qt_file_uri_to_path_buf(&uri);
    debug!("reading file: `{}`", fp.display());

    let fh = File::open(&fp).unwrap();
    let val: Value = serde_json::from_reader(fh)?;
    let image_path = val["meta"]["image"].as_str();
    debug!("{:?}", image_path);

    if let Some(fp) = image_path {
        let pb = PathBuf::from(fp);
        if pb.exists() {
            return Ok(pb);
        }
    }

    Err(format_err!(
        "Unable to guess location of image file: `{:?}`",
        image_path
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(windows))]
    #[test]
    fn test_convert_qt_uri() {
        assert_eq!(
            PathBuf::from("/home/jdoe/file.txt"),
            qt_file_uri_to_path_buf("file:///home/jdoe/file.txt")
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_convert_qt_uri() {
        assert_eq!(
            PathBuf::from("C:/Users/jdoe/file.txt"),
            qt_file_uri_to_path_buf("file:///C:/Users/jdoe/file.txt")
        );
    }
}
