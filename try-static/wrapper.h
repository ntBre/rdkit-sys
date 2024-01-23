typedef struct RDKit_ROMol RDKit_ROMol;

#ifdef __cplusplus
extern "C" {
#endif

RDKit_ROMol *RDKit_SmilesToMol(const char *smiles);
unsigned int RDKit_ROMol_getNumAtoms(RDKit_ROMol *mol);

#ifdef __cplusplus
}
#endif
