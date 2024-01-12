use std::io::Write;
use std::{
    ffi::{CStr, CString},
    fs::File,
};

use rdkit_sys::{
    RDKit_MolToSmiles, RDKit_create_mol_supplier, RDKit_mol_supplier_at_end,
    RDKit_mol_supplier_next, ROMol_delete, ROMol_new,
};

fn main() {
    let path = "/home/brent/omsf/chembl/chembl_33.sdf";
    let cpath = CString::new(path).unwrap();
    let mut out = File::create("out.smiles").unwrap();
    unsafe {
        let m = RDKit_create_mol_supplier(cpath.as_ptr());
        let mut mol = ROMol_new();
        while !RDKit_mol_supplier_at_end(m) {
            ROMol_delete(mol);
            mol = RDKit_mol_supplier_next(m);
            let smiles = RDKit_MolToSmiles(mol);
            let s = CStr::from_ptr(smiles);
            writeln!(out, "{}", s.to_str().unwrap()).unwrap();
        }
    }
}
