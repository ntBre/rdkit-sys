use std::io::Write;
use std::{
    ffi::{CStr, CString},
    fs::File,
};

use rdkit_sys::{
    RDKit_MolToSmiles, RDKit_ROMol_delete, RDKit_ROMol_new,
    RDKit_create_mol_supplier, RDKit_mol_supplier_at_end,
    RDKit_mol_supplier_next,
};

fn main() {
    let path = "/home/brent/omsf/chembl/data/chembl_33.sdf";
    let cpath = CString::new(path).unwrap();
    let mut out = File::create("out.smiles").unwrap();
    let smarts = CString::new("[#6X4:1]-[#6X4:2]-[#6X4:3]-[#6X4:4]").unwrap();

    unsafe {
        let m = RDKit_create_mol_supplier(cpath.as_ptr(), true);
        let mut mol = RDKit_ROMol_new();
        let mut count = 0;
        while !RDKit_mol_supplier_at_end(m) && count < 50 {
            RDKit_ROMol_delete(mol);
            mol = RDKit_mol_supplier_next(m);
            let smiles = RDKit_MolToSmiles(mol);
            let mut len = 0;
            let mut match_size = 0;
            let matches = rdkit_sys::find_smarts_matches(
                mol,
                smarts.as_ptr(),
                &mut len,
                &mut match_size,
            );
            let matches = Vec::from_raw_parts(matches, len, len);
            for (i, mat) in matches.chunks(match_size).enumerate() {
                println!(
                    "Match {i} : {} {} {} {}",
                    mat[0], mat[1], mat[2], mat[3]
                );
            }
            let s = CStr::from_ptr(smiles);
            writeln!(out, "{}", s.to_str().unwrap()).unwrap();
            // c++ version took like 400 seconds to run this with -O2
            // this took about 540 seconds in --release
            count += 1;
        }
    }
}
