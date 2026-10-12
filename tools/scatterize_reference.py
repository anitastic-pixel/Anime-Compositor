"""Scatterize, worked a second way.

D-676 adds Scatterize (`core.scatterize`), after CycoreFX's CC Scatterize (Simulation): the layer
blown apart into dust, every pixel a dot thrown away from where it was, and the layer twisted
like a ribbon about its middle row. The CycoreFX manual names the controls and says what each is
for (Amount, how far the pixels fly in every direction, below 0 the other way; Right Twist and
Left Twist, any number of turns either way; the Transfer Mode, Composite, Screen, Add or Alpha
Add). It publishes no formula, so the rule, the units, the ranges and the settings when added are
ours. Nothing moves by itself: a frame is the settings as they are at that frame (document 20).

On a drawing w by h, with the settings held inside their ranges:

1. Every pixel (i, j) of the drawing whose alpha is above 0 is a dot of its own premultiplied
   colour, starting at its centre (x, y) = (i + 1/2, j + 1/2).
2. The twist: the dot's column turns about the middle row by a = L + (R - L) x / w degrees, L =
   `left_twist`, R = `right_twist`, so the left edge turns by L, the right edge by R and the
   columns between by a share of each. Seen straight on, the dot is then at y' = h / 2 + (y - h / 2)
   cos a and depth z = (y - h / 2) sin a (above 0 nearer the viewer).
3. The scatter: u_k = (U(0, i, j, 0, k) + 1) / 2, U Noise's hash (D-119) at seed 0; the dot moves
   by D u_0 (cos 2 pi u_1, sin 2 pi u_1), D = `amount` h / 1080, so Amount is the farthest a dot
   flies in pixels on a picture 1080 high, and a negative Amount throws every dot the opposite
   way. The dot ends at P = (x + D u_0 cos 2 pi u_1, y' + D u_0 sin 2 pi u_1).
4. The dots are laid onto a clear layer farthest first: by z, then by j w + i. A dot covers the
   pixel whose centre is e from P by c = clamp(1 - e, 0, 1), so with nothing moved each dot covers
   its own pixel exactly. With s its colour and alpha times c, each pixel d becomes, by the
   Transfer Mode: `composite` s + d (1 - s_alpha), every channel; `screen` s + d - s d, every
   channel; `add` the colours s + d, the alpha s_alpha + d_alpha (1 - s_alpha); `alpha_add` the
   colours s + d, the alpha min(1, s_alpha + d_alpha).

`amount` -10000 to 10000, 50 when added; `right_twist` and `left_twist` -36000 to 36000 degrees,
0; `transfer_mode` `composite`, `screen`, `add` or `alpha_add`, composite. Every number keyable,
read as it is at the frame. The layer does not grow: dots thrown past its edge are cut there.
The dots are one pixel of the drawing each at any resolution, so a draft throws a quarter as
many dots, each a draft pixel.

**This file never runs the build's code path.** It throws every dot and lays it onto the pixels
round it, one dot after another, where the build bins the dots into tiles and works a pixel at a
time, in double precision from the drawings' 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size: Bubbles' colour
field, or the field with its right half clear. The projects go into `Fixtures/scatterize`, the
expected frames into `Fixtures/scatterize/expected_scatterize.json`. Every case is "not built yet"
until B-555 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/scatterize_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
import smooth_reference as S  # noqa: E402
import bubbles_reference as BU  # noqa: E402
import hair_reference as HR  # noqa: E402

W, H, FPS = 48, 32, 24
assert (BU.W, BU.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "scatterize"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.scatterize"
SETTINGS = [("amount", -10000, 10000, 50), ("right_twist", -36000, 36000, 0),
            ("left_twist", -36000, 36000, 0)]
WORDS = [("transfer_mode", ("composite", "screen", "add", "alpha_add"), "composite")]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def dots(src, v, w=W, h=H):
    """Steps 1 to 3: every dot as (z, order, P, colour), farthest first."""
    big = v["amount"] * h / 1080
    out = []
    for j in range(h):
        for i in range(w):
            p = src[j * w + i]
            if p[3] <= 0:
                continue
            x, y = i + 0.5, j + 0.5
            a = math.radians(v["left_twist"] + (v["right_twist"] - v["left_twist"]) * x / w)
            yt = h / 2 + (y - h / 2) * math.cos(a)
            z = (y - h / 2) * math.sin(a)
            u0, u1 = ((U(0, i, j, 0, k) + 1) / 2 for k in range(2))
            r, t = big * u0, 2 * math.pi * u1
            out.append((z, j * w + i, (x + r * math.cos(t), yt + r * math.sin(t)), p))
    out.sort(key=lambda d: (d[0], d[1]))
    return out


def lay(s, d, mode):
    if mode == "composite":
        return [s[k] + d[k] * (1 - s[3]) for k in range(4)]
    if mode == "screen":
        return [s[k] + d[k] - s[k] * d[k] for k in range(4)]
    if mode == "add":
        return [s[k] + d[k] for k in range(3)] + [s[3] + d[3] * (1 - s[3])]
    return [s[k] + d[k] for k in range(3)] + [min(1.0, s[3] + d[3])]


def scatterize(src, v, mode, w=W, h=H):
    out = [[0.0] * 4 for _ in range(w * h)]
    for _, _, (px, py), col in dots(src, v, w, h):
        # Only the pixels whose centres are within 1 of P are covered.
        for Y in range(math.floor(py - 1.5), math.floor(py + 0.5) + 2):
            for X in range(math.floor(px - 1.5), math.floor(px + 0.5) + 2):
                if not (0 <= X < w and 0 <= Y < h):
                    continue
                c = clamp(1 - math.hypot(X + 0.5 - px, Y + 0.5 - py), 0, 1)
                if c > 0:
                    out[Y * w + X] = lay([q * c for q in col], out[Y * w + X], mode)
    return out


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    return all(c[name] in words for name, words, _ in WORDS)


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c) or f < c["in_frame"]:
        return BU.frame(src, c["shift"])
    return BU.frame(scatterize(src, settings(c, f), c["transfer_mode"]), c["shift"])


def plain(c):
    return BU.frame(BU.working(c["drawing"]), c["shift"])


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift)
    return c


# A cloud: dots thrown up to about 9 pixels.
CLOUD = dict(amount=300)


def cloud(**kw):
    return case(**{**CLOUD, **kw})


CASES = {
    "FX-SCATZ-001": ("As added: on a picture 32 pixels high each dot is thrown up to a pixel and a "
                     "half, a fine dusting.", case(), (3,)),
    "FX-SCATZ-002": ("Amount 300: a cloud of dots thrown up to about 9 pixels.", cloud(), (3,)),
    "FX-SCATZ-003": ("Amount -300: every dot thrown the opposite way to FX-SCATZ-002.",
                     cloud(amount=-300), (3,)),
    "FX-SCATZ-004": ("Amount 0: every dot on its own pixel, the picture untouched.",
                     case(amount=0), (3,)),
    "FX-SCATZ-005": ("Right Twist 90, Amount 0: the right edge turned edge-on, the picture "
                     "squeezed towards the middle row more and more to the right.",
                     case(amount=0, right_twist=90), (3,)),
    "FX-SCATZ-006": ("Left Twist 180, Amount 0: the left edge turned upside down, the picture "
                     "squeezed through the middle row a third of the way across.",
                     case(amount=0, left_twist=180), (3,)),
    "FX-SCATZ-007": ("Right Twist 60, Left Twist -60, Amount 100: the two ends turned opposite "
                     "ways, dusted.", case(amount=100, right_twist=60, left_twist=-60), (3,)),
    "FX-SCATZ-008": ("Screen: the cloud of FX-SCATZ-002, the dots screened.",
                     cloud(transfer_mode="screen"), (3,)),
    "FX-SCATZ-009": ("Add: the dots' colours added, so where they bunch they brighten.",
                     cloud(transfer_mode="add"), (3,)),
    "FX-SCATZ-010": ("Alpha Add: colours and opacities added.",
                     cloud(transfer_mode="alpha_add"), (3,)),
    "FX-SCATZ-011": ("The picture with its right half clear: only the left half's dots, some "
                     "thrown across into the clear half.", cloud(drawing="half"), (3,)),
    "FX-SCATZ-012": ("Amount keyed from 0 at frame 0 to 300 at frame 12: at frame 0 the picture "
                     "untouched, then the cloud gathering out.", case(amount=keyed((0, 0), (12, 300))),
                     (0, 6, 12)),
    "FX-SCATZ-013": ("FX-SCATZ-002 moved 3 pixels right: the same, moved.", cloud(shift=3), (3,)),
    "FX-SCATZ-014": ("Both twists 360, Amount 0: a whole turn at every column, the picture "
                     "untouched.", case(amount=0, right_twist=360, left_twist=360), (3,)),
    "FX-SCATZ-015": ("Right Twist 720, Amount 50: the ribbon turned twice over across the "
                     "picture.", case(right_twist=720), (3,)),
    "FX-SCATZ-016": ("Amount 3000: most dots thrown past the edge, a thin scatter left.",
                     case(amount=3000), (3,)),
    "FX-SCATZ-017": ("Right Twist 90 with Add, Amount 0: the dots piled where the right edge is "
                     "edge-on add up bright.", case(amount=0, right_twist=90, transfer_mode="add"),
                     (3,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-SCATZ-018": ("Amount 10001, above 10000.", case(amount=10001)),
    "FX-SCATZ-019": ("Amount -10001, below -10000.", case(amount=-10001)),
    "FX-SCATZ-020": ("Right Twist 36001, above 36000.", case(right_twist=36001)),
    "FX-SCATZ-021": ("Left Twist -36001, below -36000.", case(left_twist=-36001)),
    "FX-SCATZ-022": ("Transfer Mode written \"multiply\".", case(transfer_mode="multiply")),
    "FX-SCATZ-023": ("Amount keyed to 20000 at frame 4, above 10000.",
                     case(amount=keyed((0, 0), (4, 20000)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = HR.project_json(fx, {**HR.case(drawing=c["drawing"], rate=c["rate"],
                                       in_frame=c["in_frame"], shift=c["shift"])})
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    fx0 = p["compositions"][0]["layers"][0]["effects"][0]
    fx0["type_id"], fx0["parameters"] = TYPE, params
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in BU.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        assert valid(c), fx
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} "
            f"{sum(max(abs(a - b) for a, b in zip(px[i], before[i])) > 1e-9 for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        assert not valid(c), fx
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_scatterize.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def near(a, b, tol=1e-12):
    return all(abs(x - y) <= tol for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    field = plain(case())
    for fx in ("FX-SCATZ-004", "FX-SCATZ-014"):
        assert near(c[fx]["3"], field), fx
    assert near(c["FX-SCATZ-012"]["0"], field)
    k = c["FX-SCATZ-012"]
    assert k["0"] != k["6"] != k["12"]
    two = c["FX-SCATZ-002"]["3"]
    for n in (1, 3, 5, 6, 7, 8, 9, 10, 11, 15, 16, 17):
        assert c["FX-SCATZ-%03d" % n]["3"] != two, n
    # FX-SCATZ-003's dots are FX-SCATZ-002's thrown back through their start.
    src = BU.working("field")
    d2 = {o: p for _, o, p, _ in dots(src, settings(cloud(), 0))}
    for _, o, p, _ in dots(src, settings(cloud(amount=-300), 0)):
        x, y = o % W + 0.5, o // W + 0.5
        assert abs(p[0] - x + d2[o][0] - x) < 1e-9 and abs(p[1] - y + d2[o][1] - y) < 1e-9
    # Edge-on at the right edge: the last column's dots all on the middle row.
    five = dots(src, settings(case(amount=0, right_twist=90), 0))
    edge = [p for _, o, p, _ in five if o % W == W - 1]
    assert all(abs(p[1] - H / 2) < 0.5 for p in edge)
    # The half picture throws no dot from its clear half.
    assert all(o % W < W // 2 for _, o, _, _ in dots(BU.working("half"), settings(cloud(), 0)))
    moved = c["FX-SCATZ-013"]["3"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two[y * W:y * W + W - 3]
    # Add brightens where the dots pile, past what Composite gives.
    add, comp = c["FX-SCATZ-017"]["3"], c["FX-SCATZ-005"]["3"]
    assert max(p[0] for p in add) > max(p[0] for p in comp) + 0.5
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
