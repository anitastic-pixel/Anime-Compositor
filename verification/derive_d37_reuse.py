# Derivation for D-37: how often the reference shot asks for a drawing it has already asked for.
#
# This reads Fixtures/reference_shot/exposure_sheet.json, which is the specification of the shot,
# and counts. It never runs the compositor, so nothing here is a measurement of the code under
# test - it is arithmetic over the fixture, in the same spirit as derive_b02_expected.py.
#
#     python verification/derive_d37_reuse.py
#
# The numbers it prints are the ones quoted in verification/D-37_decode_cost.md. If that file and
# this script ever disagree, this script is right and the artifact was edited by hand.

import json
import os
from collections import OrderedDict

HERE = os.path.dirname(os.path.abspath(__file__))
SHEET = os.path.join(HERE, os.pardir, "Fixtures", "reference_shot", "exposure_sheet.json")

with open(SHEET, encoding="utf-8") as f:
    sheet = json.load(f)

frames = sheet["frames"]
layers = sorted(sheet["frame_to_drawing"])
per_frame = sheet["frame_to_drawing"]

# One playthrough, in the order a player asks: frame by frame, and within a frame bottom layer
# first, which is the order compose::plan_frame walks the stack.
requests = [(name, per_frame[name][f]) for f in range(frames) for name in layers]

print("frames: %d, layers: %d" % (frames, len(layers)))
print("decode requests in one playthrough: %d" % len(requests))
print("distinct drawings the shot uses: %d" % len(set(requests)))
print()

print("How often each layer changes its drawing:")
for name in layers:
    v = per_frame[name]
    changes = sum(1 for i in range(1, frames) if v[i] != v[i - 1])
    print(
        "  %s: %d distinct drawings, %d changes across the shot, so %d of its %d requests "
        "repeat the previous frame" % (name, len(set(v)), changes, frames - 1 - changes, frames)
    )
print()


def misses(capacity):
    """Decodes a playthrough still needs with a least-recently-used cache of `capacity` cels.

    capacity 0 means unbounded, which is the floor: every distinct drawing decoded exactly once.
    """
    held = OrderedDict()
    count = 0
    for key in requests:
        if key in held:
            held.move_to_end(key)
            continue
        count += 1
        held[key] = True
        if capacity and len(held) > capacity:
            held.popitem(last=False)
    return count


print("Decodes per playthrough, by how many cels a cache is allowed to hold:")
print("  | Cels held | Decodes | Of the 960 requests, avoided |")
print("  |---|---|---|")
for capacity in [1, 4, 8, 16, 24, 32, 48, 56, 57, 64, 0]:
    m = misses(capacity)
    label = "unbounded" if capacity == 0 else str(capacity)
    print("  | %s | %d | %.1f%% |" % (label, m, 100.0 * (1 - float(m) / len(requests))))
print()

# A decoded cel is width x height x 4 bytes whatever its file compresses to, so the memory a
# cache costs is the count times one frame's worth of samples.
cel = sheet["width"] * sheet["height"] * 4
print(
    "One decoded cel at %dx%d RGBA is %d bytes, so holding all %d costs %.0f MB."
    % (
        sheet["width"],
        sheet["height"],
        cel,
        len(set(requests)),
        len(set(requests)) * cel / 1e6,
    )
)
