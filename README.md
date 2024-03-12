[![test](https://github.com/ntBre/rdkit-sys/actions/workflows/test.yml/badge.svg)](https://github.com/ntBre/rdkit-sys/actions/workflows/test.yml)

# rdkit-sys

C and Rust wrappers for [RDKit][rdkit]

## Getting started

### Obtaining RDKit

I have not been able to get this to work with an RDKit installed with conda, so
you must currently build RDKit yourself. To build from source, run this
somewhere on your computer:

``` shell
git clone --depth 1 https://github.com/rdkit/rdkit
cd rdkit
mkdir build
cd build
cmake .. -DRDK_BUILD_INCHI_SUPPORT=ON
make # optionally with -j and however many CPUs you want to give it
```

`rdkit-sys` includes InChi key support, so you must compile RDKit with InChi
support enabled.

### Running dependent crates

The build script should take care of building the shared library and including
the absolute path in the binary itself via `rpath` linker arguments. However, it
does require the environment variable `RDROOT` to be set to locate the RDKit
libraries and header files from the steps above. Probably the easiest way to do
this is to add a Cargo config file in `.cargo/config.toml` in your project:

``` toml
[env]
RDROOT = "/path/to/your/rdkit"
```

But you can also prefix your Cargo commands with `RDROOT=/path/to/rdkit` or
however else you like to set environment variables.

[rdkit]: https://github.com/rdkit/rdkit
