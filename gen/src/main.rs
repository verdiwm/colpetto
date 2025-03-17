use std::{fs, iter, path::Path};

use anyhow::{Context, Result};

const VERSIONS: &[&str] = &["1.27", "1.26", "1.25", "1.24", "1.23", "1.22"];

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

    for (version, next) in VERSIONS
        .into_iter()
        .zip(iter::once(None).chain(VERSIONS.into_iter().map(Some)))
    {
        dbg!(&version, &next);
        let bindings = base_builder
            .clone()
            .header(format!("src/headers/libinput-{version}.h"))
            .generate()
            .context("Failed to generate bindings")?;

        let version = version.replace(".", "_");
        let next = next.map(|next| next.replace(".", "_"));

        bindings
            .write_to_file(format!("../src/sys/sys_{version}.rs"))
            .context("Couldn't write bindings")?;

        let cfg = if let Some(next) = next {
            format!(r#"all(feature = "{version}", not(feature = "{next}"))"#)
        } else {
            format!(r#"feature = "{version}""#)
        };

        module.push_str(&format!(
            r#"#[cfg({cfg})]
mod sys_{version};
#[cfg({cfg})]
pub use sys_{version}::*;

"#
        ));
    }

    fs::write("../src/sys/mod.rs", &module)?;

    Ok(())
}
