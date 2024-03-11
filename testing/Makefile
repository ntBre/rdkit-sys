# RDROOT := /home/brent/omsf/clone/rdkit
# LIB_PATH = $(RDROOT)/build/lib
# CPPFLAGS += -I$(RDROOT)/Code -I$(RDROOT)/External/INCHI-API

rdkit := $(addprefix -lRDKit,GraphMol SmilesParse MolDraw2D RDInchiLib)

LIB_PATH = $(CONDA_PREFIX)/lib
CPPFLAGS += -I$(CONDA_PREFIX)/include/rdkit -I$(CONDA_PREFIX)/include/rdkit/GraphMol
LDFLAGS += -L$(LIB_PATH) $(rdkit) -Wl,-rpath,$(LIB_PATH)

run.%: %
	./$<
