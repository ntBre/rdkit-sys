#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let smiles = "Cc1ccccc1";
    unsafe {
        let s = CString::new(smiles).unwrap();
        let mol = RDKit_SmilesToMol(s.as_ptr());
        println!("{} atoms", RDKit_ROMol_getNumAtoms(mol));
    }
}
