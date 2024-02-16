use rustc_version::{version_meta, Channel};

fn main() {
    let version_meta = version_meta().unwrap();
    if version_meta.channel == Channel::Nightly {
        println!("cargo:rustc-cfg=nightly_build");
    }
    if version_meta.semver.minor > 75 {
        println!("cargo:rustc-cfg=simd_prelude");
    }

}
