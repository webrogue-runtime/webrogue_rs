cmake_minimum_required(VERSION 3.23)

project(webrogue)

exec_program(git "${CMAKE_CURRENT_LIST_DIR}/external" ARGS clone https://github.com/libsdl-org/SDL.git OUTPUT_VARIABLE ignored)
exec_program(git "${CMAKE_CURRENT_LIST_DIR}/external/SDL" ARGS apply ../sdl3.patch OUTPUT_VARIABLE ignored)

set(SDL_SHARED OFF)
set(SDL_STATIC ON)
add_subdirectory("${CMAKE_CURRENT_LIST_DIR}/external/SDL" SDL3)

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
    -sASYNCIFY
    -sASYNCIFY_STACK_SIZE=65535
    -sFORCE_FILESYSTEM 
    -fno-rtti
    -sWASMFS
    -lopfs.js
    -sOFFSCREEN_FRAMEBUFFER
    # -sOFFSCREENCANVAS_SUPPORT
    -sMIN_WEBGL_VERSION=2
    --embed-file ${CMAKE_SOURCE_DIR}/../examples/raylib/raylib.wrapp@raylib.wrapp
)

add_executable(
    webrogue
    src/main.c    
    crates/backend/src/em_js.c
    ../crates/gfx_ffi/src/webrogue_gfx_ffi_sdl3.c
)

# add_dependencies(
#     webrogue
#     ${CMAKE_SOURCE_DIR}/../examples/raylib/raylib.wrapp@raylib.wrapp
# )

add_dependencies(webrogue webrogue_web)
target_link_libraries(
    webrogue 
    "${CMAKE_SOURCE_DIR}/target/wasm32-unknown-emscripten/release/libwebrogue_web.a"
    SDL3-static
)

target_link_options(
    webrogue PRIVATE
    -sWASM_BIGINT
    # -sFETCH 
    # --js-library ${CMAKE_CURRENT_LIST_DIR}/game/vmlib.js 
    "-sASYNCIFY_IMPORTS=[wr_em_js_initWasmModule, wr_em_js_execFunc, wr_em_js_continueFuncExecution, wr_em_js_readModMem, wr_em_js_writeModMem]"
)
