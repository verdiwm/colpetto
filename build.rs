fn main() {
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
