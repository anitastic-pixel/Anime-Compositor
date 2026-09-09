# Derivation for P-02: how much of each fixture's layer stack composites identically frame after
# frame, and would therefore be reusable from a cache of the partial composite.
#
# Document 32 section 6.2 argues that a run of frames whose lower layers hold the same drawings,
# transforms, effects and mattes composites those layers identically every frame, so a
# deterministic renderer can cache that partial composite on a hash of its inputs. Document 33
# agrees it is promising *if the stable prefix dominates cost*, and neither document knows whether
# it does. This script counts, on paper. It never runs the compositor.
#
#     python verification/derive_p02_prefix.py
#
# The numbers it prints are the ones quoted in verification/P-02_prefix_reuse.md. If that file and
# this script ever disagree, this script is right and the artifact was edited by hand.
#
# Two workloads, the same two P-01 measured:
#
#   the reference shot          verification/B-08a_project.json, four layers
#   the declared ten-layer      built from the reference shot the way tests/common/mod.rs builds
#                               it: ten layers whose exposures are the reference shot's layers
#                               1,2,3,4,2,3,4,2,3,4, two alpha mattes, three effect instances
#
# Layer order is bottom of the stack first, which is src/compose.rs line 138's order and
# Composition::layers_in_order's contract.

import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.join(HERE, os.pardir)
SHEET = os.path.join(REPO, "Fixtures", "reference_shot", "exposure_sheet.json")
BASE = os.path.join(HERE, "B-08a_project.json")

# From tests/common/mod.rs: which reference-shot layer each of the ten copies takes its exposure
# from, and which layer is matted by which.
SOURCES = [1, 2, 3, 4, 2, 3, 4, 2, 3, 4]
MATTES = {5: 4, 8: 7}
# The three effect instances the declared fixture puts on layers 2, 6 and 9 (one-based). Their
# parameters are constants in that file, so they are constants here.
EFFECTS = {2: "core.exposure", 6: "core.gaussian_blur", 9: "core.tint"}

with open(SHEET, encoding="utf-8") as f:
    sheet = json.load(f)
with open(BASE, encoding="utf-8") as f:
    base = json.load(f)

FRAMES = sheet["frames"]
EXPOSURE = sheet["frame_to_drawing"]  # "layer1".."layer4" -> 240 drawing ids


def assert_nothing_is_animated(doc, where):
    """P-02's result is only valid if no property is animated.

    A layer's partial composite is stable between two frames when *every* input is the same: the
    drawing, the transform, the opacity, the blend mode, the mask, every effect parameter, and the
    matte. With no keyframes anywhere, the only input that moves is the drawing, and this script
    can decide stability by comparing drawings alone. With a keyframe present it could not, and it
    would silently overcount. So it refuses instead.
    """
    found = []

    def walk(node, path):
        if isinstance(node, dict):
            if isinstance(node.get("keyframes"), list) and node["keyframes"]:
                found.append(path)
            for k, v in node.items():
                walk(v, path + "/" + k)
        elif isinstance(node, list):
            for i, v in enumerate(node):
                walk(v, path + "/" + str(i))

    walk(doc, "")
    assert not found, "%s has animated properties, so this script cannot decide stability: %s" % (
        where,
        found,
    )


assert_nothing_is_animated(base, "verification/B-08a_project.json")


class Workload(object):
    """A layer stack, bottom first, and what each layer shows on each frame.

    `signature[i][f]` is everything about layer i on frame f that its own pixels depend on. Two
    frames composite layer i identically exactly when these are equal, which is the whole basis of
    the count below.
    """

    def __init__(self, name, layers):
        self.name = name
        self.layers = layers  # list of dicts, bottom first
        self.signature = [
            [self.sign(i, f) for f in range(FRAMES)] for i in range(len(layers))
        ]

    def sign(self, index, frame):
        layer = self.layers[index]
        drawing = EXPOSURE[layer["exposure"]][frame]
        part = (layer["asset"], drawing, layer["effect"], layer["blend"])
        matte = layer["matte"]
        if matte is not None:
            # A matted layer's pixels depend on the matte layer's pixels as well. The matte is
            # always below in both fixtures, so this recursion terminates and stays inside the
            # prefix it belongs to.
            part = part + ("matte", self.sign(matte, frame))
        return part


def reference_shot():
    order = base["compositions"][0]["layer_order"]
    layers = []
    for layer_id in order:
        layer = next(
            l for l in base["compositions"][0]["layers"] if l["id"] == layer_id
        )
        layers.append(
            {
                "name": layer["name"],
                "asset": layer["asset_id"],
                "exposure": layer["name"],
                "effect": tuple(e["type_id"] for e in layer.get("effects") or []),
                "blend": layer.get("blend_mode", "normal"),
                "matte": None,
            }
        )
    return Workload("the reference shot (4 layers)", layers)


