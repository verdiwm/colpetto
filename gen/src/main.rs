use std::{fs, path::Path};

use anyhow::{Context, Result};

const VERSIONS: &[&str] = &["1.26", "1.27"];

fn main() -> Result<()> {
    let base_builder = bindgen::builder()
        .allowlist_type("^libinput_.*$")
        .allowlist_function("^libinput_.*$")
        .use_core()
        .generate_cstr(true)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .constified_enum("libinput_device_capability")
        .prepend_enum_name(false)
        .constified_enum_module("libinput_event_type");

    if Path::new("../src/sys").exists() {
        fs::remove_dir_all("../src/sys")?;
    }

    fs::create_dir_all("../src/sys")?;

    let mut module = String::new();

    for version in VERSIONS {
        let bindings = base_builder
            .clone()
            .header(format!("src/headers/libinput-{version}.h"))
            .generate()
            .context("Failed to generate bindings")?;

        let version = version.replace(".", "_");

        bindings
            .write_to_file(format!("../src/sys/sys_{version}.rs"))
            .context("Couldn't write bindings")?;

        module.push_str(&format!(
            r#"
        #[cfg(feature = "{version}")]
        mod sys_{version};
        #[cfg(feature = "{version}")]
        pub use sys_{version}::*;
        "#
        ));
    }

    fs::write("../src/sys/mod.rs", &module)?;

    Ok(())
}
