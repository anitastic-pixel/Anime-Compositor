"""Posterize Time, worked a second way.

D-196 proposes Posterize Time (`core.posterize_time`): a time effect that holds what its layer
shows for several frames at a time, as if drawn at a lower frame rate. This file pins the rule.

At the holder's composition frame n, in a composition of F frames a second (its numerator over
its denominator) starting at frame S, on a layer whose in point is frame I:

1. On an adjustment layer nothing is held: the effect changes nothing. Otherwise h = n, and for
   each switched-on Posterize Time in stack order, with r its `frame_rate` at frame h: if r < F,
   s = floor((h - S) r / F + 1e-9) and h = S + ceil(s F / r - 1e-9); then h = max(h, I).
2. The layer is shown at n exactly when it is without the effect, by its in and out points at n.
3. What it shows is its content at h: its drawing, solid, shapes or composition by the layer's
   own timing at h, its masks at h, and every effect of its stack, before and after Posterize
   Time, with every setting at h, an effect's other layer and an Echo's copies read at h too.
4. Its transform, opacity, motion blur and depth are at n: a moving layer keeps moving while its
   drawing holds.
5. Posterize Time's own step in the stack changes nothing.

`frame_rate`, 0.1 to 99 frames a second, 12 when added, keyable.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, eight frames, in `Fixtures/posterize_time/`:
the drawing `holder`, a ball running right across eight drawings on ones, with the effect. The
expected pixels are in `Fixtures/posterize_time/expected_posterize_time.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/posterize_time_reference.py
"""

import json
import sys
from math import ceil, floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "posterize_time"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 8
DW, DH = 14, 5  # every drawing of the run
CLEAR = [0.0, 0.0, 0.0, 0.0]


def ball(k):
    """Drawing k, 1 to 8: a ball three pixels square in rows 1 to 3, its left column at k, a
    colour of its own, and a half-covering smear in the column behind it."""
    colour = (230, 20 + 25 * k, 220 - 25 * k)
    rows = []
    for y in range(DH):
        row = []
        for x in range(DW):
            if 1 <= y <= 3 and k <= x < k + 3:
                row.append(colour + (255,))
            elif 1 <= y <= 3 and x == k - 1:
                row.append(colour + (128,))
            else:
                row.append((0, 0, 0, 0))
        rows.append(row)
    return rows


DRAWINGS = {f"ball_{k}": ball(k) for k in range(1, 9)}
ONES = [{"start_frame": i, "end_frame_exclusive": i + 1, "drawing_number": i + 1}
        for i in range(FRAMES)]


def decoded(number):
    L.DRAWINGS[f"ball_{number}"] = DRAWINGS[f"ball_{number}"]
    return L.decoded(f"ball_{number}")


# --- the rule -------------------------------------------------------------------------------

def held(c, n):
    """Step 1: the frame the holder's content is taken from at composition frame n."""
    if c["on"] == "adjust":
        return n
    fps = c["fps"][0] / c["fps"][1]
    h = n
    for rate in c["rates"]:
        r = value_at(rate, h)
        if r < fps:
            s = floor(h * r / fps + 1e-9)  # the composition starts at frame 0
            h = ceil(s * fps / r - 1e-9)
        h = max(h, c["in"])
    return h


def drawing(c, m):
    """The holder's drawing at composition frame m, or empty outside its in and out points."""
    if not c["in"] <= m < FRAMES:
        return L.EMPTY
    return decoded(m - c["in"] + 1)


def add(*ps):
    w, h = max(p["w"] for p in ps), max(p["h"] for p in ps)
    return L.pic(w, h, [[min(1.0, sum(v)) for v in zip(*(L.at(p, x, y) for p in ps))]
                        for y in range(h) for x in range(w)])


def content(c, n):
    """Steps 1 and 3: the holder's picture after its stack, taken at the held frame."""
    if not c["in"] <= n < FRAMES:
        return L.EMPTY  # step 2
    h = held(c, n)
    p = drawing(c, h)
    if c["before"] is not None:
        p = L.exposure(p, value_at(c["before"], h))
    if c["echo"]:
        p = add(drawing(c, h), drawing(c, h - 1))  # Echo reads the drawing, not what came before
    if c["after"] is not None:
        p = L.exposure(p, value_at(c["after"], h))
    return p


# --- the cases ------------------------------------------------------------------------------

def case(rates=(12,), fps=(24, 1), start=0, position=(0, 0), before=None, after=None,
         echo=False, on="holder", off=False, nested=False):
    return {"rates": list(rates), "fps": fps, "in": start, "position": position,
            "before": before, "after": after, "echo": echo, "on": on, "off": off,
            "nested": nested}


def place(c, p, n):
    """Step 4: the picture moved by the holder's position at n."""
    dx, dy = (round(v) for v in value_at(c["position"], n))
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


