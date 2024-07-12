simple: BUILD.d

BUILD.d: main.wasm
	touch BUILD.d

main.wasm: main.c Makefile
	../wasi-sdk-22/bin/clang main.c --target=wasm32-wasip1 -g -O0 -Wl,--no-entry -o main.wasm #-nostdlib 
