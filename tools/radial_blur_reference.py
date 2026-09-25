"""Radial blur, worked a second way.

D-95 adds `core.radial_blur`. It smears a layer round a centre, as a camera spinning or zooming
during the exposure would: each pixel becomes the plain average of samples taken along the arc
of a circle about the centre through it (spin), or along the line from the centre through it
(zoom). `amount` is 0 to 100: the arc in degrees for a spin, and the stretch in per cent of the
distance from the centre for a zoom. `center` is two numbers, per cent of the drawing's width and
height, 50, 50 its middle. It is this program's own method, modelled on After Effects' Radial
Blur; nothing is ported. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. With c the centre in layer pixels, (center_x / 100 * width, center_y / 100 * height)
of the drawing's own size, p the centre of the output pixel, d = p - c and r = |d|, the path is
r * amount * pi / 180 for a spin and r * amount / 100 for a zoom, and n = min(ceil(path) + 1,
256). With n = 1 the output is the sample at p. Otherwise, for k = 0 to n - 1, a spin samples at
c + d turned by -amount / 2 + k * amount / (n - 1) degrees, and a zoom at c + s_k * d with
s_k = 1 - amount / 200 + k * (amount / 100) / (n - 1). The output is (1 / n) times the sum of
document 21's bilinear samples of the layer there, transparent outside the layer; premultiplied
red, green, blue and alpha are averaged alike. The layer does not grow: the blur is worked at
the layer's own pixels only, so nothing is drawn outside it, and an earlier effect that grew the
layer is sampled over its grown pixels with the centre still in the drawing's own size.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding directional blur's drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/radial_blur/media`, the projects
into `Fixtures/radial_blur`, and the expected frames into
`Fixtures/radial_blur/expected_radial_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/radial_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402

W, H = D.W, D.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "radial_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
AMOUNT = (0, 100)
CENTER = (-1000, 1000)
MOST = 256


# --- the rule -------------------------------------------------------------------------------

def bilinear(layer, x, y):
    """Document 21's sample from pixel centres, transparent outside the layer. A layer is its
    pixels and the layer-space rectangle they cover, which an earlier effect may have grown."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx, sy = x0 + dx - layer["left"], y0 + dy - layer["top"]
            if 0 <= sx < layer["w"] and 0 <= sy < layer["h"] and wx * wy:
                p = layer["px"][sy * layer["w"] + sx]
                for i in range(4):
                    out[i] += p[i] * wx * wy
    return out


def samples(kind, amount, center, x, y):
    """Where the pixel (x, y) of layer space is sampled."""
    cx, cy = center[0] / 100 * W, center[1] / 100 * H
    px, py = x + 0.5, y + 0.5
    dx, dy = px - cx, py - cy
    r = math.sqrt(dx * dx + dy * dy)
    path = r * amount * math.pi / 180 if kind == "spin" else r * amount / 100
    n = min(math.ceil(path) + 1, MOST)
    if n == 1:
        return [(px, py)]
    out = []
    for k in range(n):
        if kind == "spin":
            t = math.radians(-amount / 2 + k * amount / (n - 1))
            out.append((cx + dx * math.cos(t) - dy * math.sin(t),
                        cy + dx * math.sin(t) + dy * math.cos(t)))
        else:
            s = 1 - amount / 200 + k * (amount / 100) / (n - 1)
            out.append((cx + s * dx, cy + s * dy))
    return out


def blurred(layer, kind, amount, center, x, y):
    """The output at the pixel (x, y) of layer space: nothing outside the layer."""
    if not (0 <= x - layer["left"] < layer["w"] and 0 <= y - layer["top"] < layer["h"]):
        return [0.0] * 4
    at = samples(kind, amount, center, x, y)
    total = [0.0] * 4
    for sx, sy in at:
        s = bilinear(layer, sx, sy)
        for i in range(4):
            total[i] += s[i]
    return [v / len(at) for v in total]


# --- the cases ------------------------------------------------------------------------------

def case(kind="spin", amount=30, center=(50, 50), shift=0, before=None):
    """`before` is a directional blur (direction, length) ahead of the radial blur."""
    return {"drawing": "bars", "type": kind, "amount": amount, "center": center,
            "shift": shift, "before": before}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def layer_of(c):
    drawn = [D.working(p) for row in D.DRAWINGS[c["drawing"]] for p in row]
    layer = {"px": drawn, "left": 0, "top": 0, "w": W, "h": H}
    if c["before"]:
        direction, length = c["before"]
        g = math.ceil(length / 2)
        layer = {"px": [D.blurred(drawn, direction, length, x, y)
                        for y in range(-g, H + g) for x in range(-g, W + g)],
                 "left": -g, "top": -g, "w": W + 2 * g, "h": H + 2 * g}
    return layer


