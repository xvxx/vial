use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    io::{self, Read},
    path::{Path, PathBuf},
    str,
};

type Result<T> = std::result::Result<T, io::Error>;

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
pub fn to_string(path: &str) -> Result<String> {
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

pub fn iter(dir: &str) -> std::vec::IntoIter<PathBuf> {
    if let Ok(files) = files_in_dir(".") {
        files.into_iter()
    } else {
        vec![].into_iter()
    }
}

fn files_in_dir(path: &str) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let meta = fs::metadata(&path)?;
        if meta.is_dir() {
            files.extend_from_slice(&files_in_dir(path.to_str().unwrap_or("bad"))?);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter() {
        assert!(iter(".").count() > 0);

        let mut expected = vec!["./Cargo.toml", "./LICENSE"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        for file in iter(".").take(2) {
            assert_eq!(expected.remove(0), file.to_str().unwrap());
        }
    }
}
