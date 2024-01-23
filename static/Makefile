RDROOT := /home/brent/omsf/clone/rdkit
CODE := $(RDROOT)/build/Code
EXTERNAL := $(RDROOT)/build/External

boost := -lboost_serialization
libs := $(addprefix -lRDKit,$(addsuffix _static,GraphMol SmilesParse RDGeneral	\
RingDecomposerLib DataStructs RDGeometryLib)) $(boost)

dirs := $(addprefix -L$(CODE)/,GraphMol GraphMol/SmilesParse RDGeneral RDBoost DataStructs Geometry)

ext :=  $(addprefix -L$(EXTERNAL)/,RingFamilies)

ars := $(RDROOT)/build/External/RingFamilies/libRDKitRingDecomposerLib_static.a	\
$(addprefix $(CODE)/,RDGeneral/libRDKitRDGeneral_static.a			\
DataStructs/libRDKitDataStructs_static.a GraphMol/libRDKitGraphMol_static.a		\
GraphMol/SmilesParse/libRDKitSmilesParse_static.a								\
RDBoost/libRDKitRDBoost_static.a DataStructs/libRDKitDataStructs_static.a)

CPPFLAGS += -I$(RDROOT)/Code $(ars)


LDFLAGS += -static $(dirs) $(ext) $(libs)

main: main.cpp
	g++ -o $@ $^ $(CPPFLAGS) $(LDFLAGS)

run: main
	LD_LIBRARY_PATH=/home/brent/omsf/clone/rdkit/build/lib ./main
