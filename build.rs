use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let rdroot = std::env::var("RDROOT")
        .unwrap_or_else(|_| "/home/brent/omsf/clone/rdkit".to_owned());

    Command::new("make")
        .arg("include/libshim.so")
        .output()
        .unwrap();

    let include = std::fs::canonicalize("./include").unwrap();
    let include = include.display();
    println!("cargo:rustc-link-search={include}");

    println!("cargo:rustc-link-lib=shim");

    println!("cargo:rustc-env=LD_LIBRARY_PATH={include}:{rdroot}/build/lib");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=RDROOT");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
