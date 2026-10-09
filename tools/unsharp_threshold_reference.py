"""D-317's Unsharp Mask Threshold, worked a second way.

After Effects' Unsharp Mask has three settings: Amount, Radius and Threshold. Threshold says how
far apart two tones must be before they count as an edge, so flat areas and fine grain are left
alone while real edges are crisped. `core.sharpen` (Sharpen, found by "unsharp mask") has had
Amount and Radius since D-147 (`tools/sharpen_reference.py`, FX-SHARPEN-001 to 018); D-317
added `threshold`, 0 to 255 (0), keyable. This file is the reference for the threshold.

The rule, document 21's words: D-147's sharpen, except that a channel with
|e_c - eb_c| < threshold / 255 keeps its value, e_c being the pixel's encoded straight colour
and eb_c its blur's (both as D-147 has them). At 0 nothing is kept, so the picture is D-147's.
This program's reading: the comparison is made per channel against the blurred copy, the
difference the sharpen would add, where After Effects' manual speaks of adjacent pixels; D-317
settled it.

Every case is a composition 16 by 10 holding one drawing the same size, unmoved unless the case
says. `picture` is sharpen_reference's (a patch of skin, a soft line, a box of line with skin
and a blue band). `grain` is new: skin with a fine grain of up to two levels in each channel,
and a dark line two pixels wide down columns 10 and 11. It goes into
`Fixtures/sharpen/media/grain.png`, the projects into `Fixtures/sharpen` as FX-SHARPEN-019 to
030, and the expected frames into `Fixtures/sharpen/expected_sharpen_threshold.json`;
FX-SHARPEN-001 to 018 and their expected file are not touched.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawings' 8-bit values, blurring with the two-dimensional kernel summed directly
(`light_wrap_reference.blurred`), where the build blurs in single precision one axis at a time.
Each case's threshold is checked to lie at least 0.05 of a level from every difference it is
compared with, so single-precision rounding cannot move a channel across it.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/unsharp_threshold_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from light_wrap_reference import blurred  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import sharpen_reference as SR  # noqa: E402

W, H = SR.W, SR.H
OUT = SR.OUT
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("amount", "radius", "threshold")
RANGES = {"amount": (0, 500), "radius": (0, 100), "threshold": (0, 255)}
MARGIN = 0.05  # levels of 255 between a threshold and any difference it meets


# --- the rule -------------------------------------------------------------------------------

def enc(p):
    return [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]


def differences(px, radius):
    """Per pixel that shows, the encoded colour and its blur's, or None."""
    out = []
    for o, b in zip(px, blurred(px, radius, (W, H))):
        if o[3] > 0:
            e = enc(o)
            out.append((e, enc(b) if b[3] > 0 else e))
        else:
            out.append(None)
    return out


def unsharp(px, amount, radius, threshold):
    if amount == 0 or radius == 0:
        return [list(p) for p in px]
    out = []
    for p, d in zip(px, differences(px, radius)):
        q = list(p)
        if d:
            e, eb = d
            for c in range(3):
                if abs(e[c] - eb[c]) < threshold / 255:
                    continue
                v = min(1.0, max(0.0, e[c] + amount / 100 * (e[c] - eb[c])))
                q[c] = srgb_to_linear(v) * p[3]
        out.append(q)
    return out


# --- the drawings ---------------------------------------------------------------------------

def grain(x, y):
    if x in (10, 11):
        return SR.LINE
    return tuple(SR.SKIN[c] + (x * 7 + y * 3 + c * 2) % 5 - 2 for c in range(3)) + (255,)


DRAWINGS = {"picture": SR.DRAWINGS["picture"],
            "grain": [[grain(x, y) for x in range(W)] for y in range(H)]}


def pixels(name):
    return [R.working(p) for row in DRAWINGS[name] for p in row]


# --- the cases ------------------------------------------------------------------------------

def case(drawing="picture", amount=100, radius=1, threshold=0, shift=0):
    return {"drawing": drawing, "amount": amount, "radius": radius, "threshold": threshold,
            "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    return R.frame(unsharp(pixels(c["drawing"]), *(held(c, k, frame_no) for k in NAMES)),
                   c["shift"])


def plain(c):
    return R.frame(pixels(c["drawing"]), c["shift"])


