cd $(dirname $0)
set -ex

sh scripts/download_sdl_src.sh
xcodegen -c
