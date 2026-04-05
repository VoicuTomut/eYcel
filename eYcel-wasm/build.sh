#!/bin/bash
# Build eYcel WASM module and prepare web directory
set -e

echo "Building eYcel WASM..."
wasm-pack build --target web --release

echo "Linking pkg to web directory..."
cd web
ln -sf ../pkg pkg

echo ""
echo "Build complete! To serve locally:"
echo "  cd web && python3 -m http.server 8080"
echo "  Then open http://localhost:8080"
