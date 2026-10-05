"""Time Remapping, worked a second way.

D-323 (from D-308, approved by the owner on 2026-10-04 after P-26's tutorial 1): a raster or
composition layer may carry a Time Remap property, `time_remap`, a keyable number of source
frames. Absent, the layer plays as document 20's D-216 rule says. Present, at composition frame
`n` inside the layer's in and out points, the source time is

    t = R(u),    u = I + (n - I) * 100 / S

the property read at the key time `u` like every other key of the layer, so a stretch stretches
the remap keys too. The source offset is not used. A `t` within 1e-9 of a whole frame is that
whole frame, so a straight line of keys never lands a hair short of a drawing. Then, as D-216:

    f = floor(t),    w = t - f

and the picture is `P(f)`, mixed with `P(f + 1)` by `w` only when both frame blending switches
are on and `f + 1` is inside the source. `P` of a frame outside the source is clear.

Turning Time Remapping on writes two linear keys that leave the picture as it was: at the in
point `I`, the value `O` (the offset), and at `k = max(I + 1, ceil(u(out - 1)))`, the value
`k - I + O`. Freeze Frame at the playhead `n` writes one hold key, replacing any others, at
`u(n)` rounded half away from zero, holding the source time `t` there. Turning it off removes
the property. Moving the layer moves its remap keys with it.

**This file never runs the build's code path.** It reuses `frame_blending_reference.py`'s
drawings and one-pixel-high frames, with its source time replaced by the rule above.

The projects go into `Fixtures/time_remap`, the expected numbers into
`Fixtures/time_remap/expected_time_remap.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/time_remap_reference.py
"""

import copy
import json
import sys
from math import ceil, floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import frame_blending_reference as fb  # noqa: E402
from motion_blur_reference import key, null, png, prop, solid, value_at  # noqa: E402
from ease_reference import EASY_EASE  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "time_remap"
SNAP = 1e-9


def snap(t):
    r = floor(t + 0.5)
    return float(r) if abs(t - r) <= SNAP else t


def plain_time(layer, n):
    return ((n - layer["in_frame"]) * 100) / layer.get("time_stretch", 100) + layer["source_offset_frames"]


def source_time(layer, n):
    """(t, f, w) at composition frame `n`, by the remap when the layer has one."""
    if "time_remap" in layer:
        t = snap(value_at(layer["time_remap"], fb.key_time(layer, n)))
    else:
        t = plain_time(layer, n)
    f = floor(t)
    return t, f, t - f


fb.source_time = source_time  # fb.source and fb.render now draw by the remap


def remap(*keys):
    return prop(keys[0]["value"], list(keys))


def lin(frame, value):
    return key(frame, value)


def hold(frame, value):
    return key(frame, value, "hold")


def eased(frame, value):
    return key(frame, value, "ease", ease=list(EASY_EASE))


def raster(keys, **kw):
    r = fb.raster(**kw)
    if keys is not None:
        r["time_remap"] = remap(*keys)
    return r


def nested(keys, offset=0, **kw):
    r = fb.nested(inner="comp-slide", **kw)
    r["source_offset_frames"] = offset
    if keys is not None:
        r["time_remap"] = remap(*keys)
    return r


def project(name, layers, blend=True):
    inner = [fb.INNER_SLIDE] if any(l["kind"] == "composition" for l in layers) else None
    return fb.project(name, layers, blend=blend, inner=inner)


# --- times -----------------------------------------------------------------------------------

def layer_for(i, o, s, keys):
    l = {"in_frame": i, "source_offset_frames": o, "time_remap": remap(*keys)}
    if s != 100:
        l["time_stretch"] = s
    return l


def snap_case():
    """Two keys, 0 and 49, from 0 to 49: some frames between land a hair off whole in 64-bit
    numbers. Returns the first such frame and its raw value."""
    for n in range(1, 49):
        raw = 0 + (n / 49) * (49 - 0)
        if raw != n:
            return n, raw
    raise AssertionError("no frame lands off whole")


SNAP_N, SNAP_RAW = snap_case()

