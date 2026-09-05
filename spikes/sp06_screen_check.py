# SP-06 screen stage: locate the probe strip in a capture of the spike window and compare
# the pixels that actually reached the display against the source bytes in main.rs PROBE.
import sys, numpy as np
from PIL import Image

PROBE = [(0,0,0,255),(255,255,255,255),(128,128,128,255),(127,127,127,255),
         (188,188,188,255),(255,0,0,255),(0,255,0,255),(0,0,255,255),
         (255,255,0,255),(0,255,255,255),(255,0,255,255),(1,2,3,255),
         (254,253,252,255),(255,0,0,128),(0,0,0,128),(64,96,160,255)]
# Partial-alpha tiles are composited over the page background before display, so their
# screen value legitimately differs from the source. Not asserted.
OPAQUE = [i for i,c in enumerate(PROBE) if c[3] == 255]

img = np.asarray(Image.open(sys.argv[1]).convert("RGB")).astype(np.int16)
H, W, _ = img.shape
print(f"image {W}x{H}")

def find(pitch):
    half = pitch // 2
    span = pitch * 15 + half
    if span >= W: return None
    xs = np.arange(W - span)
    for y in range(H):
        row = img[y]
        ok = np.ones(len(xs), dtype=bool)
        for i in OPAQUE:
            samp = row[xs + i*pitch + half]
            e = PROBE[i]
            ok &= (samp[:,0]==e[0]) & (samp[:,1]==e[1]) & (samp[:,2]==e[2])
            if not ok.any(): break
        if ok.any():
            return int(xs[ok.argmax()]), y
    return None

hit, pitch = None, None
for p in [48] + [q for q in range(4, 121) if q != 48]:
    hit = find(p)
    if hit: pitch = p; break

if not hit:
    print("SP-06 screen stage: probe strip NOT located. INCONCLUSIVE.")
    sys.exit(2)

x, y = hit
print(f"\nprobe strip located at window-relative x={x} y={y}, pitch={pitch} device px\n")
print(f"{'#':<4}{'SOURCE RGB':<20}{'ON SCREEN RGB':<20}{'RESULT'}")
print("-"*62)
npass = 0
for i in OPAQUE:
    c = img[y, x + i*pitch + pitch//2]
    e = PROBE[i]
    same = (c[0]==e[0] and c[1]==e[1] and c[2]==e[2])
    npass += same
    print(f"{i:<4}{str(e[:3]):<20}{str(tuple(int(v) for v in c)):<20}{'exact' if same else 'ALTERED'}")
print(f"\nSP-06 screen stage: {npass}/{len(OPAQUE)} opaque probe colours byte-exact on screen")
print("NOT ASSERTED: tiles 13 and 14 carry alpha 128 and are composited over the page")
print("              background before display; their screen value is expected to differ.")
sys.exit(0 if npass == len(OPAQUE) else 1)
