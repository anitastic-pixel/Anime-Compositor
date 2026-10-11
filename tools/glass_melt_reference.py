"""Glass Melt, worked a second way.

D-509 adds Glass Melt (`core.glass_melt`), after CycoreFX's CC Glass Wipe (Transition,
EFFECTS.md row 271), under a name of our own: a transition in which the layer melts away like
glass in the order of a gradient's brightness, bending as it goes, to reveal another layer. The
CycoreFX HD 1.8.1 manual names the controls (Completion, Layer to Reveal, Gradient Layer, whose
luminance is the displacement and reveal map, Softness, which blurs it, removing small details
and the look of depth, and Displacement Amount, higher values distorting more) but publishes no
formula and no settings when added, so the rule below and those settings are ours, the "rule
ours" precedent. Its bump and bend are CC Glass's (D-317), its wipe Gradient Wipe's (D-194).

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is its
own W by H:

1. The gradient G: with `gradient_layer` "" the input itself; otherwise D-189's map of the named
   layer (`tools/effect_layer_reference.py`) fitted to W by H by `stretch` (as CC Glass reads its
   bump), lying on the drawing at its corner, transparent outside it. The reveal R: with
   `layer_to_reveal` "" nothing (transparent); otherwise D-189's map of the named layer fitted to
   W by H by `center` (the layer revealed as it is, centred, as CycoreFX's manual has CC
   Twister's back layer), lying the same way. If either names a layer that is not in the
   composition (D-189's warning), the output is O.
2. The bump: at each pixel of the input, h0 = P(q) q.a, q the pixel of G there and P Colorama's
   luminance phase, the sRGB encoding of the clamped luma of q's straight colour (CC Glass's
   bump with the property `luminance`). h is h0 blurred by document 21's Gaussian at sigma
   `softness / 2` with the input's outer pixels repeated (Repeat Edge Pixels: the input keeps its
   size); none at softness 0.
3. The slope: n = ((h(x + 1, y) - h(x - 1, y)) / 2, (h(x, y + 1) - h(x, y - 1)) / 2) times
   1.25 max(softness, 1), the neighbours held inside the input. The bend D = `displacement_amount`
   times n, in pixels.
4. The wipe: with c = completion / 100, a band B = 0.1 and E = -B / 2 + c (1 + B),
   k = clamp((h - E) / B + 0.5, 0, 1): 1 where the layer stays, 0 where it has gone; the darkest
   parts go first. At completion 0 k is 1 everywhere, at 100 0 everywhere.
5. Output pixel (x, y), with document 21's bilinear (pixel centres at halves, transparent
   outside): k O(x + 1/2 + (1 - k) Dx, y + 1/2 + (1 - k) Dy) + (1 - k) R(x + 1/2 - k Dx,
   y + 1/2 - k Dy). The going layer bends more the further it has gone; the revealed layer
   arrives bent and settles flat.
6. The layer does not grow. For a draft `softness` and `displacement_amount` are divided by the
   draft's divisor (they are distances).

`completion` 0 to 100, 0 when added; `layer_to_reveal` D-189's word, "" when added (nothing);
`gradient_layer` D-189's word, "" when added (the layer itself); `softness` 0 to 100, 10 when
added; `displacement_amount` -500 to 500, 50 when added. The three numbers are keyable. As added,
completion 0, nothing changes.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values. No case multiplies a slope by more than a few hundred, so the build's single
precision bump stays inside the tolerance.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/glass_melt/`: the
map layers, every one switched off, and on top of them the drawing `holder` with the effect. The
expected pixels are in `Fixtures/glass_melt/expected_glass_melt.json`, with a project whose layer
settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glass_melt_reference.py
"""

import json
import sys
from math import ceil, exp
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import displacement_map_reference as D  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glass_melt"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]
BAND = 0.1