def render(c, frame_no):
    layer = layer_of(c)
    amount = clamp(value_at(c["amount"], frame_no), AMOUNT)
    center = [clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    return [blurred(layer, c["type"], amount, center, x - c["shift"], y)
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-RADIAL-001": ("Spin 30 about the middle: every edge smears round the centre, the "
                      "further out the longer, and the middle of the block stays solid.",
                      case(), [0]),
    "FX-RADIAL-002": ("Zoom 30 about the middle: every edge smears along the line from the "
                      "centre, the further out the longer.",
                      case(kind="zoom"), [0]),
    "FX-RADIAL-003": ("Amount 0: the drawing, untouched.",
                      case(amount=0), [0]),
    "FX-RADIAL-004": ("Spin 30 about the top left corner, centre 0, 0: the smears are arcs about "
                      "that corner, so the far corner smears most.",
                      case(center=(0, 0)), [0]),
    "FX-RADIAL-005": ("Zoom 30 about centre 25, 50, the point (4, 5): the block, right of it, "
                      "smears left and right, and the line, left of it, the other way.",
                      case(kind="zoom", center=(25, 50)), [0]),
    "FX-RADIAL-006": ("Spin 100, the most: longer arcs than FX-RADIAL-001.",
                      case(amount=100), [0]),
    "FX-RADIAL-007": ("Zoom 100, the most: samples from half to one and a half times the "
                      "distance from the centre.",
                      case(kind="zoom", amount=100), [0]),
    "FX-RADIAL-008": ("Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 "
                      "untouched, frame 2 spins 20, frame 4 spins 40.",
                      case(amount=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-RADIAL-009": ("Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 "
                      "is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004.",
                      case(center=keyed((0, (50, 50)), (4, (0, 0)))), [0, 2, 4]),
    "FX-RADIAL-010": ("Spin 30, moved three pixels right: the centre moves with the drawing, "
                      "and nothing is drawn left of the drawing's edge.",
                      case(shift=3), [0, 3]),
    "FX-RADIAL-011": ("Spin 100 about centre -1000, -1000, far off the top left: every pixel's "
                      "arc is longer than 255 pixels, so each takes the most samples, 256.",
                      case(amount=100, center=(-1000, -1000)), [0]),
    "FX-RADIAL-012": ("A directional blur, direction 90 and length 4, then spin 30 about centre "
                      "0, 0: the directional blur grew the layer two pixels on every side, and "
                      "the spin still turns about the drawing's own top left corner.",
                      case(center=(0, 0), before=(90, 4)), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RADIAL-013": ("Amount 101, above 100.", case(amount=101)),
    "FX-RADIAL-014": ("Amount -1, below 0.", case(amount=-1)),
    "FX-RADIAL-015": ("Amount keyed to 150 at frame 4.", case(amount=keyed((0, 0), (4, 150)))),
    "FX-RADIAL-016": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-RADIAL-017": ("Type \"twist\", which is not a type.", case(kind="twist")),
    "FX-RADIAL-018": ("Type \"Spin\": the word is exact, so a capital is not the type.",
                      case(kind="Spin")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["before"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.directional_blur",
                        "enabled": True,
                        "parameters": {"direction": c["before"][0], "length": c["before"][1]}})
    effects.append({
        "instance_id": f"fx-0-{len(effects)}", "type_id": "core.radial_blur", "enabled": True,
        "parameters": {"type": c["type"], "amount": setting_json(c["amount"]),
                       "center": setting_json(c["center"])}})
    comp["layers"][0]["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in D.DRAWINGS.items():
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

    (OUT / "expected_radial_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    one, two = c["FX-RADIAL-001"]["0"], c["FX-RADIAL-002"]["0"]

    assert c["FX-RADIAL-003"]["0"] == drawn
    # The middle of the block, a pixel from the centre, stays solid skin under either.
    assert near([one[at(7, 4)], two[at(7, 4)]], [drawn[at(7, 4)]] * 2)
    # Spin: the line on the left edge, seven pixels out, smears up and down past its ends, and
    # hardly sideways where it is level with the centre: under a hundredth.
    assert one[at(0, 1)][3] > 0 and one[at(0, 8)][3] > 0
    assert one[at(1, 4)][3] < 0.01 and one[at(1, 5)][3] < 0.01
    # Zoom: the same line smears sideways, toward and away from the centre, and less past its
    # ends.
    assert two[at(1, 4)][3] > 0.2 and two[at(1, 5)][3] > 0.2
    assert two[at(0, 1)][3] < one[at(0, 1)][3]
    assert c["FX-RADIAL-004"]["0"] != one
    # About (4, 5): the line is left of the centre and the block right of it, so a zoom smears
    # the line onto column 1 and the block onto column 10, and column 3, next to the centre,
    # stays empty: its samples reach only half a pixel either way.
    five = c["FX-RADIAL-005"]["0"]
    assert five[at(1, 5)][3] > 0 and five[at(10, 4)][3] > 0
    assert all(five[at(3, y)][3] < 1e-9 for y in range(H))
    # The most smears further: the block's far corner is less covered at 100 than at 30.
    assert c["FX-RADIAL-006"]["0"][at(9, 6)][3] < one[at(9, 6)][3]
    assert c["FX-RADIAL-007"]["0"][at(9, 6)][3] < two[at(9, 6)][3]
    eight, nine = c["FX-RADIAL-008"], c["FX-RADIAL-009"]
    assert eight["0"] == drawn and near(eight["2"], render(case(amount=20), 0))
    assert near(eight["4"], render(case(amount=40), 0))
    assert near(nine["0"], one) and near(nine["4"], c["FX-RADIAL-004"]["0"])
    assert near(nine["2"], render(case(center=(25, 25)), 0))
    # Moved: FX-RADIAL-001 three columns on, and nothing in the three columns left of the
    # drawing, where the spin would have carried the line had the layer grown.
    ten = c["FX-RADIAL-010"]
    assert ten["0"] == ten["3"]
    assert all(ten["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(ten["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    assert any(one[at(0, y)][3] > 0 for y in range(H))
    # The cap: every pixel's arc is past 255 pixels, and the frame is not empty.
    assert all(len(samples("spin", 100, (-1000, -1000), x, y)) == MOST
               for x in range(W) for y in range(H))
    assert any(p[3] > 0 for p in c["FX-RADIAL-011"]["0"])
    # After a directional blur: turned about the drawing's corner, not the grown layer's, and
    # the grown pixels feed it, so column 0 differs from a spin of the plain drawing.
    twelve = c["FX-RADIAL-012"]["0"]
    wrong = case(center=(-2 / W * 100, -2 / H * 100), before=(90, 4))
    assert twelve != render(wrong, 0) and twelve != c["FX-RADIAL-004"]["0"]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
