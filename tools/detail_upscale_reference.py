"""Detail-preserving Upscale, worked a second way.

D-407 adds `core.detail_upscale`, after After Effects' Detail-preserving Upscale (Distort
group): the layer enlarged by a per cent, the layer growing with it about its own middle, with an
optional softening before and a sharpening after. Adobe's page names the controls and says what
they do (Reduce Noise "is applied before scaling"; high Detail sharpens edges and "may introduce
ringing or halos") but gives no formula, and the effect is related to Photoshop's Preserve
Details resampling, which Adobe does not publish either. The rule is this program's own: a
Gaussian softening, a Lanczos-3 resampling, then an unsharp mask.

The settings. `scale`, 100 to 1000 per cent, 100 when added, how much larger; `reduce_noise`,
0 to 100, 0 when added; `detail`, 0 to 100, 20 when added (ours). All three keyable. After
Effects' Fit To Comp Width / Fit To Comp Height buttons and its Alpha choice are not part of
this contract.

The rule. On the picture as it reaches the effect, w by h pixels, premultiplied linear light,
s = scale / 100:

1. Reduce Noise. A Gaussian of sigma = reduce_noise / 50 pixels, taps t in -r..r with
   r = ceil(3 sigma), weights exp(-t^2 / (2 sigma^2)) divided by their sum, across then down,
   every channel, clear outside the picture, kept w by h. Sigma 0 leaves the picture as it is.
2. Scale. The layer grows by gx = ceil(w (s - 1) / 2) each side across and gy = ceil(h (s - 1)
   / 2) down (worked on the per cent: ceil(w (scale - 100) / 200)): the output is W = w + 2 gx by H = h + 2 gy, the picture's middle staying where it
   was. Output column x reads the picture at u = w / 2 + (x + 1/2 - W / 2) / s - 1/2 (pixel
   centres at whole numbers); the taps i = floor(u) - 2 .. floor(u) + 3, weights
   L(u - i) = sinc(t) sinc(t / 3) for |t| < 3 (exactly 0 at every other whole t), divided by the sum of all six, a tap outside the
   picture clear. Across, then down (v likewise with h, H), every channel. At s = 1 every u is
   whole and the picture is as it was.
3. Detail. With a = detail / 50 and G the step 1 Gaussian of sigma = s / 2 output pixels (clear
   outside the W by H layer): out = up + a (up - G(up)), every channel.
4. Alpha held to 0..1, each colour to 0..alpha.

Scale is a share, kept by a draft; both sigmas are distances, scaled by a draft.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 32 pixels by 20 holding one drawing 16 by 10 (Tiles' card, D-404)
in its middle, unmoved unless the case says, so the grown layer and its edges show. The drawing
goes into `Fixtures/detail_upscale/media`, the projects into `Fixtures/detail_upscale`, and the
expected frames into `Fixtures/detail_upscale/expected_detail_upscale.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/detail_upscale_reference.py
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

DW, DH = T.W, T.H  # the drawing, 16 by 10
W, H = 2 * DW, 2 * DH  # the composition, 32 by 20
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "detail_upscale"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"scale": (100, 1000), "reduce_noise": (0, 100), "detail": (0, 100)}
NAMES = ("scale", "reduce_noise", "detail")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def gaussian(sigma):
    if sigma <= 0:
        return [1.0]
    r = math.ceil(3 * sigma)
    k = [math.exp(-t * t / (2 * sigma * sigma)) for t in range(-r, r + 1)]
    total = sum(k)
    return [v / total for v in k]


def convolve(px, w, h, kernel):
    """Across then down, clear outside, kept w by h."""
    r = len(kernel) // 2
    if r == 0:
        return [list(p) for p in px]
    mid = []
    for y in range(h):
        for x in range(w):
            acc = [0.0] * 4
            for t, k in enumerate(kernel):
                i = x + t - r
                if 0 <= i < w:
                    p = px[y * w + i]
                    for ch in range(4):
                        acc[ch] += k * p[ch]
            mid.append(acc)
    out = []
    for y in range(h):
        for x in range(w):
            acc = [0.0] * 4
            for t, k in enumerate(kernel):
                j = y + t - r
                if 0 <= j < h:
                    p = mid[j * w + x]
                    for ch in range(4):
                        acc[ch] += k * p[ch]
            out.append(acc)
    return out


def lanczos(t):
    if t == 0:
        return 1.0
    if abs(t) >= 3 or t == math.floor(t):
        return 0.0
    a = math.pi * t
    return 3 * math.sin(a) * math.sin(a / 3) / (a * a)


def taps(n, big, s, x):
    """The taps and weights of output place x of `big`, from a row of n."""
    u = n / 2 + (x + 0.5 - big / 2) / s - 0.5
    f = math.floor(u)
    idx = list(range(f - 2, f + 4))
    wts = [lanczos(u - i) for i in idx]
    total = sum(wts)
    return [(i, v / total) for i, v in zip(idx, wts)]


def growth(w, h, scale):
    """Worked on the per cent, so a whole per cent gives a whole answer exactly."""
    return math.ceil(w * (scale - 100) / 200), math.ceil(h * (scale - 100) / 200)


def upscale(layer, st):
    w, h = layer["w"], layer["h"]
    s = st["scale"] / 100
    px = convolve(layer["px"], w, h, gaussian(st["reduce_noise"] / 50))
    gx, gy = growth(w, h, st["scale"])
    big_w, big_h = w + 2 * gx, h + 2 * gy
    across = []
    for y in range(h):
        for x in range(big_w):
            acc = [0.0] * 4
            for i, k in taps(w, big_w, s, x):
                if 0 <= i < w:
                    p = px[y * w + i]
                    for ch in range(4):
                        acc[ch] += k * p[ch]
            across.append(acc)
    up = []
    for y in range(big_h):
        row_taps = taps(h, big_h, s, y)
        for x in range(big_w):
            acc = [0.0] * 4
            for j, k in row_taps:
                if 0 <= j < h:
                    p = across[j * big_w + x]
                    for ch in range(4):
                        acc[ch] += k * p[ch]
            up.append(acc)
    a = st["detail"] / 50
    if a > 0:
        soft = convolve(up, big_w, big_h, gaussian(s / 2))
        up = [[u + a * (u - v) for u, v in zip(p, q)] for p, q in zip(up, soft)]
    out = []
    for p in up:
        al = min(1.0, max(0.0, p[3]))
        out.append([min(al, max(0.0, c)) for c in p[:3]] + [al])
    return {"px": out, "left": layer["left"] - gx, "top": layer["top"] - gy,
            "w": big_w, "h": big_h}


# --- the cases ------------------------------------------------------------------------------

def case(**kw):
    c = {"drawing": "card", "scale": 100, "reduce_noise": 0, "detail": 20, "shift": 0}
    c.update(kw)
    return c


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        lo, hi = RANGES[k]
        held[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return held


def pixel(layer, i, j):
    if 0 <= i < layer["w"] and 0 <= j < layer["h"]:
        return layer["px"][j * layer["w"] + i]
    return EMPTY


def placed(layer, shift):
    """The layer's buffer laid in the composition, the drawing's corner at (8, 5), moved
    `shift` pixels across."""
    ox, oy = (W - DW) // 2, (H - DH) // 2
    out = []
    for y in range(H):
        for x in range(W):
            out.append(list(pixel(layer, x - ox - shift - layer["left"], y - oy - layer["top"])))
    return out


def render(c, frame_no):
    return placed(upscale(T.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return placed(T.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-UPSCALE-001": ("The settings as they start: scale 100, no softening, Detail 20: the "
                       "drawing the same size, its edges a little sharpened (an unsharp mask of "
                       "0.4 at sigma 0.5).", case(), [0, 4]),
    "FX-UPSCALE-002": ("Scale 100, Detail 0: the drawing as it is.", case(detail=0), [0]),
    "FX-UPSCALE-003": ("Scale 200, Detail 0: Lanczos-3 alone; the layer grows to 32 by 20 and "
                       "fills the composition.", case(scale=200, detail=0), [0]),
    "FX-UPSCALE-004": ("Scale 200, Detail 20 (as added): FX-UPSCALE-003 sharpened at sigma 1.",
                       case(scale=200), [0]),
    "FX-UPSCALE-005": ("Scale 200, Detail 100: sharpened twice over, halos at the line and the "
                       "band.", case(scale=200, detail=100), [0]),
    "FX-UPSCALE-006": ("Scale 150, Detail 0: the layer grows 4 each side across and 3 down "
                       "(24 by 16), the middle unmoved.", case(scale=150, detail=0), [0]),
    "FX-UPSCALE-007": ("Scale 200, Reduce Noise 100, Detail 0: softened at sigma 2 before the "
                       "enlarging.", case(scale=200, reduce_noise=100, detail=0), [0]),
    "FX-UPSCALE-008": ("Scale 200, Reduce Noise 50, Detail 50: softened at sigma 1, enlarged, "
                       "sharpened by 1.", case(scale=200, reduce_noise=50, detail=50), [0]),
    "FX-UPSCALE-009": ("Scale 300: the layer 48 by 30, cut by the composition's edges.",
                       case(scale=300), [0]),
    "FX-UPSCALE-010": ("Scale 100, Reduce Noise 100, Detail 0: softened only.",
                       case(reduce_noise=100, detail=0), [0]),
    "FX-UPSCALE-011": ("Scale keyed from 100 at frame 0 to 300 at frame 4, linear, Detail 0: "
                       "as it is, then 200 at frame 2, then 300.",
                       case(scale=keyed((0, 100), (4, 300)), detail=0), [0, 2, 4]),
    "FX-UPSCALE-012": ("Detail keyed from 0 at frame 0 to 100 at frame 4, scale 200: frame 0 is "
                       "FX-UPSCALE-003, frame 4 FX-UPSCALE-005.",
                       case(scale=200, detail=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-UPSCALE-013": ("Scale 200, Detail 20, the layer moved 3 pixels right: FX-UPSCALE-004 "
                       "moved; columns 0 to 2 empty.", case(scale=200, shift=3), [0]),
    "FX-UPSCALE-014": ("Scale eased from 200 at frame 0 to 100 at frame 4 on a curve that "
                       "overshoots, Detail 0: at frame 2 it would pass below 100 and is held "
                       "there, so frames 2 and 4 show the drawing as it is.",
                       case(scale=keyed((0, 200, OVERSHOOT), (4, 100)), detail=0), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-UPSCALE-015": ("Scale 99, below 100.", case(scale=99)),
    "FX-UPSCALE-016": ("Scale 1001, above 1000.", case(scale=1001)),
    "FX-UPSCALE-017": ("Reduce Noise -1, below 0.", case(reduce_noise=-1)),
    "FX-UPSCALE-018": ("Reduce Noise 101, above 100.", case(reduce_noise=101)),
    "FX-UPSCALE-019": ("Detail -1, below 0.", case(detail=-1)),
    "FX-UPSCALE-020": ("Detail 101, above 100.", case(detail=101)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([DW / 2, DH / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.detail_upscale", "enabled": True,
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

    (OUT / "expected_detail_upscale.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def near(a, b, tol=1e-12):
    return all(abs(u - v) < tol for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(0 <= u <= p[3] for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    # Lanczos taps sum to 1 and a whole u reads that pixel alone.
    assert all(abs(sum(k for _, k in taps(16, 32, 2, x)) - 1) < 1e-12 for x in range(32))
    assert [k for _, k in taps(16, 16, 1, 5)] == [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
    assert growth(16, 10, 150) == (4, 3) and growth(16, 10, 200) == (8, 5)
    assert growth(16, 10, 300) == (16, 10) and growth(16, 10, 100) == (0, 0)

    first = one("FX-UPSCALE-001")
    assert first == c["FX-UPSCALE-001"]["4"] and first != drawn
    # Outside the drawing at scale 100 everything stays clear (the sharpening is kept W by H).
    assert all(first[at(x, y)] == EMPTY for x in range(W) for y in range(H)
               if not (8 <= x < 24 and 5 <= y < 15))
    two = one("FX-UPSCALE-002")
    assert two == drawn
    three = one("FX-UPSCALE-003")
    # The skin far from every feature stays the skin: composition (14, 6) reads the drawing at
    # (6.75, 2.75), whose six by six taps lie in columns 4..9, rows 0..5, all skin.
    skin = T.drawn_layer("card")["px"][2 * DW + 6]
    assert near([three[at(14, 6)]], [skin], 1e-12)
    # The grown layer fills the composition: every edge pixel of row 10 is drawn on.
    assert three[at(0, 10)][3] > 0 and three[at(31, 10)][3] > 0
    four, five = one("FX-UPSCALE-004"), one("FX-UPSCALE-005")
    assert three != four != five
    six = one("FX-UPSCALE-006")
    # 24 by 16 about the middle: columns 4..27 and rows 2..17 hold it, the rest is clear.
    assert all(six[at(x, y)] == EMPTY for x in range(W) for y in range(H)
               if not (4 <= x < 28 and 2 <= y < 18))
    assert six[at(4, 10)][3] > 0 and six[at(27, 10)][3] > 0
    seven, ten = one("FX-UPSCALE-007"), one("FX-UPSCALE-010")
    assert seven != three and ten != drawn
    # Softened only: total covering kept to within what the clear edge takes away.
    assert sum(p[3] for p in ten) < sum(p[3] for p in drawn)
    nine = one("FX-UPSCALE-009")
    assert all(p[3] > 0 for p in nine[at(0, 0):at(0, 0) + W])
    eleven = c["FX-UPSCALE-011"]
    assert eleven["0"] == drawn and near(eleven["2"], three, 1e-12)
    twelve = c["FX-UPSCALE-012"]
    assert twelve["0"] == three and twelve["4"] == five and twelve["2"] not in (three, five)
    thirteen = one("FX-UPSCALE-013")
    assert all(thirteen[at(x, y)] == four[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(thirteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    raw = value_at(CASES["FX-UPSCALE-014"][1]["scale"], 2)
    fourteen = c["FX-UPSCALE-014"]
    assert raw < 100 and fourteen["0"] == three
    assert fourteen["2"] == fourteen["4"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
