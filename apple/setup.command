cd $(dirname $0)
set -ex

sh iOS/rust/scripts/download_angle_ios_xcframeworks.sh
sh scripts/download_sdl_src.sh

xcodegen -c
