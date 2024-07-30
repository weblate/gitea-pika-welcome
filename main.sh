#! /bin/bash

set -e

VERSION="5.0.0"

source ./pika-build-config.sh

echo "$PIKA_BUILD_ARCH" > pika-build-arch

# Clone Upstream
mkdir -p pika-welcome
cp -rvf ./* ./pika-welcome/ || true
cd ./pika-welcome/

# Get build deps
apt-get build-dep ./ -y
apt-get install curl -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | CARGO_HOME=/root/.cargo sh -s -- -y

# Build package
LOGNAME=root dh_make --createorig -y -l -p pika-welcome_"$VERSION" || echo "dh-make: Ignoring Last Error"
dpkg-buildpackage --no-sign

# Move the debs to output
cd ../
mkdir -p ./output
mv ./*.deb ./output/
