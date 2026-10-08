"""Moment Map, worked a second way.

D-347 adds `core.moment_map`, this program's Moment Map, after After Effects' Time
Displacement: each pixel of a layer is taken from a different moment of that same layer, earlier
or later by the brightness of a map under it. Adobe's page gives the rule: bright moves a pixel
later, dark earlier, mid grey not at all, as far as Max Displacement Time, in steps of 1 / Time
Resolution seconds; the map is another layer's picture, stretched to the layer or centred.

At the holder's composition frame n, at fps frames a second, with its input O, linear and
premultiplied, whose drawing's own top-left pixel is at (ox, oy), and whose drawing is W by H at n:

1. On an adjustment layer the output is O. Otherwise `max_time` (seconds) and `resolution`
   (steps a second) are their values at frame n, held inside their ranges.
2. The map M is D-189's layer setting, fitted to W by H by `fit`. With the name "" it is the
   holder's own picture at n by document 21's steps 1 and 2 (its drawing and masks, its effects
   not run), as D-189 makes a layer's own map; a layer not in the composition is the same, with
   the warning.
3. At each pixel, with m = M(x, y) and covering a, v is 0.5 where a is 0, and otherwise
   0.5 + a (k - 0.5), the map laid over mid grey, where k = linear_to_srgb(0.2126 r + 0.7152 g +
   0.0722 b) of the straight linear colour clamp(m.rgb / a, 0, 1): Displacement Map's
   `luminance` (D-193). The pixel's moment is d = (2 v - 1) max_time seconds from now, q =
   round(d resolution) steps (halves away from zero) and its frame n + s with
   s = floor(q fps / resolution): the frame holding that moment, as a drawing is held.
4. P_s is the holder's own picture at composition frame n + s by document 21's steps 1 and 2
   alone, as Echo's copies are (D-195): its effects not run, the ones before this effect
   included, its masks applied; outside the layer's in and out points it is empty. It lies on
   the drawing corner to corner, cut to W by H, transparent outside it.
5. The output pixel (x, y) is P_s(x, y), all four numbers, with its top-left pixel at (ox, oy),
   transparent elsewhere: what effects before this one made is replaced. Bounds do not grow.
6. For a draft each P_s and the map are made at the holder's divisor (D-189, step 3); no
   setting is a distance.

`max_time`, -10 to 10 seconds, 1 when added; `resolution`, 1 to 999 a second, 60 when added; both
keyable. `layer`, D-189's word, "" when added; `fit`, `stretch`, `center` or `tile`, `stretch`
when added. Adobe's 8-bit levels put no shift at 128 and white at 127/128 of the most; here mid
grey 0.5 is no shift and white the whole of it, as Displacement Map reads its map.

Sample times are whole composition frames: a layer drawn here has no picture between frames, so
a resolution above the frame rate gives no in-between pictures (FX-TDISP-006).

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values, pixel by pixel, where the build draws each frame it needs once.

Every case is a project of one composition 16 by 10 at 10 frames a second, twelve frames, in
`Fixtures/time_displacement/`: the drawing `holder`, twelve drawings one a frame, each pixel's red
the drawing's number and its green and blue its place, with the effect; under it the map layers,
every one switched off. The expected pixels are in
`Fixtures/time_displacement/expected_time_displacement.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/time_displacement_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from mask_reference import linear_to_srgb  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "time_displacement"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 16, 10, 12, 10
CLEAR = [0.0, 0.0, 0.0, 0.0]
NEAREST_TIE = [1.0]  # how near any d * resolution came to a half; checked below


def run(k):
    """Drawing k, 1 to 12: red 20 k, green by column, blue by row; the bottom right 4 by 3 clear."""
    return [[(0, 0, 0, 0) if x >= 12 and y >= 7 else (20 * k, 10 + 15 * x, 10 + 24 * y, 255)
             for x in range(W)] for y in range(H)]


DRAWINGS = {f"run_{k}": run(k) for k in range(1, FRAMES + 1)}
DRAWINGS["ramp"] = [[(17 * x,) * 3 + (255,) for x in range(W)] for _ in range(H)]
DRAWINGS["steps"] = [[(0, 0, 0, 255), (85, 85, 85, 255), (170, 170, 170, 255),
                      (255, 255, 255, 255)],
                     [(255, 255, 255, 255), (170, 170, 170, 255), (85, 85, 85, 255),
                      (0, 0, 0, 255)]]
DRAWINGS["veil"] = [[(255, 255, 255, 128)] * W for _ in range(H)]
L.DRAWINGS.update(DRAWINGS)
SPANS = [{"start_frame": i, "end_frame_exclusive": i + 1, "drawing_number": i + 1}
         for i in range(FRAMES)]
ORANGE = [1.0, 0.25, 0.0]


# --- the rule -------------------------------------------------------------------------------

def picture(c, m):
    """P at composition frame m: the holder's drawing through its mask, or empty."""
    if not c["in"] <= m < c["out"]:
        return L.EMPTY
    return L.masked(L.decoded(f"run_{m - c['in'] + 1}"), c["mask"], 1)


