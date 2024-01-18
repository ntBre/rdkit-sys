RDROOT := /home/brent/omsf/clone/rdkit

boost := $(addprefix -lboost_,system serialization iostreams)
rdkit := $(addprefix -lRDKit, GraphMol SmilesParse FileParsers Fingerprints)

CPPFLAGS += -I$(RDROOT)/Code -I$(RDROOT)/Code/DataStructs -Wall -Wextra	\
			-pedantic -O2
LDFLAGS += -L$(RDROOT)/build/Code -static-libstdc++ -static $(boost) $(rdkit)

shim.o: shim.cpp shim.h
	g++ -c $< $(CPPFLAGS) $(LDFLAGS)

libshim.a: shim.o
	ar rvs $@ $<
