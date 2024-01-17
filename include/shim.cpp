#include <GraphMol/Atom.h>
#include <GraphMol/FileParsers/MolSupplier.h>
#include <GraphMol/Fingerprints/MorganFingerprints.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolOps.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <GraphMol/Substruct/SubstructMatch.h>

#include <assert.h>

#include "shim.h"

using namespace RDKit;
using namespace RDKit::MolOps;
using namespace RDKit::MorganFingerprints;

#ifdef __cplusplus
extern "C" {
#endif

RDKit_SDMolSupplier *RDKit_create_mol_supplier(const char *filename) {
  std::string input_file = std::string(filename);
  SDMolSupplier *mol_supplier = new SDMolSupplier(input_file);
  return reinterpret_cast<RDKit_SDMolSupplier *>(mol_supplier);
}

void RDKit_delete_mol_supplier(RDKit_SDMolSupplier *m) {
  delete reinterpret_cast<SDMolSupplier *>(m);
}

bool RDKit_mol_supplier_at_end(RDKit_SDMolSupplier *m) {
  SDMolSupplier *s = reinterpret_cast<SDMolSupplier *>(m);
  return s->atEnd();
}

RDKit_ROMol *RDKit_mol_supplier_next(RDKit_SDMolSupplier *m) {
  SDMolSupplier *s = reinterpret_cast<SDMolSupplier *>(m);
  return reinterpret_cast<RDKit_ROMol *>(s->next());
}

RDKit_ROMol *RDKit_ROMol_new() {
  ROMol *mol = new ROMol();
  return reinterpret_cast<RDKit_ROMol *>(mol);
}

void RDKit_ROMol_delete(RDKit_ROMol *mol) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  delete m;
}

void RDKit_SanitizeMol(RDKit_ROMol *mol, unsigned int ops) {
  RWMol *m = reinterpret_cast<RWMol *>(mol);
  unsigned int failed;
  sanitizeMol(*m, failed, ops);
  assert(failed == 0);
}

void RDKit_SetAromaticity(RDKit_ROMol *mol, unsigned int model) {
  RWMol *m = reinterpret_cast<RWMol *>(mol);
  AromaticityModel modl = static_cast<AromaticityModel>(model);
  setAromaticity(*m, modl);
}

void RDKit_AssignStereochemistry(RDKit_ROMol *mol) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  assignStereochemistry(*m);
}

void RDKit_AddHs(RDKit_ROMol *mol) {
  RWMol *m = reinterpret_cast<RWMol *>(mol);
  addHs(*m);
}

RDKit_ROMol *RDKit_SmilesToMol(const char *smiles) {
  return reinterpret_cast<RDKit_ROMol *>(SmilesToMol(smiles));
}

RDKit_ROMol *RDKit_SmartsToMol(const char *smarts) {
  return reinterpret_cast<RDKit_ROMol *>(SmartsToMol(smarts));
}

char *RDKit_MolToSmiles(RDKit_ROMol *mol) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  std::string s = MolToSmiles(*m);
  char *ret = new char[s.size() + 1];
  strcpy(ret, s.c_str());
  return ret;
}

// returns a flattened array of matches with total length `len` and match size
// `pair_size`
int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len,
                         size_t *pair_size) {
  ROMol *mol = reinterpret_cast<ROMol *>(rdmol);
  std::unique_ptr<RDKit::RWMol> qmol =
      std::unique_ptr<RWMol>(RDKit::SmartsToMol(smarts));
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
  size_t r = 0;
  int *ret = NULL;
  if (RDKit::SubstructMatch(*mol, *qmol, res)) {
    *len = res.size() * map_list.size();
    ret = (int *)malloc(*len * sizeof(int));
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

int *find_smarts_matches_mol(RDKit_ROMol *rdmol, RDKit_ROMol *smarts,
                             size_t *len, size_t *pair_size) {
  ROMol *mol = reinterpret_cast<ROMol *>(rdmol);
  RWMol *qmol = reinterpret_cast<RWMol *>(smarts);
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
  size_t r = 0;
  int *ret = NULL;
  if (RDKit::SubstructMatch(*mol, *qmol, res)) {
    *len = res.size() * map_list.size();
    ret = (int *)malloc(*len * sizeof(int));
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

uint32_t *RD(MorganFingerprint)(RD(ROMol) * mol, unsigned int radius,
                                size_t *len) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  SparseIntVect<uint32_t> *svec = getFingerprint(*m, radius);
  *len = svec->getLength();
  uint32_t *ret = (uint32_t *)malloc(*len * sizeof(uint32_t));
  assert(ret != NULL);
  for (size_t i = 0; i < *len; ++i) {
    ret[i] = svec->getVal(i);
  }
  return ret;
}

#ifdef __cplusplus
}
#endif
