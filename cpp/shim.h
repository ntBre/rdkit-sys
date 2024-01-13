#include <stdbool.h>
#include <stddef.h>

typedef struct RDKit_SDMolSupplier RDKit_SDMolSupplier;
typedef struct RDKit_ROMol RDKit_ROMol;

#ifdef __cplusplus
extern "C" {
#endif

  RDKit_SDMolSupplier *RDKit_create_mol_supplier(const char *filename);
  bool RDKit_mol_supplier_at_end(RDKit_SDMolSupplier *m);
  RDKit_ROMol *RDKit_mol_supplier_next(RDKit_SDMolSupplier *m);

  RDKit_ROMol *ROMol_new();
  void ROMol_delete(RDKit_ROMol *mol);
  void ROMol_reset(RDKit_ROMol *orig, RDKit_ROMol *other);

  char *RDKit_MolToSmiles(RDKit_ROMol *mol);

  int *find_smarts_matches(RDKit_ROMol *rdmol, const char *smarts, size_t *len, size_t *pair_size);

#ifdef __cplusplus
}
#endif

/* RDKit::SDMolSupplier RDKit_create_mol_supplier(const char *filename); */
