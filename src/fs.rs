use failure::Error;
use serde_json::{self, Value};
use std::fs::File;
use std::path::PathBuf;


#[cfg(not(windows))]
fn normalize(path: &str) -> PathBuf {
    PathBuf::from(path)
}

#[cfg(windows)]
fn normalize(path: &str) -> PathBuf {
    // Windows is odd. The uri comes in as like `/C:/foo/bar/...` after we
    // chop off the file protocol scheme from the front.

    // Slice [1..2] should be the drive letter. Pull it out...
    let drive = &path[1..2];
    let mut buf = PathBuf::from(&format!(r"{}:\", drive));
    // ... then after the drive letter, push in each path segment after the
    // drive letter.
    for segment in path[4..].split("/") {
        buf.push(segment);
    }
    buf
}

fn qt_file_uri_to_path_buf(uri: &str) -> Result<PathBuf, Error> {
    let pb = normalize(&uri.replace("file://", ""));
    debug!("stripped protocol: `{}`", pb.display());
    Ok(pb)
}

pub fn guess_image_path(uri: &str) -> Result<PathBuf, Error> {
    let fp = qt_file_uri_to_path_buf(&uri)?;
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
            qt_file_uri_to_path_buf("file:///home/jdoe/file.txt").unwrap()
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_convert_qt_uri() {
        assert_eq!(
            PathBuf::from("C:\\Users\\jdoe\\file.txt"),
            qt_file_uri_to_path_buf("file:///C:/Users/jdoe/file.txt").unwrap()
        );
    }
}
