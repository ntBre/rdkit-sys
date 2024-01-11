#include "shim.h"

// extern "C" {
//   RDKit_SDMolSupplier RDKit_create_mol_supplier(const char *filename) {
// 	std::string input_file = std::string(filename);
// 	RDKit_SDMolSupplier mol_supplier = RDKit::SDMolSupplier(input_file);
// 	return mol_supplier;
//   }
// }

void RDKit_create_mol_supplier(void) {
  std::string input_file = "chembl_33.sdf";
  RDKit::SDMolSupplier mol_supplier(input_file, true);
}
