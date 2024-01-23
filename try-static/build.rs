use std::env;
use std::path::PathBuf;
use std::{path::Path, process::Command};

fn main() {
    let rdroot = Path::new("/home/brent/omsf/clone/rdkit");
    let code = rdroot.join("build/Code");
    let external = rdroot.join("build/External");

    // wrapper.o: wrapper.cpp
    if !Command::new("g++")
        .arg("-c")
        .arg("-o")
        .arg("wrapper.o")
        .arg("wrapper.cpp")
        .arg("-I")
        .arg(rdroot.join("Code"))
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!("failed to build wrapper.o");
    }

    // libwrapper.a: wrapper.o
    if !Command::new("ar")
        .arg("rcs")
        .arg("libwrapper.a")
        .arg("wrapper.o")
        .arg("--thin")
        .args([
            external.join("RingFamilies/libRDKitRingDecomposerLib_static.a"),
            code.join("RDGeneral/libRDKitRDGeneral_static.a"),
            code.join("DataStructs/libRDKitDataStructs_static.a"),
            code.join("GraphMol/libRDKitGraphMol_static.a"),
            code.join("GraphMol/SmilesParse/libRDKitSmilesParse_static.a"),
            code.join("RDBoost/libRDKitRDBoost_static.a"),
            code.join("DataStructs/libRDKitDataStructs_static.a"),
            code.join("Geometry/libRDKitRDGeometryLib_static.a"),
        ])
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!("failed to create libwrapper.a");
    }

    // -L.
    println!(
        "cargo:rustc-link-search={}",
        Path::new(".").canonicalize().unwrap().to_str().unwrap()
    );
    // -lwrapper
    println!("cargo:rustc-link-lib=wrapper");

    println!("cargo:rustc-link-lib=boost_serialization");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=m");

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
