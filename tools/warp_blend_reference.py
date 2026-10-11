"""Warp Blend, worked a second way.

D-518 adds Warp Blend (`core.warp_blend`), after CycoreFX's CC WarpoMatic (Transition,
EFFECTS.md row 280), under a name of our own: a transition in which the layer and another layer
are pulled out of shape along what differs between them while one blends into the other. The
CycoreFX HD 1.8.1 manual names the controls (Completion; Layer to Reveal, which may be the layer
itself, to warp it alone; Reactor, the information in the two layers that controls the warp:
Brightness, Contrast Differences, Brightness Differences or Local Differences; Smoothness, which
blurs the Reactor, removing small details; Warp Amount, negative reversing the warp; Warp
Direction: Joint, both layers pulled the same way, Opposing, opposite ways, Twisting, one layer
twisting out of place as the other twists into place; Blend Span, the share of the transition
during which the two layers blend) but publishes no formula and no settings when added, so the
rule below and those settings are ours, the "rule ours" precedent. Its blur and bend are Glass
Melt's (D-509).

At the holder's composition frame n, with its input A, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is its
own W by H:

1. The reveal B: with `layer_to_reveal` "" nothing (transparent); otherwise D-189's map of the
   named layer (`tools/effect_layer_reference.py`) fitted to W by H by `center`, lying on the
   drawing at its corner, transparent outside it. If it names a layer that is not in the
   composition (D-189's warning), the output is A.
2. The reactor: at each pixel, with q a pixel's luminance phase (Colorama's, the sRGB encoding of
   the clamped luma of its straight colour) times its covering, as Glass Melt's bump, and q_c its
   red, green and blue phases (each channel's straight value clamped and encoded) times its
   covering, from A's pixel a and B's pixel b there:
   - `brightness`: (q(a) + q(b)) / 2;
   - `brightness_differences`: |q(a) - q(b)|;
   - `contrast_differences`: ||q(a) - 1/2| - |q(b) - 1/2||;
   - `local_differences`: the largest over c of |q_c(a) - q_c(b)|.
   h is the reactor blurred by document 21's Gaussian at sigma `smoothness / 2` with the outer
   pixels repeated; none at smoothness 0.
3. The pull: D = `warp_amount` times ((h(x + 1, y) - h(x - 1, y)) / 2, (h(x, y + 1) -
   h(x, y - 1)) / 2) times 1.25 max(smoothness, 1), the neighbours held inside, in pixels
   (Glass Melt's bend). With `warp_direction` "twisting" D is turned a quarter, (-Dy, Dx).
4. With c = completion / 100, at the pixel centre P = (x + 1/2, y + 1/2), document 21's bilinear
   reads (pixel centres at halves, transparent outside): A' = A(P + c D); B' = B(P - (1 - c) D)
   for "joint" and "twisting", B(P + (1 - c) D) for "opposing". A is pulled further the further
   the transition has gone; B arrives pulled and settles flat.
5. The blend: with s = `blend_span` / 100, m = clamp((c - (1/2 - s/2)) / s, 0, 1), and with s 0,
   m is 1 where c >= 1/2 and 0 below. The output is (1 - m) A' + m B'. At completion 0 the
   output is A, at 100 it is B.
6. The layer does not grow. For a draft `smoothness` and `warp_amount` are divided by the
   draft's divisor (they are distances).

`completion` 0 to 100, 0 when added; `layer_to_reveal` D-189's word, "" when added (nothing);
`reactor` "brightness" (when added), "contrast_differences", "brightness_differences" or
"local_differences", exact; `smoothness` 0 to 100, 10 when added; `warp_amount` -500 to 500, 50
when added; `warp_direction` "joint" (when added), "opposing" or "twisting", exact; `blend_span`
0 to 100, 50 when added. Completion, smoothness, warp amount and blend span are keyable. As
added, completion 0, nothing changes.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values. No case multiplies a slope by more than a few hundred, so the build's single
precision reactor stays inside the tolerance.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/warp_blend/`: the
map layers, every one switched off, and on top of them the drawing `holder` with the effect. The
expected pixels are in `Fixtures/warp_blend/expected_warp_blend.json`, with a project whose layer
settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/warp_blend_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import glass_melt_reference as G  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "warp_blend"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]

DRAWINGS = {n: G.DRAWINGS[n] for n in ("holder", "paint", "small")}
REVEALS = {"paint": "paint", "small": "small", "late": "paint"}
REACTORS = ("brightness", "contrast_differences", "brightness_differences", "local_differences")
DIRECTIONS = ("joint", "opposing", "twisting")


# --- the rule -------------------------------------------------------------------------------

def channels(q):
    """q's red, green and blue phases, each times its covering."""
    a = q[3]
    if a <= 0:
        return [0.0, 0.0, 0.0]
    return [S.linear_to_srgb(min(1.0, max(0.0, q[c] / a))) * a for c in range(3)]


