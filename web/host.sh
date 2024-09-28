cd $(dirname $0)
set -ex

sh build.sh

cd root
python3 -m http.server
