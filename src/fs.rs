use failure::Error;
use serde_json::{self, Value};
use std::fs::File;
use std::path::PathBuf;
use url::Url;

fn strip_protocol(uri: &str) -> Result<String, Error> {
    debug!("parsing uri: `{}`", &uri);
    let parsed = Url::parse(&uri)?;
    let path = parsed.path();
    debug!("stripped protocol: `{}`", path);
    Ok(path.to_owned())
}

pub fn guess_image_path(uri: &str) -> Result<PathBuf, Error> {
    let fp = strip_protocol(&uri)?;
    debug!("reading file: `{}`", &fp);

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
