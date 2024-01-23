#![allow(non_camel_case_types)]
use std::ffi::{c_char, CString};

#[repr(C)]
struct RDKit_ROMol {
    _unused: [u8; 0],
}

extern "C" {
    fn RDKit_SmilesToMol(smiles: *const c_char) -> *mut RDKit_ROMol;
    fn RDKit_ROMol_getNumAtoms(mol: *mut RDKit_ROMol) -> usize;
}

fn main() {
    let smiles = "Cc1ccccc1";
    unsafe {
        let s = CString::new(smiles).unwrap();
        let mol = RDKit_SmilesToMol(s.as_ptr());
        println!("{} atoms", RDKit_ROMol_getNumAtoms(mol));
    }
}
