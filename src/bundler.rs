use {
    crate::{asset, Result},
    std::{
        env,
        fs::{self, File},
        io::{self, Read, Write},
        path::Path,
    },
};

pub fn bundle_assets(dir: &str) -> Result<()> {
    #[cfg(not(debug_assertions))]
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        let mut dest = File::create(Path::new(&out_dir).join("bundle.rs"))?;

        dest.write_all(b"#[macro_export]\n macro_rules! vial_bundled_assets {\n  () => {{\n")?;
        dest.write_all(b"    let mut map = std::collections::HashMap::new();\n")?;
        for path in asset::iter(dir) {
            println!("-> {:?}", path);
            dest.write_all(
                format!(
                    "    map.insert({:?}.into(), &include_bytes!(\"../{}\")[..]);\n",
                    path,
                    path.as_path().to_string_lossy()
                )
                .as_bytes(),
            );
        }
        dest.write_all(b"    map\n  }};\n}")?;
        println!("cargo:rustc-cfg=bundle_assets");
        println!("cargo:rustc-cfg=release");
    }
    println!("cargo:rustc-env=ASSET_DIR={}", dir);
    Ok(())
}
