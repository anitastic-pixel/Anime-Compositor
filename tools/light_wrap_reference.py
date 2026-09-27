"""Light wrap, worked a second way.

D-132 adds `core.light_wrap`. It lets the light of what is behind a layer bleed onto the layer's
edges, as a lit background does onto a cel laid over it. It is the first effect that reads the
layers beneath its own: the owner chose "everything beneath" on 2026-09-26, the frame drawn so
far, as an adjustment layer reads it (D-66). `width` is 0 to 500 (10), a distance: how far in
from the edge the light reaches. `intensity` is 0 to 400 (100), per cent. `blend` is "screen"
or "add" (screen). It is this program's own method, modelled in spirit on the light wrap
compositors use, Nuke's LightWrap node the best known (the plate behind, blurred, laid on the
edge of the layer in front); nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

Where it runs. Not in the layer's own space: the layer's other effects, its mask, transform and
matte run first, in their order, wherever Light Wrap stands in the stack; then, when the layer
is about to be drawn onto the frame, before its opacity and blend mode, each enabled Light Wrap
runs in stack order, in the frame's pixels. B is the frame drawn so far, everything beneath the
layer; L is the layer placed, a picture the frame's size, premultiplied.

The rule. s = width / 3. Bb is B through document 21's Gaussian blur at s and Ab is L's covering
through the same blur, each on the frame's own size, transparent outside it, anything grown
past it cut off. At a pixel with L.a > 0: w = intensity / 100 * (1 - Ab) * Bb.rgb, the light
that reaches the edge; l = L.rgb / L.a; l' = l + w (add) or 1 - (1 - l)(1 - clamp(w, 0, 1))
(screen); the pixel becomes (l' * L.a, L.a). A pixel with L.a = 0, or one no light reaches
(w = 0, where the rule is the identity in arithmetic), is left exactly as it is. The layer
is then drawn by its opacity and blend mode as usual. With nothing beneath, nothing changes; on
an adjustment layer it changes nothing, as that has no drawing of its own. Width 0 leaves the
blur out, so only a pixel the layer part covers takes the light, through its uncovered part.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, where the build works in single precision on its buffers, and
it blurs with the two-dimensional kernel summed directly (`edges_reference.gaussian`).

Every case is a composition 16 pixels by 10 of two drawings the same size, unmoved unless the
case says: bright bands of colour behind, and a small box in front with Light Wrap on it, both
drawn below. The drawings go into `Fixtures/light_wrap/media`, the projects into
`Fixtures/light_wrap`, and the expected frames into `Fixtures/light_wrap/expected_light_wrap.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/light_wrap_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as A  # noqa: E402
from adjust_reference import over, exposure, prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from edges_reference import gaussian  # noqa: E402
from solid_reference import multiply  # noqa: E402

W, H = R.W, R.H
FRAMES = S.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "light_wrap"
TOLERANCE = 2e-5  # document 25's default for a filter
WIDTH, INTENSITY, BLEND_WORDS = (0, 500), (0, 400), ("screen", "add")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def blurred(px, s, size=(W, H)):
    """Document 21's Gaussian at s on a frame-sized picture, cut back to the frame."""
    w, h = size
    g = gaussian({"px": px, "left": 0, "top": 0, "w": w, "h": h}, s, "transparent")
    return [g["px"][(y - g["top"]) * g["w"] + x - g["left"]] for y in range(h) for x in range(w)]


def light(L, B, width, intensity, size=(W, H)):
    """w per pixel: the light of the frame beneath that reaches the layer's edge."""
    s = width / 3
    Bb = blurred(B, s, size)
    Ab = blurred([[0.0, 0.0, 0.0, p[3]] for p in L], s, size)
    return [[intensity / 100 * (1 - ab[3]) * bb[c] for c in range(3)] for ab, bb in zip(Ab, Bb)]


