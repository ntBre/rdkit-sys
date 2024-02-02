#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define RD(name) RDKit_##name
#define NEWTYPE(name) RDKit_##name RDKit_##name

typedef struct RDKit_SDMolSupplier RDKit_SDMolSupplier;
typedef struct RDKit_MultithreadedSDMolSupplier
    RDKit_MultithreadedSDMolSupplier;
typedef struct RDKit_ROMol RDKit_ROMol;
typedef struct NEWTYPE(ChemicalReaction);

typedef struct {
  unsigned int bit;
  int count;
} Pair;

#ifdef __cplusplus
extern "C" {
#endif

// SDMolSupplier
RDKit_SDMolSupplier *RDKit_create_mol_supplier(const char *filename,
                                               bool removeHs);
void RDKit_delete_mol_supplier(RDKit_SDMolSupplier *m);
bool RDKit_mol_supplier_at_end(RDKit_SDMolSupplier *m);
RDKit_ROMol *RDKit_mol_supplier_next(RDKit_SDMolSupplier *m);

// MultithreadedSDMolSupplier
RDKit_MultithreadedSDMolSupplier *
    RD(MultithreadedSDMolSupplier_new)(const char *filename);
bool RD(MultithreadedSDMolSupplier_at_end)(RD(MultithreadedSDMolSupplier) * m);
RD(ROMol) *
    RD(MultithreadedSDMolSupplier_next)(RD(MultithreadedSDMolSupplier) * m);
void RD(MultithreadedSDMolSupplier_delete)(RD(MultithreadedSDMolSupplier) * m);

// ROMol
RDKit_ROMol *RDKit_ROMol_new();
void RDKit_ROMol_delete(RDKit_ROMol *mol);
unsigned int RD(ROMol_getNumAtoms)(RD(ROMol) * mol);

// MolOps
unsigned int RDKit_SanitizeMol(RDKit_ROMol *mol, unsigned int ops);
void RDKit_SetAromaticity(RDKit_ROMol *mol, unsigned int model);
void RDKit_AssignStereochemistry(RDKit_ROMol *mol);
void RDKit_AddHs(RDKit_ROMol *mol);

// Mol <-> String conversions
RDKit_ROMol *RDKit_SmilesToMol(const char *smiles);
RDKit_ROMol *RDKit_SmartsToMol(const char *smarts);
char *RD(MolToSmiles)(RD(ROMol) * mol);
char *RD(MolToInchiKey)(RD(ROMol) * mol);

int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len,
                         size_t *pair_size);
int *find_smarts_matches_mol(RDKit_ROMol *rdmol, RDKit_ROMol *smarts,
                             size_t *len, size_t *pair_size);

Pair *RD(MorganFingerprint)(RD(ROMol) * mol, unsigned int radius, size_t *len);
void RD(MorganFingerprintBitVector)(RD(ROMol) * mol, unsigned int radius,
                                    size_t len, bool *buf);

char *RD(MolDrawSVG)(RD(ROMol) * mol, int width, int height, const char *legend,
                     const int *hl_atoms, size_t hl_atom_count);

RD(ChemicalReaction) * RD(RxnSmartsToChemicalReaction)(const char *smarts);

#ifdef __cplusplus
}
#endif
