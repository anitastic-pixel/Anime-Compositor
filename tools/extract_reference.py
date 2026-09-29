"""Extract, worked a second way.

D-212 adds `core.extract`. It keys a layer out by the brightness of one of its channels, as a
white paper background is taken out from behind a scanned drawing, or a black one from behind a
flash of light: every shown pixel whose value in `channel` lies between `black_point` and
`white_point` is kept, and every other one turns transparent, with `black_softness` and
`white_softness` fading it in over that many steps inside each point instead of all at once.
`invert` keeps the other side instead. It is After Effects' Extract in purpose and names, and
this program's own rule. Nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour as Threshold (D-138) takes it, the value v, 0 to 255, is by `channel`:
"luminance", 255 Y(e) with Y(e) = 0.2126 e_r + 0.7152 e_g + 0.0722 e_b, Threshold's luma;
"red", "green" or "blue", 255 times that channel of e; "alpha", 255 a. With b the black point,
w the white point, s the black softness and t the white softness:

    low  = clamp((v - b) / s, 0, 1) when s > 0, else 1 when v + 1e-4 >= b and 0 otherwise
    high = clamp((w - v) / t, 0, 1) when t > 0, else 1 when v - 1e-4 <= w and 0 otherwise
    m = low * high, or 1 - low * high with `invert` "on"

and the output is the pixel with all four channels times m. The 1e-4 keeps an 8-bit value
sitting exactly on a point, the same way in the build's single precision as here, as
Threshold's does. A pixel with a == 0 is left as it is. The layer does not grow, and a draft
changes nothing, as no setting is a distance. A black point above the white point keeps
nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Threshold's bands of colour from black to white (`tools/threshold_reference.py`). The
drawing goes into `Fixtures/extract/media`, the projects into `Fixtures/extract`, and the
expected frames into `Fixtures/extract/expected_extract.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/extract_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import threshold_reference as T  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "extract"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"black_point": (0, 255), "white_point": (0, 255), "black_softness": (0, 255),
          "white_softness": (0, 255)}
WORDS = ("channel", "invert")
NAMES = ("channel", "black_point", "white_point", "black_softness", "white_softness", "invert")
NUDGE = T.NUDGE  # an 8-bit value exactly on a point is kept
CLIFF = T.CLIFF  # no pixel decided by a hard point may sit closer than this to it


# --- the rule -------------------------------------------------------------------------------

def value(w, channel):
    """The pixel's value, 0 to 255, in `channel`."""
    if channel == "alpha":
        return 255 * w[3]
    e = T.encoded(w)
    if channel == "luminance":
        return T.luma255(e)
    return 255 * e[("red", "green", "blue").index(channel)]


def kept(v, n, invert):
    b, w, s, t = (n[k] for k in RANGES)
    low = min(1, max(0, (v - b) / s)) if s > 0 else (1 if v + NUDGE >= b else 0)
    high = min(1, max(0, (w - v) / t)) if t > 0 else (1 if v - NUDGE <= w else 0)
    return 1 - low * high if invert == "on" else low * high


def extract(pixels, n, c):
    out = []
    for p in pixels:
        w = R.working(p)
        if w[3] > 0:
            m = kept(value(w, c["channel"]), n, c["invert"])
            w = [u * m for u in w]
        out.append(w)
    return out


# --- the cases ------------------------------------------------------------------------------

