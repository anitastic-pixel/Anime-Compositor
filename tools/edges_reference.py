"""Repeat Edge Pixels, worked a second way.

D-109 adds `edges` to Gaussian Blur, Directional Blur and Radial Blur: `"transparent"`, the rule
each has always had and what a file without the setting means, or `"repeat"`. With `"repeat"`,
every point a blur samples is first held inside the rectangle of its input's pixel centres,
x between 0.5 and width - 0.5 and y between 0.5 and height - 0.5 (for Gaussian Blur, whose taps
fall on whole pixels, each tap's column and row held inside the input's), so the pixels at the
edge carry on past it instead of fading into transparency. The layer then does not grow. The
input is the effect's own input: the drawing, or what the effects before it made of it, grown
pixels and all. It is After Effects' Repeat Edge Pixels; nothing is ported. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, where the build works in single precision on its buffers.
Directional Blur and Radial Blur are `directional_blur_reference.py`'s and
`radial_blur_reference.py`'s rules, run with their `edges` argument; Gaussian Blur is document
21's, summed directly here.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: `plate`, a picture that fills the layer to its edges, or directional blur's
`bars`. The drawings go into `Fixtures/edges/media`, the projects into `Fixtures/edges`, and the
expected frames into `Fixtures/edges/expected_edges.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/edges_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import radial_blur_reference as R  # noqa: E402

W, H = D.W, D.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "edges"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def gaussian(layer, sigma, edges):
    """Document 21's Gaussian blur of a layer (its pixels and the layer-space rectangle they
    cover), summed directly. Transparent: outside is transparent black and the layer grows by
    the kernel radius. Repeat: each tap's column and row held inside the layer's, and no
    growth."""
    r = math.ceil(3 * sigma)
    if r == 0:
        return layer
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    total = sum(one)
    one = [v / total for v in one]
    g = 0 if edges == "repeat" else r
    w, h = layer["w"], layer["h"]

    def pixel(sx, sy):
        if edges == "repeat":
            sx, sy = min(max(sx, 0), w - 1), min(max(sy, 0), h - 1)
        elif not (0 <= sx < w and 0 <= sy < h):
            return None
        return layer["px"][sy * w + sx]

    out = []
    for y in range(-g, h + g):
        for x in range(-g, w + g):
            acc = [0.0] * 4
            for j in range(-r, r + 1):
                for i in range(-r, r + 1):
                    p = pixel(x + i, y + j)
                    if p:
                        for c in range(4):
                            acc[c] += p[c] * one[i + r] * one[j + r]
            out.append(acc)
    return {"px": out, "left": layer["left"] - g, "top": layer["top"] - g,
            "w": w + 2 * g, "h": h + 2 * g}


def directional(layer, direction, length, edges):
    """directional_blur_reference's rule, on the drawing itself."""
    assert (layer["left"], layer["top"], layer["w"], layer["h"]) == (0, 0, W, H)
    g = 0 if edges == "repeat" else math.ceil(length / 2)
    return {"px": [D.blurred(layer["px"], direction, length, x, y, edges)
                   for y in range(-g, H + g) for x in range(-g, W + g)],
            "left": -g, "top": -g, "w": W + 2 * g, "h": H + 2 * g}


def radial(layer, kind, amount, center, edges):
    """radial_blur_reference's rule: the layer keeps its rectangle."""
    return dict(layer, px=[R.blurred(layer, kind, amount, center, x, y, edges)
                           for y in range(layer["top"], layer["top"] + layer["h"])
                           for x in range(layer["left"], layer["left"] + layer["w"])])


# --- the drawings ---------------------------------------------------------------------------

def plate(x, y):
    """A picture to the layer's edges: red rising to the right, green rising downward."""
    return (16 * x, 25 * y, 160, 255)


DRAWINGS = {"plate": [[plate(x, y) for x in range(W)] for y in range(H)],
            "bars": D.DRAWINGS["bars"]}


# --- the cases ------------------------------------------------------------------------------

GAUSS, DIR, RAD = "core.gaussian_blur", "core.directional_blur", "core.radial_blur"


def case(*effects, drawing="plate", shift=0):
    """Each effect is (type, parameters, edges), edges None when the file does not say."""
    return {"drawing": drawing, "shift": shift, "effects": effects}


def gauss(sigma, edges="repeat"):
    return (GAUSS, {"sigma_px": sigma}, edges)


def streak(direction, length, edges="repeat"):
    return (DIR, {"direction": direction, "length": length}, edges)


def turn(kind, amount, center, edges="repeat"):
    return (RAD, {"type": kind, "amount": amount, "center": center}, edges)