DRAWINGS = {
    "holder": D.DRAWINGS["holder"],
    "ramp": D.DRAWINGS["ramp"],
    "card": D.DRAWINGS["card"],
    "paint": D.DRAWINGS["paint"],
    # The paint's middle 8 by 6, (4, 2) to (11, 7): centred on 16 by 10 it lies where it was cut.
    "small": [row[4:12] for row in D.DRAWINGS["paint"][2:8]],
    # A soft round bump: white in the middle, black at the edges.
    "dome": [[(v, v, v, 255) for x in range(W)
              for v in [max(0, min(255, round(255 - 40 * (((x - 7.5) / 2) ** 2 +
                                                            ((y - 4.5) / 1.5) ** 2) ** 0.5)))]]
             for y in range(H)],
}


def decoded(name):
    return D.decoded(name) if name in D.DRAWINGS else _decoded(name)


def _decoded(name):
    from adjust_reference import srgb_to_linear
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


GRADIENTS = {"ramp": "ramp", "card": "card", "dome": "dome", "white": None}
REVEALS = {"paint": "paint", "small": "small", "late": "paint"}


# --- the rule -------------------------------------------------------------------------------

def phase(q):
    """Colorama's luminance phase of q, times its covering."""
    a = q[3]
    if a <= 0:
        return 0.0
    b = [min(1.0, max(0.0, q[c] / a)) for c in range(3)]
    return S.linear_to_srgb(0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2]) * a


def held_blur(h, w, ht, sigma):
    """Document 21's Gaussian at sigma on the plane h (w by ht), separable, the weights
    normalised after truncation at ceil(3 sigma), the outer pixels repeated."""
    r = ceil(3 * sigma)
    if r == 0:
        return h
    one = [exp(-(k * k) / (2 * sigma * sigma)) for k in range(-r, r + 1)]
    total = sum(one)
    one = [v / total for v in one]
    wide = [sum(one[i + r] * h[y * w + min(max(x + i, 0), w - 1)] for i in range(-r, r + 1))
            for y in range(ht) for x in range(w)]
    return [sum(one[j + r] * wide[min(max(y + j, 0), ht - 1) * w + x] for j in range(-r, r + 1))
            for y in range(ht) for x in range(w)]