def render(c, n):
    if c["off"]:
        return plain(c, n)
    return place(c, content(c, n), n)


def plain(c, n):
    """The holder at n with no Posterize Time."""
    return place(c, drawing(c, n), n)


EXPOSURE = keyed((0, 0), (4, 2))

CASES = {
    "FX-PTIME-001": ("As added, 12 a second in a composition of 24: on twos. Frames 0 and 1 show "
                     "drawing 1, 2 and 3 drawing 3, 4 and 5 drawing 5, 6 and 7 drawing 7.",
                     case(), tuple(range(FRAMES))),
    "FX-PTIME-002": ("24 a second, the composition's own rate: nothing is held.", case(rates=(24,)),
                     (1, 3, 5)),
    "FX-PTIME-003": ("30 a second, above the composition's rate: nothing is held.",
                     case(rates=(30,)), (3, 5)),
    "FX-PTIME-004": ("8 a second: on threes. Frames 1 and 2 show frame 0's drawing, 3 to 5 frame "
                     "3's.", case(rates=(8,)), (1, 2, 3, 4, 5)),
    "FX-PTIME-005": ("10 a second: uneven holds, frames 0 to 2, then 3 and 4, then 5 on.",
                     case(rates=(10,)), (2, 3, 4, 5, 6)),
    "FX-PTIME-006": ("6 a second: on fours. Frame 3 shows frame 0's drawing, 4 and 7 frame 4's.",
                     case(rates=(6,)), (3, 4, 7)),
    "FX-PTIME-007": ("The layer's in point at frame 3, 12 a second: at frame 2 there is no layer; "
                     "at frame 3 its first drawing, not a frame before it; frames 4 and 5 its "
                     "second.", case(start=3), (2, 3, 4, 5)),
    "FX-PTIME-008": ("The position keyed from (0, 0) at frame 0 to (4, 0) at frame 4: the drawing "
                     "holds and the layer still moves. Frame 3 shows drawing 3 moved 3 right.",
                     case(position=keyed((0, [0, 0]), (4, [4, 0]))), (2, 3)),
    "FX-PTIME-009": ("An Exposure before it, keyed from 0 at frame 0 to +2 at frame 4: held with "
                     "the drawing. Frame 3 is drawing 3 at +1.", case(before=EXPOSURE), (3,)),
    "FX-PTIME-010": ("The same Exposure after it: held too, FX-PTIME-009.",
                     case(after=EXPOSURE), (3,)),
    "FX-PTIME-011": ("The effect on an adjustment layer above the holder: nothing changes.",
                     case(on="adjust"), (3,)),
    "FX-PTIME-012": ("Two, 12 then 8: frame 4 is held at 4 by the first and 3 by the second; "
                     "frames 4 and 5 show frame 3's drawing, frame 6 its own.",
                     case(rates=(12, 8)), (4, 5, 6)),
    "FX-PTIME-013": ("An Echo after it, one echo one frame back, Add: frame 3 is frame 2's "
                     "drawing and frame 1's added, the same as frame 2.",
                     case(echo=True), (2, 3)),
    "FX-PTIME-014": ("Switched off: nothing is held.", case(off=True), (3,)),
    "FX-PTIME-015": ("The rate keyed, 24 held until frame 4 and 8 from there: frames 0 to 3 on "
                     "ones, then frames 4 and 5 show frame 3's drawing and frame 6 its own.",
                     case(rates=(keyed((0, 24, "hold"), (4, 8)),)), (3, 4, 5, 6)),
    "FX-PTIME-016": ("On a composition layer showing a composition of the same drawings: frame 3 "
                     "shows the inner composition's frame 2.", case(nested=True), (3,)),
    "FX-PTIME-017": ("12 a second in a composition of 30: frames 0 to 2 show frame 0's drawing, "
                     "3 and 4 frame 3's, 5 to 7 frame 5's.", case(fps=(30, 1)), (2, 3, 4, 5)),
}

INVALID = {
    "FX-PTIME-018": ("Frame rate 0.05, below 0.1.", case(rates=(0.05,))),
    "FX-PTIME-019": ("Frame rate 100, above 99.", case(rates=(100,))),
    "FX-PTIME-020": ("Frame rate keyed to 120 at frame 4.", case(rates=(keyed((0, 12), (4, 120)),))),
}


