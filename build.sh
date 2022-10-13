#!/bin/sh
penne --config penne_wasm4.toml \
	--backend /opt/wasi-sdk/bin/clang \
	src/wasm4.pn src/sprites.pn src/main.pn \
	-o bin/corn-dev.wasm || exit 1
wasm-opt bin/corn-dev.wasm -o bin/corn.wasm \
	-Oz --strip-dwarf --strip-producers --zero-filled-memory || exit 1
ls -al bin/corn*.wasm
