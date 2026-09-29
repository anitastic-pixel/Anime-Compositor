"""HSV key, worked a second way.

D-184 adds `core.hsv_key`. It makes transparent the pixels whose hue, saturation and value all
fall inside chosen windows, as a green screen is taken out from behind a figure, or, with
`invert` on, every pixel that does not. It is OpenToonz's HSV Key, `toonz/sources/stdfx/
hsvkeyfx.cpp` with the colour conversion `OLDRGB2HSV` from `toonz/sources/toonzlib/
hsvutil.cpp`, with D-184's changes marked below. Document 21 is the rule in words; this file is
the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

The rule below is ported from OpenToonz, commit 6571328019e3a6a99c13f408ee6f4755c9cddf73, under
its licence, kept in `docs/third_party/OpenToonz-LICENSE.txt`:

    Copyright (c) 2016 - 2026, DWANGO Co., Ltd.
    Copyright (c) 2016 - 2026, the respective contributors.
    All rights reserved. BSD 3-Clause "New" or "Revised" License; the full text, its conditions
    and its disclaimer are in the file named above.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: colour key's figure in front of a green screen, drawn in
`tools/color_key_reference.py`. The drawing goes into `Fixtures/hsv_key/media`, the projects
into `Fixtures/hsv_key`, and the expected frames into `Fixtures/hsv_key/expected_hsv_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/hsv_key_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import color_key_reference as K  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "hsv_key"
TOLERANCE = 2e-5  # document 25's default for a filter
TOP = {"hue": 360, "hue_range": 180, "saturation": 100, "saturation_range": 100,
       "value": 100, "value_range": 100}
NUMBERS = list(TOP)


# --- the rule -------------------------------------------------------------------------------

def hsv(q):
    """OpenToonz's OLDRGB2HSV on 8-bit red, green and blue over 255: hue in degrees, saturation
    and value 0 to 1. A grey has saturation 0 and hue 0. Of equal largest channels, red counts
    before green and green before blue."""
    r, g, b = (v / 255 for v in q)
    hi, lo = max(r, g, b), min(r, g, b)
    v = hi
    s = (hi - lo) / hi if hi != 0 else 0.0
    if s == 0:
        return 0.0, s, v
    d = hi - lo
    if r == hi:
        h = (g - b) / d
    elif g == hi:
        h = 2 + (b - r) / d
    else:
        h = 4 + (r - g) / d
    h *= 60
    if h < 0:
        h += 360
    return h, s, v


def margins(q, k):
    """How far inside each window the colour is, hue, saturation and value; at or above 0 is in.
    D-184's change: the hue's distance is the short way round the circle, where OpenToonz cuts
    its window at 0 and 360."""
    h, s, v = hsv(q)
    d = abs(h - k["hue"])
    return (k["hue_range"] - min(d, 360 - d),
            k["saturation_range"] - abs(100 * s - k["saturation"]),
            k["value_range"] - abs(100 * v - k["value"]))


def matches(q, k):
    return all(m >= 0 for m in margins(q, k))


def hsv_key(pixels, k):
    """D-184's change: the colour is the pixel's straight colour, as a drawing program stores
    it, where OpenToonz reads premultiplied values; a pixel that does not show stays."""
    out = []
    for p in pixels:
        w = R.working(p)
        if p[3] > 0 and matches(p[:3], k) != (k["invert"] == "on"):
            w = [0.0] * 4
        out.append(w)
    return out


# --- the cases ------------------------------------------------------------------------------

def case(hue=120, hue_range=40, saturation=60, saturation_range=40, value=60, value_range=40,
         invert="off", shift=0):
    """The defaults are D-184's green key, the one it is added with."""
    return {"drawing": "screen", "hue": hue, "hue_range": hue_range, "saturation": saturation,
            "saturation_range": saturation_range, "value": value, "value_range": value_range,
            "invert": invert, "shift": shift}


def held(c, frame_no):
    k = {n: min(TOP[n], max(0.0, value_at(c[n], frame_no))) for n in NUMBERS}
    k["invert"] = c["invert"]
    return k


