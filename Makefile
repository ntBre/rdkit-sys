lib := cpp/libshim.so

$(lib): cpp/shim.h cpp/shim.cpp
	cd cpp && make $(notdir $@)

run: $(lib)
	cargo run --release