CASES = {
    "FX-SHARPEN-019": ("Threshold 0 written in the file: FX-SHARPEN-001, every pixel.",
                       case(), [0]),
    "FX-SHARPEN-020": ("Threshold 12: the channels nearer their blur than 12 levels, a few "
                       "inside the box, keep the drawing's value; every other channel is "
                       "FX-SHARPEN-001's.",
                       case(threshold=12), [0]),
    "FX-SHARPEN-021": ("Threshold 30: more channels kept, the skin inside the box among them; "
                       "the line is still pushed to black.",
                       case(threshold=30), [0]),
    "FX-SHARPEN-022": ("Amount 200, threshold 110: only the hardest edges, the line against the "
                       "skin, are crisped, twice as hard as FX-SHARPEN-001; the band and its "
                       "skin keep their colours.", case(amount=200, threshold=110), [0]),
    "FX-SHARPEN-023": ("Threshold 255: no channel is 255 levels from its blur, so the drawing "
                       "is untouched.", case(threshold=255), [0]),
    "FX-SHARPEN-024": ("The grain drawing, threshold 0: the grain is crisped along with the "
                       "line, every pixel changing.", case("grain"), [0]),
    "FX-SHARPEN-025": ("The grain drawing, threshold 16: the grain, under 16 levels from its "
                       "blur, keeps the drawing's values exactly; the line and the skin beside "
                       "it are crisped as in FX-SHARPEN-024.", case("grain", threshold=16), [0]),
    "FX-SHARPEN-026": ("Threshold keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 "
                       "FX-SHARPEN-001, frame 2 threshold 20, frame 4 threshold 40, each "
                       "keeping more.", case(threshold=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-SHARPEN-027": ("The grain drawing, amount 300, radius 2, threshold 10, moved three "
                       "pixels right: the grain still kept, the line's halo wider and harder.",
                       case("grain", amount=300, radius=2, threshold=10, shift=3), [0]),
}

INVALID = {
    "FX-SHARPEN-028": ("Threshold 256, above 255.", case(threshold=256)),
    "FX-SHARPEN-029": ("Threshold -1, below 0.", case(threshold=-1)),
    "FX-SHARPEN-030": ("Threshold keyed to 300 at frame 4.",
                       case(threshold=keyed((0, 10), (4, 300)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = SR.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                             "amount": c["amount"], "radius": c["radius"]})
    p["assets"][0].update({"id": f"asset-{c['drawing']}", "name": c["drawing"],
                           "path": f"media/{c['drawing']}.png"})
    layer = p["compositions"][0]["layers"][0]
    layer["asset_id"] = f"asset-{c['drawing']}"
    layer["effects"][0]["parameters"] = {k: setting_json(c[k]) for k in NAMES}
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "grain.png").write_bytes(S.png(DRAWINGS["grain"]))
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
    (OUT / "expected_sharpen_threshold.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    pic, grn = plain(case()), plain(case("grain"))
    kept = lambda f, g, i, ch: f[i][ch] == g[i][ch]  # noqa: E731

    # No threshold lies within MARGIN levels of a difference it is compared with.
    for fx, (_, cc, frames) in CASES.items():
        for f in frames:
            t = held(cc, "threshold", f)
            for d in differences(pixels(cc["drawing"]), held(cc, "radius", f)):
                if d:
                    for u, v in zip(*d):
                        assert t == 0 or abs(abs(u - v) * 255 - t) >= MARGIN, (fx, f, u, v)

    # 019: D-147's sharpen, worked by sharpen_reference itself.
    one = SR.render(SR.case(), 0)
    assert like(c["FX-SHARPEN-019"]["0"], one)
    # Each threshold: a channel either keeps the drawing's value exactly or is 001's, and the
    # number kept grows with the threshold.
    counts = []
    for fx in ("FX-SHARPEN-019", "FX-SHARPEN-020", "FX-SHARPEN-021"):
        f = c[fx]["0"]
        n = 0
        for i in range(W * H):
            for ch in range(3):
                assert kept(f, pic, i, ch) or abs(f[i][ch] - one[i][ch]) < 1e-12, (fx, i, ch)
                n += pic[i][3] > 0 and kept(f, pic, i, ch) and not abs(one[i][ch] - pic[i][ch]) < 1e-15
        counts.append(n)
    assert counts[0] == 0 < counts[1] < counts[2], counts
    lines = [at(x, y) for x in range(8, 16) for y in range(1, 9) if x in (8, 15) or y in (1, 8)]
    assert all(enc(c["FX-SHARPEN-021"]["0"][i]) == [0.0] * 3 for i in lines)
    # 022: the band and the skin beside it keep their colours; the line's corner still moves.
    tt = c["FX-SHARPEN-022"]["0"]
    assert all(tt[at(x, y)] == pic[at(x, y)] for x in range(10, 14) for y in range(3, 7))
    assert any(tt[i] != pic[i] for i in lines)
    assert c["FX-SHARPEN-023"]["0"] == pic
    # 024: every pixel of the grain changes; 025: away from the line the grain is kept exactly.
    g0, g16 = c["FX-SHARPEN-024"]["0"], c["FX-SHARPEN-025"]["0"]
    assert all(not near(g0[i], grn[i], 1e-9) for i in range(W * H))
    far = [at(x, y) for x in list(range(0, 7)) + [15] for y in range(H)]
    assert all(g16[i] == grn[i] for i in far)
    assert all(near(g16[at(x, y)], g0[at(x, y)]) for x in (9, 10, 11, 12) for y in range(H))
    # 026: frame 0 is 001; frames 2 and 4 are thresholds 20 and 40.
    k = c["FX-SHARPEN-026"]
    assert like(k["0"], one)
    assert k["2"] == render(case(threshold=20), 0) and k["4"] == render(case(threshold=40), 0)
    # 027: moved three right, the first three columns empty, the grain far from the line kept.
    s = c["FX-SHARPEN-027"]["0"]
    assert all(s[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    gs = plain(case("grain", shift=3))
    assert all(s[at(x + 3, y)] == gs[at(x + 3, y)] for x in range(0, 4) for y in range(H))
    # Every case keeps every covering.
    for fx, frames in c.items():
        base = plain(CASES[fx][1] if fx in CASES else INVALID[fx][1])
        for px in frames.values():
            assert all(px[i][3] == base[i][3] for i in range(W * H)), fx
    print("checked")


if __name__ == "__main__":
    main()
