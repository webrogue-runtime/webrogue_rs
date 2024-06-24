cd $(dirname $0)


../wasi-sdk/bin/clang main.c -g --target=wasm32-wasip1 -O0 -Wl,--no-entry -o main.wasm #-nostdlib 
