"""Channel Wipe, worked a second way.

D-512 adds Channel Wipe (`core.channel_wipe`), after CycoreFX's CC Image Wipe (Transition,
EFFECTS.md row 273), under a name of our own: a transition that wipes the layer away in the order
of a chosen channel of a gradient layer, the lowest values first. The CycoreFX HD 1.8.1 manual
names the controls (Completion; Border Softness; Auto Softness, which follows how fast Completion
is animated; and a Gradient group of Layer, Property (Red, Green, Blue, Alpha, Luminance,
Lightness, Hue, Saturation), Blur and Inverse Gradient) but publishes no formula and no settings
when added, so the rule below and those settings are ours, the "rule ours" precedent.

By D-383 ("Both names, one engine") and D-394 ("Second name, one engine"), as D-402 Tritone was
accepted, Channel Wipe is a second effect over Gradient Wipe's engine (D-194,
`tools/gradient_wipe_reference.py`), which takes the extra settings: Gradient Wipe is the engine
with the property `luminance`, no blur, and nothing wiped when no layer is named. Gradient Wipe
and every saved Gradient Wipe draw as before; its own fixtures are not touched by this file.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. With `completion` 0 the output is O; with `completion` 100 every pixel is (0, 0, 0, 0). The
   gradient G: with `layer` "" the input itself, lying on itself; otherwise D-189's map of the
   named layer (`tools/effect_layer_reference.py`), fitted to the drawing's own W by H by `fit`,
   lying on the drawing at its corner, transparent outside it. A layer named that is not in the
   composition (D-189's warning) leaves O as it is.
2. The reading: at each pixel of the input, q the pixel of G there, linear and premultiplied, and
   e = the sRGB encoding of clamp(q.rgb, 0, 1), each channel. `red`, `green` and `blue` are e's
   channels; `alpha` is q's covering; `luminance` is Gradient Wipe's picture luma, the sRGB
   encoding of clamp(0.2126 r + 0.7152 g + 0.0722 b, 0, 1) of q; `hue` (degrees / 360),
   `saturation` and `lightness` are Hue/Saturation's HSL of e (D-113). All are read on the
   premultiplied numbers, so a clear gradient reads 0 in every property, as Gradient Wipe's clear
   map reads black.
3. With `blur` above 0 the readings, one per pixel of the input, are blurred by document 21's
   Gaussian at sigma `blur / 2` with the input's outer pixels repeated (as Glass Melt blurs its
   bump). With `invert` `on`, v is 1 - v.
4. Gradient Wipe's step 3: c = completion / 100, s = softness / 100, E = -s / 2 + c (1 + s);
   k = clamp((v - E) / s + 0.5, 0, 1) when s > 0, otherwise 1 where v >= E and 0 elsewhere; the
   output is O k, all four numbers.
5. The layer does not grow. For a draft `blur` is divided by the draft's divisor (a distance);
   the map is made at the holder's size (D-189 step 3).

`layer` D-189's word, "" when added (the layer itself); `fit` `stretch` when added, `center` or
`tile`; `completion` 0 to 100, 0 when added; `softness` 0 to 100, 10 when added; `property` one
of the eight words, `luminance` when added; `blur` 0 to 100 pixels, 0 when added; `invert` `off`
or `on`, `off` when added. The three numbers are keyable. Auto Softness is not built (a gap:
document 21 hands an effect the values at its frame only).

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/channel_wipe/`:
the map layers, every one switched off, and on top of them the drawing `holder` with the effect.
The expected pixels are in `Fixtures/channel_wipe/expected_channel_wipe.json`, with a project
whose layer settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/channel_wipe_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from hue_saturation_reference import to_hsl  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402
import displacement_map_reference as D  # noqa: E402
import gradient_wipe_reference as G  # noqa: E402
import glass_melt_reference as M  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "channel_wipe"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]
CLIFF = 1e-5  # no pixel decided by a hard edge lies this near it
PROPERTIES = ("red", "green", "blue", "alpha", "luminance", "lightness", "hue", "saturation")

DRAWINGS = {n: D.DRAWINGS[n] for n in ("holder", "ramp", "card", "veil", "paint")}
MAPS = {  # what each map layer's picture is at frame 0; `late` starts at frame 4
    "ramp": D.decoded("ramp"), "card": D.decoded("card"), "veil": D.decoded("veil"),
    "paint": D.decoded("paint"), "late": K.solid_pic([1.0] * 3),
}


# --- the rule -------------------------------------------------------------------------------

def reading(word, q):
    """Step 2 on the premultiplied pixel q."""
    if word == "alpha":
        return q[3]
    if word == "luminance":
        return K.luma(q)
    e = [S.linear_to_srgb(min(1.0, max(0.0, q[c]))) for c in range(3)]
    h, s, l = to_hsl(*e)
    return {"red": e[0], "green": e[1], "blue": e[2], "hue": h / 360, "saturation": s,
            "lightness": l}[word]


def wipe(o, g, completion, softness, prop, blur, invert):
    """Steps 1 to 4 on the picture o, the gradient g already fitted to it (o itself for "")."""
    if completion == 0:
        return o
    if completion == 100:
        return L.pic(o["w"], o["h"], [list(CLEAR) for _ in range(o["w"] * o["h"])])
    w, ht = o["w"], o["h"]
    v = [reading(prop, g["px"][i]) for i in range(w * ht)]
    if blur > 0:
        v = M.held_blur(v, w, ht, blur / 2)
    c, s = completion / 100, softness / 100
    edge = -s / 2 + c * (1 + s)
    out = []
    for i, p in enumerate(o["px"]):
        x = 1 - v[i] if invert == "on" else v[i]
        if s > 0:
            k = min(1.0, max(0.0, (x - edge) / s + 0.5))
        else:
            assert abs(x - edge) > CLIFF, ("a pixel on the cliff", i, x, edge)
            k = 1.0 if x >= edge else 0.0
        out.append([q * k for q in p])
    return L.pic(w, ht, out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="", fit="stretch", completion=0, softness=10, prop="luminance", blur=0,
         invert="off", shift=(0, 0), on="holder"):
    return {"layer": layer, "fit": fit, "completion": completion, "softness": softness,
            "property": prop, "blur": blur, "invert": invert, "shift": shift, "on": on}


def gradient(c, frame):
    """The gradient fitted to the drawing, or None when the layer named is not there."""
    if c["layer"] == "":
        return D.decoded("holder")  # the input itself
    if c["layer"] not in MAPS:
        return None
    p = L.EMPTY if c["layer"] == "late" and frame < 4 else MAPS[c["layer"]]
    return L.fit(p, c["fit"], W, H)


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    o = D.decoded("holder")
    g = gradient(c, frame)
    out = o if g is None else wipe(o, g, value_at(c["completion"], frame),
                                   value_at(c["softness"], frame), c["property"],
                                   value_at(c["blur"], frame), c["invert"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = D.decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-CWIPE-001": ("As added: the layer itself as the gradient, completion 0, so nothing "
                     "changes.", case(), (0,)),
    "FX-CWIPE-002": ("The layer itself, completion 50, softness 0, luminance: its red and blue "
                     "squares, darker than half, go, its cream ones stay, and the clear corner "
                     "stays clear; Gradient Wipe's FX-GWIPE-016 exactly.",
                     case(completion=50, softness=0), (0,)),
    "FX-CWIPE-003": ("The grey ramp, completion 50, softness 0, luminance: the eight dark columns "
                     "gone; Gradient Wipe's FX-GWIPE-003 exactly.",
                     case("ramp", completion=50, softness=0), (0,)),
    "FX-CWIPE-004": ("The ramp, completion 50, softness 100: kept by the ramp's brightness; "
                     "FX-GWIPE-004 exactly.", case("ramp", completion=50, softness=100), (0,)),
    "FX-CWIPE-005": ("FX-CWIPE-003 with Invert on: the light columns go; FX-GWIPE-008 exactly.",
                     case("ramp", completion=50, softness=0, invert="on"), (0,)),
    "FX-CWIPE-006": ("A white map clear at the top and more covering down each row, completion "
                     "50, softness 0, luminance: a clear map reads 0, so rows 0 and 1 go; "
                     "FX-GWIPE-012 exactly.", case("veil", completion=50, softness=0), (0,)),
    "FX-CWIPE-007": ("The same map, alpha: rows 0 to 4, less than half covered, go, rows 5 to 9 "
                     "stay.", case("veil", completion=50, softness=0, prop="alpha"), (0,)),
    "FX-CWIPE-008": ("The painted map, red (40 + 8 y of 255 down each row), completion 30, "
                     "softness 0: rows 0 to 4 gone, 5 to 9 kept.",
                     case("paint", completion=30, softness=0, prop="red"), (0,)),
    "FX-CWIPE-009": ("The painted map, green (30 + 14 x across each row), completion 50, softness "
                     "0: columns 0 to 6 gone, 7 to 15 kept.",
                     case("paint", completion=50, softness=0, prop="green"), (0,)),
    "FX-CWIPE-010": ("The painted map, blue, completion 50, softness 20: falling to the right, so "
                     "the right side fades.",
                     case("paint", completion=50, softness=20, prop="blue"), (0,)),
    "FX-CWIPE-011": ("The painted map, lightness, completion 50, softness 30.",
                     case("paint", completion=50, softness=30, prop="lightness"), (0,)),
    "FX-CWIPE-012": ("The painted map, hue, completion 50, softness 10.",
                     case("paint", completion=50, softness=10, prop="hue"), (0,)),
    "FX-CWIPE-013": ("The painted map, saturation, completion 60, softness 20.",
                     case("paint", completion=60, softness=20, prop="saturation"), (0,)),
    "FX-CWIPE-014": ("The ramp blurred, blur 4, completion 50, softness 0: an even slope is not "
                     "changed by the blur inside, so the cut is FX-CWIPE-003's.",
                     case("ramp", completion=50, softness=0, blur=4), (0,)),
    "FX-CWIPE-015": ("The ramp blurred, blur 6, completion 10, softness 0: the outer pixels "
                     "repeated, the dark end is lifted, so fewer columns go than without blur.",
                     case("ramp", completion=10, softness=0, blur=6), (0,)),
    "FX-CWIPE-016": ("The checker stretched to 16 by 10, blur 3, completion 50, softness 20: "
                     "soft squares.", case("card", completion=50, softness=20, blur=3), (0,)),
    "FX-CWIPE-017": ("The 4 by 2 checker centred, alpha, completion 50, softness 0: outside it "
                     "the map is clear, 0, so only its eight pixels are kept.",
                     case("card", "center", completion=50, softness=0, prop="alpha"), (0,)),
    "FX-CWIPE-018": ("Completion 100: every pixel clear.", case("ramp", completion=100), (0,)),
    "FX-CWIPE-019": ("Completion keyed from 0 at frame 0 to 100 at frame 4, the ramp, softness 0: "
                     "frame 0 whole, frame 2 FX-CWIPE-003, frame 4 clear.",
                     case("ramp", completion=keyed((0, 0), (4, 100)), softness=0), (0, 2, 4)),
    "FX-CWIPE-020": ("Blur keyed from 0 at frame 0 to 12 at frame 4, the ramp at 10, softness 0: "
                     "frame 2 FX-CWIPE-015.",
                     case("ramp", completion=10, softness=0, blur=keyed((0, 0), (4, 12))),
                     (0, 2, 4)),
    "FX-CWIPE-021": ("A layer that is not in the composition, `gone`, completion 50: nothing "
                     "changes, and the warning every frame.", case("gone", completion=50), (0, 4)),
    "FX-CWIPE-022": ("A white solid whose in point is frame 4, alpha, completion 50: before it the "
                     "map is empty, alpha 0, so at frame 0 everything is gone; at frame 4 nothing "
                     "is.", case("late", completion=50, prop="alpha"), (0, 4)),
    "FX-CWIPE-023": ("FX-CWIPE-009 on the holder moved 2 right and 1 down: the same picture moved, "
                     "since the map lies on the layer.",
                     case("paint", completion=50, softness=0, prop="green", shift=(2, 1)), (0,)),
    "FX-CWIPE-024": ("The effect on an adjustment layer above the holder, the painted map, green: "
                     "the map lies on the frame, the holder's own rectangle here, so this is "
                     "FX-CWIPE-009.",
                     case("paint", completion=50, softness=0, prop="green", on="adjust"), (0,)),
}

INVALID = {
    "FX-CWIPE-025": ("Completion 101, above 100.", case("ramp", completion=101)),
    "FX-CWIPE-026": ("Softness -1, below 0.", case("ramp", softness=-1)),
    "FX-CWIPE-027": ("Blur 101, above 100.", case("ramp", completion=50, blur=101)),
    "FX-CWIPE-028": ("Blur -1, below 0.", case("ramp", completion=50, blur=-1)),
    "FX-CWIPE-029": ("A property written \"value\".", case("ramp", completion=50, prop="value")),
    "FX-CWIPE-030": ("A fit written \"fill\".", case("ramp", "fill", completion=50)),
    "FX-CWIPE-031": ("Invert written \"yes\".", case("ramp", completion=50, invert="yes")),
    "FX-CWIPE-032": ("A layer written as the number 3, not a word.", case(3, completion=50)),
    "FX-CWIPE-033": ("Completion keyed to 200 at frame 4.",
                     case("ramp", completion=keyed((0, 30), (4, 200)))),
}

KEYS = ("layer", "fit", "completion", "softness", "property", "blur", "invert")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.channel_wipe", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.raster("card", "asset-card", enabled=False, out_frame=FRAMES),
            L.raster("veil", "asset-veil", enabled=False, out_frame=FRAMES),
            L.raster("paint", "asset-paint", enabled=False, out_frame=FRAMES),
            L.solid("late", W, H, [1.0] * 3, enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    holder = L.raster("holder", "asset-holder", position=c["shift"], out_frame=FRAMES)
    layers = map_layers() + [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = [effect("fx-1", c)]
    return project("proj-" + fx.lower(), layers)


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def drawing(id, fx):
    lay = L.raster(id, "asset-holder", out_frame=FRAMES)
    lay["effects"] = [fx]
    return lay


LOADS = {
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a Channel Wipe whose "
                       "layer is the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case("b", completion=50))),
                        drawing("b", effect("fx-b-0", case("a", completion=50)))]),
    "cycle_mixed.json": ("`a`'s Channel Wipe names `b`, and `b`'s Gradient Wipe names `a`: refused "
                         "the same.",
                         [drawing("a", effect("fx-a-0", case("b", completion=50))),
                          drawing("b", G.effect("fx-b-0", G.case("a", completion=50)))]),
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
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    for name, (says, layers) in LOADS.items():
        write(name, project("proj-" + name[:-5].replace("_", "-"), map_layers() + layers))
        expected["loads"][name] = {"says": says, "refused": "EFFECT_LAYER_CYCLE"}

    (OUT / "expected_channel_wipe.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    gone = [CLEAR] * (W * H)
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    row = lambda px, y: px[y * W:(y + 1) * W]  # noqa: E731
    assert c["FX-CWIPE-001"]["0"] == drawn
    assert c["FX-CWIPE-021"]["0"] == drawn and c["FX-CWIPE-021"]["4"] == drawn
    assert c["FX-CWIPE-018"]["0"] == gone
    # One engine: at luminance, blur 0, Gradient Wipe's own numbers exactly.
    for ours, theirs in (("002", G.case("holder", completion=50)),
                         ("003", G.case("ramp", completion=50)),
                         ("004", G.case("ramp", completion=50, softness=100)),
                         ("005", G.case("ramp", completion=50, invert="on")),
                         ("006", G.case("veil", completion=50))):
        assert c["FX-CWIPE-" + ours]["0"] == G.render(theirs, 0), ours
    # Alpha against luminance on the veil: rows 0 to 4 go by alpha, only 0 and 1 by luminance.
    assert c["FX-CWIPE-007"]["0"] == gone[:5 * W] + drawn[5 * W:]
    assert c["FX-CWIPE-006"]["0"] == gone[:2 * W] + drawn[2 * W:]
    # Red down the rows, green across the columns.
    assert c["FX-CWIPE-008"]["0"] == gone[:5 * W] + drawn[5 * W:]
    for x in range(W):
        assert col(c["FX-CWIPE-009"]["0"], x) == (col(drawn, x) if x >= 7 else col(gone, x)), x
    # Blue falls to the right: the left column more kept than the right.
    b = c["FX-CWIPE-010"]["0"]
    assert b[0][3] > b[W - 1][3]
    # The other properties each give a picture of their own.
    pics = [c["FX-CWIPE-0" + n]["0"] for n in ("08", "09", "10", "11", "12", "13")]
    assert all(p not in (drawn, gone) for p in pics)
    assert len({json.dumps(p) for p in pics}) == len(pics)
    # The blurred even ramp cuts where the plain one does; the lifted dark end keeps more.
    assert c["FX-CWIPE-014"]["0"] == c["FX-CWIPE-003"]["0"]
    plain10 = render(case("ramp", completion=10, softness=0), 0)
    count = lambda px: sum(1 for x in range(W) if col(px, x) == col(gone, x))  # noqa: E731
    assert 0 < count(c["FX-CWIPE-015"]["0"]) < count(plain10)
    assert c["FX-CWIPE-016"]["0"] not in (drawn, gone)
    keep = {(x, y) for x in range(6, 10) for y in range(4, 6)}
    assert c["FX-CWIPE-017"]["0"] == [drawn[y * W + x] if (x, y) in keep else CLEAR
                                      for y in range(H) for x in range(W)]
    # Keyed.
    assert c["FX-CWIPE-019"]["0"] == drawn and c["FX-CWIPE-019"]["2"] == c["FX-CWIPE-003"]["0"]
    assert c["FX-CWIPE-019"]["4"] == gone
    assert c["FX-CWIPE-020"]["0"] == plain10
    assert c["FX-CWIPE-020"]["2"] == c["FX-CWIPE-015"]["0"]
    assert c["FX-CWIPE-022"]["0"] == gone and c["FX-CWIPE-022"]["4"] == drawn
    # Moved and adjusted.
    moved = c["FX-CWIPE-023"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-CWIPE-009"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-CWIPE-024"]["0"] == c["FX-CWIPE-009"]["0"]
    # A wipe only takes away.
    for fx, v in c.items():
        for px in v.values():
            for p, q in zip(px, drawn):
                assert fx == "FX-CWIPE-023" or all(-1e-12 <= a <= b + 1e-12 for a, b in zip(p, q))
    assert row(drawn, 0)  # the holder has a first row
    print("checks passed")


if __name__ == "__main__":
    main()
