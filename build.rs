fn main() {
    pkg_config::Config::new()
        .atleast_version("1.26.1")
        .probe("libinput")
        .expect("Failed to link to libinput");
}