def map_picture(c, n):
    """Step 2's map at frame n, W by H."""
    name = c["layer"]
    if name in ("", "gone"):
        return L.fit(picture(c, n), "stretch", W, H)
    if name == "orange":
        p = L.pic(W, H, [ORANGE + [1.0] for _ in range(W * H)])
    elif name == "ramp_dark":
        p = L.exposure(L.decoded("ramp"), -1)
    else:
        p = L.decoded(name)
    return L.fit(p, c["fit"], W, H)


def value(m):
    a = m[3]
    if a <= 0:
        return 0.5
    r, g, b = (min(1.0, max(0.0, v / a)) for v in m[:3])
    return 0.5 + a * (linear_to_srgb(0.2126 * r + 0.7152 * g + 0.0722 * b) - 0.5)


def half_away(x):
    return floor(x + 0.5) if x >= 0 else -floor(-x + 0.5)


def shift(m, most, resolution):
    """Step 3: the frames from now this map pixel takes its picture."""
    d = (2 * value(m) - 1) * most
    f = d * resolution
    NEAREST_TIE[0] = min(NEAREST_TIE[0], abs(abs(f - floor(f)) - 0.5))
    q = half_away(f)
    return floor(q * FPS / resolution)


def displaced(c, n):
    """Steps 1 to 5 at frame n: the output, W by H, or EMPTY outside the holder's frames."""
    base = picture(c, n)
    if base is L.EMPTY:
        return base
    most, resolution = value_at(c["max_time"], n), value_at(c["resolution"], n)
    m = map_picture(c, n)
    out = []
    for y in range(H):
        for x in range(W):
            s = shift(L.at(m, x, y), most, resolution)
            out.append(list(L.at(picture(c, n + s), x, y)))
    return L.pic(W, H, out)


# --- the cases ------------------------------------------------------------------------------

def case(max_time=1, resolution=60, layer="", fit="stretch", start=0, end=FRAMES, move=(0, 0),
         mask=None, before=None, after=None, on="holder"):
    return {"max_time": max_time, "resolution": resolution, "layer": layer, "fit": fit,
            "in": start, "out": end, "move": move, "mask": mask, "before": before,
            "after": after, "on": on}


def placed(p, c):
    dx, dy = c["move"]
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by `move`; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        return plain(c, frame)
    out = displaced(c, frame)
    if c["after"] is not None:
        out = L.exposure(out, c["after"])
    return placed(out, c)


def plain(c, frame):
    return placed(picture(c, frame), c)


