#!/bin/bash
set -e

echo "Building Recipe Management System..."

# Build API component
echo "Building recipe-api component..."
cd components/api

# Try cargo-component first, fall back to regular cargo if not available
if command -v cargo-component &> /dev/null; then
    echo "Using cargo-component..."
    cargo component build --release --target wasm32-wasip2
    mkdir -p build
    cp ../../target/wasm32-wasip2/release/recipe_api.wasm build/recipe_api_s.wasm
else
    echo "cargo-component not found, using regular cargo build..."
    cargo build --release --target wasm32-wasip2
    mkdir -p build
    cp ../../target/wasm32-wasip2/release/recipe_api.wasm build/recipe_api_s.wasm
fi

echo "Build complete!"
echo "Component built: components/api/build/recipe_api_s.wasm"
echo "Size: $(du -h components/api/build/recipe_api_s.wasm | cut -f1)"