def declared_fixture():
    layers = []
    for index, source in enumerate(SOURCES):
        n = index + 1
        layers.append(
            {
                "name": "copy%d" % n,
                "asset": "asset-%d" % n,
                "exposure": "layer%d" % source,
                "effect": (EFFECTS[n],) if n in EFFECTS else (),
                "blend": "normal",
                "matte": MATTES[n] - 1 if n in MATTES else None,
            }
        )
    return Workload("the declared ten-layer fixture (10 layers)", layers)


def report(w):
    n = len(w.layers)
    print("=" * 78)
    print(w.name)
    print("=" * 78)
    print()
    print("Layers, bottom of the stack first:")
    for i, layer in enumerate(w.layers):
        distinct = len(set(w.signature[i]))
        changes = sum(
            1 for f in range(1, FRAMES) if w.signature[i][f] != w.signature[i][f - 1]
        )
        print(
            "  %2d. %-8s exposes %-7s %3d distinct states, changes on %3d of %d frames"
            % (i + 1, layer["name"], layer["exposure"], distinct, changes, FRAMES - 1)
        )
    print()

    # How deep the stable prefix goes between consecutive frames: the count of layers from the
    # bottom whose signature is the same on frame f as on frame f-1.
    depths = []
    for f in range(1, FRAMES):
        depth = 0
        while depth < n and w.signature[depth][f] == w.signature[depth][f - 1]:
            depth += 1
        depths.append(depth)

    print("Stable prefix depth between one frame and the next, over %d transitions:" % len(depths))
    print("  | Depth | Transitions | Share |")
    print("  |---|---|---|")
    for depth in range(n + 1):
        count = depths.count(depth)
        if count:
            print(
                "  | %d of %d layers | %d | %.1f%% |"
                % (depth, n, count, 100.0 * count / len(depths))
            )
    print()

    # What a cache of partial composites would actually save, with no bound on its size: for each
    # frame, the deepest prefix whose exact input tuple this playthrough has composited before.
    # Saving is that many layer composites the frame does not have to do.
    held = set()
    saved = 0
    per_depth = {}
    for f in range(FRAMES):
        prefix = ()
        best = 0
        keys = []
        for i in range(n):
            prefix = prefix + (w.signature[i][f],)
            keys.append(prefix)
            if prefix in held:
                best = i + 1
        saved += best
        per_depth[best] = per_depth.get(best, 0) + 1
        held.update(keys)
    total = FRAMES * n
    print(
        "With an unbounded cache of partial composites, over one 240-frame playthrough:\n"
        "  layer composites without it: %d\n"
        "  layer composites a cache hit removes: %d, which is %.1f%% of them\n"
        "  distinct prefix composites the cache would hold: %d"
        % (total, saved, 100.0 * saved / total, len(held))
    )
    print()
    print("  Reuse depth reached, by frame:")
    for depth in sorted(per_depth):
        print(
            "    %d of %d layers on %d frames (%.1f%%)"
            % (depth, n, per_depth[depth], 100.0 * per_depth[depth] / FRAMES)
        )
    print()

    # The same simulation with a bound on the cache, which is the only kind that could ship: D-40
    # sets DEFAULT_BUDGET_BYTES to 1 GiB for the whole preview cache, and a prefix cache would be
    # competing with the cel cache for it, not adding to it.
    def bounded(capacity):
        from collections import OrderedDict

        held = OrderedDict()
        saved = 0
        for f in range(FRAMES):
            prefix = ()
            best = 0
            keys = []
            for i in range(n):
                prefix = prefix + (w.signature[i][f],)
                keys.append(prefix)
                if prefix in held:
                    held.move_to_end(prefix)
                    best = i + 1
            saved += best
            for key in keys:
                held[key] = True
                held.move_to_end(key)
                if capacity and len(held) > capacity:
                    held.popitem(last=False)
        return saved

    print("  With a bound on the cache, which is the only kind that could ship:")
    print("  | Prefix composites held | Memory | Layer composites removed | Share |")
    print("  |---|---|---|---|")
    for capacity in [1, 4, 8, 16, 32, 64, 128, 256, 512, 0]:
        s = bounded(capacity)
        label = "unbounded" if capacity == 0 else str(capacity)
        held_n = len(held) if capacity == 0 else capacity
        cost = held_n * sheet["width"] * sheet["height"] * 16 / float(1 << 30)
        print(
            "  | %s | %.2f GiB | %d | %.1f%% |"
            % (label, cost, s, 100.0 * s / total)
        )
    print()

    # What holding those composites costs. A composited prefix is a whole composition-sized
    # working buffer: four f32 channels a pixel, which is what src/lib.rs's WorkingBuffer is.
    buffer_bytes = sheet["width"] * sheet["height"] * 4 * 4
    print(
        "  One prefix composite is a %dx%d working buffer, four f32 channels a pixel: %d bytes.\n"
        "  Holding all %d costs %.2f GiB, against the 1 GiB DEFAULT_BUDGET_BYTES D-40 set."
        % (
            sheet["width"],
            sheet["height"],
            buffer_bytes,
            len(held),
            len(held) * buffer_bytes / float(1 << 30),
        )
    )
    print()
    return depths, saved, total, len(held)


for workload in [reference_shot(), declared_fixture()]:
    report(workload)