TIMES = {
    "FX-TREMAP-001": ("Keys 0 at 0 and 11 at 11: t is n, as with no remap.", 0, 0, 100,
                      [lin(0, 0), lin(11, 11)], [0, 3, 11]),
    "FX-TREMAP-002": ("Keys 0 at 0 and 11 at 6: twelve frames of source in six, then held on 11.",
                      0, 0, 100, [lin(0, 0), lin(6, 11)], [0, 3, 6, 9]),
    "FX-TREMAP-003": ("Tutorial 1's burst then real speed: 0 at 0, 8 at 4, 16 at 12; two frames "
                      "of source a frame, then one.", 0, 0, 100,
                      [lin(0, 0), lin(4, 8), lin(12, 16)], [0, 2, 4, 8, 11]),
    "FX-TREMAP-004": ("One key, 5 at 3: every frame is source frame 5, a freeze.", 0, 0, 100,
                      [hold(3, 5)], [0, 3, 9]),
    "FX-TREMAP-005": ("11 at 0 and 0 at 11: backwards.", 0, 0, 100, [lin(0, 11), lin(11, 0)], [0, 5, 11]),
    "FX-TREMAP-006": ("A hold key, 2 at 0, then 9 at 6: 2 until frame 6, then 9.", 0, 0, 100,
                      [hold(0, 2), lin(6, 9)], [0, 5, 6, 8]),
    "FX-TREMAP-007": ("Stretch 200, keys 0 at 0 and 8 at 4: the keys are read at u = n / 2, so "
                      "t is n until u passes 4.", 0, 0, 200, [lin(0, 0), lin(4, 8)], [0, 1, 2, 8, 10]),
    "FX-TREMAP-008": ("Easy ease from 0 at 0 to 8 at 8: slow, fast, slow.", 0, 0, 100,
                      [eased(0, 0), lin(8, 8)], [0, 2, 4, 6, 8]),
    "FX-TREMAP-009": ("In 3 and an offset of 5: the offset is not used, 0 at 3 and 6 at 9 give "
                      "t = n - 3.", 3, 5, 100, [lin(3, 0), lin(9, 6)], [3, 6, 9]),
    "FX-TREMAP-030": (f"0 at 0 and 49 at 49: at frame {SNAP_N} the straight line lands at "
                      f"{SNAP_RAW!r} in 64-bit numbers, within 1e-9 of {SNAP_N}, so t is "
                      f"{SNAP_N} and the drawing is frame {SNAP_N}'s.", 0, 0, 100,
                      [lin(0, 0), lin(49, 49)], [0, SNAP_N, 49]),
}

# --- pictures --------------------------------------------------------------------------------

F6, F8, F10, F12 = (list(range(k)) for k in (6, 8, 10, 12))

PIXELS = {
    "FX-TREMAP-010": ("The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, "
                      "green, green, as without a remap.",
                      project("FX-TREMAP-010", [raster([lin(0, 0), lin(5, 5)], out_frame=6)]), F6),
    "FX-TREMAP-011": ("One key, 2 at 0: frozen on source frame 2, blue, all six frames.",
                      project("FX-TREMAP-011", [raster([hold(0, 2)], out_frame=6)]), F6),
    "FX-TREMAP-012": ("A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, "
                      "4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops "
                      "at the inner composition's last frame.",
                      project("FX-TREMAP-012", [nested([lin(0, 0), lin(2, 4), lin(5, 7)])]), F8),
    "FX-TREMAP-013": ("The same layer backwards, 7 at 0 and 0 at 7: the dot moves left.",
                      project("FX-TREMAP-013", [nested([lin(0, 7), lin(7, 0)])]), F8),
    "FX-TREMAP-014": ("The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame "
                      "blending switches on: frames 3 and 7 fall half way between two drawings "
                      "and are half of each.",
                      project("FX-TREMAP-014", [raster([lin(0, 0), lin(8, 4)], mix=True)]), F10),
    "FX-TREMAP-015": ("The same with the layer's frame blending off: each source time rounds "
                      "down, each drawing held four frames.",
                      project("FX-TREMAP-015", [raster([lin(0, 0), lin(8, 4)])]), F10),
    "FX-TREMAP-016": ("Keys -2 at 0 and 10 at 12: before the inner composition starts and after "
                      "it ends the layer is clear.",
                      project("FX-TREMAP-016", [nested([lin(0, -2), lin(12, 10)])]), F12),
    "FX-TREMAP-017": ("Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot "
                      "moves a pixel a frame as with no remap and no stretch.",
                      project("FX-TREMAP-017", [nested([lin(0, 0), lin(4, 8)], stretch=200)]), F10),
    "FX-TREMAP-018": ("In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used.",
                      project("FX-TREMAP-018", [nested([lin(3, 0), lin(9, 6)], offset=5, in_frame=3)]), F12),
    "FX-TREMAP-019": ("Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each "
                      "source time rounds down.",
                      project("FX-TREMAP-019", [nested([eased(0, 0), lin(7, 7)])]), F8),
}

