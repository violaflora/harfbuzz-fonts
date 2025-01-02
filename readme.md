# harfbuzz-fonts

my fonts based on [harfbuzz/harfbuzz-wasm-examples](https://github.com/harfbuzz/harfbuzz-wasm-examples), using the same base files as [annieversary](https://github.com/annieversary/harfbuzz-wasm-fonts)'s repo.
contains some directories copied directly from that repo:

- `bin` contains some helpful binaries, like otfsurgeon
- `base-fonts` contains the base .ttf files
- `harfbuzz-wasm` contains the rust crate with harfbuzz bindings

## fonts:
- `thornify` makes your text input more old-englishlike by converting 'th' to thorn (þ), 'w' to wynn (ƿ), j to i and u to v.

![The sentence "the quick brown fox jumps over the lazy dog" in a regular font and then in the thornify font.](https://raw.githubusercontent.com/violaflora/harfbuzz-fonts/refs/heads/main/thornify/thornify.png?token=GHSAT0AAAAAACU4ZYCWEAJ3YOMDZQPPEHTGZ3WN3RA)

## building:
each font directory contains a `Makefile`, so you can build the font by running `make`

you can use a tool like FontGoggles (built with wasm harfbuzz support) to visualize the font.
fontgoggles for apple silicon (the regular version will not work on apple silicon and you'll be really sad and confused) can be found [here](https://github.com/harfbuzz/harfbuzz-wasm-examples/tree/main/fontgoggles-wasm-m1)
