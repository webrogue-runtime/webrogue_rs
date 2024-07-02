
# For more information about using CMake with Android Studio, read the
# documentation: https://d.android.com/studio/projects/add-native-code.html.
# For more examples on how to use CMake, see https://github.com/android/ndk-samples.
cmake_minimum_required(VERSION 3.22.1)
project("webrogue")

# include(FetchContent)
# FetchContent_Declare(
#     Corrosion
#     GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
#     GIT_TAG 0a3bdf4
# )
# FetchContent_MakeAvailable(Corrosion)
# # rustup target add --toolchain stable-x86_64-unknown-linux-gnu i686-linux-android
# corrosion_import_crate(
#     MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
# #   NO_DEFAULT_FEATURES
# #   FEATURES jit cranelift
#     CRATE_TYPES cdylib
#     CRATES webrogue_android
# )

add_subdirectory(rust_cmake)

set(SDL_SHARED ON)
set(SDL_STATIC OFF)
exec_program(git ${CMAKE_CURRENT_SOURCE_DIR}/external/SDL ARGS submodule update --init)
add_subdirectory(external/SDL)
set(SDL2TTF_VENDORED ON)
set(SDL2TTF_INSTALL OFF)
exec_program(git ${CMAKE_CURRENT_SOURCE_DIR}/external/SDL_ttf ARGS submodule update --init)
exec_program(git ${CMAKE_CURRENT_SOURCE_DIR}/external/SDL_ttf/external/freetype ARGS submodule update --init)
add_subdirectory(external/SDL_ttf)

add_library(
    webrogue SHARED
    webrogue_runtime.cpp
)
target_link_libraries(
    webrogue

    android
    log
    webrogue_android
    SDL2
    SDL2_ttf
)

execute_process(
    COMMAND ${CMAKE_COMMAND} -E copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/external/SDL/android-project/app/src/main/java/org/libsdl/app/ ${CMAKE_CURRENT_SOURCE_DIR}/../java/org/libsdl/app
)
