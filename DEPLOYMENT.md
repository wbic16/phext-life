# Deployment Guide

## GitHub Pages (Recommended for Mobile)

**Enable GitHub Pages:**
1. Go to https://github.com/wbic16/phext-life
2. Settings → Pages
3. Source: Deploy from branch
4. Branch: `exo` / `(root)`
5. Save

**Your site will be live at:** https://wbic16.github.io/phext-life/

**Share URL on phone:** Just open the GitHub Pages URL in Safari/Chrome

---

## Local Testing

### Web Version (No Installation)
```bash
cd phext-life
python3 -m http.server 8000
# Open http://localhost:8000 in browser
```

Or use any static file server:
```bash
npx serve .
# Or
php -S localhost:8000
```

### Python Version (Requires Dependencies)
```bash
pip install -r requirements.txt
python3 phext-life-11d.py --epochs 100
```

---

## Mobile Usage

**Controls:**
- ▶ Start - Begin evolution
- ⏸ Pause - Stop evolution
- ⏭ Step - Single epoch
- 🔄 Reset - Randomize all programs
- ⏩ Faster - 2× speed
- ⏪ Slower - 0.5× speed

**What to look for:**
- **Epoch 0-10:** Random noise (multicolored chaos)
- **Epoch 10-50:** Patterns emerge (self-replicators forming)
- **Epoch 50-500:** Waves spread (replicators taking over neighbors)
- **Epoch 500+:** Dominant pattern (or new replicator emerges)

**Colors:**
- Red/Green: Head movement
- Blue/Yellow: Alt head movement
- Magenta/Cyan: Increment/decrement
- Orange/Purple: Copy operations
- White/Gray: Loops
- Black: Data (non-instruction)

---

## Performance

**Web Version:**
- Desktop: 60+ epochs/second
- Mobile: 10-20 epochs/second
- Uses pure JavaScript (no WASM)
- Works offline after first load

**Python Version:**
- Desktop: 100+ epochs/second (with numba JIT)
- Can run much longer simulations
- Saves images to disk

---

## Sharing

**Quick share on Discord/Twitter:**
```
🔱 Artificial Life in 11D Phext Space
Watch self-replicating programs evolve: https://wbic16.github.io/phext-life/
729 programs compete in 9×9×9 coordinate space
```

**QR Code for phone:**
Generate at https://www.qr-code-generator.com/ with your GitHub Pages URL

---

## Tips

**Fast evolution:**
1. Start simulation
2. Click "Faster" 3-4 times (5-10× speed)
3. Watch for patterns to emerge

**Find interesting seeds:**
- Reset and start multiple times
- Some random initializations produce replicators faster
- Record the epoch when first pattern appears

**Screenshot cool patterns:**
- Pause when interesting pattern visible
- Right-click canvas → Save image
- Share on social media!

---

Built by Phex 🔱 · March 7, 2026
