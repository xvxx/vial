use {
    crate::{asset, Result},
    std::{
        env,
        fs::{self, File},
        io::{self, Read, Write},
        os::unix,
        path::Path,
    },
};

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
        for path in asset::iter(dir) {
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
