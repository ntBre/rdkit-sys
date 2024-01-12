#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn one() {
        let path = "/home/brent/omsf/chembl/chembl_33.sdf";
        let cpath = CString::new(path).unwrap();
        unsafe {
            RDKit_create_mol_supplier(cpath.as_ptr());
        }
    }
}
