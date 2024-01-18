#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define RD(name) RDKit_##name

typedef struct RDKit_SDMolSupplier RDKit_SDMolSupplier;
typedef struct RDKit_ROMol RDKit_ROMol;

typedef struct {
  unsigned int bit;
  int count;
} Pair;

#ifdef __cplusplus
extern "C" {
#endif

RDKit_SDMolSupplier *RDKit_create_mol_supplier(const char *filename);
void RDKit_delete_mol_supplier(RDKit_SDMolSupplier *m);
bool RDKit_mol_supplier_at_end(RDKit_SDMolSupplier *m);
RDKit_ROMol *RDKit_mol_supplier_next(RDKit_SDMolSupplier *m);

RDKit_ROMol *RDKit_ROMol_new();
void RDKit_ROMol_delete(RDKit_ROMol *mol);

// MolOps
void RDKit_SanitizeMol(RDKit_ROMol *mol, unsigned int ops);
void RDKit_SetAromaticity(RDKit_ROMol *mol, unsigned int model);
void RDKit_AssignStereochemistry(RDKit_ROMol *mol);
void RDKit_AddHs(RDKit_ROMol *mol);

RDKit_ROMol *RDKit_SmilesToMol(const char *smiles);
RDKit_ROMol *RDKit_SmartsToMol(const char *smarts);
char *RDKit_MolToSmiles(RDKit_ROMol *mol);

int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len,
                         size_t *pair_size);
int *find_smarts_matches_mol(RDKit_ROMol *rdmol, RDKit_ROMol *smarts,
                             size_t *len, size_t *pair_size);

Pair *RD(MorganFingerprint)(RD(ROMol) * mol, unsigned int radius, size_t *len);
void RD(MorganFingerprintBitVector)(RD(ROMol) * mol, unsigned int radius,
                                    size_t len, size_t *buf);

#ifdef __cplusplus
}
#endif
