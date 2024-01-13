use std::env;
use std::path::PathBuf;

fn main() {
    let rdkit_root = std::env::var("RDROOT")
        .unwrap_or_else(|_| "/home/brent/omsf/clone/rdkit".to_owned());

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={rdkit_root}/build/lib");
    println!("cargo:rustc-link-search=./include");

    println!("cargo:rustc-link-lib=RDKitGraphMol");
    println!("cargo:rustc-link-lib=RDKitSmilesParse");
    println!("cargo:rustc-link-lib=RDKitFileParsers");
    println!("cargo:rustc-link-lib=shim");

    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH=./include:{rdkit_root}/build/lib"
    );

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=RDROOT");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{rdkit_root}/Code"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
