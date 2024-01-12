use std::env;
use std::path::PathBuf;

fn main() {
    const RDKIT_ROOT: &str = "/home/brent/omsf/clone/rdkit";

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={RDKIT_ROOT}/build/lib");
    println!("cargo:rustc-link-search=./cpp");

    println!("cargo:rustc-link-lib=RDKitGraphMol");
    println!("cargo:rustc-link-lib=RDKitSmilesParse");
    println!("cargo:rustc-link-lib=RDKitFileParsers");
    println!("cargo:rustc-link-lib=shim");

    println!("cargo:rustc-env=LD_LIBRARY_PATH=./cpp:{RDKIT_ROOT}/build/lib");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{RDKIT_ROOT}/Code"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
