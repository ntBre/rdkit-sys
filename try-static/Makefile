RDROOT := /home/brent/omsf/clone/rdkit
CODE := $(RDROOT)/build/Code
EXTERNAL := $(RDROOT)/build/External

boost := -lboost_serialization
rdlibs := GraphMol SmilesParse RDGeneral RingDecomposerLib DataStructs	\
          RDGeometryLib
libs := $(addprefix -lRDKit,$(addsuffix _static,$(rdlibs))) $(boost)

rddirs := GraphMol GraphMol/SmilesParse RDGeneral RDBoost DataStructs Geometry
dirs := $(addprefix -L$(CODE)/,$(rddirs))

ext :=  $(addprefix -L$(EXTERNAL)/,RingFamilies)

ars := $(addsuffix _static.a,$(EXTERNAL)/RingFamilies/libRDKitRingDecomposerLib	\
$(addprefix $(CODE)/,RDGeneral/libRDKitRDGeneral								\
DataStructs/libRDKitDataStructs GraphMol/libRDKitGraphMol						\
GraphMol/SmilesParse/libRDKitSmilesParse RDBoost/libRDKitRDBoost				\
DataStructs/libRDKitDataStructs))

CPPFLAGS += -I$(RDROOT)/Code $(ars)

LDFLAGS += -static $(dirs) $(ext) $(libs)

main: main.cpp
	g++ -o $@ $^ $(CPPFLAGS) $(LDFLAGS)

run: main
	LD_LIBRARY_PATH=/home/brent/omsf/clone/rdkit/build/lib ./main

wrapper.o: wrapper.cpp
	g++ -c -o $@ $^ -I$(RDROOT)/Code

libwrapper.a: wrapper.o
	ar rcs $@ $^

try: try.c libwrapper.a
	gcc -o $@ $< -L. -lwrapper $(ars) $(LDFLAGS) -lstdc++ -lm