def effect(fid, rate, enabled=True):
    return {"instance_id": fid, "type_id": "core.posterize_time", "enabled": enabled,
            "parameters": {"frame_rate": setting_json(rate)}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": setting_json(stops)}}


def echo_effect(fid):
    return {"instance_id": fid, "type_id": "core.echo", "enabled": True,
            "parameters": {"echo_time": -1, "echoes": 1, "intensity": 1, "decay": 1,
                           "operator": "add"}}


def project_json(fx, c):
    comps = []
    if c["nested"]:
        inner = L.raster("ball", "asset-ball", spans=ONES, out_frame=FRAMES)
        comps.append(L.composition("comp-inner", W, H, FRAMES, [inner]))
        holder = L.nested("holder", "comp-inner", out_frame=FRAMES)
    else:
        holder = L.raster("holder", "asset-ball", spans=ONES, in_frame=c["in"], out_frame=FRAMES)
    holder["transform"]["position"] = L.prop(list(c["position"])) \
        if not isinstance(c["position"], dict) else setting_json(c["position"])
    fxs = [effect(f"fx-{i + 1}", r, not c["off"]) for i, r in enumerate(c["rates"])]
    if c["before"] is not None:
        fxs.insert(0, exposure("fx-0", c["before"]))
    if c["echo"]:
        fxs.append(echo_effect("fx-8"))
    if c["after"] is not None:
        fxs.append(exposure("fx-9", c["after"]))
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = fxs
        layers.append(adjust)
    else:
        holder["effects"] = fxs
    main = L.composition("comp-main", W, H, FRAMES, layers)
    main["frame_rate"] = {"numerator": c["fps"][0], "denominator": c["fps"][1]}
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-ball", "kind": "image_sequence", "name": "ball",
                        "pattern": "ball_####.png",
                        "frames": {str(k): f"media/ball_{k}.png" for k in range(1, 9)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [main] + comps}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)), "frames": rendered}
        print(f"{fx}: " + ", ".join(f"frame {f} shows {held(c, f) if not c['off'] else f}"
                                    for f in frames))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"1": plain(c, 1), "3": plain(c, 3)}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_posterize_time.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = lambda f: plain(case(), f)  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b) for p, q in zip(u, v))  # noqa: E731
    shows = lambda fx, pairs, d=drawn: all(c[fx][str(f)] == d(h) for f, h in pairs)  # noqa: E731
    assert shows("FX-PTIME-001", [(0, 0), (1, 0), (2, 2), (3, 2), (4, 4), (5, 4), (6, 6), (7, 6)])
    assert shows("FX-PTIME-002", [(1, 1), (3, 3), (5, 5)])
    assert shows("FX-PTIME-003", [(3, 3), (5, 5)])
    assert shows("FX-PTIME-004", [(1, 0), (2, 0), (3, 3), (4, 3), (5, 3)])
    assert shows("FX-PTIME-005", [(2, 0), (3, 3), (4, 3), (5, 5), (6, 5)])
    assert shows("FX-PTIME-006", [(3, 0), (4, 4), (7, 4)])
    late = lambda f: plain(case(start=3), f)  # noqa: E731
    assert all(p == CLEAR for p in c["FX-PTIME-007"]["2"])
    assert shows("FX-PTIME-007", [(3, 3), (4, 4), (5, 4)], late)
    for f, dx in (("2", 2), ("3", 3)):
        assert all(c["FX-PTIME-008"][f][y * W + x + dx] == drawn(2)[y * W + x]
                   for y in range(H) for x in range(W - dx))
    assert c["FX-PTIME-008"]["3"] != c["FX-PTIME-001"]["3"]
    assert near(c["FX-PTIME-009"]["3"], [[p[0] * 2, p[1] * 2, p[2] * 2, p[3]] for p in drawn(2)])
    assert c["FX-PTIME-010"]["3"] == c["FX-PTIME-009"]["3"]
    assert shows("FX-PTIME-011", [(3, 3)])
    assert shows("FX-PTIME-012", [(4, 3), (5, 3), (6, 6)])
    add2 = [[min(1.0, a + b) for a, b in zip(p, q)] for p, q in zip(drawn(2), drawn(1))]
    assert c["FX-PTIME-013"]["2"] == c["FX-PTIME-013"]["3"] and near(c["FX-PTIME-013"]["3"], add2)
    assert shows("FX-PTIME-014", [(3, 3)])
    assert shows("FX-PTIME-015", [(3, 3), (4, 3), (5, 3), (6, 6)])
    assert shows("FX-PTIME-016", [(3, 2)])
    assert shows("FX-PTIME-017", [(2, 0), (3, 3), (4, 3), (5, 5)])
    for fx in INVALID:
        assert c[fx]["1"] == drawn(1) and c[fx]["3"] == drawn(3)
    # Holding twice is holding once, at every rate and frame a case can reach.
    for rate in (0.1, 6, 8, 10, 12, 12.5, 23.9, 24):
        for fps in ((24, 1), (30, 1), (25, 1), (24000, 1001)):
            for n in range(200):
                once = held(case(rates=(rate,), fps=fps), n)
                assert held(case(rates=(rate,), fps=fps), once) == once <= n, (rate, fps, n)
    print("checks passed")


if __name__ == "__main__":
    main()
