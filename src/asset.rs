use std::{
    borrow::Cow,
    fs,
    io::{self, Read},
    path::Path,
    str,
};

/// Does the asset exist on disk? `path` is the relative path,
/// ex: asset::exists("index.html") checks for "./static/index.html"
/// (or in the embedded fs, in release mode).
pub fn exists(path: &str) -> bool {
    let path = path.replace("..", ".");
    Path::new(&path).exists()
}

/// Like fs::read_to_string(), but with an asset.
pub fn to_string(path: &str) -> Result<String, io::Error> {
    let path = path.replace("..", ".");
    if let Some(bytes) = read(&path) {
        if let Ok(utf8) = str::from_utf8(bytes.as_ref()) {
            return Ok(utf8.to_string());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("{} not found", path),
    ))
}

/// Read a file to [u8].
pub fn read(path: &str) -> Option<Cow<'static, [u8]>> {
    let path = path.replace("..", ".");
    let mut buf = vec![];
    if let Ok(file) = fs::File::open(path) {
        if let Ok(meta) = file.metadata() {
            if !meta.is_dir() && file.read_to_end(&mut buf).is_ok() {
                return Some(Cow::from(buf));
            }
        }
    }
    None
}