def reactor(kind, a, b):
    if kind == "brightness":
        return (G.phase(a) + G.phase(b)) / 2
    if kind == "brightness_differences":
        return abs(G.phase(a) - G.phase(b))
    if kind == "contrast_differences":
        return abs(abs(G.phase(a) - 0.5) - abs(G.phase(b) - 0.5))
    return max(abs(x - y) for x, y in zip(channels(a), channels(b)))


def blend_share(c, span):
    s = span / 100
    if s == 0:
        return 1.0 if c >= 0.5 else 0.0
    return min(1.0, max(0.0, (c - (0.5 - s / 2)) / s))


def warp(o, rv, completion, kind, smoothness, amount, direction, span):
    """Steps 2 to 5 on the input o with the reveal rv (None: nothing), each W by H in the
    drawing's space."""
    w, ht = o["w"], o["h"]
    none = L.pic(w, ht, [CLEAR] * (w * ht))
    b = rv if rv is not None else none
    h = G.held_blur([reactor(kind, L.at(o, i % w, i // w), L.at(b, i % w, i // w))
                     for i in range(w * ht)], w, ht, smoothness / 2)
    at = lambda x, y: h[min(max(y, 0), ht - 1) * w + min(max(x, 0), w - 1)]  # noqa: E731
    gain = 1.25 * max(smoothness, 1)
    c = completion / 100
    m = blend_share(c, span)
    out = []
    for y in range(ht):
        for x in range(w):
            dx = amount * (at(x + 1, y) - at(x - 1, y)) / 2 * gain
            dy = amount * (at(x, y + 1) - at(x, y - 1)) / 2 * gain
            if direction == "twisting":
                dx, dy = -dy, dx
            sx, sy = x + 0.5, y + 0.5
            p = L.bilinear(o, sx + c * dx, sy + c * dy)
            t = 1 if direction == "opposing" else -1
            q = L.bilinear(b, sx + t * (1 - c) * dx, sy + t * (1 - c) * dy)
            out.append([(1 - m) * u + m * v for u, v in zip(p, q)])
    return L.pic(w, ht, out)


# --- the cases ------------------------------------------------------------------------------

def case(completion=0, reveal="", reactor="brightness", smoothness=10, amount=50,
         direction="joint", span=50, shift=(0, 0), on="holder"):
    return {"completion": completion, "layer_to_reveal": reveal, "reactor": reactor,
            "smoothness": smoothness, "warp_amount": amount, "warp_direction": direction,
            "blend_span": span, "shift": shift, "on": on}


def reveal_picture(name, frame):
    if name == "":
        return None
    if name == "holder":
        return G.decoded("holder")
    p = L.EMPTY if name == "late" and frame < 4 else G.decoded(REVEALS[name])
    return L.fit(p, "center", W, H)


def worked(c, frame):
    o = G.decoded("holder")
    if c["layer_to_reveal"] not in set(REVEALS) | {"", "holder"}:
        return o
    return warp(o, reveal_picture(c["layer_to_reveal"], frame), value_at(c["completion"], frame),
                c["reactor"], value_at(c["smoothness"], frame), value_at(c["warp_amount"], frame),
                c["warp_direction"], value_at(c["blend_span"], frame))


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = worked(c, frame)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = G.decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


BASE = dict(reveal="paint", smoothness=2, amount=8)

CASES = {
    "FX-WARPB-001": ("As added: completion 0, so nothing changes.", case(), (0,)),
    "FX-WARPB-002": ("Completion 100 with nothing to reveal: every pixel clear.",
                     case(100), (0,)),
    "FX-WARPB-003": ("Completion 100 revealing the paint: the paint exactly.",
                     case(100, "paint"), (0,)),
    "FX-WARPB-004": ("Completion 25, brightness, smoothness 2, amount 8, joint, span 50: the "
                     "blend has not begun, so this is the holder alone, pulled a quarter of the "
                     "way.", case(25, **BASE), (0,)),
    "FX-WARPB-005": ("Completion 50, otherwise FX-WARPB-004: half each, both pulled half the "
                     "way.", case(50, **BASE), (0,)),
    "FX-WARPB-006": ("Completion 75, otherwise FX-WARPB-004: the blend done, so this is the "
                     "paint alone, a quarter of the pull left.", case(75, **BASE), (0,)),
    "FX-WARPB-007": ("FX-WARPB-005 opposing: the paint pulled the other way.",
                     case(50, direction="opposing", **BASE), (0,)),
    "FX-WARPB-008": ("FX-WARPB-005 twisting: both pulled a quarter turn round.",
                     case(50, direction="twisting", **BASE), (0,)),
    "FX-WARPB-009": ("FX-WARPB-005 with amount -8: pulled the other way.",
                     case(50, "paint", "brightness", 2, -8), (0,)),
    "FX-WARPB-010": ("FX-WARPB-005 with brightness differences.",
                     case(50, "paint", "brightness_differences", 2, 8), (0,)),
    "FX-WARPB-011": ("FX-WARPB-005 with contrast differences.",
                     case(50, "paint", "contrast_differences", 2, 8), (0,)),
    "FX-WARPB-012": ("FX-WARPB-005 with local differences.",
                     case(50, "paint", "local_differences", 2, 8), (0,)),
    "FX-WARPB-013": ("Smoothness 0, amount 4, completion 50: the reactor's own steps pull.",
                     case(50, "paint", "brightness", 0, 4), (0,)),
    "FX-WARPB-014": ("Smoothness 6, amount 20, completion 50: a broad, soft pull.",
                     case(50, "paint", "brightness", 6, 20), (0,)),
    "FX-WARPB-015": ("Blend span 0, completion 50: the paint alone, half the pull left.",
                     case(50, span=0, **BASE), (0,)),
    "FX-WARPB-016": ("Blend span 100, completion 30: three tenths the paint.",
                     case(30, span=100, **BASE), (0,)),
    "FX-WARPB-017": ("Completion 50 with nothing to reveal: the pulled holder at half its "
                     "covering.", case(50, "", "brightness", 2, 8), (0,)),
    "FX-WARPB-018": ("A reveal layer that is not in the composition, `gone`: nothing changes, and "
                     "the warning every frame.", case(50, "gone", "brightness", 2, 8), (0, 4)),
    "FX-WARPB-019": ("The paint with its in point at frame 4 as the reveal: before it, "
                     "FX-WARPB-017; at frame 4 FX-WARPB-005.",
                     case(50, "late", "brightness", 2, 8), (0, 4)),
    "FX-WARPB-020": ("The paint's middle 8 by 6 as the reveal, centred: it lies where it was "
                     "cut, clear round it.", case(50, "small", "brightness", 2, 8), (0,)),
    "FX-WARPB-021": ("The holder revealing itself, completion 50: the layer warped on its own, "
                     "pulled half one way and half the other.",
                     case(50, "holder", "brightness", 2, 8), (0,)),
    "FX-WARPB-022": ("FX-WARPB-005 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the reveal lies on the layer.",
                     case(50, shift=(2, 1), **BASE), (0,)),
    "FX-WARPB-023": ("The effect on an adjustment layer above the holder: the reveal lies on the "
                     "frame, which here is the holder's own rectangle, so this is FX-WARPB-005.",
                     case(50, on="adjust", **BASE), (0,)),
    "FX-WARPB-024": ("Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, "
                     "frame 2 FX-WARPB-005, frame 4 the paint.",
                     case(keyed((0, 0), (4, 100)), **BASE), (0, 2, 4)),
    "FX-WARPB-025": ("Amount keyed from 0 at frame 0 to 16 at frame 4, completion 50: frame 2 "
                     "is FX-WARPB-005.",
                     case(50, "paint", "brightness", 2, keyed((0, 0), (4, 16))), (0, 2, 4)),
    "FX-WARPB-026": ("Smoothness keyed from 0 at frame 0 to 4 at frame 4, completion 50, amount "
                     "8: frame 0 smoothness 0, frame 4 softer.",
                     case(50, "paint", "brightness", keyed((0, 0), (4, 4)), 8), (0, 4)),
    "FX-WARPB-027": ("Blend span keyed from 0 at frame 0 to 100 at frame 4, completion 30: "
                     "frame 0 the pulled holder alone, frame 4 FX-WARPB-016.",
                     case(30, span=keyed((0, 0), (4, 100)), **BASE), (0, 4)),
}

INVALID = {
    "FX-WARPB-028": ("Completion 101, above 100.", case(101, "paint")),
    "FX-WARPB-029": ("Completion -1, below 0.", case(-1, "paint")),
    "FX-WARPB-030": ("Smoothness 101, above 100.", case(50, "paint", smoothness=101)),
    "FX-WARPB-031": ("Smoothness -1, below 0.", case(50, "paint", smoothness=-1)),
    "FX-WARPB-032": ("Warp amount 501, above 500.", case(50, "paint", amount=501)),
    "FX-WARPB-033": ("Warp amount -501, below -500.", case(50, "paint", amount=-501)),
    "FX-WARPB-034": ("Blend span 101, above 100.", case(50, "paint", span=101)),
    "FX-WARPB-035": ("Blend span -1, below 0.", case(50, "paint", span=-1)),
    "FX-WARPB-036": ("Reactor \"hue\", not one of the four words.", case(50, "paint", "hue")),
    "FX-WARPB-037": ("Warp direction \"sideways\", not one of the three words.",
                     case(50, "paint", direction="sideways")),
    "FX-WARPB-038": ("A reveal layer written as the number 3, not a word.", case(50, 3)),
    "FX-WARPB-039": ("Completion keyed to 200 at frame 4.",
                     case(keyed((0, 30), (4, 200)), "paint")),
}

KEYS = ("completion", "layer_to_reveal", "reactor", "smoothness", "warp_amount",
        "warp_direction", "blend_span")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.warp_blend", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster("paint", "asset-paint", enabled=False, out_frame=FRAMES),
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
    "cycle_two.json": ("`a` and `b`, each the holder's drawing, each with a Warp Blend revealing "
                       "the other: refused, `EFFECT_LAYER_CYCLE`.",
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
        if c["layer_to_reveal"] == "gone":
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

    (OUT / "expected_warp_blend.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    paint = [list(L.at(G.decoded("paint"), x, y)) for y in range(H) for x in range(W)]
    gone = [CLEAR] * (W * H)
    assert c["FX-WARPB-001"]["0"] == drawn
    assert near(c["FX-WARPB-002"]["0"], gone)
    assert near(c["FX-WARPB-003"]["0"], paint)
    assert c["FX-WARPB-018"]["0"] == drawn and c["FX-WARPB-018"]["4"] == drawn
    # The blend: at 25 the paint plays no part, at 75 the holder none.
    assert blend_share(0.25, 50) == 0 and blend_share(0.75, 50) == 1 and blend_share(0.5, 50) == 0.5
    assert blend_share(0.5, 0) == 1 and blend_share(0.4999, 0) == 0
    assert abs(blend_share(0.3, 100) - 0.3) < 1e-15
    # Every case pulls something: the bent cases differ from the unbent blend.
    for fx in ("FX-WARPB-004", "FX-WARPB-005", "FX-WARPB-006", "FX-WARPB-013", "FX-WARPB-014"):
        assert c[fx]["0"] != drawn and c[fx]["0"] != paint, fx
    flat5 = [[(a + b) / 2 for a, b in zip(p, q)] for p, q in zip(drawn, paint)]
    assert not near(c["FX-WARPB-005"]["0"], flat5, 1e-4)
    # The three directions and the reversed amount all differ.
    five = c["FX-WARPB-005"]["0"]
    for fx in ("FX-WARPB-007", "FX-WARPB-008", "FX-WARPB-009", "FX-WARPB-010",
               "FX-WARPB-011", "FX-WARPB-012"):
        assert not near(c[fx]["0"], five, 1e-4), fx
    # Opposing and joint read the holder the same way, so at 25 (the holder alone) they agree.
    assert near(render(case(25, direction="opposing", **BASE), 0), c["FX-WARPB-004"]["0"])
    # Nothing revealed: half the pulled holder.
    half = render(case(50, "", "brightness", 2, 8), 0)
    assert c["FX-WARPB-017"]["0"] == half
    # Keyed frames agree with the fixed cases.
    assert c["FX-WARPB-024"]["0"] == drawn
    assert near(c["FX-WARPB-024"]["2"], five)
    assert near(c["FX-WARPB-024"]["4"], paint)
    assert near(c["FX-WARPB-025"]["2"], five)
    assert near(c["FX-WARPB-025"]["0"], flat5)
    assert near(c["FX-WARPB-026"]["0"], render(case(50, "paint", "brightness", 0, 8), 0))
    assert near(c["FX-WARPB-027"]["4"], c["FX-WARPB-016"]["0"])
    assert near(c["FX-WARPB-027"]["0"], render(case(30, span=0, **BASE), 0))
    # Late reveal: nothing before its in point, FX-WARPB-005 at it.
    assert near(c["FX-WARPB-019"]["0"], c["FX-WARPB-017"]["0"])
    assert c["FX-WARPB-019"]["4"] == five
    # The holder revealing itself still moves (the brightness is its own, doubled and halved).
    assert c["FX-WARPB-021"]["0"] != drawn
    # Moved and adjusted.
    moved = c["FX-WARPB-022"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == five[y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert c["FX-WARPB-023"]["0"] == five
    print("checks passed")


if __name__ == "__main__":
    main()
