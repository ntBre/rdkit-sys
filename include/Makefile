RDROOT ?= /home/brent/omsf/clone/rdkit

boost := $(addprefix -lboost_,system serialization iostreams)
rdkit := $(addprefix -lRDKit,GraphMol SmilesParse FileParsers Fingerprints	\
							 MolDraw2D RDInchiLib MolInterchange Depictor)

CPPFLAGS += -I$(RDROOT)/Code -I$(RDROOT)/Code/DataStructs	\
			-I$(RDROOT)/External/INCHI-API					\
			-std=c++17 -Wall -Wextra -pedantic -O2

LDFLAGS += -L$(RDROOT)/build/lib $(boost) $(rdkit)

libshim.so: shim.h shim.cpp
	$(CXX) -fPIC -shared -o $@ shim.cpp $(CPPFLAGS) $(LDFLAGS) -Wl,-rpath,$(RDROOT)/build/lib
