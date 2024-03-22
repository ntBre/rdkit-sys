#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString},
        fs::read_to_string,
    };

    use super::*;

    #[test]
    fn mol_supplier() {
        let path = c"testfiles/probe_mol.sdf";
        let smarts =
            CString::new("[#6X4:1]-[#6X4:2]-[#6X4:3]-[#6X4:4]").unwrap();

        unsafe {
            let m = RDKit_create_mol_supplier(path.as_ptr(), true);
            let mut mol = RDKit_ROMol_new();
            while !RDKit_mol_supplier_at_end(m) {
                RDKit_ROMol_delete(mol);
                mol = RDKit_mol_supplier_next(m);
                let smiles = RDKit_MolToSmiles(mol);
                let mut len = 0;
                let mut match_size = 0;
                let matches = find_smarts_matches(
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
                println!("{}", s.to_str().unwrap());
            }
        }
    }

    #[test]
    fn from_json() {
        let s = read_to_string("testfiles/commonchem.json").unwrap();
        let cs = CString::new(s).unwrap();
        unsafe {
            let mol = RDKit_JSONToMol(cs.as_ptr());
            let json = RDKit_MolToJSON(mol);
            let got = CStr::from_ptr(json);
            let want =
                read_to_string("testfiles/want.commonchem.json").unwrap();
            assert_eq!(got.to_str().unwrap(), want.trim());
        }
    }
}
