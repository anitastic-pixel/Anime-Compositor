"""Time Difference, worked a second way.

D-521 adds Time Difference (`core.time_difference`), after After Effects' Time Difference (Time,
EFFECTS.md row 296): the colour difference between the layer and a target layer, the target
read at a time offset from now, so what moved shows and what stood still goes black (or grey).
Adobe's help names the settings (Target, Time Offset, Contrast, Absolute Difference, Alpha
Channel and its nine choices) and what each is for, but publishes no formula and no defaults we
could read, so the rule below and the settings when added are ours, the "rule ours" precedent.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is W by
H:

1. The shift is k = floor(time_offset x rate + 0.000001) whole frames, the composition's rate
   (After Effects' Time Offset is in seconds).
2. The target T is a picture W by H. With no layer named (""), it is the holder's own picture
   at composition frame n + k by document 21's steps 1 and 2 alone: its drawing and masks at
   that frame by the layer's own timing, its effects not run, the ones before Time Difference
   included. With a layer named, it is D-189's map of that layer at frame n + k, centred on the
   drawing (`tools/effect_layer_reference.py`). A frame outside the target's in and out points
   gives an empty, clear target. On an adjustment layer with no layer named there is no picture
   of its own, so the output is O.
3. A layer named that is not in the composition (D-189's warning), or one that reads the holder
   back (D-189's refusal), leaves the output O.
4. At pixel (x, y) of O, S = O(x, y) and M = T(x - ox, y - oy), transparent outside. Each is
   taken to straight encoded colour: c = sRGB(clamp(rgb / a, 0, 1)) when a > 0, else 0, and the
   alpha min(a, 1); d is S - M, all four numbers.
5. The gain is g = 1 + contrast / 10. With `absolute_difference` `on`, e = clamp(g |d|, 0, 1),
   so no difference is black; `off`, e = clamp(0.5 + g d / 2, 0, 1), so no difference is mid
   grey. All four numbers.
6. The covering A by `alpha_channel`: `original` S.a; `target` M.a; `blend` their mean; `max`
   the larger; `full_on` 1; `lightness_of_result` (max + min) / 2 of e's colour;
   `max_of_result` the largest of e's colour; `alpha_difference` e.a; `alpha_difference_only`
   e.a, with e's colour set to white.
7. The output pixel is linear(e.rgb) A, alpha A, over the whole of O. The bounds do not grow.
8. For a draft the target is made at the holder's divisor; no setting is a distance.

`layer` D-189's word, "" when added; `time_offset` -60 to 60 seconds, 0 when added; `contrast`
0 to 100, 0 when added; both keyable; `absolute_difference` `on` when added, or `off`;
`alpha_channel` `original` when added, or the other eight words above.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, eight frames at 24 a second, in
`Fixtures/time_difference/`: switched-off layers to name (`plate`, a still of the background;
`late`, the plate from frame 4; `twin`, the holder's own drawings), and on top the layer
`holder`, a ball running right over a striped background in five drawings, with the effect. The
expected pixels are in `Fixtures/time_difference/expected_time_difference.json`, with a project
whose layer settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/time_difference_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "time_difference"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 16, 10, 8, 24
CLEAR = [0.0, 0.0, 0.0, 0.0]
ALPHAS = ("original", "target", "blend", "max", "full_on", "lightness_of_result",
          "max_of_result", "alpha_difference", "alpha_difference_only")


def background(x, y):
    """The plate: a colour that changes across and down; row 4 clear, a stripe the ball runs
    over; column 0 half covering; the bottom right 4 by 3 clear."""
    if y == 4 or (x >= 12 and y >= 7):
        return (0, 0, 0, 0)
    return (60 + 10 * x, 80 + 12 * y, 150, 128 if x == 0 else 255)


def ball(k):
    """Drawing k, 1 to 5: the background with a ball 3 pixels square in rows 3 to 5, its left
    column at 2k - 1, and a half-covering trail on the stripe in the column behind it."""
    x0 = 2 * k - 1
    colour = (240, 200, 40)
    rows = []
    for y in range(H):
        row = []
        for x in range(W):
            if 3 <= y <= 5 and x0 <= x < x0 + 3:
                row.append(colour + (255,))
            elif y == 4 and x == x0 - 1:
                row.append(colour + (128,))
            else:
                row.append(background(x, y))
        rows.append(row)
    return rows


DRAWINGS = {f"run_{k}": ball(k) for k in range(1, 6)}
DRAWINGS["plate"] = [[background(x, y) for x in range(W)] for y in range(H)]
ONES = [{"start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1},
        {"start_frame": 1, "end_frame_exclusive": 2, "drawing_number": 2},
        {"start_frame": 2, "end_frame_exclusive": 3, "drawing_number": 3},
        {"start_frame": 3, "end_frame_exclusive": 4, "drawing_number": 4},
        {"start_frame": 4, "end_frame_exclusive": 8, "drawing_number": 5}]


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(W, H, [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                         srgb_to_linear(b / 255) * a / 255, a / 255]
                        for row in rows for r, g, b, a in row])


def drawn(m):
    """The holder's (and the twin's) drawing at composition frame m, or empty outside 0 to 7."""
    if not 0 <= m < FRAMES:
        return L.EMPTY
    return decoded("run_%d" % next(s["drawing_number"] for s in ONES
                                    if s["start_frame"] <= m < s["end_frame_exclusive"]))


# --- the rule -------------------------------------------------------------------------------

def shift(c, n):
    return floor(value_at(c["time_offset"], n) * FPS + 1e-6)


def target(c, n):
    """Steps 1 to 3: the target at frame n, or None when the output is O."""
    m = n + shift(c, n)
    name = c["layer"]
    if name == "":
        return None if c["on"] == "adjust" else drawn(m)
    if name in ("holder", "twin"):
        return drawn(m)
    if name == "plate":
        return decoded("plate") if 0 <= m < FRAMES else L.EMPTY
    if name == "late":
        return decoded("plate") if 4 <= m < FRAMES else L.EMPTY
    return None  # not in the composition


def straight(p):
    a = p[3]
    if a <= 0:
        return [0.0, 0.0, 0.0, 0.0]
    return [S.linear_to_srgb(min(1.0, max(0.0, p[c] / a))) for c in range(3)] + [min(1.0, a)]


def differ(o, t, contrast, absolute, alpha):
    """Steps 4 to 7 on the picture o, t the target lying on it."""
    g = 1 + contrast / 10
    out = []
    for y in range(o["h"]):
        for x in range(o["w"]):
            s, m = straight(L.at(o, x, y)), straight(L.at(t, x, y))
            d = [s[i] - m[i] for i in range(4)]
            if absolute == "on":
                e = [min(1.0, max(0.0, g * abs(v))) for v in d]
            else:
                e = [min(1.0, max(0.0, 0.5 + g * v / 2)) for v in d]
            rgb = e[:3]
            a = {"original": s[3], "target": m[3], "blend": (s[3] + m[3]) / 2,
                 "max": max(s[3], m[3]), "full_on": 1.0,
                 "lightness_of_result": (max(rgb) + min(rgb)) / 2,
                 "max_of_result": max(rgb), "alpha_difference": e[3],
                 "alpha_difference_only": e[3]}[alpha]
            if alpha == "alpha_difference_only":
                rgb = [1.0, 1.0, 1.0]
            out.append([srgb_to_linear(v) * a for v in rgb] + [a])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="", time_offset=0, contrast=0, absolute_difference="on", alpha_channel="original",
         shift=(0, 0), before=None, after=None, on="holder"):
    return {"layer": layer, "time_offset": time_offset, "contrast": contrast,
            "absolute_difference": absolute_difference, "alpha_channel": alpha_channel,
            "shift": shift, "before": before, "after": after, "on": on}


def render(c, n):
    """The composition's pixels at frame n: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder through it."""
    o = drawn(n)
    if c["before"] is not None:
        o = L.exposure(o, c["before"])
    t = target(c, n)
    if t is not None:
        o = differ(o, t, value_at(c["contrast"], n), c["absolute_difference"], c["alpha_channel"])
    if c["after"] is not None:
        o = L.exposure(o, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, n):
    dx, dy = c["shift"]
    o = drawn(n)
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


