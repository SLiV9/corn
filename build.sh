#!/bin/sh
/usr/local/bin/pennec src/wasm4.pn src/main.pn --wasm || exit 1
/opt/wasi-sdk/bin/clang -nostartfiles -nostdlib \
	-Wl,--allow-undefined -Wl,--import-memory \
	-Wl,--initial-memory=65536 -Wl,--max-memory=65536 \
	-Wl,-z,stack-size=14752 -Wl,--stack-first \
	-Wl,--no-entry -Wl,--export=update -Wl,--export=start \
	bin/src/main.pn.ll \
	-o bin/corn-dev.wasm || exit 1
wasm-opt bin/corn-dev.wasm -o bin/corn.wasm \
	-Oz --strip-dwarf --strip-producers --zero-filled-memory || exit 1
ls -al bin/corn*.wasm
