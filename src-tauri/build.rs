use std::{env, path::PathBuf};

fn main() {
    // change to virtualMIDI sdk install location
    let path = "C:/Program Files (x86)/Tobias Erichsen/teVirtualMIDISDK/C-Binding";
    // .lib or .dll path (no file extension)
    println!("cargo:rustc-link-search=native={}", path);
    // .lib or .dll name
    println!("cargo:rustc-link-lib=teVirtualMIDI64");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/teVirtualMIDI.h", path))
        // allowlists to stop IDE DDOS
        .allowlist_function("virtualMIDI.*")
        .allowlist_type("LPVM_.*")
        .allowlist_type("VM_.*")
        .allowlist_var("TE_VM_.*")
        .allowlist_var("VIRTUALMIDI_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    tauri_build::build()
}