BACK = -1 / 24
CASES = {
    "FX-TDIFF-001": ("As added: the layer against itself now, so nothing differs and every pixel "
                     "that shows is black, the clear ones clear.", case(), (2,)),
    "FX-TDIFF-002": ("Against itself one frame back: the ball's old and new places show, the "
                     "rest is black. At frame 0 the frame before is outside the layer, a clear "
                     "target, so the layer shows as it is.", case(time_offset=BACK), (0, 2)),
    "FX-TDIFF-003": ("Against itself one frame ahead. At frame 4 the next frame holds the same "
                     "drawing, so all black; at frame 7 the next frame is past the out point, so "
                     "the layer as it is.", case(time_offset=1 / 24), (2, 4, 7)),
    "FX-TDIFF-004": ("Absolute Difference off, one frame back: no difference is mid grey, "
                     "lighter where the layer is lighter now, darker where it was lighter.",
                     case(time_offset=BACK, absolute_difference="off"), (2,)),
    "FX-TDIFF-005": ("Contrast 20, one frame back: every difference three times as strong, "
                     "held at white.", case(time_offset=BACK, contrast=20), (2,)),
    "FX-TDIFF-006": ("Alpha Channel Target, one frame back: the covering is the target's, so "
                     "the ball's old place on the stripe shows and its new place does not.",
                     case(time_offset=BACK, alpha_channel="target"), (2,)),
    "FX-TDIFF-007": ("Alpha Channel Blend: the mean of the two coverings.",
                     case(time_offset=BACK, alpha_channel="blend"), (2,)),
    "FX-TDIFF-008": ("Alpha Channel Max: the larger of the two coverings.",
                     case(time_offset=BACK, alpha_channel="max"), (2,)),
    "FX-TDIFF-009": ("Alpha Channel Full On: every pixel covered, the clear corner black.",
                     case(time_offset=BACK, alpha_channel="full_on"), (2,)),
    "FX-TDIFF-010": ("Alpha Channel Lightness Of Result: covered as far as the difference is "
                     "light, so the still background goes clear.",
                     case(time_offset=BACK, alpha_channel="lightness_of_result"), (2,)),
    "FX-TDIFF-011": ("Alpha Channel Max Of Result: covered by the strongest channel of the "
                     "difference.", case(time_offset=BACK, alpha_channel="max_of_result"), (2,)),
    "FX-TDIFF-012": ("Alpha Channel Alpha Difference: covered by how much the covering changed, "
                     "only on the stripe where the ball came and went.",
                     case(time_offset=BACK, alpha_channel="alpha_difference"), (2,)),
    "FX-TDIFF-013": ("Alpha Channel Alpha Difference Only: as FX-TDIFF-012, with every colour "
                     "white.", case(time_offset=BACK, alpha_channel="alpha_difference_only"), (2,)),
    "FX-TDIFF-014": ("The plate as the target, now: the background matches and goes black, only "
                     "the ball and its trail show.", case(layer="plate"), (2, 4)),
    "FX-TDIFF-015": ("The plate from frame 4 as the target: at frame 2 it is clear, so the layer "
                     "shows as it is; at frame 4 it is FX-TDIFF-014.", case(layer="late"), (2, 4)),
    "FX-TDIFF-016": ("A layer that is not in the composition, `gone`: nothing changes, and the "
                     "warning every frame.", case(layer="gone"), (0, 4)),
    "FX-TDIFF-017": ("A second layer of the same drawings as the target, one frame back: "
                     "FX-TDIFF-002.", case(layer="twin", time_offset=BACK), (2,)),
    "FX-TDIFF-018": ("Time Offset keyed from 0 at frame 0 to -4 frames at frame 4: at frame 0 "
                     "black; at frame 2 two frames back; at frame 4 four frames back.",
                     case(time_offset=keyed((0, 0), (4, -4 / 24))), (0, 2, 4)),
    "FX-TDIFF-019": ("FX-TDIFF-002 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the target lies on the layer.",
                     case(time_offset=BACK, shift=(2, 1)), (2,)),
    "FX-TDIFF-020": ("An Exposure of +1 before Time Difference, against itself now: the target "
                     "is the drawing without its effects, so the brightening shows as a "
                     "difference.", case(before=1), (2,)),
    "FX-TDIFF-021": ("FX-TDIFF-004 with an Exposure of -1 after Time Difference: the grey "
                     "picture darkened.", case(time_offset=BACK, absolute_difference="off",
                                                after=-1), (2,)),
    "FX-TDIFF-022": ("On an adjustment layer above the holder with no target named: an "
                     "adjustment layer has no picture of its own, so nothing changes.",
                     case(time_offset=BACK, on="adjust"), (2,)),
    "FX-TDIFF-023": ("On an adjustment layer with the plate as the target: the frame beneath, "
                     "here the holder's own rectangle, against the plate, FX-TDIFF-014.",
                     case(layer="plate", on="adjust"), (2,)),
}