def render(c, frame_no):
    pixels = [p for row in K.DRAWINGS[c["drawing"]] for p in row]
    return R.frame(hsv_key(pixels, held(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    pixels = [p for row in K.DRAWINGS[c["drawing"]] for p in row]
    return R.frame([R.working(p) for p in pixels], c["shift"])


SCREEN = dict(hue=142, hue_range=5, saturation=100, saturation_range=10, value=50,
              value_range=50)

CASES = {
    "FX-HSV-001": ("As it is added, a green key: hue 120 within 40, saturation 60 within 40, "
                   "value 60 within 40. The screen, its half-covering right edge, the shadow and "
                   "the green spill down the figure's left side go; the skin, the line and the "
                   "grey button stay.",
                   case(), [0]),
    "FX-HSV-002": ("Hue 142 within 5, saturation 100 within 10, value 50 within 50: the screen, "
                   "its edge and the shadow go; the spill, 13.4 degrees of hue away, stays.",
                   case(**SCREEN), [0]),
    "FX-HSV-003": ("FX-HSV-002 with value 70 within 5: the screen, value 69.4, goes; the shadow, "
                   "value 39.2, stays.",
                   case(**{**SCREEN, "value": 70, "value_range": 5}), [0]),
    "FX-HSV-004": ("FX-HSV-002 inverted: the screen, its edge and the shadow stay, and "
                   "everything else that shows goes, the figure and the spill.",
                   case(**SCREEN, invert="on"), [0]),
    "FX-HSV-005": ("Hue 350 within 40, saturation 55 within 50, value 50 within 50: the skin, "
                   "hue 25.7, is 35.7 degrees away the short way round, past 360, and goes. The "
                   "grey button's hue counts as 0, but its saturation 0 is 55 from 55; it "
                   "stays, and so does everything else.",
                   case(hue=350, saturation=55, saturation_range=50, value=50, value_range=50),
                   [0]),
    "FX-HSV-006": ("Hue 0 within 0, saturation 0 within 0, value 50 within 5: only the grey "
                   "button, value 50.2, goes.",
                   case(hue=0, hue_range=0, saturation=0, saturation_range=0, value=50,
                        value_range=5), [0]),
    "FX-HSV-007": ("Hue within 180, saturation 50 within 50, value 50 within 50: every colour "
                   "is inside, so the frame is empty.",
                   case(hue=0, hue_range=180, saturation=50, saturation_range=50, value=50,
                        value_range=50), [0]),
    "FX-HSV-008": ("OpenToonz's own starting values, everything 0: only pure black would go, and "
                   "the drawing has none that shows, so it is untouched.",
                   case(hue=0, hue_range=0, saturation=0, saturation_range=0, value=0,
                        value_range=0), [0]),
    "FX-HSV-009": ("Hue 142, saturation 70 within 30, value 50 within 50, hue range keyed from 0 "
                   "at frame 0 to 20 at frame 4, linear: frame 0 takes nothing, the screen being "
                   "0.31 degrees off; frame 2, range 10, takes the screen, its edge and the "
                   "shadow; frame 4, range 20, the spill too.",
                   case(hue=142, hue_range=keyed((0, 0), (4, 20)), saturation=70,
                        saturation_range=30, value=50, value_range=50), [0, 2, 4]),
    "FX-HSV-010": ("FX-HSV-002 moved three pixels right: the same, moved.",
                   case(**SCREEN, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-HSV-011": ("Hue 361, above 360.", case(hue=361)),
    "FX-HSV-012": ("Hue range 181, above 180.", case(hue_range=181)),
    "FX-HSV-013": ("Saturation 101, above 100.", case(saturation=101)),
    "FX-HSV-014": ("Value range -1, below 0.", case(value_range=-1)),
    "FX-HSV-015": ("Invert \"yes\", which is not \"off\" or \"on\".", case(invert="yes")),
    "FX-HSV-016": ("Hue range keyed to 200 at frame 4.",
                   case(hue_range=keyed((0, 40), (4, 200)))),
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
        "instance_id": "fx-0-0", "type_id": "core.hsv_key", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NUMBERS + ["invert"]}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in K.DRAWINGS.items():
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

    (OUT / "expected_hsv_key.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    empty = [[0.0] * 4] * (W * H)
    at = lambda x, y: y * W + x  # noqa: E731
    gone = lambda f, x, y: f[at(x, y)] == [0.0] * 4  # noqa: E731
    same = lambda f, x, y: f[at(x, y)] == drawn[at(x, y)]  # noqa: E731
    screen, shadow, spill, skin, line, button, edge = (1, 1), (3, 9), (4, 4), (6, 3), (5, 3), \
        (7, 4), (15, 4)
    figure = (spill, skin, line, button)

    # The conversion, against values worked by hand.
    close = lambda a, b: all(abs(u - v) < 1e-3 for u, v in zip(a, b))  # noqa: E731
    assert close(hsv(K.GREEN[:3]), (141.695, 1, 0.694))
    assert close(hsv(K.SPILL[:3]), (128.571, 0.4375, 0.627))
    assert close(hsv(K.SKIN[:3]), (25.714, 0.2276, 0.965))
    assert close(hsv(K.LINE[:3]), (264, 0.2778, 0.141))
    assert hsv(K.GREY[:3])[:2] == (0.0, 0.0) and hsv((0, 0, 0)) == (0.0, 0.0, 0.0)
    assert hsv((255, 0, 0))[0] == 0 and 359 < hsv((255, 0, 1))[0] < 360
    assert hsv((0, 0, 255))[0] == 240

    # No colour in any case sits within 1e-6 of a window's edge, so the build's arithmetic
    # cannot tip a pixel across one, except exactly on it: a grey's hue and saturation are
    # exactly 0 and a colour with a channel at 0 has saturation exactly 100, in any precision,
    # and the settings are whole numbers. Those count as inside.
    colours = {p[:3] for row in K.DRAWINGS["screen"] for p in row if p[3] > 0}
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            k = held(cs, f)
            for q in colours:
                hm, sm, vm = margins(q, k)
                assert abs(hm) > 1e-6 or (hm == 0 and q == K.GREY[:3]), (fx, f, q, hm)
                assert abs(sm) > 1e-6 or (sm == 0 and (q == K.GREY[:3] or min(q) == 0)), \
                    (fx, f, q, sm)
                assert abs(vm) > 1e-6, (fx, f, q, vm)

    one = c["FX-HSV-001"]["0"]
    assert all(gone(one, *p) for p in (screen, edge, shadow, spill))
    assert all(same(one, *p) for p in (skin, line, button)) and same(one, 0, 0)
    two = c["FX-HSV-002"]["0"]
    assert all(gone(two, *p) for p in (screen, edge, shadow))
    assert all(same(two, *p) for p in figure)
    three = c["FX-HSV-003"]["0"]
    assert gone(three, *screen) and gone(three, *edge) and same(three, *shadow)
    four = c["FX-HSV-004"]["0"]
    assert all(same(four, *p) for p in (screen, edge, shadow)) and same(four, 0, 0)
    assert all(gone(four, *p) for p in figure)
    five = c["FX-HSV-005"]["0"]
    assert sum(five[i] != drawn[i] for i in range(W * H)) == 15 and gone(five, *skin)
    six = c["FX-HSV-006"]["0"]
    assert sum(six[i] != drawn[i] for i in range(W * H)) == 1 and gone(six, *button)
    seven = c["FX-HSV-007"]["0"]
    assert seven == empty
    assert c["FX-HSV-008"]["0"] == drawn
    nine = c["FX-HSV-009"]
    assert nine["0"] == drawn
    assert all(gone(nine["2"], *p) for p in (screen, edge, shadow)) and same(nine["2"], *spill)
    assert gone(nine["4"], *spill) and same(nine["4"], *skin) and same(nine["4"], *button)
    moved = c["FX-HSV-010"]["0"]
    assert moved == c["FX-HSV-010"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
