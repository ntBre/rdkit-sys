use std::env;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

fn main() {
    println!("cargo:rerun-if-changed=include/shim.h");
    println!("cargo:rerun-if-changed=include/shim.cpp");
    println!("cargo:rerun-if-changed=include/Makefile");
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=RDROOT");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let Ok(rdroot) = std::env::var("RDROOT") else {
        panic!("rdkit-sys: RDROOT must be set in the environment");
    };

    let rdlibs = match Path::new(&rdroot).join("build/lib").canonicalize() {
        Ok(rdlibs) => rdlibs,
        Err(e) => {
            let path = Path::new(&rdroot);
            println!("path: {path:?} exists: {}", path.exists());
            println!("path: {path:?} is a directory: {}", path.is_dir());
            println!("path: {path:?} is absolute: {}", path.is_absolute());
            let path = path.join("build").join("lib");
            println!("path: {path:?} exists: {}", path.exists());
            println!("path: {path:?} is a directory: {}", path.is_dir());
            println!("path: {path:?} is absolute: {}", path.is_absolute());
            println!("canonical path again: {:?}", path.canonicalize());
            panic!("failed to canonicalize rdroot: {rdroot:?} with {e:?}")
        }
    };
    let rdlibs = rdlibs.display();

    let rdroot = Path::new(&rdroot).canonicalize().unwrap();

    let output = Command::new("make")
        .env("RDROOT", rdroot)
        .arg("include/libshim.so")
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!("command failed");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        exit(1)
    }

    std::fs::rename("include/libshim.so", out_path.join("libshim.so"))
        .expect("failed to move shared library to OUT_DIR");

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
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{rdlibs},-rpath,{}",
        out_path.display()
    );
    println!("cargo:rustc-link-search={rdlibs}");
    println!("cargo:rustc-link-search={}", out_path.display());

    println!("cargo:include={}", out_path.display());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
