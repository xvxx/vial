use {
    crate::Result,
    std::{fs, path::PathBuf},
};

/// You should use the
/// [`vial::bundle_assets!()`](macro.bundle_assets.html) macro instead
/// of calling this function directly.
///
/// Prepares everything in `dir` to be bundled into a binary by
/// writing a `bundle.rs` file in `$OUT_DIR` and symlinking the assets
/// directory.
///
/// Sets `ASSET_DIR` and `cfg(bundle_assets)` for the user's program,
/// which uses them to find the assets.
///
/// Only runs in release mode.
#[doc(hidden)]
pub fn bundle_assets(dir: &str) -> Result<()> {
    #[cfg(not(debug_assertions))]
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        // symlink assets dir into out dir
        let link = Path::new(&out_dir).join(dir);
        if link.exists() {
            fs::remove_file(&link);
        }
        unix::fs::symlink(env::current_dir()?.join(dir), link)?;
        let bundle_rs = Path::new(&out_dir).join("bundle.rs");
        if bundle_rs.exists() {
            fs::remove_file(&bundle_rs);
        }
        let mut dest = File::create(bundle_rs)?;

        dest.write_all(
            b"{
    let mut map = std::collections::HashMap::new();
",
        )?;
        for path in walk(dir) {
            dest.write_all(
                format!(
                    "    map.insert({:?}.into(), &include_bytes!(\"{}\")[..]);\n",
                    path,
                    path.as_path().to_string_lossy().trim_start_matches("./")
                )
                .as_bytes(),
            );
        }
        dest.write_all(
            b"    map
}",
        )?;
        println!("cargo:rustc-cfg=bundle_assets");
    }
    println!("cargo:rustc-env=ASSET_DIR={}", dir);
    Ok(())
}

#[allow(dead_code)]
/// Iterator over all the files in a directory.
fn walk(dir: &str) -> std::vec::IntoIter<PathBuf> {
    if let Ok(files) = files_in_dir(dir) {
        files.into_iter()
    } else {
        vec![].into_iter()
    }
}

#[allow(dead_code)]
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
    fn test_walk() {
        assert!(walk(".").count() > 0);

        let mut expected = vec!["./Cargo.toml", "./LICENSE-APACHE"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        for file in walk(".").take(2) {
            assert_eq!(expected.remove(0), file.to_str().unwrap());
        }
    }
}
