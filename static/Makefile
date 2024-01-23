RDROOT := /home/brent/omsf/clone/rdkit

CPPFLAGS += -I$(RDROOT)/Code

libs := $(addprefix -lRDKit,GraphMol SmilesParse)

LDFLAGS += -L$(RDROOT)/build/lib $(libs)

run: main
	LD_LIBRARY_PATH=/home/brent/omsf/clone/rdkit/build/lib ./main
