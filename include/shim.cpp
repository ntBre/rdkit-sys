#include <GraphMol/Atom.h>
#include <GraphMol/FileParsers/MolSupplier.h>
#include <GraphMol/Fingerprints/MorganFingerprints.h>
#include <GraphMol/GraphMol.h>
#include <GraphMol/MolDraw2D/MolDraw2DSVG.h>
#include <GraphMol/MolDraw2D/MolDraw2DUtils.h>
#include <GraphMol/MolOps.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <GraphMol/Substruct/SubstructMatch.h>
#include <SparseIntVect.h>
#include <inchi.h>

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

unsigned int RD(ROMol_getNumAtoms)(RD(ROMol) * mol) {
  return reinterpret_cast<ROMol *>(mol)->getNumAtoms();
}

unsigned int RDKit_SanitizeMol(RDKit_ROMol *mol, unsigned int ops) {
  RWMol *m = reinterpret_cast<RWMol *>(mol);
  unsigned int failed;
  sanitizeMol(*m, failed, ops);
  return failed;
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

char *RD(MolToInchiKey)(RD(ROMol) * mol) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  ExtraInchiReturnValues rv;
  std::string inchi = MolToInchi(*m, rv);
  std::string inchi_key = InchiToInchiKey(inchi);
  char *ret = new char[inchi_key.size() + 1];
  strcpy(ret, inchi_key.c_str());
  return ret;
}

// returns a flattened array of matches with total length `len` and match size
// `pair_size`
int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len,
                         size_t *pair_size) {
  std::unique_ptr<RDKit::RWMol> qmol =
      std::unique_ptr<RWMol>(RDKit::SmartsToMol(smarts));
  return find_smarts_matches_mol(
      rdmol, reinterpret_cast<RDKit_ROMol *>(qmol.get()), len, pair_size);
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

Pair *RD(MorganFingerprint)(RD(ROMol) * mol, unsigned int radius, size_t *len) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  SparseIntVect<uint32_t> *svec = getFingerprint(*m, radius);
  std::map<unsigned int, int> st = svec->getNonzeroElements();
  size_t i = 0;
  *len = st.size();
  Pair *ret = (Pair *)malloc(*len * sizeof(Pair));
  assert(ret != NULL);
  for (auto &[idx, value] : st) {
    ret[i++] = Pair{idx, value};
  }
  return ret;
}

void RD(MorganFingerprintBitVector)(RD(ROMol) * mol, unsigned int radius,
                                    size_t len, bool *buf) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  ExplicitBitVect *svec = getFingerprintAsBitVect(*m, radius, len);
  for (size_t i = 0; i < len; ++i) {
    buf[i] = svec->getBit(i);
  }
  delete svec;
}

char *RD(MolDrawSVG)(RD(ROMol) * mol, int width, int height, const char *legend,
                     const int *hl_atoms, size_t hl_atom_count) {
  ROMol *m = reinterpret_cast<ROMol *>(mol);
  // convert to compatible types
  std::vector<int> highlight_atoms;
  for (size_t i = 0; i < hl_atom_count; ++i) {
    highlight_atoms.push_back(hl_atoms[i]);
  }
  std::string leg(legend);

  MolDraw2DSVG drawer = MolDraw2DSVG(width, height);
  MolDraw2DUtils::prepareAndDrawMolecule(drawer, *m, leg, &highlight_atoms);
  drawer.finishDrawing();
  std::string svg = drawer.getDrawingText();

  char *ret = new char[svg.size() + 1];
  strcpy(ret, svg.c_str());
  return ret;
}

#ifdef __cplusplus
}
#endif
