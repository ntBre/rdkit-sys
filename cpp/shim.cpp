#include <GraphMol/FileParsers/MolSupplier.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>

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


  bool RDKit_mol_supplier_at_end(RDKit_SDMolSupplier *m) {
	SDMolSupplier *s = reinterpret_cast<SDMolSupplier*>(m);
	return s->atEnd();
  }

  RDKit_ROMol *RDKit_mol_supplier_next(RDKit_SDMolSupplier *m) {
	SDMolSupplier *s = reinterpret_cast<SDMolSupplier*>(m);
	return reinterpret_cast<RDKit_ROMol*>(s->next());
  }

  RDKit_ROMol *ROMol_new() {
	ROMol *mol = new ROMol();
	return reinterpret_cast<RDKit_ROMol*>(mol);
  }

  void ROMol_delete(RDKit_ROMol *mol) {
	ROMol *m = reinterpret_cast<ROMol*>(mol);
	delete m;
  }

  char *RDKit_MolToSmiles(RDKit_ROMol *mol) {
	ROMol *m = reinterpret_cast<ROMol*>(mol);
	std::string s = MolToSmiles(*m);
	char *ret = new char[s.size() + 1];
	strcpy(ret, s.c_str());
	return ret;
  }

#ifdef __cplusplus
}
#endif

// RDKit::SDMolSupplier *RDKit_create_mol_supplier(void) {
//   std::string input_file = "chembl_33.sdf";
//   RDKit::SDMolSupplier *mol_supplier = new RDKit::SDMolSupplier(input_file, true);
//   return mol_supplier;
// }

