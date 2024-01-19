#include <GraphMol/GraphMol.h>
#include <GraphMol/MolDraw2D/MolDraw2DSVG.h>
#include <GraphMol/MolDraw2D/MolDraw2DUtils.h>
#include <GraphMol/SmilesParse/SmilesParse.h>

#include <fstream>
#include <iostream>

using namespace RDKit;
using namespace std;

int main() {
  ofstream output;
  output.open("mol.html");

  string smiles = "C1=CC=CC=C1";

  // args to the function
  ROMol *mol = SmilesToMol(smiles);
  int width = 400;
  int height = 300;
  string legend = smiles;
  vector<int> highlight_atoms = {1, 2, 3, 4};

  MolDraw2DSVG drawer = MolDraw2DSVG(width, height);
  MolDraw2DUtils::prepareAndDrawMolecule(drawer, *mol, legend,
                                         &highlight_atoms);
  drawer.finishDrawing();
  string svg = drawer.getDrawingText();

  output << svg;
  output.close();
  return 0;
}