RAMP = dict(layer="ramp", max_time=0.5)
CASES = {
    "FX-TDISP-001": ("As added: Max Displacement 1 second, Time Resolution 60, the layer's own "
                     "brightness as the map, Stretch: each pixel from a moment chosen by its own "
                     "brightness; moments past the layer's twelve frames are clear.", case(),
                     (6,)),
    "FX-TDISP-002": ("The ramp as the map, black at the left to white at the right, Max 0.5: "
                     "column x is 4 x - 30 sixtieths of a second from now, so column 0 shows "
                     "frame 1 and column 15 frame 11 at frame 6; at frame 0 the left half asks "
                     "for frames before the layer, clear.", case(**RAMP), (0, 6)),
    "FX-TDISP-003": ("The same at Max -0.5: the other way, the left shows the frames after.",
                     case(layer="ramp", max_time=-0.5), (6,)),
    "FX-TDISP-004": ("The ramp, Max 0.5, Time Resolution 2: moments in half seconds, so only "
                     "frames 1, 6 and 11 are seen.", case(**RAMP, resolution=2), (6,)),
    "FX-TDISP-005": ("The ramp, Max 0.4, Time Resolution 3: a third of a second is 3.33 frames, "
                     "held to the frame holding it: frame 9 after and frame 2 before.",
                     case(layer="ramp", max_time=0.4, resolution=3), (6,)),
    "FX-TDISP-006": ("The ramp, Max 0.5, Time Resolution 100, past the frame rate: every pixel "
                     "is still one of the drawings, nothing in between; the same frame as "
                     "FX-TDISP-002.", case(**RAMP, resolution=100), (6,)),
    "FX-TDISP-007": ("The layer from frame 3 to frame 8 only: the moments outside it are clear.",
                     case(**RAMP, start=3, end=9), (6,)),
    "FX-TDISP-008": ("A small map, 4 by 2 steps of grey, stretched over the layer.",
                     case(layer="steps", fit="stretch", max_time=0.5), (6,)),
    "FX-TDISP-009": ("The same map centred: outside it there is no map, so no shift.",
                     case(layer="steps", fit="center", max_time=0.5), (6,)),
    "FX-TDISP-010": ("The same map tiled.", case(layer="steps", fit="tile", max_time=0.5),
                     (6,)),
    "FX-TDISP-011": ("An orange solid as the map: its brightness, 0.664, is 20 sixtieths of a "
                     "second, so the whole layer is 3 frames ahead: frame 6 shows frame 9.",
                     case(layer="orange"), (6,)),
    "FX-TDISP-012": ("A white map at half covering, laid over mid grey: half the way to white, "
                     "30 sixtieths, so frame 6 shows frame 11.", case(layer="veil"), (6,)),
    "FX-TDISP-013": ("The ramp with an Exposure of -1 on it: a map's effects count (D-189), so "
                     "it is darker and every column reaches further back.",
                     case(layer="ramp_dark", max_time=0.5), (6,)),
    "FX-TDISP-014": ("A mask keeping columns 0 to 7: every moment is masked; the left half is "
                     "FX-TDISP-002's.", case(**RAMP, mask=L.rect(0, 0, 8, 10)), (6,)),
    "FX-TDISP-015": ("An Exposure of +1 before it is not seen: the moments are the drawings and "
                     "the layer's own map is its drawing. FX-TDISP-001.", case(before=1), (6,)),
    "FX-TDISP-016": ("An Exposure of -1 after it darkens the result.",
                     case(**RAMP, after=-1), (6,)),
    "FX-TDISP-017": ("The holder moved 2 right and 1 down: FX-TDISP-002 moved, since the map "
                     "lies on the layer.", case(**RAMP, move=(2, 1)), (6,)),
    "FX-TDISP-018": ("The effect on an adjustment layer above the holder: nothing changes.",
                     case(on="adjust"), (6,)),
    "FX-TDISP-019": ("Max keyed from 0 at frame 0 to 1 at frame 12, the ramp: at frame 0 "
                     "nothing moves; at frame 6 it is 0.5, FX-TDISP-002.",
                     case(layer="ramp", max_time=keyed((0, 0), (12, 1))), (0, 6)),
    "FX-TDISP-020": ("Max 0: every pixel from now, the layer as it is.", case(max_time=0), (6,)),
    "FX-TDISP-021": ("A layer that is not in the composition, `gone`: the layer itself is the "
                     "map, FX-TDISP-001, and the warning every frame.", case(layer="gone"),
                     (6,)),
}

INVALID = {
    "FX-TDISP-022": ("Max 10.5 seconds, above 10.", case(max_time=10.5)),
    "FX-TDISP-023": ("Max -10.5 seconds, below -10.", case(max_time=-10.5)),
    "FX-TDISP-024": ("Time Resolution 0.5, below 1.", case(resolution=0.5)),
    "FX-TDISP-025": ("Time Resolution 1000, above 999.", case(resolution=1000)),
    "FX-TDISP-026": ("A fit written \"fill\".", case(layer="ramp", fit="fill")),
    "FX-TDISP-027": ("A layer written as the number 3, not a word.", case(layer=3)),
    "FX-TDISP-028": ("Max keyed to 12 at frame 4.", case(max_time=keyed((0, 1), (4, 12)))),
}

