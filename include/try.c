#include "shim.h"

int main() {
  RDKit_ROMol *mol = RDKit_SmilesToMol(
      "[H]Cl.[H]N([H])/C(=[N+](/[O-])C([H])([H])[H])C([H])([H])[H]");
  return 0;
}
