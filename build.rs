// TODO: remove once DS1302 arrives

#![feature(
    multiple_supertrait_upcastable,
    non_exhaustive_omitted_patterns_lint,
    strict_provenance_lints,
    unqualified_local_imports
)]

use std::time::SystemTime;

fn main() {
    println!("cargo::rerun-if-changed=always-rebuild");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo::rustc-env=BUILD_TIMESTAMP={timestamp}");
}
