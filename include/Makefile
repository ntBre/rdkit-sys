RDROOT := /home/brent/omsf/clone/rdkit

boost := $(addprefix -lboost_,system serialization iostreams)
rdkit := $(addprefix -lRDKit,GraphMol SmilesParse FileParsers)

CPPFLAGS += -I$(RDROOT)/Code -Wall -Wextra -pedantic -O2
LDFLAGS += -L$(RDROOT)/build/lib $(boost) $(rdkit)

run: search
	LD_LIBRARY_PATH=$(RDROOT)/build/lib ./search

libshim.so: shim.h shim.cpp
	g++ -fPIC -shared -o $@ shim.cpp $(CPPFLAGS) $(LDFLAGS)

try: libshim.so try.c
	gcc try.c -L. -lshim -o try $(LDFLAGS)

runtry: try
	LD_LIBRARY_PATH=$(RDROOT)/build/lib:. ./try
