"""Magnify, worked a second way.

D-405 adds `core.magnify`, after After Effects' Magnify (Distort group): a round or square
part of the layer enlarged about its centre, laid back over the layer by a blending mode.
Adobe's page says what each control does and gives no formula; the rule is this program's own.

The settings. `shape`, "circle" or "square" ("circle" when added); `center`, two numbers, per
cent of the picture as it reaches the effect, -1000 to 1000 each, (50, 50); `magnification`,
100 to 1000 per cent, 200; `link`, "none", "size" or "size_feather" ("none"); `size`, 0 to 1000
pixels, 100, the area's radius; `feather`, 0 to 1000 pixels, 0; `opacity`, 0 to 100 per cent,
100; `scaling`, "standard", "soft" or "scatter" ("standard"); `blending_mode`, "none",
"normal", "add", "multiply", "screen", "overlay" or "soft_light" ("normal"); `resize_layer`,
"off" or "on" ("off"). The numbers are keyable; the words are not.

The rule. On the picture as it reaches the effect, w by h pixels, its centre point
c = (center / 100 w, center / 100 h), and m = magnification / 100:

    R = size, or m size when link is "size" or "size_feather"
    F = feather, or m feather when link is "size_feather"

The layer grows by g pixels on every side when resize_layer is "on" and link is "none", g the
smallest whole number that holds the square of half-side R round c inside the grown picture:
g = ceil(max(0, R - c_x, c_x + R - w, R - c_y, c_y + R - h)). Otherwise g = 0.

Each output pixel, its centre p in the picture's own coordinates (an output pixel (X, Y) of the
grown buffer has p = (X + 1/2 - g, Y + 1/2 - g)), v = p - c, and d = sqrt(v_x^2 + v_y^2) for a
circle or max(|v_x|, |v_y|) for a square. Its covering by the area is k = 1 when d <= R and 0
otherwise if F is 0 (for a circle the test is v_x^2 + v_y^2 <= R^2), else
k = clamp((R - d) / F, 0, 1): the feather lies inside the edge. Where k > 0 the enlarged picture
is read at q = c + v / m:

    standard: the pixel holding q, (floor(q_x), floor(q_y)), sharp and blocky;
    soft:     document 21's bilinear sample at q;
    scatter:  the pixel holding q + j (u_x, u_y), j = 1 - 1/m, u_x and u_y Noise's hash
              grade::unit(mix(0), i, j', 0, 0) and (.., 1) at the pixel's own whole-number
              place (i, j') = (X - g, Y - g) in the picture, so the blocks' edges break up.

Outside the picture each read is clear. The area's pixel is A = S k opacity / 100 (premultiplied).
The original's pixel O is the picture's at (X - g, Y - g), clear outside it. Then, by
blending_mode: "none" A alone (clear round the area); "normal" A over O; the rest document 21's
layer blend of A over O at 8 bits (each straight colour, Add held to 1, Overlay and Soft Light
on the encoded colours): out = (1 - a_A) O + (1 - a_O) A + a_A a_O B(c_A, c_O), alpha
a_A + a_O - a_A a_O.

Size and feather are distances, scaled by a draft; the centre is a share; magnification is not.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size (Tiles' card,
D-404), unmoved unless the case says. The drawing goes into `Fixtures/magnify/media`, the
projects into `Fixtures/magnify`, and the expected frames into
`Fixtures/magnify/expected_magnify.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/magnify_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import tiles_reference as T  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from noise_reference import mix  # noqa: E402
from lightning_extras_reference import unit  # noqa: E402
from color_link_reference import MIX  # noqa: E402

W, H = T.W, T.H  # 16 by 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "magnify"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"center": (-1000, 1000), "magnification": (100, 1000), "size": (0, 1000),
          "feather": (0, 1000), "opacity": (0, 100)}
WORDS = {"shape": ("circle", "square"), "link": ("none", "size", "size_feather"),
         "scaling": ("standard", "soft", "scatter"),
         "blending_mode": ("none", "normal", "add", "multiply", "screen", "overlay",
                           "soft_light"),
         "resize_layer": ("off", "on")}
NAMES = ("shape", "center", "magnification", "link", "size", "feather", "opacity", "scaling",
         "blending_mode", "resize_layer")
EMPTY = [0.0] * 4
BASE = mix(0)


# --- the rule -------------------------------------------------------------------------------

def to_srgb(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def blend(mode, a, o):
    if mode == "none":
        return list(a)
    if mode == "normal":
        return [a[i] + o[i] * (1 - a[3]) for i in range(4)]
    sa, so = a[3], o[3]
    ca = [a[i] / sa if sa > 0 else 0.0 for i in range(3)]
    co = [o[i] / so if so > 0 else 0.0 for i in range(3)]
    out = []
    for i in range(3):
        if mode == "multiply":
            b = ca[i] * co[i]
        elif mode == "screen":
            b = ca[i] + co[i] - ca[i] * co[i]
        elif mode == "add":
            b = min(1.0, ca[i] + co[i])
        else:
            e = lambda v: to_srgb(min(max(v, 0.0), 1.0))  # noqa: E731
            b = to_linear(MIX[mode](e(co[i]), e(ca[i])))
        out.append((1 - sa) * o[i] + (1 - so) * a[i] + sa * so * b)
    return out + [sa + so - sa * so]


def pixel(layer, i, j):
    if 0 <= i < layer["w"] and 0 <= j < layer["h"]:
        return layer["px"][j * layer["w"] + i]
    return EMPTY


def radius(s):
    m = s["magnification"] / 100
    r = s["size"] * m if s["link"] in ("size", "size_feather") else s["size"]
    f = s["feather"] * m if s["link"] == "size_feather" else s["feather"]
    return r, f


def growth(s, w, h, c):
    if s["resize_layer"] != "on" or s["link"] != "none":
        return 0
    r, _ = radius(s)
    return math.ceil(max(0.0, r - c[0], c[0] + r - w, r - c[1], c[1] + r - h))


def magnify(layer, s):
    w, h = layer["w"], layer["h"]
    c = (s["center"][0] / 100 * w, s["center"][1] / 100 * h)
    m = s["magnification"] / 100
    r, f = radius(s)
    g = growth(s, w, h, c)
    jit = 1 - 1 / m
    out = []
    for Y in range(h + 2 * g):
        for X in range(w + 2 * g):
            vx, vy = X + 0.5 - g - c[0], Y + 0.5 - g - c[1]
            if s["shape"] == "circle":
                d = math.sqrt(vx * vx + vy * vy)
                inside = vx * vx + vy * vy <= r * r
            else:
                d = max(abs(vx), abs(vy))
                inside = d <= r
            k = (1.0 if inside else 0.0) if f == 0 else min(max((r - d) / f, 0.0), 1.0)
            o = pixel(layer, X - g, Y - g)
            a = EMPTY
            if k > 0:
                qx, qy = c[0] + vx / m, c[1] + vy / m
                if s["scaling"] == "soft":
                    p = bilinear(layer, qx, qy)
                else:
                    if s["scaling"] == "scatter":
                        qx += jit * unit(BASE, X - g, Y - g, 0, 0)
                        qy += jit * unit(BASE, X - g, Y - g, 0, 1)
                    p = pixel(layer, math.floor(qx), math.floor(qy))
                a = [v * (k * (s["opacity"] / 100)) for v in p]
            out.append(blend(s["blending_mode"], a, o))
    return {"px": out, "left": -g, "top": -g, "w": w + 2 * g, "h": h + 2 * g}


# --- the cases ------------------------------------------------------------------------------

def case(**kw):
    c = {"drawing": "card", "shape": "circle", "center": (50, 50), "magnification": 200,
         "link": "none", "size": 100, "feather": 0, "opacity": 100, "scaling": "standard",
         "blending_mode": "normal", "resize_layer": "off", "shift": 0}
    c.update(kw)
    return c


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        if k in WORDS:
            held[k] = c[k]
            continue
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def placed(layer, shift):
    """The layer's buffer laid in the composition, moved `shift` pixels across."""
    out = []
    for y in range(H):
        for x in range(W):
            i, j = x - shift - layer["left"], y - layer["top"]
            out.append(list(pixel(layer, i, j)))
    return out


