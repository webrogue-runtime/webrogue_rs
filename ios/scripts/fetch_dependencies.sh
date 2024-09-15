cd $(dirname $(dirname $0))
set -ex

if [ ! -f "external/SDL2.zip" ]; then
    curl https://codeload.github.com/libsdl-org/SDL/zip/refs/tags/release-2.30.6 -o external/SDL2.zip
fi
if [ ! -d "external/SDL2" ]; then
    tar -xf external/SDL2.zip -C external
    mv external/SDL-release-2.30.6 external/SDL2
    patch --forward external/SDL2/src/video/uikit/SDL_uikitappdelegate.m external/sdl.patch
fi
