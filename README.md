# rdkit-sys

[bindgen](https://github.com/rust-lang/rust-bindgen) doesn't work very well on
C++ libraries, but I finally considered that it should be straightforward to
call C++ from C. So here I'm writing C bindings to RDKit, which I then hope to
bindgen into a Rust library.

It's a big mess right now, with some files left from trying to adapt things
from the other [rdkit-sys](https://github.com/rdkit-rs/rdkit) crate and some
from my experiments today.

The next step is to compile a Rust version of the `cpp/try.c` example using
a basic SDF reader and `ROMol`s.
