#include "shim.h"

int main(void) {
  RDKit_SDMolSupplier *m = RDKit_create_mol_supplier("single.sdf");
  return 0;
}
