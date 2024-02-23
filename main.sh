# Clone Upstream
mkdir -p ./pika-welcome
rsync -av --progress ./* ./pika-welcome --exclude ./pika-welcome
cd ./pika-welcome

# Get build deps
apt-get build-dep ./ -y
apt-get install curl -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | CARGO_HOME=/root/.cargo sh -s -- -y

# Build package
dpkg-buildpackage --no-sign

# Move the debs to output
cd ../
mkdir -p ./output
mv ./*.deb ./output/
