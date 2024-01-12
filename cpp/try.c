#include <stdio.h>

#include "shim.h"

int main(void) {
  char *path = "/home/brent/omsf/chembl/chembl_33.sdf";
  RDKit_SDMolSupplier *m = RDKit_create_mol_supplier(path);
  RDKit_ROMol *mol = ROMol_new();
  while (!RDKit_mol_supplier_at_end(m)) {
	ROMol_delete(mol);
	mol = RDKit_mol_supplier_next(m);
	/* ROMol_reset(mol, RDKit_mol_supplier_next(m)); */
	char *smiles = RDKit_MolToSmiles(mol);
	printf("%s\n", smiles);
  }
  return 0;
}
