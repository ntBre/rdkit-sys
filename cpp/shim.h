#include <GraphMol/FileParsers/MolSupplier.h>

/* extern "C" { */
/*   typedef RDKit::SDMolSupplier RDKit_SDMolSupplier; */
/*   RDKit_SDMolSupplier RDKit_create_mol_supplier(const char *filename); */
/* } */

RDKit::SDMolSupplier RDKit_create_mol_supplier(const char *filename);