def case(channel="luminance", black_point=0, white_point=255, black_softness=0,
         white_softness=0, invert="off", shift=0):
    return {"drawing": "tones", "channel": channel, "black_point": black_point,
            "white_point": white_point, "black_softness": black_softness,
            "white_softness": white_softness, "invert": invert, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def numbers(c, frame_no):
    return {k: held(c, k, frame_no) for k in RANGES}


def render(c, frame_no):
    pixels = [p for row in T.DRAWINGS[c["drawing"]] for p in row]
    return R.frame(extract(pixels, numbers(c, frame_no), c), c["shift"])


def plain(c):
    return R.frame([R.working(p) for row in T.DRAWINGS[c["drawing"]] for p in row], c["shift"])


CASES = {
    "FX-EXTRACT-001": ("As it starts: luminance, black point 0, white point 255, no softness: "
                       "every pixel is kept, black and white too, and the frame is the "
                       "drawing.", case(), [0]),
    "FX-EXTRACT-002": ("Black point 60: black, blue, the line, red and the soft line, all "
                       "darker, turn transparent; the dark grey at 64 is kept.",
                       case(black_point=60), [0]),
    "FX-EXTRACT-003": ("White point 190: the light grey, the skin, white and the soft skin, all "
                       "brighter, turn transparent; green at 182.4 is kept.",
                       case(white_point=190), [0]),
    "FX-EXTRACT-004": ("Black point 60 and white point 190: only the middle band is kept, from "
                       "the dark grey to green.", case(black_point=60, white_point=190), [0]),
    "FX-EXTRACT-005": ("Black point 128: grey 128, exactly on the point, is kept; grey 127 "
                       "goes.", case(black_point=128), [0]),
    "FX-EXTRACT-006": ("Black point 20 with black softness 100: blue, below 20, goes; the "
                       "pixels from 20 to 120 fade in, the line faintly, red and the dark grey "
                       "more; grey 127 and above are kept whole.",
                       case(black_point=20, black_softness=100), [0]),
    "FX-EXTRACT-007": ("White softness 60: the pixels from 195 to 255 fade out, the skin at 219 "
                       "part way, white wholly; the light grey at 192 is kept whole.",
                       case(white_softness=60), [0]),
    "FX-EXTRACT-008": ("Invert on, black point 60: the other side: black, blue, the line, red "
                       "and the soft line are kept, and every brighter pixel goes.",
                       case(black_point=60, invert="on"), [0]),
    "FX-EXTRACT-009": ("FX-EXTRACT-006 inverted: each fading pixel keeps what 006 took, so the "
                       "two add up to the drawing.",
                       case(black_point=20, black_softness=100, invert="on"), [0]),
    "FX-EXTRACT-010": ("Channel red, black point 128: red, the skins and grey 128 are kept, "
                       "blue, green, the line and grey 127 go, by their red alone.",
                       case(channel="red", black_point=128), [0]),
    "FX-EXTRACT-011": ("Channel green, black point 128: green, the skins and grey 128 are kept, "
                       "red and blue go.", case(channel="green", black_point=128), [0]),
    "FX-EXTRACT-012": ("Channel blue, white point 100: blue, the skins, the greys from 127 and "
                       "white go; black, red, green, the line and the dark grey are kept.",
                       case(channel="blue", white_point=100), [0]),
    "FX-EXTRACT-013": ("Channel alpha, black point 200: the two half-covered columns go, every "
                       "fully covered pixel is kept whatever its colour.",
                       case(channel="alpha", black_point=200), [0]),
    "FX-EXTRACT-014": ("Black point 200 above white point 100: nothing is kept.",
                       case(black_point=200, white_point=100), [0]),
    "FX-EXTRACT-015": ("Black point keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 "
                       "is FX-EXTRACT-001, frame 2 at 127.5 keeps grey 128 and loses grey 127, "
                       "and at frame 4 only white is left.",
                       case(black_point=keyed((0, 0), (4, 255))), [0, 2, 4]),
    "FX-EXTRACT-016": ("White point eased from 255 at frame 0 to 0 at frame 4 on a curve that "
                       "overshoots: at frame 2 it has gone below 0 and is held there, so frames "
                       "2 and 4 keep only black.",
                       case(white_point=keyed((0, 255, OVERSHOOT), (4, 0))), [0, 2, 4]),
    "FX-EXTRACT-017": ("FX-EXTRACT-004 moved three pixels right: the same, moved.",
                       case(black_point=60, white_point=190, shift=3), [0]),
}

INVALID = {
    "FX-EXTRACT-018": ("Black point -1, below 0.", case(black_point=-1)),
    "FX-EXTRACT-019": ("Black point 256, above 255.", case(black_point=256)),
    "FX-EXTRACT-020": ("White point 256, above 255.", case(white_point=256)),
    "FX-EXTRACT-021": ("Black softness 256, above 255.", case(black_softness=256)),
    "FX-EXTRACT-022": ("White softness -1, below 0.", case(white_softness=-1)),
    "FX-EXTRACT-023": ("Channel \"luma\", which is not one of the five.",
                       case(channel="luma")),
    "FX-EXTRACT-024": ("Channel \"Red\", written with a capital.", case(channel="Red")),
    "FX-EXTRACT-025": ("Invert \"yes\", which is not \"on\" or \"off\".", case(invert="yes")),
    "FX-EXTRACT-026": ("Black point keyed to 300 at frame 4.",
                       case(black_point=keyed((0, 0), (4, 300)))),
}

# D-215, accepted on 2026-09-29: a number written as a word is a fault in the file's shape, as
# D-164 settled, and the whole file is refused on opening.
REFUSED = {
    "FX-EXTRACT-027": ("Black point written \"60\", a word, not a number. The file is refused "
                       "on opening, as a fault in its shape (D-215, as D-164).",
                       case(black_point="60")),
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
        "instance_id": "fx-0-0", "type_id": "core.extract", "enabled": True,
        "parameters": {k: (c[k] if k in WORDS else setting_json(c[k])) for k in NAMES}}]
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
    for fx, (says, c) in REFUSED.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "refused": "PROJECT_SCHEMA_INVALID"}
        print(f"{fx}: refused")
    (OUT / "expected_extract.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items() if "frames" in v}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    col = {q: x for x, q in enumerate(T.BANDS) if q != T.NONE}
    every = set(col)

    def share(f, q):
        """How much of column q's pixel frame f keeps, 0 to 1, on row 4."""
        p, d = f[at(col[q], 4)], drawn[at(col[q], 4)]
        m = p[3] / d[3]
        assert all(abs(p[i] - d[i] * m) < 1e-12 for i in range(4)), q  # all four, times m
        return m

    def whole(f):
        return {q for q in col if share(f, q) > 1 - 1e-9}

    def gone(f):
        return {q for q in col if share(f, q) < 1e-9}  # white at 255 - 1e-14 fades to 1e-15

    # Every case keeps the empty pixels empty; every drawn row is the same; a hard point never
    # decides a pixel within 1e-5 of it.
    for fx, frames in c.items():
        cc = (CASES.get(fx) or INVALID.get(fx))[1]
        for f, px in frames.items():
            base = plain(cc)
            for i in range(W * H):
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
            if fx in CASES:
                n = numbers(cc, int(f))
                for i in range(W * H):
                    if base[i][3] > 0:
                        v = value(base[i], cc["channel"])
                        if n["black_softness"] == 0:
                            assert abs(v + NUDGE - n["black_point"]) >= CLIFF, (fx, f, i)
                        if n["white_softness"] == 0:
                            assert abs(v - NUDGE - n["white_point"]) >= CLIFF, (fx, f, i)
                if cc["shift"] == 0:
                    for x in range(W):
                        assert all(px[at(x, y)] == px[at(x, 4)] for y in range(1, H - 1))

    B = T
    assert c["FX-EXTRACT-001"]["0"] == drawn
    two = c["FX-EXTRACT-002"]["0"]
    assert gone(two) == {B.BLACK, B.BLUE, B.LINE, B.RED, B.SOFT_LINE} and whole(two) == every - gone(two)
    three = c["FX-EXTRACT-003"]["0"]
    assert gone(three) == {B.LIGHT, B.SKIN, B.WHITE, B.SOFT_SKIN} and B.GREEN in whole(three)
    four = c["FX-EXTRACT-004"]["0"]
    assert whole(four) == {B.DARK, B.DIM, B.GREY, B.SHADOW, B.GREEN} and gone(four) == every - whole(four)
    five = c["FX-EXTRACT-005"]["0"]
    assert B.GREY in whole(five) and B.DIM in gone(five)
    six = c["FX-EXTRACT-006"]["0"]
    assert gone(six) == {B.BLACK, B.BLUE}
    assert 0 < share(six, B.LINE) < share(six, B.RED) < share(six, B.DARK) < 1
    assert abs(share(six, B.LINE) - (27.5724 - 20) / 100) < 1e-5
    assert whole(six) == {B.DIM, B.GREY, B.SHADOW, B.GREEN, B.LIGHT, B.SKIN, B.WHITE, B.SOFT_SKIN}
    seven = c["FX-EXTRACT-007"]["0"]
    assert B.WHITE in gone(seven) and B.LIGHT in whole(seven)
    assert abs(share(seven, B.SKIN) - (255 - 219.0704) / 60) < 1e-5
    assert share(seven, B.SOFT_SKIN) == share(seven, B.SKIN)
    eight = c["FX-EXTRACT-008"]["0"]
    assert whole(eight) == gone(two) and gone(eight) == whole(two)
    nine = c["FX-EXTRACT-009"]["0"]
    assert all(abs(p[i] + q[i] - d[i]) < 1e-12 for p, q, d in zip(six, nine, drawn) for i in range(4))
    ten = c["FX-EXTRACT-010"]["0"]
    assert whole(ten) == {B.RED, B.GREY, B.SHADOW, B.LIGHT, B.SKIN, B.WHITE, B.SOFT_SKIN}
    eleven = c["FX-EXTRACT-011"]["0"]
    assert {B.GREEN, B.SHADOW, B.SKIN, B.GREY} <= whole(eleven) and {B.RED, B.BLUE} <= gone(eleven)
    twelve = c["FX-EXTRACT-012"]["0"]
    assert whole(twelve) == {B.BLACK, B.RED, B.GREEN, B.LINE, B.DARK, B.SOFT_LINE}
    thirteen = c["FX-EXTRACT-013"]["0"]
    assert gone(thirteen) == {B.SOFT_SKIN, B.SOFT_LINE} and whole(thirteen) == every - gone(thirteen)
    assert gone(c["FX-EXTRACT-014"]["0"]) == every
    fifteen = c["FX-EXTRACT-015"]
    assert fifteen["0"] == drawn and whole(fifteen["4"]) == {B.WHITE}
    assert B.GREY in whole(fifteen["2"]) and B.DIM in gone(fifteen["2"])
    sixteen = c["FX-EXTRACT-016"]
    assert sixteen["0"] == drawn and sixteen["2"] == sixteen["4"] and whole(sixteen["4"]) == {B.BLACK}
    moved = c["FX-EXTRACT-017"]["0"]
    assert all(moved[at(x, y)] == four[at(x - 3, y)] for x in range(3, W) for y in range(H))
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
