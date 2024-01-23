#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>

#include "wrapper.h"

using namespace RDKit;
using namespace std;

#ifdef __cplusplus
extern "C" {
#endif

RDKit_ROMol *RDKit_SmilesToMol(const char *smiles) {
  return reinterpret_cast<RDKit_ROMol *>(SmilesToMol(smiles));
}

unsigned int RDKit_ROMol_getNumAtoms(RDKit_ROMol *mol) {
  auto m = reinterpret_cast<ROMol *>(mol);
  return m->getNumAtoms();
}

#ifdef __cplusplus
}
#endif
