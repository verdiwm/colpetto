const FEATURES: &[&str] = &["1_22", "1_23", "1_24", "1_25", "1_26", "1_27", "1_28"];

fn main() {
    let enabled_count = FEATURES
        .iter()
        .filter(|f| std::env::var(format!("CARGO_FEATURE_{}", f.to_uppercase())).is_ok())
        .count();

    if enabled_count != 1 {
        panic!(
            "You must enable exactly one libinput version feature: {}",
            FEATURES.join(", ")
        );
    }

    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // The required libinput version is declared in `[package.metadata.system-deps]`,
    // gated on the selected version feature. system-deps probes the matching module,
    // and the same table is exposed via `cargo metadata` for build-less discovery.
    system_deps::Config::new().probe().unwrap();

    println!("cargo:rerun-if-changed=src/logger.c");

    cc::Build::new()
        .file("src/logger.c")
        .flag_if_supported("-Wno-unused-parameter") // This gets a false positive because of libinput_log_handler
        .compile("colpetto_logger");
}
