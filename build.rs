use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=RDROOT");

    let rdkit_root = std::env::var("RDROOT")
        .unwrap_or_else(|_| "/home/brent/omsf/clone/rdkit".to_owned());

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={rdkit_root}/build/lib");

    // let include = std::fs::canonicalize("./include").unwrap();
    // let include = include.display();
    // println!("cargo:rustc-link-search={include}");

    println!("cargo:rustc-link-lib=RDKitGraphMol");
    println!("cargo:rustc-link-lib=RDKitRDGeneral");
    println!("cargo:rustc-link-lib=RDKitSmilesParse");
    println!("cargo:rustc-link-lib=RDKitFileParsers");
    println!("cargo:rustc-link-lib=RDKitSubstructMatch");

    println!("cargo:rustc-env=LD_LIBRARY_PATH={rdkit_root}/build/lib");

    cc::Build::new()
        .cpp(true)
        .compiler("g++")
        .cpp_link_stdlib("stdc++")
        .file("include/shim.cpp")
        .flag(&format!("-I{rdkit_root}/Code"))
        .flag(&format!("-L{rdkit_root}/build/lib"))
        // .flag("-lboost_system")
        // .flag("-lboost_serialization")
        // .flag("-lboost_iostreams")
        .flag("-lRDKitGraphMol")
        .flag("-lRDKitRDGeneral")
        // .flag("-lRDKitSmilesParse")
        // .flag("-lRDKitFileParsers")
        .compile("shim");

    let bindings = bindgen::Builder::default()
        .header("include/shim.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
