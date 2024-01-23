use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let rdroot = Path::new("/home/brent/omsf/clone/rdkit");
    let code = rdroot.join("build/Code");
    let external = rdroot.join("build/External");

    // let include = std::fs::canonicalize("./include").unwrap();
    // let include = include.display();
    // println!("cargo:rustc-link-search={include}");

    // println!("cargo:rustc-link-lib=shim");

    // println!("cargo:rustc-env=LD_LIBRARY_PATH={include}:{rdroot}/build/lib");

    // println!("cargo:rerun-if-changed=wrapper.h");
    // println!("cargo:rerun-if-env-changed=RDROOT");

    let headers_path = rdroot.join("Code");

    // make wrapper.o:
    //
    // g++ -I/home/brent/omsf/clone/rdkit/Code
    // /home/brent/omsf/clone/rdkit/build/External/RingFamilies/libRDKitRingDecomposerLib_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/RDGeneral/libRDKitRDGeneral_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/DataStructs/libRDKitDataStructs_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/GraphMol/libRDKitGraphMol_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/GraphMol/SmilesParse/libRDKitSmilesParse_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/RDBoost/libRDKitRDBoost_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/DataStructs/libRDKitDataStructs_static.a
    // -c -o wrapper.o wrapper.cpp
    if !Command::new("g++").arg("-I").arg(headers_path).args([
        "/home/brent/omsf/clone/rdkit/build/External/RingFamilies/libRDKitRingDecomposerLib_static.a",
    "/home/brent/omsf/clone/rdkit/build/Code/RDGeneral/libRDKitRDGeneral_static.a",
        "/home/brent/omsf/clone/rdkit/build/Code/DataStructs/libRDKitDataStructs_static.a",
    "/home/brent/omsf/clone/rdkit/build/Code/GraphMol/libRDKitGraphMol_static.a",
        "/home/brent/omsf/clone/rdkit/build/Code/GraphMol/SmilesParse/libRDKitSmilesParse_static.a",
    "/home/brent/omsf/clone/rdkit/build/Code/RDBoost/libRDKitRDBoost_static.a",
        "/home/brent/omsf/clone/rdkit/build/Code/DataStructs/libRDKitDataStructs_static.a",
    ]).arg("-c").arg("-o").arg("wrapper.o").arg("wrapper.cpp").output().unwrap().status.success() {
        panic!("failed to build wrapper.o");
    }

    if !Command::new("ar")
        .arg("rcs")
        .arg("/home/brent/Projects/rdkit-sys/try-static/libwrapper.a")
        .arg("/home/brent/Projects/rdkit-sys/try-static/wrapper.o")
        .output()
        .unwrap()
        .status
        .success()
    {
        panic!("failed to create libwrapper.a");
    }

    panic!();

    println!(
        "cargo:rustc-link-search=/home/brent/Projects/rdkit-sys/try-static"
    );
    for rddir in [
        "GraphMol",
        "GraphMol/SmilesParse",
        "RDGeneral",
        "RDBoost",
        "DataStructs",
        "Geometry",
    ] {
        println!("cargo:rustc-link-search={}/{rddir}", code.to_str().unwrap());
    }

    for extdir in ["RingFamilies"] {
        println!(
            "cargo:rustc-link-search={}/{extdir}",
            external.to_str().unwrap()
        );
    }

    println!("cargo:rustc-link-lib=wrapper");
    println!("cargo:rustc-link-lib=stdc++");
    for rdlib in [
        "GraphMol",
        "SmilesParse",
        "RDGeneral",
        "RingDecomposerLib",
        "DataStructs",
        "RDGeometryLib",
    ] {
        println!("cargo:rustc-link-lib=RDKit{rdlib}_static");
    }

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
