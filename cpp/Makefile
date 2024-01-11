boost := $(addprefix -lboost_,system serialization iostreams)
rdkit := $(addprefix -lRDKit,GraphMol SmilesParse FileParsers)

CPPFLAGS += -I/home/brent/omsf/clone/rdkit/Code -Wall -Wextra -pedantic -O2
LDFLAGS += -L/home/brent/omsf/clone/rdkit/build/lib $(boost) $(rdkit)

run: search
	LD_LIBRARY_PATH=/home/brent/omsf/clone/rdkit/build/lib ./search

shim.so: shim.h shim.cpp
	g++ -fPIC -shared -o $@ shim.cpp $(CPPFLAGS) $(LDFLAGS)
