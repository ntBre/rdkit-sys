#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};

    use super::*;

    #[test]
    fn one() {
        let path = "/home/brent/omsf/chembl/chembl_33.sdf";
        let cpath = CString::new(path).unwrap();
        unsafe {
            let m = RDKit_create_mol_supplier(cpath.as_ptr());
            let mut mol = ROMol_new();
            while !RDKit_mol_supplier_at_end(m) {
                ROMol_delete(mol);
                mol = RDKit_mol_supplier_next(m);
                let smiles = RDKit_MolToSmiles(mol);
                let s = CStr::from_ptr(smiles);
                println!("{}", s.to_str().unwrap());
            }
        }
    }
}
