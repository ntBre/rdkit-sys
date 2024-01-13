RDROOT := /home/brent/omsf/clone/rdkit

boost := $(addprefix -lboost_,system serialization iostreams)
rdkit := $(addprefix -lRDKit,GraphMol SmilesParse FileParsers)

CPPFLAGS += -I$(RDROOT)/Code -Wall -Wextra -pedantic -O2
LDFLAGS += -L$(RDROOT)/build/lib $(boost) $(rdkit)

libshim.so: shim.h shim.cpp
	g++ -fPIC -shared -o $@ shim.cpp $(CPPFLAGS) $(LDFLAGS)
