#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <inchi.h>

#include <assert.h>

using namespace RDKit;
using namespace std;

int main() {
  string smiles = "C1=CC=CC=C1";

  // args to the function
  ROMol *mol = SmilesToMol(smiles);

  ExtraInchiReturnValues rv;
  string inchi = MolToInchi(*mol, rv);
  assert(rv.returnCode == 0);
  string inchi_key = InchiToInchiKey(inchi);
  cout << inchi_key << endl;
  return 0;
}
