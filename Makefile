lib := include/libshim.so

$(lib): include/shim.h include/shim.cpp
	cd include && make $(notdir $@)

run:
	cargo run --release
