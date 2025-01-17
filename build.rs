const FEATURES: &[&str] = &["1_22", "1_23", "1_24", "1_25", "1_26", "1_27"];

fn main() {
    let enabled_version = FEATURES
        .iter()
        .find(|f| std::env::var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok());

    let version = match enabled_version {
        None => {
            panic!(
                "No libinput version selected. You must enable exactly one of these features: {}",
                FEATURES.join(", ")
            );
        }
        Some(v) => v,
    };

    let enabled_count = FEATURES
        .iter()
        .filter(|f| std::env::var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok())
        .count();

    if enabled_count > 1 {
        panic!(
            "Multiple libinput versions selected. You must enable exactly one of these features: {}",
            FEATURES.join(", ")
        );
    }

    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let pkg_version = version.replace('_', ".");

    pkg_config::Config::new()
        .atleast_version(&pkg_version)
        .probe("libinput")
        .unwrap_or_else(|_| {
            panic!(
                "Failed to link to libinput version {}. Make sure it's installed on your system.",
                pkg_version
            )
        });

    println!("cargo:rerun-if-changed=src/logger.c");

    cc::Build::new()
        .file("src/logger.c")
        .flag_if_supported("-Wno-unused-parameter") // This gets a false positive because of libinput_log_handler
        .compile("logger");
}
