use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    io::{self, Read},
    path::Path,
    str,
};

pub fn hash(path: &str) -> String {
    let mut hasher = DefaultHasher::new();
    last_modified(path).hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn last_modified(path: &str) -> String {
    let path = normalize_path(path);
    if let Ok(meta) = fs::metadata(path) {
        if let Ok(time) = meta.modified() {
            return format!("{:?}", time);
        }
    }
    String::new()
}

pub fn normalize_path(path: &str) -> String {
    format!(
        "./{}",
        path.trim_start_matches('.')
            .trim_start_matches('/')
            .replace("..", ".")
    )
}

/// Does the asset exist on disk? `path` is the relative path,
/// ex: asset::exists("index.html") checks for "./static/index.html"
/// (or in the embedded fs, in release mode).
pub fn exists(path: &str) -> bool {
    let path = normalize_path(path);
    if let Ok(mut file) = fs::File::open(path) {
        if let Ok(meta) = file.metadata() {
            return !meta.is_dir();
        }
    }
    false
}

/// Like fs::read_to_string(), but with an asset.
pub fn to_string(path: &str) -> Result<String, io::Error> {
    let path = normalize_path(path);
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
    let path = normalize_path(path);
    let mut buf = vec![];
    if let Ok(mut file) = fs::File::open(path) {
        if let Ok(meta) = file.metadata() {
            if !meta.is_dir() && file.read_to_end(&mut buf).is_ok() {
                return Some(Cow::from(buf));
            }
        }
    }
    None
}
