set -ex

cd $BUILT_PRODUCTS_DIR

if [ ! -d "angle_ios" ]; then
    mkdir angle_ios
fi

cd angle_ios

if [ ! -f "ios_headers.zip" ]; then
    wget https://github.com/webrogue-runtime/angle-builder/releases/latest/download/ios_headers.zip -O ios_headers.zip
fi
if [ ! -d "ios_headers" ]; then
    tar -xf ios_headers.zip
fi
