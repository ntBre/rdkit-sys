#include <stdbool.h>
#include <stddef.h>

typedef struct RDKit_SDMolSupplier RDKit_SDMolSupplier;
typedef struct RDKit_ROMol RDKit_ROMol;

#ifdef __cplusplus
extern "C" {
#endif

RDKit_SDMolSupplier *RDKit_create_mol_supplier(const char *filename);
void RDKit_delete_mol_supplier(RDKit_SDMolSupplier *m);
bool RDKit_mol_supplier_at_end(RDKit_SDMolSupplier *m);
RDKit_ROMol *RDKit_mol_supplier_next(RDKit_SDMolSupplier *m);

RDKit_ROMol *RDKit_ROMol_new();
void RDKit_ROMol_delete(RDKit_ROMol *mol);

RDKit_ROMol *RDKit_SmilesToMol(const char *smiles);
char *RDKit_MolToSmiles(RDKit_ROMol *mol);

int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len,
                         size_t *pair_size);

#ifdef __cplusplus
}
#endif