def light_wrap(L, B, width, intensity, blend, size=(W, H)):
    """The placed layer L with the light of B laid on its edges; `size` is the frame's."""
    out = []
    for p, w in zip(L, light(L, B, width, intensity, size)):
        a = p[3]
        if a <= 0 or w == [0.0, 0.0, 0.0]:  # no light: the same in arithmetic, kept exactly
            out.append(list(p))
            continue
        l = [p[c] / a for c in range(3)]
        if blend == "add":
            l2 = [l[c] + w[c] for c in range(3)]
        else:
            l2 = [1 - (1 - l[c]) * (1 - min(1.0, max(0.0, w[c]))) for c in range(3)]
        out.append([v * a for v in l2] + [a])
    return out


BLENDS = {"normal": over, "multiply": multiply}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    """Bottom layer first. A drawing's other effects in its own space, then it is placed, then
    each enabled Light Wrap on the frame beneath, then its opacity and blend mode."""
    frame = [list(EMPTY) for _ in range(W * H)]
    for layer in c["layers"]:
        fx = [e for e in layer.get("effects", []) if e.get("enabled", True)]
        wraps = [e for e in fx if e["type"] == "wrap"]
        if layer.get("kind") == "adjustment":
            assert len(wraps) == len(fx), "only Light Wrap is worked on an adjustment layer here"
            continue  # its only effects change nothing, so the frame is left as it was
        px = [R.working(p) for row in DRAWINGS[layer["drawing"]] for p in row]
        for e in fx:
            if e["type"] == "exposure":
                px = exposure(px, e["stops"])
        L = R.frame(px, layer.get("shift", 0))
        for e in wraps:
            L = light_wrap(L, frame, held(value_at(e["width"], frame_no), WIDTH),
                           held(value_at(e["intensity"], frame_no), INTENSITY), e["blend"])
        op = layer.get("opacity", 1)
        mix = BLENDS[layer.get("mode", "normal")]
        frame = [mix([v * op for v in s], d) for s, d in zip(L, frame)]
    return frame


def plain(c):
    """The same composition with every Light Wrap taken out."""
    return render({"layers": [dict(l, effects=[e for e in l.get("effects", [])
                                               if e["type"] != "wrap"])
                              for l in c["layers"]]}, 0)


# --- the drawings ---------------------------------------------------------------------------

LINE, SOFT, SKIN, NONE = R.LINE, R.SOFT, R.SKIN, R.NONE
BANDS = [(255, 210, 60, 255), (60, 200, 255, 255), (255, 80, 200, 255), (140, 255, 90, 255)]


def box(x, y):
    """A box of line in columns 5 to 10 and rows 2 to 7, filled with skin, with the line at half
    covering down column 4, rows 3 to 6, its antialiased edge. The rest is empty."""
    if x == 4 and 3 <= y <= 6:
        return SOFT
    if not (5 <= x <= 10 and 2 <= y <= 7):
        return NONE
    if x in (5, 10) or y in (2, 7):
        return LINE
    return SKIN


