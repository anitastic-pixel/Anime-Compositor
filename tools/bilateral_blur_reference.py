"""Bilateral Blur, worked a second way.

D-358 adds an effect modelled on After Effects' Bilateral Blur, by a rule of our own; Adobe does
not publish theirs and nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

  `core.bilateral_blur`: Radius (0 to 50 pixels), Threshold (0 to 255 steps of the sRGB curve),
  Colorize ("on" or "off").

Each pixel that shows looks at the disc round it, the taps (dx, dy) with dx^2 + dy^2 <= Radius^2,
itself among them. A tap outside the layer, or one that does not show, is left out. A pixel that
does not show stays transparent, and every pixel keeps its own covering: nothing grows, and the
drawing's outline is kept.

Each tap's weight is the product of three:

  - its distance, a bell exp(-(dx^2 + dy^2) / (2 s^2)) with s = Radius / 2;
  - its covering a, so a faint edge pixel counts for little;
  - its likeness, a bell exp(-(e_tap - e_own)^2 / (2 Threshold^2)), where e is a value through
    the sRGB curve on a 0 to 255 scale: like colours count fully, colours a Threshold apart about
    six tenths, three Thresholds apart almost nothing. So flat colour and grain are smoothed and
    an edge much wider than Threshold is kept.

With Colorize on each of red, green and blue is worked alone: its own likeness from that
channel's e, and the channel's new straight linear value the weighted mean of the taps' straight
linear values of it. With Colorize off the effect works one value, the pixel's luminance in linear
light, Y = 0.2126 R + 0.7152 G + 0.0722 B of its straight colour, its likeness from e of Y, and
the pixel becomes grey: red, green and blue each the weighted mean of the taps' Y. The pixel
premultiplied again by its own covering is the result.

Radius below 1 leaves only the pixel itself, and Threshold 0 counts only the pixel itself (the
limit of the bell): with Colorize on the layer is then exactly as it was, with Colorize off it is
its own luminance, grey.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: D-203's skin with specks, a hole, a line two pixels wide, a patch of grain and a column
at half covering (`tools/median_smart_blur_reference.py`). The drawing goes into
`Fixtures/bilateral_blur/media`, the projects into `Fixtures/bilateral_blur`, and the expected
frames into `Fixtures/bilateral_blur/expected_bilateral_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bilateral_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import median_smart_blur_reference as M  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bilateral_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
RADIUS = (0, 50)
THRESHOLD = (0, 255)
EMPTY = [0.0] * 4
LUMA = (0.2126, 0.7152, 0.0722)


# --- the rule -------------------------------------------------------------------------------

def disc(radius):
    r = math.floor(radius)
    return [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
            if dx * dx + dy * dy <= radius * radius]


def e(v):
    """A straight linear value through the sRGB curve, 0 to 255."""
    return 255 * S.linear_to_srgb(v)


def bilateral_blur(layer, radius, threshold, colorize):
    w, h = layer["w"], layer["h"]
    straight = [None if p[3] <= 0 else [p[i] / p[3] for i in range(3)] for p in layer["px"]]
    # The values worked: three channels with Colorize on, the one luminance with it off.
    vals = [None if c is None else (c if colorize == "on" else [sum(k * v for k, v in zip(LUMA, c))])
            for c in straight]
    taps = disc(radius)
    s = radius / 2
    px = []
    for y in range(h):
        for x in range(w):
            i = y * w + x
            a = layer["px"][i][3]
            if a <= 0:
                px.append(EMPTY)
                continue
            own = vals[i]
            out = []
            for ch in range(len(own)):
                if len(taps) == 1 or threshold == 0:
                    out.append(own[ch])
                    continue
                num = den = 0.0
                for dx, dy in taps:
                    tx, ty = x + dx, y + dy
                    if not (0 <= tx < w and 0 <= ty < h):
                        continue
                    j = ty * w + tx
                    if vals[j] is None:
                        continue
                    d = e(vals[j][ch]) - e(own[ch])
                    wt = (math.exp(-(dx * dx + dy * dy) / (2 * s * s)) * layer["px"][j][3]
                          * math.exp(-d * d / (2 * threshold * threshold)))
                    num += wt * vals[j][ch]
                    den += wt
                out.append(num / den)
            if len(out) == 1:
                out = out * 3
            px.append([v * a for v in out] + [a])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(radius=5, threshold=20, colorize="on", shift=0):
    return {"type": "core.bilateral_blur", "drawing": "specks", "shift": shift,
            "parameters": {"radius": radius, "threshold": threshold, "colorize": colorize}}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    p = c["parameters"]
    layer = M.drawn_layer(c["drawing"])
    layer = bilateral_blur(layer, held(value_at(p["radius"], frame_no), RADIUS),
                           held(value_at(p["threshold"], frame_no), THRESHOLD), p["colorize"])
    return frame(layer, c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(M.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-BILAT-001": ("Bilateral Blur as it starts, Radius 5, Threshold 20, Colorize on: the "
                     "grain, 8 levels either side of the skin, is smoothed toward the skin, while "
                     "the line and the dark specks, far more than 20 from the skin, stay as they "
                     "are to well within a level; the white speck, 41 and 65 from the skin in "
                     "green and blue, stays far lighter than the skin. The hole stays a hole and "
                     "the half-covered column keeps its half covering.",
                     case(), [0]),
    "FX-BILAT-002": ("Radius 0: the drawing, untouched.", case(0), [0]),
    "FX-BILAT-003": ("Threshold 0: each pixel counts only itself, so the drawing is unchanged.",
                     case(5, 0), [0]),
    "FX-BILAT-004": ("Colorize off: the same smoothing worked on luminance alone, and every "
                     "pixel that shows is grey, red, green and blue equal; the line stays dark "
                     "and the outline is kept.",
                     case(5, 20, "off"), [0]),
    "FX-BILAT-005": ("Colorize off with Radius 0: no smoothing, each pixel only its own "
                     "luminance, grey.",
                     case(0, 20, "off"), [0]),
    "FX-BILAT-006": ("Threshold 255: likeness hardly matters, a soft blur that stays inside the "
                     "drawing: the line and the specks are blurred into the skin, and the hole, "
                     "the empty column and the empty row stay empty.",
                     case(5, 255), [0]),
    "FX-BILAT-007": ("Radius 1.5, a disc of nine: a lighter smoothing of the grain than "
                     "FX-BILAT-001.", case(1.5), [0]),
    "FX-BILAT-008": ("Radius keyed from 0 at frame 0 to 10 at frame 4, linear: frame 0 is the "
                     "drawing and frame 2 is FX-BILAT-001.",
                     case(keyed((0, 0), (4, 10))), [0, 2, 4]),
    "FX-BILAT-009": ("Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is "
                     "the drawing and frame 2 is FX-BILAT-001.",
                     case(5, keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-BILAT-010": ("Radius keyed from 0 at frame 0 to 50 at frame 4, eased past its end "
                     "(about 64 at frame 2): frame 2 is held at 50, the same as frame 4.",
                     case(keyed((0, 0, OVERSHOOT), (4, 50))), [0, 2, 4]),
    "FX-BILAT-011": ("Bilateral Blur as it starts, the layer moved three pixels right: "
                     "FX-BILAT-001 moved with it.", case(shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BILAT-012": ("Bilateral Blur, Radius 51, above 50.", case(51)),
    "FX-BILAT-013": ("Bilateral Blur, Radius -1, below 0.", case(-1)),
    "FX-BILAT-014": ("Bilateral Blur, Threshold 256, above 255.", case(5, 256)),
    "FX-BILAT-015": ("Bilateral Blur, Threshold -1, below 0.", case(5, -1)),
    "FX-BILAT-016": ("Bilateral Blur, Threshold keyed to 300 at frame 4.",
                     case(5, keyed((0, 0), (4, 300)))),
    "FX-BILAT-017": ("Bilateral Blur, Colorize \"sometimes\", which is not one.",
                     case(5, 20, "sometimes")),
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
        "instance_id": "fx-0-0", "type_id": c["type"], "enabled": True,
        "parameters": {k: v if isinstance(v, str) else setting_json(v)
                       for k, v in c["parameters"].items()}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in M.DRAWINGS.items():
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

    (OUT / "expected_bilateral_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, eps=1e-9: all(abs(a - b) < eps for a, b in zip(p, q))  # noqa: E731
    look = lambda ch: R.working(M.PALETTE[ch])  # noqa: E731
    skin, soft = look("s"), look("h")
    dark_specks = ((2, 1), (12, 4))
    grain = [(x, y) for x in range(8) for y in range(5, 9) if M.MAP[y][x] in "+-"]
    lines = [(x, y) for x in (9, 10) for y in range(9)]
    level = 1 / 255  # far less than a level once through the curve, for these dark values

    assert len(disc(0)) == 1 and len(disc(0.99)) == 1 and len(disc(1.5)) == 9
    assert abs(e(1.0) - 255) < 1e-9 and e(0.0) == 0

    one = c["FX-BILAT-001"]["0"]
    assert all(near(one[at(x, y)], drawn[at(x, y)], level / 10) for x, y in dark_specks + tuple(lines))
    assert one[at(12, 1)] == EMPTY
    assert all(abs(one[at(14, y)][3] - soft[3]) < 1e-12 for y in range(9))
    assert all(one[at(x, 9)] == EMPTY for x in range(W)) and all(one[at(15, y)] == EMPTY for y in range(H))
    spread = lambda px: max(abs(px[at(x, y)][2] - skin[2]) for x, y in grain)  # noqa: E731
    assert spread(one) < spread(drawn) / 2
    assert one[at(6, 2)][2] > skin[2] + 0.1  # the white speck stays light
    assert c["FX-BILAT-002"]["0"] == drawn
    assert c["FX-BILAT-003"]["0"] == drawn
    four = c["FX-BILAT-004"]["0"]
    for i, p in enumerate(four):
        assert p[3] == drawn[i][3] and abs(p[0] - p[1]) < 1e-15 and abs(p[1] - p[2]) < 1e-15
    assert all(four[at(x, y)][0] < 0.05 for x, y in lines)
    five = c["FX-BILAT-005"]["0"]
    for i, p in enumerate(five):
        d = drawn[i]
        y = sum(k * v for k, v in zip(LUMA, d[:3]))
        assert near(p, [y, y, y, d[3]], 1e-12)
    six = c["FX-BILAT-006"]["0"]
    assert all(not near(six[at(x, y)], drawn[at(x, y)], 1e-3) for x, y in dark_specks + tuple(lines))
    assert six[at(12, 1)] == EMPTY and all(six[at(x, 9)] == EMPTY for x in range(W))
    seven = c["FX-BILAT-007"]["0"]
    assert spread(one) < spread(seven) < spread(drawn)
    eight = c["FX-BILAT-008"]
    assert eight["0"] == drawn and eight["2"] == one and eight["4"] not in (drawn, one)
    nine = c["FX-BILAT-009"]
    assert nine["0"] == drawn and nine["2"] == one
    ten = c["FX-BILAT-010"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 50)), 2) > 50
    assert ten["0"] == drawn and ten["2"] == ten["4"] != drawn
    eleven = c["FX-BILAT-011"]["0"]
    assert all(eleven[at(x + 3, y)] == one[at(x, y)] for x in range(W - 3) for y in range(H))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
