RDROOT := /home/brent/omsf/clone/rdkit

boost := $(addprefix -lboost_,system serialization iostreams)
rdkit := $(addprefix -lRDKit,GraphMol SmilesParse FileParsers Fingerprints	\
							 MolDraw2D RDInchiLib)

CPPFLAGS += -I$(RDROOT)/Code -I$(RDROOT)/Code/DataStructs	\
			-I$(RDROOT)/External/INCHI-API					\
			-Wall -Wextra -pedantic -O2

LDFLAGS += -L$(RDROOT)/build/lib $(boost) $(rdkit)

libshim.so: shim.h shim.cpp
	g++ -fPIC -shared -o $@ shim.cpp $(CPPFLAGS) $(LDFLAGS)
