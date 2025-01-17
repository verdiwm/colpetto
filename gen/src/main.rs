use anyhow::Result;

fn main() -> Result<()> {
    let bindings = bindgen::Builder::default()
        .header("/usr/include/libinput.h")
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
        .constified_enum_module("libinput_event_type")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("../src/sys.rs")
        .expect("Couldn't write bindings!");
    Ok(())
}
