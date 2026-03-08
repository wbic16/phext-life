#!/bin/bash
# Build phext-life WASM module
set -e

echo "🔱 Building Phext Life WASM module..."

cd phext-life-wasm

# Check if wasm-pack is installed
export PATH="$HOME/.cargo/bin:$PATH"
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack via cargo..."
    cargo install wasm-pack
fi

# Build for web
echo "Compiling Rust → WASM..."
wasm-pack build --target web --release --out-dir ../pkg

cd ..

echo ""
echo "✅ WASM module built successfully!"
echo "   Output: pkg/phext_life_wasm.js"
echo "   Size: $(du -h pkg/phext_life_wasm_bg.wasm | cut -f1)"
echo ""
echo "To use:"
echo "  1. Start web server: python3 -m http.server 8765"
echo "  2. Open http://localhost:8765/index-wasm.html"
echo "  3. Or access from phone: http://$(hostname):8765/index-wasm.html"