# --- commands --------------------------------------------------------------------------------

ON = {
    "FX-TREMAP-040": ("In 0, out 12: keys 0 at 0 and 11 at 11.", 0, 12, 0, 100),
    "FX-TREMAP-041": ("In 5, out 20, offset 3: keys 3 at 5 and 17 at 19.", 5, 20, 3, 100),
    "FX-TREMAP-042": ("Stretch 200, in 0, out 12: the last frame's key time is 5.5, so keys 0 at 0 "
                      "and 6 at 6.", 0, 12, 0, 200),
    "FX-TREMAP-043": ("Stretch 50, in 0, out 6: keys 0 at 0 and 10 at 10.", 0, 6, 0, 50),
    "FX-TREMAP-044": ("One frame long, in 4, out 5, offset 2: keys 2 at 4 and 3 at 5.", 4, 5, 2, 100),
}


def turned_on(i, o, offset, s):
    layer = {"in_frame": i, "time_stretch": s}
    k = max(i + 1, ceil(fb.key_time(layer, o - 1)))
    return [lin(i, offset), lin(k, k - i + offset)]


FREEZES = {
    "FX-TREMAP-045": ("No remap, freeze at 5: one hold key, 5 at 5.", 0, 12, 0, 100, None, 5),
    "FX-TREMAP-046": ("Stretch 200, no remap, freeze at 5: the key time 2.5 rounds to 3, holding "
                      "the source time 2.5.", 0, 12, 0, 200, None, 5),
    "FX-TREMAP-047": ("Keys 0 at 0 and 11 at 6, freeze at 3: the keys are replaced by one, 5.5 at 3.",
                      0, 12, 0, 100, [lin(0, 0), lin(6, 11)], 3),
}


def frozen(i, o, offset, s, keys, n):
    layer = {"in_frame": i, "source_offset_frames": offset, "time_stretch": s}
    if keys:
        layer["time_remap"] = remap(*keys)
    t, _, _ = source_time(layer, n)
    return [hold(fb.round_half_away(fb.key_time(layer, n)), t)]


SHIFTS = {
    "FX-TREMAP-048": ("Keys 0 at 0 and 11 at 6, the layer moved 2 frames later: the keys move to 2 "
                      "and 8, the values stay.", [lin(0, 0), lin(6, 11)], 2),
}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each FX-TREMAP-012's with one change."""
    def edit(change):
        p = copy.deepcopy(PIXELS["FX-TREMAP-012"][1])
        change(p["compositions"][0])
        return p

    def lay(k, v):
        return lambda c: c["layers"][0].__setitem__(k, v)

    def add(record):
        return lambda c: (c["layers"].append(record), c["layer_order"].append(record["id"]))

    card = solid(id="card", position=[0, 0], width=8)
    card["time_remap"] = remap(lin(0, 0))
    rig = null(position=[0, 0])
    rig["time_remap"] = remap(lin(0, 0))
    adjustment = solid(id="adj", position=[0, 0])
    adjustment["kind"] = "adjustment"
    del adjustment["solid"]
    adjustment["time_remap"] = remap(lin(0, 0))
    scripted = remap(lin(0, 0), lin(5, 7))
    scripted["expression"] = {"text": "time * 24", "enabled": True}
    return {
        "FX-TREMAP-050": ("Time Remap on a solid layer, which has no source time.", edit(add(card))),
        "FX-TREMAP-051": ("Time Remap on a null layer.", edit(add(rig))),
        "FX-TREMAP-052": ("Time Remap on an adjustment layer.", edit(add(adjustment))),
        "FX-TREMAP-053": ("A Time Remap key that is a pair of numbers.",
                          edit(lay("time_remap", remap(lin(0, [0, 0]), lin(5, [7, 0]))))),
        "FX-TREMAP-054": ("A Time Remap with an expression, which is not part of this.",
                          edit(lay("time_remap", scripted))),
        "FX-TREMAP-055": ("A Time Remap written as a bare number rather than a property.",
                          edit(lay("time_remap", 3))),
    }


