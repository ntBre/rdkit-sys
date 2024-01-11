#include <GraphMol/FileParsers/MolSupplier.h>

#include "shim.h"

using namespace RDKit;

#ifdef __cplusplus
extern "C" {
#endif

  RDKit_SDMolSupplier *RDKit_create_mol_supplier(const char *filename) {
	std::string input_file = std::string(filename);
	SDMolSupplier *mol_supplier = new SDMolSupplier(input_file);
	return reinterpret_cast<RDKit_SDMolSupplier*>(mol_supplier);
  }

#ifdef __cplusplus
}
#endif

// RDKit::SDMolSupplier *RDKit_create_mol_supplier(void) {
//   std::string input_file = "chembl_33.sdf";
//   RDKit::SDMolSupplier *mol_supplier = new RDKit::SDMolSupplier(input_file, true);
//   return mol_supplier;
// }

