use std::env;
use std::path::PathBuf;

fn main() {
    /// directory where openmm was built
    const RDKIT_ROOT: &str = "/home/brent/omsf/clone/rdkit";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={RDKIT_ROOT}/build/lib");

    // Tell cargo to tell rustc to link the libOpenMM.so from that directory
    println!("cargo:rustc-link-lib=RDKitGraphMol");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point to bindgen, and lets you
    // build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.hpp")
        .clang_arg(format!("-I{RDKIT_ROOT}/Code"))
        .clang_arg(format!("-I{RDKIT_ROOT}/Code/GraphMol"))
        .clang_arg("-std=c++17")
        // Tell cargo to invalidate the built crate whenever any of the included
        // header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("RWMol")
        .allowlist_type("ROMol")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
