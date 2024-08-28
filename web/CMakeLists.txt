cmake_minimum_required(VERSION 3.23)

project(webrogue)

include(ExternalProject)
ExternalProject_Add(
    webrogue_web
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo +nightly build -Z build-std --target=wasm32-unknown-emscripten --release
    BINARY_DIR "${CMAKE_SOURCE_DIR}"
    INSTALL_COMMAND ""  
    BUILD_ALWAYS ON
)

add_link_options(
    -sUSE_SDL=2 
    -sUSE_SDL_TTF=2 
    -sASYNCIFY
    -sASYNCIFY_STACK_SIZE=65535
    -sFORCE_FILESYSTEM 
    -fno-rtti 
    -sPTHREAD_POOL_SIZE=1
    -sUSE_PTHREADS
    -sWASMFS
    -lopfs.js
    -pthread
)

add_executable(
    webrogue
    src/main.c    
    crates/backend/src/em_js.c
)

add_dependencies(webrogue webrogue_web)
target_link_libraries(
    webrogue 
    "${CMAKE_SOURCE_DIR}/target/wasm32-unknown-emscripten/release/libwebrogue_web.a"
)

target_link_options(
    webrogue PRIVATE
    -sWASM_BIGINT
    # -sFETCH 
    # --js-library ${CMAKE_CURRENT_LIST_DIR}/game/vmlib.js 
    "-sASYNCIFY_IMPORTS=[wr_em_js_initWasmModule, wr_em_js_execFunc, wr_em_js_continueFuncExecution, wr_em_js_readModMem, wr_em_js_writeModMem]"
    "-sOFFSCREEN_FRAMEBUFFER"
    "-sOFFSCREENCANVAS_SUPPORT"
)
