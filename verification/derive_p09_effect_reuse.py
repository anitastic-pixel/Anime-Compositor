# Derivation for P-09: how many of a playthrough's effect evaluations are repeats of one this
# build has already done.
#
# verification/P-01_frame_trace.md measures the effect stack at 23.2% to 65.2% of every
# declared-fixture frame, which is the largest or second largest named stage in every one of its
# rows. ADR-017 runs the stack whole-layer, before the frame plan, on the cel's own pixels, so an
# evaluation depends on exactly two things: which drawing the layer shows, and the resolved effect
# stack. Neither research document ranked this stage, so nobody has counted how often those two
# things repeat. This script counts. It never runs the compositor.
#
#     python verification/derive_p09_effect_reuse.py
#
# The numbers it prints are the ones quoted in verification/P-09_effect_reuse.md. If that file and
# this script ever disagree, this script is right and the artifact was edited by hand.
#
# The two workloads are P-01's. The reference shot carries no effects at all, which is why P-01
# measures its effect stack at 0.0%; the declared ten-layer fixture carries three instances, one
# of each kind this build has, and tests/common/mod.rs is where they are declared.

import json
import os
from collections import OrderedDict

HERE = os.path.dirname(os.path.abspath(__file__))
SHEET = os.path.join(HERE, os.pardir, "Fixtures", "reference_shot", "exposure_sheet.json")

with open(SHEET, encoding="utf-8") as f:
    sheet = json.load(f)

FRAMES = sheet["frames"]
EXPOSURE = sheet["frame_to_drawing"]
BUFFER_BYTES = sheet["width"] * sheet["height"] * 4 * 4

# From tests/common/mod.rs build_fixture: SOURCES says which reference-shot layer each copy takes
# its exposure from, and the three effect instances go on copies 2, 6 and 9 (one-based).
SOURCES = [1, 2, 3, 4, 2, 3, 4, 2, 3, 4]
INSTANCES = [
    # (layer, effect instance, its type, whether it reads neighbouring pixels)
    (2, "fx-exposure", "core.exposure", False),
    (6, "fx-blur", "core.gaussian_blur", True),
    (9, "fx-tint", "core.tint", False),
]

print("The reference shot carries no effect instances at all, so it has nothing to count and")
print("P-01 measures its effect stack at 0.0% of a frame. Everything below is the declared")
print("ten-layer fixture.")
print()

rows = []
for layer, instance, type_id, neighbours in INSTANCES:
    source = "layer%d" % SOURCES[layer - 1]
    drawings = EXPOSURE[source]
    # The stack and its parameters are constants in this fixture -- verification/P-02_prefix_reuse.md
    # asserts that no property in either project is animated -- so an evaluation's whole input is
    # the drawing it runs on.
    distinct = len(set(drawings))
    evaluations = FRAMES
    rows.append((layer, instance, type_id, source, distinct, evaluations, neighbours))

print("Per effect instance, over one 240-frame playthrough:")
print()
print("| Layer | Instance | Kind | Exposes | Evaluations | Distinct inputs | Repeats | "
      "Memory to hold every distinct result |")
print("|---|---|---|---|---|---|---|---|")
total_eval = 0
total_repeat = 0
for layer, instance, type_id, source, distinct, evaluations, _ in rows:
    repeats = evaluations - distinct
    total_eval += evaluations
    total_repeat += repeats
    print(
        "| copy%d | %s | %s | %s | %d | %d | %d (%.1f%%) | %.2f GiB |"
        % (
            layer,
            instance,
            type_id,
            source,
            evaluations,
            distinct,
            repeats,
            100.0 * repeats / evaluations,
            distinct * BUFFER_BYTES / float(1 << 30),
        )
    )
print(
    "| **all three** | | | | **%d** | **%d** | **%d (%.1f%%)** | **%.2f GiB** |"
    % (
        total_eval,
        sum(r[4] for r in rows),
        total_repeat,
        100.0 * total_repeat / total_eval,
        sum(r[4] for r in rows) * BUFFER_BYTES / float(1 << 30),
    )
)
print()
print(
    "One held result is a whole source-sized buffer, %dx%d with four f32 channels a pixel: "
    "%d bytes, the same as a decoded cel." % (sheet["width"], sheet["height"], BUFFER_BYTES)
)
print()


def bounded(keys, capacity):
    """Evaluations a least-recently-used cache of `capacity` results would remove."""
    held = OrderedDict()
    removed = 0
    for key in keys:
        if key in held:
            held.move_to_end(key)
            removed += 1
            continue
        held[key] = True
        if capacity and len(held) > capacity:
            held.popitem(last=False)
    return removed


# The requests in the order a player makes them: frame by frame, and within a frame in composition
# order, which is the order src/compose.rs line 139 walks the stack.
requests = [
    (instance, EXPOSURE["layer%d" % SOURCES[layer - 1]][f])
    for f in range(FRAMES)
    for layer, instance, _, _ in INSTANCES
]
assert len(requests) == total_eval

print("Evaluations a bounded cache of effect results would remove, all three instances sharing it:")
print()
print("| Results held | Memory | Evaluations removed | Share of %d |" % total_eval)
print("|---|---|---|---|")
for capacity in [4, 8, 12, 16, 24, 32, 48, 64, 0]:
    removed = bounded(requests, capacity)
    label = "unbounded" if capacity == 0 else str(capacity)
    held_n = len(set(requests)) if capacity == 0 else capacity
    print(
        "| %s | %.2f GiB | %d | %.1f%% |"
        % (label, held_n * BUFFER_BYTES / float(1 << 30), removed, 100.0 * removed / total_eval)
    )
print()

# The blur on its own, because it is the only one of the three whose output pixel reads more than
# its own input pixel, and it is therefore the one that costs.
blur = [k for k in requests if k[0] == "fx-blur"]
print("The blur alone, cached on its own:")
print(
    "  %d evaluations, %d distinct inputs, so %d repeats (%.1f%%), for %.2f GiB held."
    % (
        len(blur),
        len(set(blur)),
        len(blur) - len(set(blur)),
        100.0 * (len(blur) - len(set(blur))) / len(blur),
        len(set(blur)) * BUFFER_BYTES / float(1 << 30),
    )
)
