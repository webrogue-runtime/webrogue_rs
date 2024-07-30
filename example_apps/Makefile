all: bin simple opengl

# utility
bin:
	mkdir -p bin

wasi-sdk-22:
	curl -L https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-22/wasi-sdk-22.0-linux.tar.gz | tar -xz
	mv wasi-sdk-22.0 wasi-sdk-22

# apps
simple: build_simple bin/simple.wrapp

build_simple: wasi-sdk-22
	cd simple && $(MAKE)

bin/simple.wrapp: simple/BUILD.d
	cargo run --manifest-path ../crates/wrapp_cli/Cargo.toml simple bin/simple.wrapp

opengl: build_opengl bin/opengl.wrapp

build_opengl: wasi-sdk-22
	cd opengl && $(MAKE)

bin/opengl.wrapp: opengl/BUILD.d
	cargo run --manifest-path ../crates/wrapp_cli/Cargo.toml opengl bin/opengl.wrapp