INVALID = {
    "FX-TDIFF-024": ("Contrast 101, above 100.", case(time_offset=BACK, contrast=101)),
    "FX-TDIFF-025": ("Contrast -1, below 0.", case(time_offset=BACK, contrast=-1)),
    "FX-TDIFF-026": ("Time Offset 61 seconds, above 60.", case(time_offset=61)),
    "FX-TDIFF-027": ("Time Offset -61 seconds, below -60.", case(time_offset=-61)),
    "FX-TDIFF-028": ("An Alpha Channel written \"lightness\".",
                     case(time_offset=BACK, alpha_channel="lightness")),
    "FX-TDIFF-029": ("An Absolute Difference written \"maybe\".",
                     case(time_offset=BACK, absolute_difference="maybe")),
    "FX-TDIFF-030": ("A target written as the number 3, not a word.", case(layer=3)),
    "FX-TDIFF-031": ("Time Offset keyed to 100 seconds at frame 4.",
                     case(time_offset=keyed((0, 0), (4, 100)))),
}

KEYS = ("layer", "time_offset", "contrast", "absolute_difference", "alpha_channel")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.time_difference", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def named_layers():
    return [L.raster("plate", "asset-plate", enabled=False, out_frame=FRAMES),
            L.raster("late", "asset-plate", enabled=False, in_frame=4, out_frame=FRAMES),
            L.raster("twin", "asset-run", spans=ONES, enabled=False, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-run", "kind": "image_sequence", "name": "run",
                        "pattern": "run_####.png",
                        "frames": {str(k): f"media/run_{k}.png" for k in range(1, 6)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}},
                       L.still("plate", "media/plate.png")],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    """Layers are drawn first to last, so the holder is written after the layers it names and
    an adjustment layer after the holder."""
    holder = L.raster("holder", "asset-run", spans=ONES, position=c["shift"], out_frame=FRAMES)
    layers = named_layers() + [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    return project("proj-" + fx.lower(), layers)


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def drawing(id, fx):
    lay = L.raster(id, "asset-run", spans=ONES, out_frame=FRAMES)
    lay["effects"] = [fx]
    return lay


# Layer settings that go round in a circle: a whole project, refused on opening.
LOADS = {
    "cycle_two.json": ("`a` and `b`, each the ball, each with a Time Difference whose target is "
                       "the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case(layer="b"))),
                        drawing("b", effect("fx-b-0", case(layer="a")))]),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "loads": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if c["layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": plain(c, 0), "4": plain(c, 4)}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    for name, (says, layers) in LOADS.items():
        write(name, project("proj-" + name[:-5].replace("_", "-"), named_layers() + layers))
        expected["loads"][name] = {"says": says, "refused": "EFFECT_LAYER_CYCLE"}

    (OUT / "expected_time_difference.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b, eps=1e-12: all(abs(p - q) <= eps for u, v in zip(a, b)  # noqa: E731
                                       for p, q in zip(u, v))
    pic = lambda n: plain(case(), n)  # noqa: E731
    black = lambda n: [[0.0, 0.0, 0.0, p[3]] for p in pic(n)]  # noqa: E731
    assert any(0 < p[3] < 1 for p in pic(2)) and any(p[3] == 0 for p in pic(2))
    assert c["FX-TDIFF-001"]["2"] == black(2)
    # Outside the layer the target is clear, so the layer shows as it is (to rounding).
    assert near(c["FX-TDIFF-002"]["0"], pic(0)) and near(c["FX-TDIFF-003"]["7"], pic(7))
    assert c["FX-TDIFF-003"]["4"] == black(4)
    # One frame back: black wherever the colours of drawings 2 and 3 agree, else not.
    for i, (now, was) in enumerate(zip(pic(2), pic(1))):
        p = c["FX-TDIFF-002"]["2"][i]
        assert (p[:3] == [0.0] * 3) == (straight(now)[:3] == straight(was)[:3] or now[3] == 0), i
    # Absolute off: mid grey where nothing changed.
    grey = srgb_to_linear(0.5)
    assert all(abs(p[0] - grey * p[3]) < 1e-12 for p, a, b in
               zip(c["FX-TDIFF-004"]["2"], pic(2), pic(1)) if a == b and a[3] > 0)
    assert near(c["FX-TDIFF-021"]["2"], [[v / 2 for v in p[:3]] + [p[3]]
                                         for p in c["FX-TDIFF-004"]["2"]])
    # Contrast: stronger, never weaker.
    assert all(p[3] == q[3] and all(u >= v - 1e-12 for u, v in zip(p[:3], q[:3]))
               for p, q in zip(c["FX-TDIFF-005"]["2"], c["FX-TDIFF-002"]["2"]))
    assert c["FX-TDIFF-005"]["2"] != c["FX-TDIFF-002"]["2"]
    # The coverings.
    alpha = {fx: [p[3] for p in c[fx]["2"]] for fx in c if "2" in c[fx]}
    now, was = [p[3] for p in pic(2)], [p[3] for p in pic(1)]
    assert alpha["FX-TDIFF-002"] == now and alpha["FX-TDIFF-006"] == was and now != was
    assert near([alpha["FX-TDIFF-007"]], [[(a + b) / 2 for a, b in zip(now, was)]])
    assert alpha["FX-TDIFF-008"] == [max(a, b) for a, b in zip(now, was)]
    assert alpha["FX-TDIFF-009"] == [1.0] * (W * H)
    assert all(alpha["FX-TDIFF-010"][i] == 0 for i in range(W * H) if pic(2)[i] == pic(1)[i])
    assert all(alpha["FX-TDIFF-012"][i] == 0 for i in range(W * H) if now[i] == was[i])
    assert any(alpha["FX-TDIFF-012"][i] > 0 for i in range(W * H))
    assert all(p[0] == p[1] == p[2] == p[3] for p in c["FX-TDIFF-013"]["2"])
    assert alpha["FX-TDIFF-013"] == alpha["FX-TDIFF-012"]
    # The plate: black over the background, the ball showing.
    plate = [list(p) for p in decoded("plate")["px"]]
    for i, p in enumerate(c["FX-TDIFF-014"]["2"]):
        assert (p[:3] == [0.0] * 3) == (straight(plate[i])[:3] == straight(pic(2)[i])[:3] or now[i] == 0), i
    assert near(c["FX-TDIFF-015"]["2"], pic(2)) and c["FX-TDIFF-015"]["4"] == c["FX-TDIFF-014"]["4"]
    assert c["FX-TDIFF-016"]["0"] == pic(0) and c["FX-TDIFF-016"]["4"] == pic(4)
    assert c["FX-TDIFF-017"]["2"] == c["FX-TDIFF-002"]["2"]
    assert c["FX-TDIFF-018"]["0"] == black(0)
    assert c["FX-TDIFF-018"]["4"] == render(case(time_offset=-4 / 24), 4)
    assert c["FX-TDIFF-018"]["2"] == render(case(time_offset=-2 / 24), 2)
    moved = c["FX-TDIFF-019"]["2"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-TDIFF-002"]["2"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-TDIFF-020"]["2"] != black(2)
    assert c["FX-TDIFF-022"]["2"] == pic(2)
    assert c["FX-TDIFF-023"]["2"] == c["FX-TDIFF-014"]["2"]
    # Every pixel a picture can hold: numbers from 0 to 1, colour no more than covering.
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
