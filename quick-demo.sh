#!/bin/bash
# Quick demo of phext-life simulation
# Runs for 100 epochs and creates a GIF

set -e

echo "🔱 Phext Life - Quick Demo"
echo "=========================="
echo ""

# Check dependencies
if ! python3 -c "import numpy, numba, PIL, tqdm" 2>/dev/null; then
    echo "Installing dependencies..."
    pip install -r requirements.txt
fi

# Run simulation
echo "Running 100 epochs (saves every 5)..."
python3 phext-life-11d.py \
    --epochs 100 \
    --save-interval 5 \
    --seed 42 \
    --output-dir demo_output

# Create GIF if ImageMagick is available
if command -v convert &> /dev/null; then
    echo ""
    echo "Creating animation..."
    convert -delay 10 demo_output/epoch_*.png demo_output/phext_life_demo.gif
    echo ""
    echo "✅ Done! Output in demo_output/"
    echo "   - Individual frames: epoch_*.png"
    echo "   - Animation: phext_life_demo.gif"
else
    echo ""
    echo "✅ Done! Frames saved in demo_output/epoch_*.png"
    echo "   Install ImageMagick to create GIF animation:"
    echo "   convert -delay 10 demo_output/epoch_*.png demo.gif"
fi

echo ""
echo "To run longer evolution:"
echo "  python3 phext-life-11d.py --epochs 1000 --seed 137"
