# rdkit-sys

[bindgen](https://github.com/rust-lang/rust-bindgen) doesn't work very well on
C++ libraries, but I finally considered that it should be straightforward to
call C++ from C. So here I'm writing C bindings to RDKit, which I then hope to
bindgen into a Rust library.

## Getting started

### Obtaining RDKit

You can probably use an RDKit installed with conda, but I've only tested on
source builds. To build from source, run this somewhere on your computer:

``` shell
git clone https://github.com/rdkit/rdkit
mkdir build
cd build
cmake .. -DRDK_BUILD_INCHI_SUPPORT=ON
make # optionally with -j and however many CPUs you want to give it
```

`rdkit-sys` includes InChi key support, so you must compile RDKit with InChi
support enabled.

### Building the shared library

Next, build the rdkit-sys shared library that is used by dependent Rust
packages:

``` shell
make include/libshim.so RDROOT=/path/to/your/rdkit
```

I cloned RDKit to `/home/brent/omsf/clone/rdkit`, so that would be my `RDROOT`,
as you can see in `include/Makefile`. `build.rs` should do this part for you,
but if not, these are the manual instructions.

### Running dependent crates

Unfortunately, I haven't figured out how to link everything statically, so
`libshim.so` (and I think the RDKit libs) will need to be in your
`LD_LIBRARY_PATH` for executables depending upon `rdkit-sys` to work. You can
export this with a simple `build.rs` for ease of use with `cargo`:

``` rust
fn main() {
    let rdroot = std::env::var("RDROOT").unwrap();
    let include = "/path/to/rdkit-sys/include";
    println!("cargo:rustc-env=LD_LIBRARY_PATH={include}:{rdroot}/build/lib");
}
```

But for running a standalone executable without cargo, you'll have to do
something like the following.

``` shell
LD_LIBRARY_PATH=/path/to/rdroot/build/lib:/path/to/rdkit-sys/include \
your-rust-executable
```

