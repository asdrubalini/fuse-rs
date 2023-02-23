#[cfg(not(target_os = "macos"))]
const LIBFUSE_NAME: &str = "fuse";

#[cfg(target_os = "macos")]
const LIBFUSE_NAME: &str = "fuse-t";

fn main() {
    let version = if cfg!(target_os = "macos") {
        "1.0"
    } else {
        "2.6.0"
    };

    pkg_config::Config::new()
        .atleast_version(version)
        .probe(LIBFUSE_NAME)
        .map_err(|e| eprintln!("{}", e))
        .unwrap();
}
