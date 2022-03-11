use bindgen::{Builder, CargoCallbacks};
use std::{env, path::PathBuf};

fn main() {
    link_openvr();
}

#[cfg(target_os = "linux")]
fn link_openvr() {}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn link_openvr() {
    let out_dir: PathBuf = env::var("OUT_DIR").expect("Should be set").into();
    let mut openvr_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("Should be set")
        .into();
    openvr_dir.push("openvr");

    // Link openvr_api.lib
    println!(
        "cargo:rustc-link-search={}",
        openvr_dir.join("lib/win64").to_string_lossy()
    );
    println!("cargo:rustc-link-lib=openvr_api");

    // Generate bindings.rs
    let bindings = Builder::default()
        .header(openvr_dir.join("headers/openvr_capi.h").to_string_lossy())
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Failed to generate OpenVR bindings");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Failed to write OpenVR bindings");

    /*
    // Copy openvr_api.dll to OUT_DIR
    copy(
        openvr_dir.join("bin/win64/openvr_api.dll"),
        out_dir.join("openvr_api.dll"),
    )
    .expect("Failed to copy openvr_api.dll");
    */
}
