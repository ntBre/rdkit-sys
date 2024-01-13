#include <GraphMol/Atom.h>
#include <GraphMol/FileParsers/MolSupplier.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <GraphMol/Substruct/SubstructMatch.h>

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

  // returns a flattened array of matches with total length `len` and match size
  // `pair_size`
  int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len, size_t *pair_size) {
	ROMol *mol = reinterpret_cast<ROMol*>(rdmol);
	RDKit::RWMol *qmol = RDKit::SmartsToMol(smarts);
	// ordered map so we can avoid sorting later
	// this looks exactly like what the python does
	// http://www.rdkit.org/docs/GettingStartedInC%2B%2B.html#atom-map-indices-in-smarts
	std::map<int, unsigned int> idx_map;
	for (auto atom : qmol->atoms()) {
	  auto smirks_index = atom->getAtomMapNum();
	  if (smirks_index) {
		idx_map[smirks_index - 1] = atom->getIdx();
	  }
	}
	std::vector<int> map_list;
	for (const auto &[key, value] : idx_map) {
	  map_list.push_back(value);
	}

	*pair_size = map_list.size();

	std::vector<RDKit::MatchVectType> res;
	RDKit::SubstructMatchParameters params;
	params.useChirality = true;
	params.maxMatches = UINT_MAX;
	params.uniquify = false;
	size_t r = 0;
	int *ret = NULL;
	if (RDKit::SubstructMatch(*mol, *qmol, res, &params)) {
	  *len = res.size() * map_list.size();
	  ret = (int *) malloc(*len * sizeof(int));
	  if (ret == NULL) {
		exit(1);
	  }
	  for (size_t i = 0; i < res.size(); ++i) {
		for (size_t j = 0; j < map_list.size(); ++j) {
		  ret[r++] = res[i][map_list[j]].second;
		}
	  }
	}
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

