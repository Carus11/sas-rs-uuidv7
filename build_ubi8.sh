#!/bin/bash

set -e

# Create build directory for outputs
mkdir -p build

# Build the Docker image
echo "Building UBI8 Docker image..."
docker build -t sas-rs-uuidv7:ubi8 -f Dockerfile.ubi8 .

# Extract the compiled library from the container
echo "Extracting library from container..."
container_id=$(docker create sas-rs-uuidv7:ubi8)
docker cp $container_id:/build/target/release/libsas_rs_uuidv7.so ./build/
docker rm $container_id

echo "Build complete! The library is available at: $(pwd)/build/libsas_rs_uuidv7.so"