RDROOT := /home/brent/omsf/clone/rdkit

libs := $(addprefix -lRDKit,GraphMol_static SmilesParse_static RDGeneral_static	\
RingDecomposerLib_static DataStructs_static) \
-lboost_serialization

dirs := $(addprefix -L$(RDROOT)/build/Code/,GraphMol GraphMol/SmilesParse	\
RDGeneral RDBoost DataStructs)

ext :=  $(addprefix -L$(RDROOT)/build/External/,RingFamilies)

ars := $(RDROOT)/build/External/RingFamilies/libRDKitRingDecomposerLib_static.a	\
$(addprefix $(RDROOT)/build/Code/,RDGeneral/libRDKitRDGeneral_static.a			\
DataStructs/libRDKitDataStructs_static.a GraphMol/libRDKitGraphMol_static.a		\
GraphMol/SmilesParse/libRDKitSmilesParse_static.a								\
RDBoost/libRDKitRDBoost_static.a DataStructs/libRDKitDataStructs_static.a)

CPPFLAGS += -I$(RDROOT)/Code $(ars)


LDFLAGS += -static $(dirs) $(ext) $(libs)

main: main.cpp
	g++ -o $@ $^ $(CPPFLAGS) $(LDFLAGS)

run: main
	LD_LIBRARY_PATH=/home/brent/omsf/clone/rdkit/build/lib ./main
