cd $(dirname $(dirname $0))
set -ex

CONFIGURATION=Debug PLATFORM_NAME=iphonesimulator ARCHS="x86_64 arm64" sh scripts/cargo_lipo_builder.sh
