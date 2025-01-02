# harfbuzz-fonts

my version of [harfbuzz/harfbuzz-wasm-examples](https://github.com/harfbuzz/harfbuzz-wasm-examples), forked from [annieversary](https://github.com/annieversary/harfbuzz-wasm-fonts).
contains some directories copied directly from that repo:

- `bin` contains some helpful binaries, like otfsurgeon
- `base-fonts` contains the base .ttf files
- `harfbuzz-wasm` contains the rust crate with harfbuzz bindings

## fonts:
- `roman-numerals` displays numbers in roman numerals

## building
each font directory contains a `Makefile`, so you can build the font by running `make`

you can use a tool like FontGoggles (built with wasm harfbuzz support) to visualize the font.
the original repo has a build for m1 macs