//! Vial can serve static files out of an asset directory and
//! optionally bundle them into your application in `--release` mode.
//!
//! By setting an asset directory, either through the
//! [`vial::asset_dir!()`][macro.asset_dir.html] or
//! [`vial::bundle_assets!()`][macro.bundle_assets.html] macro,
//! you can then use the methods in this module to work with them:
//!
//! - **[asset::etag()](#method.etag)**: Get the ETag for an asset.
//!   Used automatically by the Router if a web request matches an
//!   asset's path.
//! - **[asset::exists()](#method.exists)**: Does an asset exist?
//!   Works regardless of whether the asset is bundled or not.
//! - **[asset::is_bundled()](#method.is_bundled)**: Are assets
//!   bundled? Only true in `--release` mode and when used with the
//!   `vial::bundle_assets!()` macro.
//! - **[asset::to_string()](#method.to_string)**: Like
//!   `fs::read_to_string()`, delivers the content of an asset as a
//!   `String`.
//! - **[asset::as_reader()](#method.as_reader)**: Like
//!   `asset::to_string()` but provides an `io::Read` of an asset,
//!   whether or not it's bundled.
//!
//! To get started, put all your `.js` and `.css` and other static
//! assets into a directory in the root of your project, then
//! reference them in HTML  as if the root of your Vial web
//! application was that asset directory.
//!
//! Next call [`vial::asset_dir!()`][macro.asset_dir.html] with the
//! path to your asset directory (maybe `assets/`?) before starting
//! your application with [`vial::run!`](macro.run.html):
//!
//! If we had a directory structure like this:
//!     .
//!     ├── README.md
//!     ├── assets
//!     │   └── img
//!     │       ├── banker.png
//!     │       └── doctor.png
//!     └── src
//!         └── main.rs
//!
//! We could serve our images like so:
//!
//! ```no_run
//! vial::routes! {
//!     GET "/" => |_| "
//!         <p><img src='/img/doctor.png'/></p>
//!         <p><img src='/img/banker.png'/></p>
//!     ";
//! }
//!
//! fn main() {
//!     vial::asset_dir!("assets/");
//!     vial::run!().unwrap();
//! }
//! ```
//!
//!
use {
    crate::{util, Error, Result},
    std::{
        borrow::Cow,
        collections::{hash_map::DefaultHasher, HashMap},
        fs,
        hash::{Hash, Hasher},
        io::{BufReader, Read},
        str,
    },
};

/// Produce an etag for an asset.
pub fn etag(path: &str) -> Cow<'_, str> {
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
    asset_dir().map(|root| {
        format!(
            "{}/{}",
            root.trim_end_matches('/'),
            path.trim_start_matches(root)
                .trim_start_matches('.')
                .trim_start_matches('/')
                .replace("..", ".")
        )
    })
}

/// Have assets been bundled into the binary?
pub fn is_bundled() -> bool {
    bundled_assets().is_some()
}

/// Access to read-only, in-memory assets in bundle mode.
fn bundled_assets() -> Option<&'static HashMap<String, &'static [u8]>> {
    unsafe { crate::BUNDLED_ASSETS.as_ref() }
}

/// Size of an asset in `asset_dir()`. `0` if the asset doesn't exist.
/// Works in bundled mode and regular mode.
pub fn size(path: &str) -> usize {
    if !exists(path) {
        return 0;
    }

    let path = match normalize_path(path) {
        Some(path) => path,
        None => return 0,
    };

    if is_bundled() {
        bundled_assets()
            .unwrap()
            .get(&path)
            .map(|a| a.len())
            .unwrap_or(0)
    } else {
        util::file_size(&path)
    }
}

/// The directory of the asset dir.
fn asset_dir() -> Option<&'static String> {
    unsafe { crate::ASSET_DIR.as_ref() }
}

/// Does the asset exist on disk? `path` is the path relative to
/// `ASSET_DIR` ex: asset::exists("index.html") checks for
/// "./static/index.html" if `ASSET_DIR` is set to `static`.
/// Works both in regular mode and bundle mode.
pub fn exists(path: &str) -> bool {
    if let Some(path) = normalize_path(path) {
        if is_bundled() {
            return bundled_assets().unwrap().contains_key(&path);
        } else if let Ok(file) = fs::File::open(path) {
            if let Ok(meta) = file.metadata() {
                return !meta.is_dir();
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

    Err(Error::AssetNotFound(path.into()))
}

/// Produces a boxed `io::Read` for an asset.
pub fn as_reader(path: &str) -> Option<Box<dyn Read>> {
    let path = normalize_path(path)?;
    if is_bundled() {
        if let Some(v) = bundled_assets().unwrap().get(&path) {
            return Some(Box::new(*v));
        }
    } else if let Ok(file) = fs::File::open(path) {
        if let Ok(meta) = file.metadata() {
            if !meta.is_dir() {
                return Some(Box::new(BufReader::new(file)));
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
