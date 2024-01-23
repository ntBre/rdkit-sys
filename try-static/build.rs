use std::path::Path;

#[allow(unused)]
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
    // if !Command::new("g++")
    //     .arg("-c")
    //     .arg("-o")
    //     .arg("wrapper.o")
    //     .arg("wrapper.cpp")
    //     .arg("-I")
    //     .arg(rdroot.join("Code"))
    //     .output()
    //     .unwrap()
    //     .status
    //     .success()
    // {
    //     panic!("failed to build wrapper.o");
    // }

    // if !Command::new("ar")
    //     .arg("rcs")
    //     .arg("/home/brent/Projects/rdkit-sys/try-static/libwrapper.a")
    //     .arg("/home/brent/Projects/rdkit-sys/try-static/wrapper.o")
    //     .output()
    //     .unwrap()
    //     .status
    //     .success()
    // {
    //     panic!("failed to create libwrapper.a");
    // }

    // successful build from Makefile for try.c from libwrapper.a built here:
    // gcc -o try try.c -L. -lwrapper
    // /home/brent/omsf/clone/rdkit/build/External/RingFamilies/libRDKitRingDecomposerLib_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/RDGeneral/libRDKitRDGeneral_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/DataStructs/libRDKitDataStructs_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/GraphMol/libRDKitGraphMol_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/GraphMol/SmilesParse/libRDKitSmilesParse_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/RDBoost/libRDKitRDBoost_static.a
    // /home/brent/omsf/clone/rdkit/build/Code/DataStructs/libRDKitDataStructs_static.a
    // -static -L/home/brent/omsf/clone/rdkit/build/Code/GraphMol
    // -L/home/brent/omsf/clone/rdkit/build/Code/GraphMol/SmilesParse
    // -L/home/brent/omsf/clone/rdkit/build/Code/RDGeneral
    // -L/home/brent/omsf/clone/rdkit/build/Code/RDBoost
    // -L/home/brent/omsf/clone/rdkit/build/Code/DataStructs
    // -L/home/brent/omsf/clone/rdkit/build/Code/Geometry
    // -L/home/brent/omsf/clone/rdkit/build/External/RingFamilies
    // -lRDKitGraphMol_static -lRDKitSmilesParse_static -lRDKitRDGeneral_static
    // -lRDKitRingDecomposerLib_static -lRDKitDataStructs_static
    // -lRDKitRDGeometryLib_static -lboost_serialization -lstdc++ -lm

    // -L.
    println!(
        "cargo:rustc-link-search=/home/brent/Projects/rdkit-sys/try-static"
    );
    // -lwrapper
    println!("cargo:rustc-link-lib=wrapper");

    println!("cargo:rustc-link-lib=boost_serialization");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=m");
}