NAMES = ("max_time", "resolution", "layer", "fit")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.moment_map", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def composition(layers):
    return {"id": "comp-main", "name": "comp-main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": FPS, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": [l["id"] for l in layers], "layers": layers}


def map_layers():
    off = dict(enabled=False, out_frame=FRAMES)
    return [L.raster("ramp", "asset-ramp", **off),
            L.raster("ramp_dark", "asset-ramp", effects=[("exposure", -1)], **off),
            L.raster("steps", "asset-steps", **off),
            L.raster("veil", "asset-veil", **off),
            L.solid("orange", W, H, ORANGE, **off)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-run", spans=SPANS, position=c["move"], in_frame=c["in"],
                      out_frame=c["out"], mask=c["mask"])
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-run", "kind": "image_sequence", "name": "run",
                        "pattern": "run_####.png",
                        "frames": {str(k): f"media/run_{k}.png" for k in range(1, FRAMES + 1)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}},
                       L.still("ramp", "media/ramp.png"), L.still("steps", "media/steps.png"),
                       L.still("veil", "media/veil.png")],
            "compositions": [composition(layers + map_layers())]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(c, 0), "4": plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_time_displacement.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = lambda f, **k: plain(case(**k), f)  # noqa: E731
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    # No moment sits within a thousandth of a step of a half, where single precision (about 1e-5
    # of a step here) could tip it.
    assert NEAREST_TIE[0] > 0.001, NEAREST_TIE
    # 002: column x shows frame 6 + floor((4 x - 30) / 6).
    for x in range(W):
        f = 6 + floor((4 * x - 30) / 6)
        assert col(c["FX-TDISP-002"]["6"], x) == col(drawn(f), x), x
        f0 = floor((4 * x - 30) / 6)
        assert col(c["FX-TDISP-002"]["0"], x) == (col(drawn(f0), x) if f0 >= 0 else [CLEAR] * H)
    assert col(c["FX-TDISP-002"]["6"], 0) == col(drawn(1), 0)
    assert col(c["FX-TDISP-002"]["6"], 15) == col(drawn(11), 15)
    # 003 mirrors 002.
    for x in range(W):
        assert col(c["FX-TDISP-003"]["6"], x) == col(drawn(6 + floor(-(4 * x - 30) / 6)), x)
    seen = lambda fx, f: {k for x in range(W) for k in range(FRAMES)  # noqa: E731
                          if col(c[fx][f], x) == col(drawn(k), x)}
    assert seen("FX-TDISP-004", "6") == {1, 6, 11}, seen("FX-TDISP-004", "6")
    assert seen("FX-TDISP-005", "6") == {2, 6, 9}, seen("FX-TDISP-005", "6")
    assert c["FX-TDISP-006"]["6"] == c["FX-TDISP-002"]["6"]
    for x in range(W):
        f = 6 + floor((4 * x - 30) / 6)
        want = col(drawn(f, start=3, end=9), x) if 3 <= f < 9 else [CLEAR] * H
        assert col(c["FX-TDISP-007"]["6"], x) == want, x
    assert c["FX-TDISP-008"]["6"] != c["FX-TDISP-010"]["6"] != drawn(6)
    # 009: the 4 by 2 map lies at (6, 4); everywhere else is now.
    for y in range(H):
        for x in range(W):
            if not (6 <= x < 10 and 4 <= y < 6):
                assert c["FX-TDISP-009"]["6"][y * W + x] == drawn(6)[y * W + x], (x, y)
    assert c["FX-TDISP-009"]["6"] != drawn(6)
    assert c["FX-TDISP-011"]["6"] == drawn(9)
    assert c["FX-TDISP-012"]["6"] == drawn(11)
    assert c["FX-TDISP-013"]["6"] != c["FX-TDISP-002"]["6"]
    for x in range(W):
        assert col(c["FX-TDISP-014"]["6"], x) == (col(c["FX-TDISP-002"]["6"], x) if x < 8
                                                    else [CLEAR] * H)
    assert c["FX-TDISP-015"]["6"] == c["FX-TDISP-001"]["6"]
    assert all(abs(p[ch] - q[ch] / (2 if ch < 3 else 1)) < 1e-12
               for p, q in zip(c["FX-TDISP-016"]["6"], c["FX-TDISP-002"]["6"]) for ch in range(4))
    moved = c["FX-TDISP-017"]["6"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-TDISP-002"]["6"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-TDISP-018"]["6"] == drawn(6)
    assert c["FX-TDISP-019"]["0"] == drawn(0)
    assert c["FX-TDISP-019"]["6"] == c["FX-TDISP-002"]["6"]
    assert c["FX-TDISP-020"]["6"] == drawn(6)
    assert c["FX-TDISP-021"]["6"] == c["FX-TDISP-001"]["6"] != drawn(6)
    # Every output pixel is one drawing's pixel, or clear: nothing is mixed.
    for fx, v in c.items():
        if fx in ("FX-TDISP-016",):
            continue
        for f, px in v.items():
            for i, p in enumerate(px):
                assert p == CLEAR or any(p == drawn(k)[i] or p == plain(case(move=(2, 1)), k)[i]
                                         for k in range(FRAMES)), (fx, f, i)
    print(f"checks passed; nearest a half came {NEAREST_TIE[0]:.4f} of a step")


if __name__ == "__main__":
    main()
