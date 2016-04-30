extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    // Set cfg flags depending on release channel
    let channel = match version_meta().channel {
        Channel::Stable => "stable",
        Channel::Beta => "beta",
        Channel::Nightly | Channel::Dev => "nightly",
    };
    println!("cargo:rustc-cfg=channel={:?}", channel);
}
