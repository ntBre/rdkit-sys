use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let rdroot = std::env::var("RDROOT")
        .unwrap_or_else(|_| "/home/brent/omsf/clone/rdkit".to_owned());

    let rdlibs = Path::new(&rdroot).join("build/lib").canonicalize().unwrap();
    let rdlibs = rdlibs.display();

    Command::new("make")
        .arg("include/libshim.so")
        .output()
        .unwrap();

    let include = std::fs::canonicalize("./include").unwrap();
    let include = include.display();

    for lib in [
        "GraphMol",
        "SmilesParse",
        "FileParsers",
        "Fingerprints",
        "MolDraw2D",
        "RDInchiLib",
    ] {
        println!("cargo:rustc-link-lib=dylib=RDKit{lib}");
    }
    println!("cargo:rustc-link-lib=dylib=shim");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{rdlibs},-rpath,{include}");
    println!("cargo:rustc-link-search=native={rdlibs}");
    println!("cargo:rustc-link-search=native={include}");

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
