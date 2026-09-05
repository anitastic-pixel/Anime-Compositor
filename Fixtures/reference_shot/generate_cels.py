#!/usr/bin/env python3
"""Generate layers 2-4 of the reference shot, per Markdown/22_Reference_Shots_and_Fixtures.md.

Layer 1 is the owner's painting and is NOT generated here.

Why generated: doc 22 says the owner draws the shot, and its stated reason is rights
clearance. Generated geometry is cleared by construction, same as owner art. What the
fixture actually needs from layers 2-4 is exact alpha behaviour - a strictly binary edge,
an exactly-50% interior - which hand-drawing cannot guarantee. Recorded as a specification
decision; see document 14.

This script is the record of how the cels were made. Rerunning it must reproduce them
byte-identically. It is not a build step: the PNGs are committed and are the fixture.
Regenerating to make a test pass is the same offense as editing an expected value.

Run:  python generate_cels.py          # writes cels + exposure_sheet.json, then self-checks
"""

import json
import math
import pathlib
import sys

from PIL import Image, ImageDraw, ImageFilter

W, H = 1920, 1080
FRAMES = 240
ROOT = pathlib.Path(__file__).parent

# Drawing counts are unspecified by doc 22 for layers 3 and 4; it fixes only cadence and
# edge type. Cycling sets are chosen to match layer 2's explicit "cycling" and to keep the
# shot small. Counts are recorded in the exposure sheet so the choice is inspectable.
L2_COUNT, L3_COUNT, L4_COUNT = 24, 12, 20

# Deliberate defects (doc 22). Required. Do not fix.
MISSING_ID = 7                      # layer 3 drawing 007 is never written -> numeric gap
JP_ID, JP_NAME = 13, "layer2_桜_013.png"   # layer 2 drawing 013 carries a Japanese filename

# One layer 4 exposure re-exposes an earlier drawing, so the drawing IDs go down as well as
# up. Real cel work does this constantly - production footage shows a level running
# 1, 4, 8, 6, 9 - and doc 20's exposure model permits it, since a span maps frames to a
# drawing number with no monotonicity constraint. Without this the shot would pass even if
# an implementation wrongly assumed drawing IDs only increase.
BACKREF_AT, BACKREF_TO = 55, 51


def _blank():
    return Image.new("RGBA", (W, H), (0, 0, 0, 0))


def layer2(i):
    """Soft antialiased edge: a feathered blob orbiting the frame. Reveals alpha/edge error."""
    im = _blank()
    a = 2 * math.pi * i / L2_COUNT
    cx, cy = W / 2 + 520 * math.cos(a), H / 2 + 300 * math.sin(a)
    r = 130
    ImageDraw.Draw(im).ellipse([cx - r, cy - r, cx + r, cy + r], fill=(230, 40, 90, 255))
    # Feather the whole RGBA together so colour stays consistent through the ramp.
    return im.filter(ImageFilter.GaussianBlur(9))


def layer3(i):
    """Hard aliased edge, pure binary alpha. Any interpolation shows up as a value that
    should not exist. Drawn with no antialiasing and never resampled or blurred."""
    im = _blank()
    d = ImageDraw.Draw(im)
    step = W // L3_COUNT
    x = i * step
    d.rectangle([x, 180, x + step - 1, 180 + 240 - 1], fill=(255, 220, 0, 255))
    d.rectangle([W - x - step, H - 300, W - x - 1, H - 300 + 200 - 1], fill=(0, 200, 255, 255))
    return im


def layer4(i):
    """Semi-transparent paint, interior at exactly 50% alpha (128/255). Primary exposure
    fixture: this is the layer whose irregular timing must survive."""
    im = _blank()
    a = 2 * math.pi * i / L4_COUNT
    cx, cy = W / 2 + 380 * math.sin(a), H / 2 - 120 * math.cos(a)
    s = 200
    ImageDraw.Draw(im).rectangle([cx - s, cy - s, cx + s, cy + s], fill=(40, 255, 120, 128))
    return im


