use std::env;
use std::path::PathBuf;
use std::{path::Path, process::Command};

fn run_cmd(mut cmd: Command) {
    let out = cmd.output().unwrap();
    if !out.status.success() {
        panic!("failed to run command with {out:?}");
    }
}

fn main() {
    let rdroot = Path::new("/home/brent/omsf/clone/rdkit");
    let code = rdroot.join("build/Code");
    let external = rdroot.join("build/External");

    let mut wrapper_o = Command::new("g++");
    wrapper_o
        .arg("-c")
        .arg("-o")
        .arg("include/wrapper.o")
        .arg("include/shim.cpp")
        .arg("-I")
        .arg(rdroot.join("Code"))
        .arg("-I")
        .arg(rdroot.join("Code/DataStructs"))
        .arg("-I")
        .arg(rdroot.join("External/INCHI-API"));
    run_cmd(wrapper_o);

    // libwrapper.a: wrapper.o
    let mut libwrapper = Command::new("ar");
    libwrapper
        .arg("rcs")
        .arg("include/libwrapper.a")
        .arg("include/wrapper.o")
        .arg("--thin")
        .args([
            external.join("RingFamilies/libRDKitRingDecomposerLib_static.a"),
            external.join("INCHI-API/libRDKitInchi_static.a"),
            external.join("INCHI-API/libRDKitRDInchiLib_static.a"),
            external.join("CoordGen/libRDKitcoordgen_static.a"),
            code.join("RDGeneral/libRDKitRDGeneral_static.a"),
            code.join("DataStructs/libRDKitDataStructs_static.a"),
            code.join("GraphMol/libRDKitGraphMol_static.a"),
            code.join("GraphMol/SmilesParse/libRDKitSmilesParse_static.a"),
            code.join("GraphMol/FileParsers/libRDKitFileParsers_static.a"),
            code.join("GraphMol/GeneralizedSubstruct/libRDKitGeneralizedSubstruct_static.a"),
            code.join("GraphMol/Substruct/libRDKitSubstructMatch_static.a"),
            code.join("GraphMol/GeneralizedSubstruct/libRDKitGeneralizedSubstruct_static.a"),
            code.join("GraphMol/GenericGroups/libRDKitGenericGroups_static.a"),
            code.join("GraphMol/Fingerprints/libRDKitFingerprints_static.a"),
            code.join("GraphMol/MolDraw2D/libRDKitMolDraw2D_static.a"),
            code.join("GraphMol/Subgraphs/libRDKitSubgraphs_static.a"),
            code.join("GraphMol/ChemReactions/libRDKitChemReactions_static.a"),
            code.join("GraphMol/Depictor/libRDKitDepictor_static.a"),
            code.join("GraphMol/MolTransforms/libRDKitMolTransforms_static.a"),
            code.join("RDBoost/libRDKitRDBoost_static.a"),
            code.join("DataStructs/libRDKitDataStructs_static.a"),
            code.join("Geometry/libRDKitRDGeometryLib_static.a"),
            code.join("DistGeom/libRDKitDistGeometry_static.a"),
        ]);
    run_cmd(libwrapper);

    // -L.
    println!(
        "cargo:rustc-link-search={}",
        Path::new("include")
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
    );
    // -lwrapper
    println!("cargo:rustc-link-lib=wrapper");

    println!("cargo:rustc-link-lib=boost_system");
    println!("cargo:rustc-link-lib=boost_serialization");
    println!("cargo:rustc-link-lib=boost_iostreams");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=freetype");

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