def melt(o, g, rv, completion, softness, amount):
    """Steps 2 to 5 on the input o, with the gradient g (None: o itself) and the reveal rv (None:
    nothing), each W by H in the drawing's space."""
    w, ht = o["w"], o["h"]
    src = o if g is None else g
    h = held_blur([phase(L.at(src, i % w, i // w)) for i in range(w * ht)], w, ht, softness / 2)
    at = lambda x, y: h[min(max(y, 0), ht - 1) * w + min(max(x, 0), w - 1)]  # noqa: E731
    gain = 1.25 * max(softness, 1)
    c = completion / 100
    edge = -BAND / 2 + c * (1 + BAND)
    out = []
    for y in range(ht):
        for x in range(w):
            nx = (at(x + 1, y) - at(x - 1, y)) / 2 * gain
            ny = (at(x, y + 1) - at(x, y - 1)) / 2 * gain
            dx, dy = amount * nx, amount * ny
            k = min(1.0, max(0.0, (at(x, y) - edge) / BAND + 0.5))
            p = L.bilinear(o, x + 0.5 + (1 - k) * dx, y + 0.5 + (1 - k) * dy)
            q = L.bilinear(rv, x + 0.5 - k * dx, y + 0.5 - k * dy) if rv is not None else CLEAR
            out.append([k * a + (1 - k) * b for a, b in zip(p, q)])
    return L.pic(w, ht, out)


# --- the cases ------------------------------------------------------------------------------

def case(completion=0, reveal="", gradient="", softness=10, amount=50, shift=(0, 0),
         on="holder"):
    return {"completion": completion, "layer_to_reveal": reveal, "gradient_layer": gradient,
            "softness": softness, "displacement_amount": amount, "shift": shift, "on": on}


def gradient_picture(name):
    if name in ("", "holder"):
        return None
    if name == "white":
        return L.pic(W, H, [[1.0, 1.0, 1.0, 1.0] for _ in range(W * H)])
    return L.fit(decoded(GRADIENTS[name]), "stretch", W, H)


def reveal_picture(name, frame):
    if name == "":
        return None
    if name == "holder":
        return decoded("holder")
    p = L.EMPTY if name == "late" and frame < 4 else decoded(REVEALS[name])
    return L.fit(p, "center", W, H)


def worked(c, frame):
    o = decoded("holder")
    known = set(GRADIENTS) | set(REVEALS) | {"", "holder"}
    if c["gradient_layer"] not in known or c["layer_to_reveal"] not in known:
        return o
    return melt(o, gradient_picture(c["gradient_layer"]), reveal_picture(c["layer_to_reveal"], frame),
                value_at(c["completion"], frame), value_at(c["softness"], frame),
                value_at(c["displacement_amount"], frame))


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = worked(c, frame)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-GMELT-001": ("As added: completion 0, so nothing changes.", case(), (0,)),
    "FX-GMELT-002": ("Completion 100 with nothing to reveal: every pixel clear.",
                     case(100), (0,)),
    "FX-GMELT-003": ("Completion 100 revealing the paint: the paint exactly.",
                     case(100, "paint"), (0,)),
    "FX-GMELT-004": ("The grey ramp as the gradient, completion 50, softness 0, no bend: the "
                     "dark half gone to the paint, the light half kept, the middle column part "
                     "of each.", case(50, "paint", "ramp", 0, 0), (0,)),
    "FX-GMELT-005": ("FX-GMELT-004 with displacement 20: the ramp's even slope bends the melting "
                     "column sideways.", case(50, "paint", "ramp", 0, 20), (0,)),
    "FX-GMELT-006": ("The ramp, completion 50, softness 4, displacement 10: the blur rounds the "
                     "ramp's ends, so the edge columns bend less.",
                     case(50, "paint", "ramp", 4, 10), (0,)),
    "FX-GMELT-007": ("The holder as its own gradient (as added), completion 40, softness 2, "
                     "displacement 8: its dark red and blue squares melt first, bending round "
                     "their edges.", case(40, "paint", "", 2, 8), (0,)),
    "FX-GMELT-008": ("The dome as the gradient, completion 50, softness 2, displacement 6: the "
                     "dark rim goes first, the bright middle stays, bent outward.",
                     case(50, "paint", "dome", 2, 6), (0,)),
    "FX-GMELT-009": ("FX-GMELT-008 with displacement -6: bent the other way.",
                     case(50, "paint", "dome", 2, -6), (0,)),
    "FX-GMELT-010": ("The dome, completion 50, nothing revealed: the gone part clear.",
                     case(50, "", "dome", 2, 6), (0,)),
    "FX-GMELT-011": ("A white gradient, softness 100, displacement 500: flat, so nothing bends, "
                     "and at completion 50 nothing is dark enough to go.",
                     case(50, "paint", "white", 100, 500), (0,)),
    "FX-GMELT-012": ("The checker stretched to 16 by 10, completion 50, softness 3, "
                     "displacement 5: soft squares melt.", case(50, "paint", "card", 3, 5), (0,)),
    "FX-GMELT-013": ("Completion keyed from 0 at frame 0 to 100 at frame 4, the dome: frame 0 "
                     "the holder, frame 2 FX-GMELT-008, frame 4 the paint.",
                     case(keyed((0, 0), (4, 100)), "paint", "dome", 2, 6), (0, 2, 4)),
    "FX-GMELT-014": ("Displacement keyed from 0 at frame 0 to 12 at frame 4, the dome at 50: "
                     "frame 2 is FX-GMELT-008.",
                     case(50, "paint", "dome", 2, keyed((0, 0), (4, 12))), (0, 2, 4)),
    "FX-GMELT-015": ("Softness keyed from 0 at frame 0 to 4 at frame 4, the ramp: frame 0 "
                     "FX-GMELT-005's softness, frame 4 a softer ramp.",
                     case(50, "paint", "ramp", keyed((0, 0), (4, 4)), 10), (0, 4)),
    "FX-GMELT-016": ("A reveal layer that is not in the composition, `gone`: nothing changes, and "
                     "the warning every frame.", case(50, "gone", "dome", 2, 6), (0, 4)),
    "FX-GMELT-017": ("A gradient layer that is not in the composition, `gone`: nothing changes, "
                     "and the warning every frame.", case(50, "paint", "gone", 2, 6), (0, 4)),
    "FX-GMELT-018": ("The paint with its in point at frame 4 as the reveal: before it nothing is "
                     "revealed, so the gone part is clear; at frame 4 FX-GMELT-008.",
                     case(50, "late", "dome", 2, 6), (0, 4)),
    "FX-GMELT-019": ("The paint's middle 8 by 6 as the reveal, centred: it lies where it was "
                     "cut, clear round it.", case(50, "small", "dome", 2, 6), (0,)),
    "FX-GMELT-020": ("The holder revealing itself, the dome, completion 50: the layer bends into "
                     "itself.", case(50, "holder", "dome", 2, 6), (0,)),
    "FX-GMELT-021": ("FX-GMELT-008 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the maps lie on the layer.",
                     case(50, "paint", "dome", 2, 6, shift=(2, 1)), (0,)),
    "FX-GMELT-022": ("The effect on an adjustment layer above the holder: the maps lie on the "
                     "frame, which here is the holder's own rectangle, so this is FX-GMELT-008.",
                     case(50, "paint", "dome", 2, 6, on="adjust"), (0,)),
    "FX-GMELT-023": ("The ramp, completion 25, softness 0, displacement 0: only the darkest "
                     "columns gone.", case(25, "paint", "ramp", 0, 0), (0,)),
    "FX-GMELT-024": ("The ramp, completion 75, softness 0, displacement 0: only the lightest "
                     "columns kept.", case(75, "paint", "ramp", 0, 0), (0,)),
}

INVALID = {
    "FX-GMELT-025": ("Completion 101, above 100.", case(101, "paint", "dome")),
    "FX-GMELT-026": ("Completion -1, below 0.", case(-1, "paint", "dome")),
    "FX-GMELT-027": ("Softness 101, above 100.", case(50, "paint", "dome", 101)),
    "FX-GMELT-028": ("Softness -1, below 0.", case(50, "paint", "dome", -1)),
    "FX-GMELT-029": ("Displacement 501, above 500.", case(50, "paint", "dome", 2, 501)),
    "FX-GMELT-030": ("Displacement -501, below -500.", case(50, "paint", "dome", 2, -501)),
    "FX-GMELT-031": ("A reveal layer written as the number 3, not a word.",
                     case(50, 3, "dome")),
    "FX-GMELT-032": ("A gradient layer written as the number 3, not a word.",
                     case(50, "paint", 3)),
    "FX-GMELT-033": ("Completion keyed to 200 at frame 4.",
                     case(keyed((0, 30), (4, 200)), "paint", "dome")),
}

KEYS = ("completion", "layer_to_reveal", "gradient_layer", "softness", "displacement_amount")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.glass_melt", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.raster("card", "asset-card", enabled=False, out_frame=FRAMES),
            L.raster("dome", "asset-dome", enabled=False, out_frame=FRAMES),
            L.solid("white", W, H, [1.0] * 3, enabled=False, out_frame=FRAMES),
            L.raster("paint", "asset-paint", enabled=False, out_frame=FRAMES),
            L.raster("small", "asset-small", enabled=False, out_frame=FRAMES),
            L.raster("late", "asset-paint", enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    """Layers are drawn first to last, so the holder is written after the map layers and an
    adjustment layer after the holder."""
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
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a Glass Melt whose "
                       "gradient layer is the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case(50, "", "b"))),
                        drawing("b", effect("fx-b-0", case(50, "", "a")))]),
    "cycle_reveal.json": ("`a` reveals `b` and `b` reveals `a`: refused the same.",
                          [drawing("a", effect("fx-a-0", case(50, "b"))),
                           drawing("b", effect("fx-b-0", case(50, "a")))]),
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
        if "gone" in (c["layer_to_reveal"], c["gradient_layer"]):
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

    (OUT / "expected_glass_melt.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    paint = [list(L.at(decoded("paint"), x, y)) for y in range(H) for x in range(W)]
    gone = [CLEAR] * (W * H)
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    assert c["FX-GMELT-001"]["0"] == drawn
    assert near(c["FX-GMELT-002"]["0"], gone)
    assert near(c["FX-GMELT-003"]["0"], paint)
    for fx in ("FX-GMELT-016", "FX-GMELT-017"):
        assert c[fx]["0"] == drawn and c[fx]["4"] == drawn, fx
    # The ramp at 50, no blur, no bend: column x's luma is 17 x / 255; k = (v - 0.5) / 0.1 + 0.5,
    # so columns 0 to 6 are the paint, 9 to 15 the holder, 7 and 8 a mix.
    r = c["FX-GMELT-004"]["0"]
    for x in range(W):
        v = 17 * x / 255
        k = min(1.0, max(0.0, (v - 0.5) / BAND + 0.5))
        want = [[k * a + (1 - k) * b for a, b in zip(p, q)]
                for p, q in zip(col(drawn, x), col(paint, x))]
        assert near(col(r, x), want, 1e-12), x
    assert 0 < min(1.0, max(0.0, (7 * 17 / 255 - 0.5) / BAND + 0.5)) < 1
    # The bend moves only where k is strictly between 0 and 1.
    for x in list(range(0, 6)) + list(range(10, 16)):
        assert col(c["FX-GMELT-005"]["0"], x) == col(r, x), x
    assert col(c["FX-GMELT-005"]["0"], 7) != col(r, 7)
    # Darker first: at 25 fewer columns go than at 50, at 75 more.
    count = lambda px: sum(1 for x in range(W) if col(px, x) == col(paint, x))  # noqa: E731
    assert count(c["FX-GMELT-023"]["0"]) < count(r) < count(c["FX-GMELT-024"]["0"])
    # Flat white: nothing bends, nothing goes.
    assert near(c["FX-GMELT-011"]["0"], drawn)
    # The dome bent one way and the other differ, and both differ from no bend.
    assert c["FX-GMELT-008"]["0"] != c["FX-GMELT-009"]["0"]
    # Nothing revealed: where the dome case shows the paint, this one is clear.
    for i in range(W * H):
        if c["FX-GMELT-008"]["0"][i] == paint[i]:
            assert c["FX-GMELT-010"]["0"][i] == CLEAR, i
    # Keyed frames agree with the fixed cases.
    assert c["FX-GMELT-013"]["0"] == drawn
    assert near(c["FX-GMELT-013"]["2"], c["FX-GMELT-008"]["0"])
    assert near(c["FX-GMELT-013"]["4"], paint)
    assert near(c["FX-GMELT-014"]["2"], c["FX-GMELT-008"]["0"])
    assert near(c["FX-GMELT-015"]["0"], render(case(50, "paint", "ramp", 0, 10), 0))
    # Late reveal: clear before its in point, FX-GMELT-008 at it.
    assert near(c["FX-GMELT-018"]["0"], c["FX-GMELT-010"]["0"])
    assert c["FX-GMELT-018"]["4"] == c["FX-GMELT-008"]["0"]
    # The small reveal: clear at the corner where the dome's rim is gone.
    assert c["FX-GMELT-019"]["0"][0] == CLEAR
    # Moved and adjusted.
    moved = c["FX-GMELT-021"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-GMELT-008"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-GMELT-022"]["0"] == c["FX-GMELT-008"]["0"]
    print("checks passed")


if __name__ == "__main__":
    main()
