//! # Assets
//!
//! 
//!
use std::{
    borrow::Cow,
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
    io::{self, BufReader, Read},
    str,
};

type Result<T> = std::result::Result<T, io::Error>;

/// Produce an etag for an asset.
pub fn etag(path: &str) -> Cow<str> {
    if is_bundled() {
        Cow::from(crate::BUILD_DATE)
    } else {
        let mut hasher = DefaultHasher::new();
        last_modified(path).hash(&mut hasher);
        Cow::from(format!("{:x}", hasher.finish()))
    }
}

/// The last modified time for an asset on disk.
/// Does nothing if `is_bundled()` is true.
fn last_modified(path: &str) -> Option<String> {
    if is_bundled() {
        return None;
    }

    let path = normalize_path(path)?;
    if let Ok(meta) = fs::metadata(path) {
        if let Ok(time) = meta.modified() {
            return Some(format!("{:?}", time));
        }
    }
    None
}

/// Cleans a path of tricky things like `..` and puts it in a format
/// we can use in other asset functions.
pub fn normalize_path(path: &str) -> Option<String> {
    if let Some(root) = asset_dir() {
        Some(format!(
            "{}/{}",
            root,
            path.trim_start_matches(root)
                .trim_start_matches('.')
                .trim_start_matches('/')
                .replace("..", ".")
        ))
    } else {
        None
    }
}

/// Have assets been bundled into the binary?
pub fn is_bundled() -> bool {
    bundled_assets().is_some()
}

/// Access to read-only, in-memory assets in bundle mode.
fn bundled_assets() -> Option<&'static HashMap<String, &'static [u8]>> {
    unsafe { crate::BUNDLED_ASSETS.as_ref() }
}

/// The directory of the asset dir.
fn asset_dir() -> Option<&'static str> {
    unsafe { crate::ASSET_DIR }
}

/// Does the asset exist on disk? `path` is the path relative to
/// `ASSET_DIR` ex: asset::exists("index.html") checks for
/// "./static/index.html" if `ASSET_DIR` is set to `static`.
/// Works both in regular mode and bundle mode.
pub fn exists(path: &str) -> bool {
    if let Some(path) = normalize_path(path) {
        if is_bundled() {
            return bundled_assets().unwrap().contains_key(&path);
        } else {
            if let Ok(file) = fs::File::open(path) {
                if let Ok(meta) = file.metadata() {
                    return !meta.is_dir();
                }
            }
        }
    }
    false
}

/// Like fs::read_to_string(), but with an asset.
pub fn to_string(path: &str) -> Result<String> {
    if let Some(bytes) = read(path) {
        if let Ok(utf8) = str::from_utf8(bytes.as_ref()) {
            return Ok(utf8.to_string());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("{} not found", path),
    ))
}

/// Produces a boxed `io::Read` for an asset.
pub fn as_reader(path: &str) -> Option<Box<dyn io::Read>> {
    let path = normalize_path(path)?;
    if is_bundled() {
        if let Some(v) = bundled_assets().unwrap().get(&path) {
            return Some(Box::new(*v));
        }
    } else {
        if let Ok(file) = fs::File::open(path) {
            if let Ok(meta) = file.metadata() {
                if !meta.is_dir() {
                    return Some(Box::new(BufReader::new(file)));
                }
            }
        }
    }
    None
}

/// Read an asset to [u8].
pub fn read(path: &str) -> Option<Cow<'static, [u8]>> {
    let path = normalize_path(path)?;
    if is_bundled() {
        if let Some(v) = bundled_assets().unwrap().get(&path) {
            return Some(Cow::from(*v));
        }
    } else {
        let mut buf = vec![];
        if let Ok(mut file) = fs::File::open(path) {
            if let Ok(meta) = file.metadata() {
                if !meta.is_dir() && file.read_to_end(&mut buf).is_ok() {
                    return Some(Cow::from(buf));
                }
            }
        }
    }
    None
}
