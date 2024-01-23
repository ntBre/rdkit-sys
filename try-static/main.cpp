#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>

using namespace RDKit;
using namespace std;

int main() {
  string smiles = "Cc1ccccc1";
  ROMol *mol = SmilesToMol(smiles);
  cout << mol->getNumAtoms() << endl;
  return 0;
}
