cd $(dirname $0)

../wasi-sdk/bin/clang main.c -g -O0 -Wl,--no-entry -o main.wasm #-nostdlib 
