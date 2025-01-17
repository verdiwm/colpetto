const FEATURES: &[&str] = &["1_22", "1_23", "1_24", "1_25", "1_26", "1_27"];

fn main() {
    let enabled = FEATURES
        .iter()
        .filter(|f| std::env::var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok())
        .count();

    if enabled == 0 {
        panic!(
            "No libinput version selected. You must enable exactly one of these features: {}",
            FEATURES.join(", ")
        );
    }

    if enabled > 1 {
        panic!(
            "Multiple libinput versions selected. You must enable exactly one of these features: {}",
            FEATURES.join(", ")
        );
    }

    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    pkg_config::Config::new()
        .atleast_version("1.27")
        .probe("libinput")
        .expect("Failed to link to libinput");

    println!("cargo::rerun-if-changed=src/logger.c");

    cc::Build::new()
        .file("src/logger.c")
        .flag_if_supported("-Wno-unused-parameter") // This gets a false positive because of libinput_log_handler
        .compile("logger");
}
