#include <stdio.h>

#include "wrapper.h"

int main() {
  const char *smiles = "Cc1ccccc1";
  RDKit_ROMol *mol = RDKit_SmilesToMol(smiles);
  printf("%d atoms\n", RDKit_ROMol_getNumAtoms(mol));
  return 0;
}