# --- printing --------------------------------------------------------------------------------

def fmt(v):
    return f"{v:.17g}" if isinstance(v, float) else str(v)


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in fb.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": fb.TOLERANCE, "pixel_tolerance": fb.PIXEL_TOLERANCE, "width": fb.W,
                "height": fb.H, "times": {}, "cases": {}, "on": {}, "freezes": {}, "shifts": {},
                "refused": {}}

    print("Times. For each frame n, the key time u, the source time t, its whole frame f and the "
          "share w of the next.\n")
    print("| case | in | offset | stretch | n | u | t | f | w |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for fx, (says, i, o, s, keys, ns) in TIMES.items():
        layer = layer_for(i, o, s, keys)
        rows = [dict(zip(("t", "f", "w"), source_time(layer, n)), n=n, u=fb.key_time(layer, n)) for n in ns]
        expected["times"][fx] = {"says": says, "in_frame": i, "source_offset_frames": o,
                                 "time_stretch": s, "time_remap": layer["time_remap"], "rows": rows}
        for r in rows:
            print(f"| {fx} | {i} | {o} | {s} | {r['n']} | {fmt(r['u'])} | {fmt(r['t'])} | {r['f']} "
                  f"| {fmt(r['w'])} |")
    print()
    for fx, (says, *_) in TIMES.items():
        print(f"{fx}: {says}")
    print()

    print("Pictures. Each value is a pixel's red, green, blue and covering, linear and "
          "premultiplied.\n")
    for fx, (says, proj, frames) in PIXELS.items():
        log = []
        rendered = {str(f): fb.render(proj, "comp-main", f, log) for f in frames}
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(proj, indent=2) + "\n", encoding="utf-8")
        said = sorted({(w["frame"], w["code"]) for w in log})
        expected["cases"][fx] = {"says": says, "project": name, "frames": rendered,
                                 "diagnostics": [{"frame": f, "code": c} for f, c in said]}
        print(f"{fx}: {says}\n")
        fb.print_frames(rendered, log)

    print("Turning Time Remapping on: the two keys written.\n")
    for fx, (says, i, o, off, s) in ON.items():
        keys = turned_on(i, o, off, s)
        expected["on"][fx] = {"says": says, "in_frame": i, "out_frame": o, "source_offset_frames": off,
                              "time_stretch": s, "keys": keys}
        print(f"- {fx}: {says} -> {[(k['frame'], k['value']) for k in keys]}")
    print()

    print("Freeze Frame: the one hold key written.\n")
    for fx, (says, i, o, off, s, keys, n) in FREEZES.items():
        after = frozen(i, o, off, s, keys, n)
        expected["freezes"][fx] = {"says": says, "in_frame": i, "out_frame": o,
                                   "source_offset_frames": off, "time_stretch": s,
                                   "keys_before": keys, "playhead": n, "keys": after}
        print(f"- {fx}: {says} -> {[(k['frame'], k['value'], k['interp']) for k in after]}")
    print()

    for fx, (says, keys, d) in SHIFTS.items():
        after = [dict(k, frame=k["frame"] + d) for k in keys]
        expected["shifts"][fx] = {"says": says, "keys_before": keys, "by": d, "keys": after}
        print(f"- {fx}: {says}")
    print()

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-TREMAP-012's file with one change:\n")
    for fx, (says, proj) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(proj, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_time_remap.json").write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    checks(expected)


def checks(e):
    """The claims the cases make, checked on the numbers just worked."""
    t = {fx: [r["t"] for r in v["rows"]] for fx, v in e["times"].items()}
    assert t["FX-TREMAP-001"] == [0, 3, 11]
    assert t["FX-TREMAP-002"] == [0, 5.5, 11, 11]
    assert t["FX-TREMAP-003"] == [0, 4, 8, 12, 15]
    assert t["FX-TREMAP-004"] == [5, 5, 5]
    assert t["FX-TREMAP-005"] == [11, 6, 0]
    assert t["FX-TREMAP-006"] == [2, 2, 9, 9]
    assert t["FX-TREMAP-007"] == [0, 1, 2, 8, 8]
    assert t["FX-TREMAP-008"][0] == 0 and t["FX-TREMAP-008"][2] == 4 and t["FX-TREMAP-008"][4] == 8
    assert t["FX-TREMAP-008"][1] < 2 < t["FX-TREMAP-008"][3] - 4, "slow at the ends"
    assert t["FX-TREMAP-009"] == [0, 3, 6]
    assert t["FX-TREMAP-030"] == [0, SNAP_N, 49] and floor(SNAP_RAW) != SNAP_N, "the snap matters"

    c = {fx: v["frames"] for fx, v in e["cases"].items()}
    assert all(not v["diagnostics"] for v in e["cases"].values())
    red, blue, green = (fb.decoded(f"bar_000{i}") for i in (1, 2, 3))
    none = [fb.CLEAR] * fb.W

    def frames(*pics):
        return {str(i): p for i, p in enumerate(pics)}

    def dot(x):
        return [[1.0, 1.0, 1.0, 1.0] if x <= i < x + 2 else fb.CLEAR for i in range(fb.W)]

    assert c["FX-TREMAP-010"] == frames(red, red, blue, blue, green, green)
    assert c["FX-TREMAP-011"] == frames(*[blue] * 6)
    assert c["FX-TREMAP-012"] == frames(*map(dot, [0, 2, 4, 5, 6, 6, 6, 6]))
    assert c["FX-TREMAP-013"] == frames(*map(dot, [6, 6, 5, 4, 3, 2, 1, 0]))
    half = fb.mix
    assert c["FX-TREMAP-014"] == frames(red, red, red, half(red, blue, 0.5), blue, blue, blue,
                                        half(blue, green, 0.5), green, green)
    assert c["FX-TREMAP-015"] == frames(*[red] * 4, *[blue] * 4, green, green)
    assert c["FX-TREMAP-016"] == frames(none, none, *map(dot, [0, 1, 2, 3, 4, 5, 6, 6]), none, none)
    assert c["FX-TREMAP-017"] == frames(*map(dot, [0, 1, 2, 3, 4, 5, 6, 6]), none, none)
    assert c["FX-TREMAP-018"] == frames(none, none, none, *map(dot, [0, 1, 2, 3, 4, 5, 6, 6, 6]))
    eased_t = [value_at(remap(eased(0, 0), lin(7, 7)), n) for n in range(8)]
    assert all(abs(x - round(x)) > 1e-6 for x in eased_t[1:7]), "no ease time near whole"
    assert c["FX-TREMAP-019"] == frames(*(dot(min(floor(x), 6)) for x in eased_t))
    assert c["FX-TREMAP-019"]["1"] == dot(0), "slow at the start"

    for fx, v in e["on"].items():
        i, o, off, s = v["in_frame"], v["out_frame"], v["source_offset_frames"], v["time_stretch"]
        before = {"in_frame": i, "source_offset_frames": off, "time_stretch": s}
        after = dict(before, time_remap=remap(*v["keys"]))
        for n in range(i, o):
            assert source_time(after, n) == source_time(before, n), (fx, n)
    assert [[(k["frame"], k["value"]) for k in v["keys"]] for v in e["on"].values()] == [
        [(0, 0), (11, 11)], [(5, 3), (19, 17)], [(0, 0), (6, 6)], [(0, 0), (10, 10)], [(4, 2), (5, 3)]]
    assert [[(k["frame"], k["value"]) for k in v["keys"]] for v in e["freezes"].values()] == [
        [(5, 5)], [(3, 2.5)], [(3, 5.5)]]
    assert e["shifts"]["FX-TREMAP-048"]["keys"][1]["frame"] == 8


if __name__ == "__main__":
    main()