def render(c):
    layer = {"px": [D.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    for kind, p, edges in c["effects"]:
        edges = edges or "transparent"
        if kind == GAUSS:
            layer = gaussian(layer, p["sigma_px"], edges)
        elif kind == DIR:
            layer = directional(layer, p["direction"], p["length"], edges)
        else:
            layer = radial(layer, p["type"], p["amount"], p["center"], edges)
    out = []
    for y in range(H):
        for x in range(W):
            lx, ly = x - c["shift"] - layer["left"], y - layer["top"]
            inside = 0 <= lx < layer["w"] and 0 <= ly < layer["h"]
            out.append(layer["px"][ly * layer["w"] + lx] if inside else [0.0] * 4)
    return out


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(drawing=c["drawing"], shift=c["shift"]))


CASES = {
    "FX-EDGES-001": ("Gaussian Blur, sigma 1, edges repeat, on a picture that fills the layer: "
                     "every pixel stays fully covered, the edges as solid as the middle, and "
                     "the middle is blurred exactly as without it.",
                     case(gauss(1))),
    "FX-EDGES-002": ("Gaussian Blur, sigma 1, edges written \"transparent\": the rule as it "
                     "always was, the edges fading, the same as a file without the setting.",
                     case(gauss(1, "transparent"))),
    "FX-EDGES-003": ("FX-EDGES-001 moved three pixels right: the layer does not grow, so the "
                     "three columns left of it stay empty.",
                     case(gauss(1), shift=3)),
    "FX-EDGES-004": ("Directional Blur, direction 90, length 6, edges repeat, on the picture: "
                     "streaked left and right, every pixel still fully covered.",
                     case(streak(90, 6))),
    "FX-EDGES-005": ("Directional Blur, direction 30, length 8, edges repeat: a slanting streak "
                     "whose samples leave the layer across both edges, every pixel still fully "
                     "covered.",
                     case(streak(30, 8))),
    "FX-EDGES-006": ("Directional Blur, direction 90, length 6, edges repeat, on the bars moved "
                     "three pixels right: the line on the drawing's left edge carries on past "
                     "it, so its column keeps more of the line than FX-DIRBLUR-009's, and "
                     "nothing is drawn left of the drawing.",
                     case(streak(90, 6), drawing="bars", shift=3)),
    "FX-EDGES-007": ("Radial Blur, spin 30 about the middle, edges repeat, on the picture: "
                     "every pixel still fully covered, the corners too.",
                     case(turn("spin", 30, [50, 50]))),
    "FX-EDGES-008": ("Radial Blur, zoom 40 about the top left corner, edges repeat: every "
                     "pixel still fully covered.",
                     case(turn("zoom", 40, [0, 0]))),
    "FX-EDGES-009": ("A Directional Blur, direction 90 and length 4, edges transparent, then "
                     "FX-EDGES-007's spin: the spin repeats the edge of its own input, the layer "
                     "the Directional Blur grew two pixels on every side, whose outer columns "
                     "have faded.",
                     case(streak(90, 4, None), turn("spin", 30, [50, 50]))),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-EDGES-010": ("Gaussian Blur with edges \"wrap\", which is not a way of treating edges.",
                     case(gauss(1, "wrap"))),
    "FX-EDGES-011": ("Directional Blur with edges \"Repeat\": the word is exact, so a capital "
                     "is not it.",
                     case(streak(90, 6, "Repeat"))),
    "FX-EDGES-012": ("Radial Blur with edges \"\", empty.",
                     case(turn("spin", 30, [50, 50], ""))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [
        {"instance_id": f"fx-0-{i}", "type_id": kind, "enabled": True,
         "parameters": dict(p, **({} if edges is None else {"edges": edges}))}
        for i, (kind, p, edges) in enumerate(c["effects"])]
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
    for fx, (says, c) in CASES.items():
        frame = render(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": {"0": frame}}
        before = plain(c)
        print(f"{fx}: {sum(frame[i] != before[i] for i in range(W * H))} changed")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_edges.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    solid = lambda px: all(abs(p[3] - 1) < 1e-12 for p in px)  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for p, q in zip(a, b))  # noqa: E731
    plain_plate = plain(case())

    # Gaussian: solid to the edges; the middle, three pixels from every edge, as without it.
    one, two = c["FX-EDGES-001"], c["FX-EDGES-002"]
    assert solid(one) and not solid(two) and two[at(0, 0)][3] < 0.75
    assert two == render(case(gauss(1, None)))
    assert all(near(one[at(x, y)], two[at(x, y)]) for x in range(3, W - 3) for y in range(3, H - 3))
    assert one[at(0, 0)] != plain_plate[at(0, 0)]
    three = c["FX-EDGES-003"]
    assert all(three[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    assert all(three[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))

    # Directional: solid to the edges; across only, so a row is untouched by the rows beside it,
    # and the middle columns, out of reach of both edges, are as without it.
    four = c["FX-EDGES-004"]
    assert solid(four) and solid(c["FX-EDGES-005"])
    loose = render(case(streak(90, 6, None)))
    assert all(near(four[at(x, y)], loose[at(x, y)]) for x in range(5, W - 5) for y in range(H))
    assert four[at(0, 0)] != loose[at(0, 0)]
    # The bars moved: nothing left of the drawing, and the line's column keeps more of the line.
    six = c["FX-EDGES-006"]
    assert all(six[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    was = render(case(streak(90, 6, None), drawing="bars", shift=3))
    assert six[at(3, 4)][3] > was[at(3, 4)][3] + 0.1

    # Radial: solid to the edges, and the spin over the grown layer is not the spin over the
    # drawing.
    assert solid(c["FX-EDGES-007"]) and solid(c["FX-EDGES-008"])
    assert c["FX-EDGES-007"] != render(case(turn("spin", 30, [50, 50], None)))
    assert c["FX-EDGES-009"] != c["FX-EDGES-007"] and not solid(c["FX-EDGES-009"])
    for name, px in c.items():
        for p in px:
            assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                      for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
