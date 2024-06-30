cd $(dirname $0)


../wasi-sdk/bin/clang main.c --target=wasm32-wasip1 -g -O0 -Wl,--no-entry -o main.wasm #-nostdlib 