DRAWINGS = {
    # Opaque bands four columns wide: yellow #ffd23c, cyan #3cc8ff, pink #ff50c8, green #8cff5a.
    "bands": [[BANDS[x // 4] for x in range(W)] for y in range(H)],
    "box": [[box(x, y) for x in range(W)] for y in range(H)],
}


# --- the cases ------------------------------------------------------------------------------

def wrap(width=10, intensity=100, blend="screen", enabled=True):
    return {"type": "wrap", "width": width, "intensity": intensity, "blend": blend,
            "enabled": enabled}


DARKER = {"type": "exposure", "stops": -1}


def front(*effects, shift=0, opacity=1, mode="normal"):
    return {"id": "art", "drawing": "box", "shift": shift, "opacity": opacity, "mode": mode,
            "effects": list(effects)}


def back(shift=0):
    return {"id": "bg", "drawing": "bands", "shift": shift}


def case(*layers):
    return {"layers": list(layers)}


def std(**kw):
    return case(back(), front(wrap(**kw)))


CASES = {
    "FX-WRAP-001": ("The defaults, width 10, intensity 100, screen, on the box over the bands: "
                    "every pixel of the box takes the bands' light, blurred at sigma 3.33, most "
                    "at its edges and least in its middle, each colour from the bands nearest "
                    "it; its covering is unchanged, and the bands around it are untouched.",
                    std(), [0]),
    "FX-WRAP-002": ("Width 3: the light reaches about a pixel in. The box's corner takes more "
                    "than twenty times the light its middle does.",
                    std(width=3), [0]),
    "FX-WRAP-003": ("Width 20, sigma 6.67: most of the blur falls past the 16 by 10 frame, "
                    "where nothing is beneath, so less light arrives than at width 10 and the "
                    "box takes it almost evenly. The frame's edge cuts the light off, as D-66 "
                    "cuts an adjustment layer's blur.",
                    std(width=20), [0]),
    "FX-WRAP-004": ("Intensity 400: four times the light, held at white before the screen; "
                    "every pixel at least as bright as FX-WRAP-001's.",
                    std(intensity=400), [0]),
    "FX-WRAP-005": ("Intensity 0: nothing changes.",
                    std(intensity=0), [0]),
    "FX-WRAP-006": ("Width 0: the blur is left out, so the fully covered pixels are unchanged "
                    "and only the half-covered edge down column 4 takes the light, through its "
                    "uncovered half: its straight colour is screened by half the band behind it.",
                    std(width=0), [0]),
    "FX-WRAP-007": ("Blend add: the light is added, not screened, so every pixel is at least "
                    "as bright as FX-WRAP-001's, and the line's colour may pass its covering.",
                    std(blend="add"), [0]),
    "FX-WRAP-008": ("Nothing beneath: the box alone, and nothing changes.",
                    case(front(wrap())), [0]),
    "FX-WRAP-009": ("The bands moved eight pixels right, so only columns 8 to 15 have anything "
                    "behind: the box's left edge, far from the light, takes less than its right "
                    "edge, and columns 0 to 7 around the box stay empty.",
                    case(back(shift=8), front(wrap())), [0]),
    "FX-WRAP-010": ("The box moved three pixels right, the bands not: its covering and its wrap "
                    "move with it, the light it takes is from the bands now behind it, and "
                    "columns 0 to 6 are the bands, untouched.",
                    case(back(), front(wrap(), shift=3)), [0]),
    "FX-WRAP-011": ("The box at opacity 50 per cent: the wrap is worked first, then the box is "
                    "drawn at half its covering over the bands.",
                    case(back(), front(wrap(), opacity=0.5)), [0]),
    "FX-WRAP-012": ("The box in blend mode multiply: the wrap is worked first, then the box, "
                    "lit, multiplies the bands.",
                    case(back(), front(wrap(), mode="multiply")), [0]),
    "FX-WRAP-013": ("Exposure -1 before Light Wrap in the stack: the box is darkened in its own "
                    "space, then placed and wrapped.",
                    case(back(), front(DARKER, wrap())), [0]),
    "FX-WRAP-014": ("Light Wrap before Exposure -1 in the stack: the other effects still run "
                    "first, so this is FX-WRAP-013 exactly.",
                    case(back(), front(wrap(), DARKER)), [0]),
    "FX-WRAP-015": ("Light Wrap on an adjustment layer above the bands and the box: it has no "
                    "drawing of its own, so nothing changes.",
                    case(back(), front(),
                         {"id": "adj", "kind": "adjustment", "effects": [wrap()]}), [0]),
    "FX-WRAP-016": ("Light Wrap switched off: nothing changes.",
                    std(enabled=False), [0]),
    "FX-WRAP-017": ("Width keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is "
                    "FX-WRAP-006, frame 2 is FX-WRAP-002, and at frame 4 the light reaches "
                    "further in, so the box's middle takes more than at frame 2.",
                    std(width=keyed((0, 0), (4, 6))), [0, 2, 4]),
    "FX-WRAP-018": ("Intensity keyed from 0 at frame 0 to 400 at frame 4, eased past its end "
                    "(530 at frame 2): frame 0 changes nothing; frame 2 is held at 400 and is "
                    "frame 4, FX-WRAP-004.",
                    std(intensity=keyed((0, 0, OVERSHOOT), (4, 400))), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-WRAP-019": ("Width 501, above 500.", std(width=501)),
    "FX-WRAP-020": ("Width -1, below 0.", std(width=-1)),
    "FX-WRAP-021": ("Intensity 401, above 400.", std(intensity=401)),
    "FX-WRAP-022": ("Intensity -1, below 0.", std(intensity=-1)),
    "FX-WRAP-023": ("Blend \"multiply\", which is not screen or add.", std(blend="multiply")),
    "FX-WRAP-024": ("Width keyed to 600 at frame 4.", std(width=keyed((0, 10), (4, 600)))),
}


# --- the project files ----------------------------------------------------------------------

def effect_json(e, i, n):
    if e["type"] == "exposure":
        return {"instance_id": f"fx-{i}-{n}", "type_id": "core.exposure", "enabled": True,
                "parameters": {"stops": e["stops"]}}
    return {"instance_id": f"fx-{i}-{n}", "type_id": "core.light_wrap",
            "enabled": e["enabled"],
            "parameters": {"width": setting_json(e["width"]),
                           "intensity": setting_json(e["intensity"]), "blend": e["blend"]}}


def layer_json(l, i):
    kind = l.get("kind", "raster")
    rec = A.layer_json({"id": l["id"], "kind": kind, "drawing": l.get("drawing"),
                        "opacity": l.get("opacity", 1), "out": FRAMES}, i)
    t = rec["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + l.get("shift", 0), H / 2])
    rec["blend_mode"] = l.get("mode", "normal")
    rec["effects"] = [effect_json(e, i, n) for n, e in enumerate(l.get("effects", []))]
    return rec


def project_json(fx, c):
    p = S.project_json(fx, {"drawing": "box", "shift": 0, "softness": 0, "threshold": 0})
    template = p["assets"][0]
    used = sorted({l["drawing"] for l in c["layers"] if "drawing" in l})
    p["assets"] = [dict(template, id="asset-" + d, name=d, path=f"media/{d}.png") for d in used]
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    comp["layer_order"] = [l["id"] for l in c["layers"]]  # bottom first, as document 19 draws
    comp["layers"] = [layer_json(l, i) for i, l in enumerate(c["layers"])]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_light_wrap.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for p, q in zip(a, b))  # noqa: E731
    drawn = plain(std())
    bands = [R.working(p) for row in DRAWINGS["bands"] for p in row]
    box_px = [R.working(p) for row in DRAWINGS["box"] for p in row]
    shows = [p[3] > 0 for p in box_px]
    corner, middle, soft = (5, 2), (7, 4), (4, 4)

    # The rule's own pieces.
    assert light_wrap(box_px, [list(EMPTY)] * (W * H), 10, 100, "screen") == box_px
    assert blurred(bands, 0) == bands and len(blurred(bands, 10 / 3)) == W * H
    w0 = light(box_px, bands, 0, 100)
    assert w0[at(*middle)] == [0.0] * 3 and w0[at(*soft)] == [(1 - SOFT[3] / 255) * v for v in bands[at(*soft)][:3]]
    assert BLENDS["normal"]([0.2, 0.2, 0.2, 0.5], bands[0]) == over([0.2, 0.2, 0.2, 0.5], bands[0])

    # Every case: the covering is never changed, the wrap only brightens, and where the box is
    # empty the frame is what it would be without the wrap.
    for name in CASES:
        before = plain(CASES[name][1])
        for f in c[name].values():
            for p, q in zip(f, before):
                assert p[3] == q[3] and all(u >= v - 1e-15 for u, v in zip(p, q)), name

    one = c["FX-WRAP-001"]["0"]
    assert [i for i in range(W * H) if one[i] != drawn[i]] == [i for i in range(W * H) if shows[i]]
    assert all(one[i] == bands[i] for i in range(W * H) if not shows[i])
    assert all(v <= p[3] + 1e-12 for p in one for v in p[:3])  # screen stays within covering
    w3 = light(box_px, bands, 3, 100)
    assert sum(w3[at(*corner)]) > 20 * sum(w3[at(*middle)]) > 0
    w10, w20 = light(box_px, bands, 10, 100), light(box_px, bands, 20, 100)
    assert sum(w10[at(*corner)]) > sum(w10[at(*middle)]) < sum(w10[at(10, 4)])
    in20 = [sum(w20[i]) for i in range(W * H) if shows[i]]
    assert all(sum(w20[i]) < sum(w10[i]) for i in range(W * H) if shows[i])
    assert max(in20) < 1.1 * min(in20)
    assert c["FX-WRAP-003"]["0"] == [over(p, b) for p, b in
                                     zip(light_wrap(box_px, bands, 20, 100, "screen"), bands)]
    three = c["FX-WRAP-004"]["0"]
    assert all(u >= v for p, q in zip(three, one) for u, v in zip(p, q)) and three != one
    assert c["FX-WRAP-005"]["0"] == drawn
    five = c["FX-WRAP-006"]["0"]
    assert [i % W for i in range(W * H) if five[i] != drawn[i]] == [4] * 4
    a = SOFT[3] / 255
    l, bnd = [box_px[at(*soft)][k] / a for k in range(3)], bands[at(*soft)]
    lit = [(1 - (1 - l[k]) * (1 - (1 - a) * bnd[k])) * a for k in range(3)] + [a]
    assert near(five[at(*soft)], over(lit, bnd))
    six = c["FX-WRAP-007"]["0"]
    assert any(v > p[3] for p in six for v in p[:3])  # add may pass the covering
    assert all(u >= v - 1e-15 for p, q in zip(six, one) for u, v in zip(p, q)) and six != one
    assert c["FX-WRAP-008"]["0"] == plain(case(front(wrap()))) == R.frame(box_px, 0)
    eight = c["FX-WRAP-009"]["0"]
    assert eight[at(5, 4)][0] - drawn[at(5, 4)][0] < eight[at(10, 4)][0] - drawn[at(10, 4)][0]
    assert all(eight[at(x, y)] == EMPTY for x in range(8) for y in range(H) if not shows[at(x, y)])
    nine = c["FX-WRAP-010"]["0"]
    moved = R.frame(box_px, 3)
    assert all(nine[at(x, y)] == bands[at(x, y)] for x in range(7) for y in range(H))
    assert nine == [over(p, b) for p, b in zip(light_wrap(moved, bands, 10, 100, "screen"), bands)]
    assert nine[at(8, 4)] != [v for v in one[at(5, 4)]]  # the same line pixel, other light
    lit1 = light_wrap(box_px, bands, 10, 100, "screen")
    assert c["FX-WRAP-011"]["0"] == [over([v * 0.5 for v in p], b) for p, b in zip(lit1, bands)]
    assert c["FX-WRAP-012"]["0"] == [multiply(p, b) for p, b in zip(lit1, bands)]
    twelve = c["FX-WRAP-013"]["0"]
    assert c["FX-WRAP-014"]["0"] == twelve
    dark = light_wrap(exposure(box_px, -1), bands, 10, 100, "screen")
    assert twelve == [over(p, b) for p, b in zip(dark, bands)]
    assert twelve != [over(p, b) for p, b in zip(exposure(lit1, -1), bands)]  # not after
    assert c["FX-WRAP-015"]["0"] == drawn and c["FX-WRAP-016"]["0"] == drawn
    sixteen = c["FX-WRAP-017"]
    assert sixteen["0"] == five and sixteen["2"] == c["FX-WRAP-002"]["0"]
    assert all(u > v for u, v in zip(sixteen["4"][at(*middle)][:3], sixteen["2"][at(*middle)][:3]))
    seventeen = c["FX-WRAP-018"]
    assert seventeen["0"] == drawn and seventeen["2"] == seventeen["4"] == three
    print("checked")


if __name__ == "__main__":
    main()