def exposure_sheet():
    """Composition frame -> drawing ID per layer.

    Layer 2 on 1s, layer 3 on 2s, layer 4 on 3s except one five-frame hold and one
    one-frame accent: 78 threes + one 5 + one 1 = 240 frames across 80 exposures.
    """
    l2 = [i % L2_COUNT for i in range(FRAMES)]
    l3 = [(i // 2) % L3_COUNT for i in range(FRAMES)]

    holds = [3] * 78
    holds.insert(20, 5)   # the five-frame hold
    holds.insert(50, 1)   # the one-frame accent
    assert sum(holds) == FRAMES, sum(holds)

    ids = [k % L4_COUNT for k in range(len(holds))]
    ids[BACKREF_AT] = ids[BACKREF_TO]          # the out-of-order re-exposure
    l4 = [d for d, h in zip(ids, holds) for _ in range(h)]
    assert len(l4) == FRAMES

    return {
        "frames": FRAMES, "fps": 24, "width": W, "height": H,
        "drawing_counts": {"layer2": L2_COUNT, "layer3": L3_COUNT, "layer4": L4_COUNT},
        "layer4_exposure_lengths": holds,
        "layer4_exposure_drawing_ids": ids,
        "layer4_back_reference": {
            "exposure_index": BACKREF_AT, "reexposes_exposure": BACKREF_TO,
            "drawing_id": ids[BACKREF_AT],
            "note": "drawing IDs decrease here; an implementation that assumes they only "
                    "increase must fail on this shot",
        },
        "defects": {
            "missing_drawing": {"layer": 3, "id": MISSING_ID},
            "japanese_filename": {"layer": 2, "id": JP_ID, "name": JP_NAME},
        },
        "frame_to_drawing": {
            "layer1": [0] * FRAMES, "layer2": l2, "layer3": l3, "layer4": l4,
        },
    }


def write():
    for layer, count, fn in ((2, L2_COUNT, layer2), (3, L3_COUNT, layer3), (4, L4_COUNT, layer4)):
        d = ROOT / f"layer{layer}"
        d.mkdir(exist_ok=True)
        for i in range(count):
            if layer == 3 and i == MISSING_ID:
                continue                      # deliberate gap, not an error
            name = JP_NAME if (layer == 2 and i == JP_ID) else f"layer{layer}_{i:03d}.png"
            fn(i).save(d / name, optimize=True)

    sheet = exposure_sheet()
    (ROOT / "exposure_sheet.json").write_text(json.dumps(sheet, indent=2, ensure_ascii=False), "utf-8")
    return sheet


def check(sheet):
    """One runnable check. Fails loudly if any property the fixture exists to test breaks."""
    import numpy as np
    ok = True

    def bad(msg):
        nonlocal ok
        print("FAIL:", msg)
        ok = False

    bg = ROOT / "layer1" / "layer1_000.png"
    if not bg.exists():
        bad(f"layer 1 background missing at {bg}")
    else:
        a = np.array(Image.open(bg).convert("RGBA"))
        if a.shape[:2] != (H, W):
            bad(f"layer 1 is {a.shape[1]}x{a.shape[0]}, want {W}x{H}")
        if a[..., 3].min() != 255:
            bad("layer 1 is not fully opaque")

    for layer, count in ((2, L2_COUNT), (3, L3_COUNT), (4, L4_COUNT)):
        for i in range(count):
            name = JP_NAME if (layer == 2 and i == JP_ID) else f"layer{layer}_{i:03d}.png"
            p = ROOT / f"layer{layer}" / name
            if layer == 3 and i == MISSING_ID:
                if p.exists():
                    bad("the deliberate missing frame was filled in; doc 22 says do not fix it")
                continue
            if not p.exists():
                bad(f"missing {p}")
                continue
            alpha = np.array(Image.open(p).convert("RGBA"))[..., 3]
            if layer == 2 and len(np.unique(alpha)) < 3:
                bad(f"{name}: soft layer has no antialiased ramp")
            if layer == 3 and not set(np.unique(alpha)) <= {0, 255}:
                bad(f"{name}: hard layer alpha is not binary")
            if layer == 4 and not set(np.unique(alpha)) <= {0, 128}:
                bad(f"{name}: semi-transparent layer interior is not exactly 128")

    lens = sheet["layer4_exposure_lengths"]
    if sum(lens) != FRAMES:
        bad(f"layer 4 exposures sum to {sum(lens)}, want {FRAMES}")
    if lens.count(5) != 1 or lens.count(1) != 1:
        bad("layer 4 needs exactly one five-frame hold and one one-frame accent")

    ids = sheet["layer4_exposure_drawing_ids"]
    drops = [k for k in range(1, len(ids)) if ids[k] < ids[k - 1] and ids[k - 1] != L4_COUNT - 1]
    if not drops:
        bad("layer 4 has no out-of-order re-exposure; drawing IDs only ever increase")
    reused = ids[sheet["layer4_back_reference"]["exposure_index"]]
    if ids.count(reused) < 2:
        bad(f"layer 4 drawing {reused} is meant to be exposed more than once")
    for name, seq in sheet["frame_to_drawing"].items():
        if len(seq) != FRAMES:
            bad(f"{name} exposure list is {len(seq)} long, want {FRAMES}")

    print("OK: all reference shot checks passed" if ok else "reference shot is NOT valid")
    return ok


if __name__ == "__main__":
    sys.exit(0 if check(write()) else 1)