def render(c, frame_no):
    return placed(magnify(T.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return placed(T.drawn_layer(c["drawing"]), c["shift"])


SMALL = {"size": 4}
CASES = {
    "FX-MAGNIFY-001": ("The settings as they start: a circle of radius 100 round the middle, "
                       "magnification 200, standard scaling, Normal: the whole picture "
                       "enlarged twice round (8, 5), blocks of 2 by 2 pixels, laid over the "
                       "drawing, which shows through only where the enlargement is clear or "
                       "soft (After Effects' doubling).", case(), [0, 4]),
    "FX-MAGNIFY-002": ("Magnification 100 with blending mode None: the area reads each pixel "
                       "itself, so the drawing comes back untouched.",
                       case(magnification=100, blending_mode="none"), [0]),
    "FX-MAGNIFY-003": ("Size 4: a circle of radius 4 round (8, 5) enlarged twice, the drawing "
                       "round it.", case(**SMALL), [0]),
    "FX-MAGNIFY-004": ("Size 4, square: the area a square 8 by 8 round (8, 5).",
                       case(shape="square", **SMALL), [0]),
    "FX-MAGNIFY-005": ("Size 4, feather 2: the circle fades over the last 2 pixels inside its "
                       "edge.", case(feather=2, **SMALL), [0]),
    "FX-MAGNIFY-006": ("Link size to magnification, size 2: the radius 200 per cent of 2, so "
                       "FX-MAGNIFY-003 exactly.", case(link="size", size=2), [0]),
    "FX-MAGNIFY-007": ("Link size and feather, size 2, feather 1: radius 4, feather 2, so "
                       "FX-MAGNIFY-005 exactly.", case(link="size_feather", size=2, feather=1),
                       [0]),
    "FX-MAGNIFY-008": ("Size 4, opacity 50: the area at half its covering, over the drawing.",
                       case(opacity=50, **SMALL), [0]),
    "FX-MAGNIFY-009": ("Soft scaling: as FX-MAGNIFY-001 with each place read by document 21's "
                       "bilinear sample, smooth instead of blocky.", case(scaling="soft"),
                       [0]),
    "FX-MAGNIFY-010": ("Scatter scaling: as FX-MAGNIFY-001 with each place nudged by up to a "
                       "half a picture pixel each way (j = 1/2 at 200 per cent) by Noise's "
                       "hash, so the blocks' edges break up.", case(scaling="scatter"), [0]),
    "FX-MAGNIFY-011": ("Size 4, blending mode None: the circle alone, clear round it.",
                       case(blending_mode="none", **SMALL), [0]),
    "FX-MAGNIFY-012": ("Size 5, Multiply over the drawing.",
                       case(blending_mode="multiply", size=5), [0]),
    "FX-MAGNIFY-013": ("Size 5, Screen.", case(blending_mode="screen", size=5), [0]),
    "FX-MAGNIFY-014": ("Size 5, Add, each straight colour held to 1.",
                       case(blending_mode="add", size=5), [0]),
    "FX-MAGNIFY-015": ("Size 5, Overlay, on the encoded colours.",
                       case(blending_mode="overlay", size=5), [0]),
    "FX-MAGNIFY-016": ("Size 5, Soft Light, on the encoded colours.",
                       case(blending_mode="soft_light", size=5), [0]),
    "FX-MAGNIFY-017": ("Resize Layer on, size 4, centre at 90, 50 (14.4, 5), the layer moved 3 "
                       "pixels left: the circle reaches 2.4 pixels past the right edge, so the "
                       "layer grows 3 pixels on every side and the area shows past the "
                       "drawing's edge, in columns 13 to 15.",
                       case(resize_layer="on", center=(90, 50), shift=-3, **SMALL), [0]),
    "FX-MAGNIFY-018": ("FX-MAGNIFY-017 with the link at size (size 2, so radius 4): After "
                       "Effects turns Resize Layer off when linked, so nothing grows and the "
                       "circle is cut at the drawing's edge.",
                       case(resize_layer="on", center=(90, 50), shift=-3, link="size", size=2),
                       [0]),
    "FX-MAGNIFY-019": ("Magnification keyed from 100 at frame 0 to 400 at frame 4, size 4, "
                       "linear: 175 at frame 1, 400 at frame 4.",
                       case(magnification=keyed((0, 100), (4, 400)), **SMALL), [0, 1, 4]),
    "FX-MAGNIFY-020": ("Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, size 3: the "
                       "lens slides across.",
                       case(center=keyed((0, (25, 50)), (4, (75, 50))), size=3), [0, 2, 4]),
    "FX-MAGNIFY-021": ("Size 4 with the layer moved 3 pixels right: the same area, moved; "
                       "columns 0 to 2 empty.", case(shift=3, **SMALL), [0]),
    "FX-MAGNIFY-022": ("Magnification 300, size 100: blocks of 3 by 3 pixels.",
                       case(magnification=300), [0]),
    "FX-MAGNIFY-023": ("Size eased from 4 at frame 0 to 0 at frame 4 on a curve that "
                       "overshoots: at frame 2 it would pass below 0 and is held there, so "
                       "frames 2 and 4 show no area, the drawing as it is.",
                       case(size=keyed((0, 4, OVERSHOOT), (4, 0))), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-MAGNIFY-024": ("Magnification 99, below 100.", case(magnification=99)),
    "FX-MAGNIFY-025": ("Magnification 1001, above 1000.", case(magnification=1001)),
    "FX-MAGNIFY-026": ("Size -1, below 0.", case(size=-1)),
    "FX-MAGNIFY-027": ("Feather 1001, above 1000.", case(feather=1001)),
    "FX-MAGNIFY-028": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-MAGNIFY-029": ("Centre at 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-MAGNIFY-030": ("Shape \"oval\".", case(shape="oval")),
    "FX-MAGNIFY-031": ("Link \"feather\".", case(link="feather")),
    "FX-MAGNIFY-032": ("Scaling \"bicubic\".", case(scaling="bicubic")),
    "FX-MAGNIFY-033": ("Blending mode \"difference\", which this build does not offer.",
                       case(blending_mode="difference")),
    "FX-MAGNIFY-034": ("Resize layer \"yes\".", case(resize_layer="yes")),
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
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.magnify", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in T.DRAWINGS.items():
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

    (OUT / "expected_magnify.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def near(a, b, tol=1e-12):
    return all(abs(u - v) < tol for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = T.drawn_layer("card")
    d = lambda x, y: layer["px"][y * W + x]  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    first = one("FX-MAGNIFY-001")
    assert first == c["FX-MAGNIFY-001"]["4"]
    # Standard at 200: the place (x, y) reads floor(8 + (x + 0.5 - 8) / 2), so columns 0 and 1
    # read column 4, 14 and 15 read column 11 (the line); a fully covered pixel is the read one.
    assert first[at(14, 3)] == d(11, 4) == first[at(15, 3)]
    assert first[at(5, 5)] == d(6, 5)
    # Where the read pixel is clear, the drawing shows through: (15, 0) reads (11, 2), the line,
    # so take a covered one; a soft read lets the drawing through, (1, 4) reads (4, 4), skin.
    assert one("FX-MAGNIFY-002") == drawn
    three = one("FX-MAGNIFY-003")
    assert three[at(0, 0)] == drawn[at(0, 0)] and three[at(8, 5)] == d(8, 5)
    assert three[at(4, 5)] == d(6, 5) and three[at(3, 5)] == drawn[at(3, 5)]  # 4.5 in, 3.5 out
    four = one("FX-MAGNIFY-004")
    assert four[at(4, 1)] == d(6, 3) and four[at(3, 1)] == drawn[at(3, 1)]
    assert three[at(4, 1)] == drawn[at(4, 1)]  # a corner the circle leaves out
    assert one("FX-MAGNIFY-006") == three and one("FX-MAGNIFY-007") == one("FX-MAGNIFY-005")
    five = one("FX-MAGNIFY-005")
    assert five != three and five[at(8, 5)] == three[at(8, 5)]
    eight = one("FX-MAGNIFY-008")
    p, o = d(8, 5), drawn[at(8, 5)]
    assert near([eight[at(8, 5)]], [[p[i] / 2 + o[i] * (1 - p[3] / 2) for i in range(4)]])
    assert not near(one("FX-MAGNIFY-009"), first, 1e-3)
    assert not near(one("FX-MAGNIFY-010"), first, 1e-3)
    eleven = one("FX-MAGNIFY-011")
    assert eleven[at(0, 0)] == EMPTY and eleven[at(8, 5)] == d(8, 5)
    for fx in ("FX-MAGNIFY-012", "FX-MAGNIFY-013", "FX-MAGNIFY-014", "FX-MAGNIFY-015",
               "FX-MAGNIFY-016"):
        px = one(fx)
        assert px[at(0, 0)] == drawn[at(0, 0)] and px != drawn, fx
    seventeen, eighteen = one("FX-MAGNIFY-017"), one("FX-MAGNIFY-018")
    # The drawing covers columns 0 to 12; the grown lens reaches 13 and 14 (centre 11.4 in the
    # composition, radius 4: up to 15.4, row 5's centre 15.5 is out).
    assert any(seventeen[at(13, y)] != EMPTY for y in range(H))
    assert all(eighteen[at(x, y)] == EMPTY for x in range(13, W) for y in range(H))
    assert all(seventeen[at(x, y)] == eighteen[at(x, y)] for x in range(13) for y in range(H))
    nineteen = c["FX-MAGNIFY-019"]
    assert nineteen["0"] == drawn and nineteen["1"] != nineteen["4"]
    twenty = c["FX-MAGNIFY-020"]
    assert len({json.dumps(twenty[f]) for f in ("0", "2", "4")}) == 3
    twenty_one = one("FX-MAGNIFY-021")
    assert all(twenty_one[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(twenty_one[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    t22 = one("FX-MAGNIFY-022")
    assert t22[at(5, 5)] == d(7, 5)  # floor(8 + (5.5 - 8) / 3) = 7
    raw = value_at(CASES["FX-MAGNIFY-023"][1]["size"], 2)
    t23 = c["FX-MAGNIFY-023"]
    assert raw < 0 and t23["0"] == three and t23["2"] == t23["4"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
