cd $(dirname $(dirname $0))
set -ex

if [ ! -f "external/SDL2.zip" ]; then
    wget https://codeload.github.com/libsdl-org/SDL/zip/refs/tags/release-2.30.6 -O external/SDL2.zip
fi
if [ ! -d "external/SDL2" ]; then
    tar -xf external/SDL2.zip -C external
    mv external/SDL-release-2.30.6 external/SDL2
    patch --forward external/SDL2/src/video/uikit/SDL_uikitappdelegate.m external/sdl.patch
fi
if [ ! -f "external/macos_fat.zip" ]; then
    wget https://github.com/webrogue-runtime/angle-builder/releases/latest/download/macos_fat.zip -O external/macos_fat.zip
fi
if [ ! -f "external/libEGL.dylib" ]; then
    unzip -j -d external external/macos_fat.zip fat/libEGL.dylib
fi
if [ ! -f "external/libGLESv2.dylib" ]; then
    unzip -j -d external external/macos_fat.zip fat/libGLESv2.dylib
fi
