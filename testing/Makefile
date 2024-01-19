RDROOT := /home/brent/omsf/clone/rdkit

rdkit := $(addprefix -lRDKit,GraphMol SmilesParse MolDraw2D RDInchiLib)

LIB_PATH = $(RDROOT)/build/lib

CPPFLAGS += -I$(RDROOT)/Code -I$(RDROOT)/External/INCHI-API
LDFLAGS += -L$(LIB_PATH) $(rdkit)

export LD_LIBRARY_PATH=$(LIB_PATH)
run.%: %
	./$<
