use {
    crate::{asset, Result},
    std::{
        env, fs,
        io::{self, Read, Write},
        path::Path,
    },
};

pub fn bundle_assets(dir: &str) -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| ".".into());
    println!("OUT: {}", out_dir);
    let mut dest = fs::File::create(Path::new(&out_dir).join("bundled_assets.rs"))?;
    println!("OK");

    dest.write_all(b"use std::collections::HashMap;\n")?;
    dest.write_all(b"pub fn bundled_assets() -> HashMap<String, Vec<u8>>{\n")?;
    dest.write_all(b"  let mut map = HashMap::new();\n")?;
    for path in asset::iter(dir) {
        println!("-> {:?}", path);
        let mut bytes = vec![];
        let mut asset = fs::File::open(Path::new(&path))?;
        asset.read_to_end(&mut bytes);
        dest.write_all(format!("  map.insert({:?}.into(), vec!{:?});\n", path, bytes).as_bytes());
    }
    dest.write_all(b"  map\n}\n")?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
