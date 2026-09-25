"""Line smoothing, worked a second way.

D-86 adds a fourth effect, `core.line_smooth`. It finds the stair steps where two flat colours
meet and puts the straight slope back: Reshetov's morphological antialiasing, as OpenToonz's
`toonz/sources/common/trop/tantialias.cpp` does it, with D-86's corner guard. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, where the
build works in single precision on its buffers, and the simplest cases are also checked below
against areas worked by hand from the slope through the middles of the steps, which is the
method's own description rather than anyone's code.

The smoothing below is ported from OpenToonz, commit 6571328019e3a6a99c13f408ee6f4755c9cddf73
(the last to touch the file, 2022-04-07), under its licence, kept in
`docs/third_party/OpenToonz-LICENSE.txt`:

    Copyright (c) 2016 - 2026, DWANGO Co., Ltd.
    Copyright (c) 2016 - 2026, the respective contributors.
    All rights reserved. BSD 3-Clause "New" or "Revised" License; the full text, its conditions
    and its disclaimer are in the file named above.

Every case is a composition 12 pixels by 8 holding one drawing the same size, unmoved unless the
case says. The drawings go into `Fixtures/smooth/media`, the projects into `Fixtures/smooth`, and
the expected frames into `Fixtures/smooth/expected_smooth.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/smooth_reference.py
"""

import json
import struct
import sys
import zlib
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402

W, H = 12, 8
FRAMES = 5
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "smooth"
TOLERANCE = 2e-5  # document 25's default for a filter; see D-86
CORNER = 4        # D-86: a corner whose two edges both run 4 pixels or more is left sharp


# --- the rule -------------------------------------------------------------------------------

def linear_to_srgb(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def encoded(p):
    """A working pixel (linear, premultiplied) as a drawing program holds it: the straight
    colour through the sRGB curve, premultiplied again."""
    a = p[3]
    if a <= 0:
        return [0.0, 0.0, 0.0, 0.0]
    return [linear_to_srgb(p[i] / a) * a for i in range(3)] + [a]


def working(e):
    a = e[3]
    if a <= 0:
        return [0.0, 0.0, 0.0, 0.0]
    return [srgb_to_linear(e[i] / a) * a for i in range(3)] + [a]


def smooth(frame, softness, threshold, w=W, h=H):
    """Document 21's line smoothing on a working buffer. A pixel the rule does not touch keeps
    its value exactly; a touched one is mixed in the encoded values and brought back."""
    if softness <= 0:
        return [list(p) for p in frame]
    P = [encoded(p) for p in frame]
    O = [list(p) for p in P]
    touched = set()
    t = threshold / 255

    def eq(a, b):
        return max(abs(a[i] - b[i]) for i in range(4)) <= t

    slope = 50.0 / softness
    for y in range(h - 1):
        _line(y, w, h, P, O, touched, y * w, (y + 1) * w, 1, w, True, slope, eq)
    for x in range(w - 1):
        _line(x, h, w, P, O, touched, x, x + 1, w, 1, False, slope, eq)
    return [working(O[i]) if i in touched else list(frame[i]) for i in range(w * h)]


def _mix(O, touched, o, b, area):
    touched.add(o)
    O[o] = [O[o][i] * (1.0 - area) + b[i] * area for i in range(4)]


def _neighbourhood(x, y, P, pix, lx, ly, dx, dy, eq):
    """OpenToonz's checkNeighbourHood: when both diagonals could be joined, join the minority."""
    c1 = c2 = 0
    if y > 1:
        c1 += eq(P[pix - dx], P[pix - 2 * dy]) + eq(P[pix - dx], P[pix - 2 * dy - dx])
        c2 += eq(P[pix], P[pix - 2 * dy]) + eq(P[pix], P[pix - 2 * dy - dx])
    if y < ly - 1:
        c1 += eq(P[pix - dx], P[pix + dy]) + eq(P[pix - dx], P[pix + dy - dx])
        c2 += eq(P[pix], P[pix + dy]) + eq(P[pix], P[pix + dy - dx])
    if x > 1:
        c1 += eq(P[pix - dx], P[pix - 2 * dx]) + eq(P[pix - dx], P[pix - 2 * dx - dy])
        c2 += eq(P[pix], P[pix - 2 * dx]) + eq(P[pix], P[pix - 2 * dx - dy])
    if x < lx - 1:
        c1 += eq(P[pix - dx], P[pix + dx]) + eq(P[pix - dx], P[pix + dx - dy])
        c2 += eq(P[pix], P[pix + dx]) + eq(P[pix], P[pix + dx - dy])
    return c1 > c2


def _filter(P, O, touched, inL, inU, out, ll, inD, outD, slope, lower):
    """OpenToonz's filterLine: the pixels under the slope take the other row's colour by the
    area of them the slope leaves on its side."""
    h0 = 0.5
    base = h0 / slope
    end = min(int(base), ll)
    for _ in range(end):
        h1 = h0 - slope
        _mix(O, touched, out, P[inU] if lower else P[inL], 0.5 * (h0 + h1))
        inL, inU, out, h0 = inL + inD, inU + inD, out + outD, h1
    if end < ll:
        _mix(O, touched, out, P[inU] if lower else P[inL], 0.5 * (base - end) * h0)


def _line(r, lx, ly, P, O, touched, inLRow, inURow, inDx, inDy, do1, slope, eq):
    """OpenToonz's processLine for one pair of rows (or, turned, of columns), with D-86's corner
    guard added at each end of a run."""
    r += 1
    inLEnd = inLRow + lx * inDx

    def crossing(aL, bL, aU, bU):
        # How far the edge across the end of the run goes, from this pair outward, while both
        # sides of it keep their colours.
        n = 0
        for a, b, step, room in ((aU, bU, inDy, ly - 1 - r), (aL, bL, -inDy, r - 1)):
            if eq(P[a], P[b]):
                continue
            n += 1
            qa, qb = a + step, b + step
            while room > 0 and eq(P[qa], P[a]) and eq(P[qb], P[b]):
                n, room, qa, qb = n + 1, room - 1, qa + step, qb + step
        return n

    def corner(length, aL, bL, aU, bU):
        return length >= CORNER and crossing(aL, bL, aU, bU) >= CORNER

    def check_length(length, L1, U1, L2, U2, unite_u):
        return length > 1 or (do1 and (
            (unite_u and r > 1 and not (eq(P[L1], P[L1 - inDy]) and eq(P[L2], P[L2 - inDy])))
            or (r < ly - 1 and not (eq(P[U1], P[U1 + inDy]) and eq(P[U2], P[U2 + inDy])))))

    def run_end(inLL, inLR):
        inUL, inUR = inLL + (inURow - inLRow), inLR + (inURow - inLRow)
        return inUL, inUR

    def right(inLL, inLR, whole):
        _, inUR = run_end(inLL, inLR)
        length = (inLR - inLL) // inDx
        L1, U1 = inLR - inDx, inUR - inDx
        x = (L1 - inLRow) // inDx
        if corner(length, L1, inLR, U1, inUR):
            return
        unite_u = eq(P[U1], P[inLR])
        unite_l = eq(P[L1], P[inUR])
        if unite_u or unite_l:
            if unite_u and unite_l:
                unite_u = not _neighbourhood(x + 1, r, P, inUR, lx, ly, inDx, inDy, eq)
            if check_length(length, L1, U1, inLR, inUR, unite_u):
                _filter(P, O, touched, L1, U1, (L1 if unite_u else U1), length, -inDx, -inDx,
                        slope / (length * (2 if whole else 1)), unite_u)

    def left(inLL, inLR, whole):
        inUL, _ = run_end(inLL, inLR)
        length = (inLR - inLL) // inDx
        L0, U0 = inLL - inDx, inUL - inDx
        x = (inLL - inLRow) // inDx
        if corner(length, L0, inLL, U0, inUL):
            return
        unite_u = eq(P[inUL], P[L0])
        unite_l = eq(P[inLL], P[U0])
        if unite_u or unite_l:
            if unite_u and unite_l:
                unite_u = _neighbourhood(x, r, P, inUL, lx, ly, inDx, inDy, eq)
            if check_length(length, L0, U0, inLL, inUL, unite_u):
                _filter(P, O, touched, inLL, inUL, (inLL if unite_u else inUL), length, inDx,
                        inDx, slope / (length * (2 if whole else 1)), unite_u)

    def same(inLL):
        return eq(P[inLL], P[inLL + (inURow - inLRow)])

    def run_from(inLL):
        # Where the run that starts at inLL ends: both rows keep their colours up to there.
        inLR = inLL + inDx
        while (inLR != inLEnd and eq(P[inLL], P[inLR])
               and eq(P[inLL + inURow - inLRow], P[inLR + inURow - inLRow])):
            inLR += inDx
        return inLR

    inLL = inLRow
    inLR = inLEnd
    if not same(inLL):
        inLR = run_from(inLL)
        if inLR != inLEnd:
            right(inLL, inLR, True)
        inLL = inLR
    while inLL != inLEnd and same(inLL):
        inLL += inDx
    while inLL != inLEnd:
        inLR = run_from(inLL)
        if inLR == inLEnd:
            break
        left(inLL, inLR, False)
        right(inLL, inLR, False)
        inLL = inLR
        while inLL != inLEnd and same(inLL):
            inLL += inDx
    if inLL != inLEnd:
        left(inLL, inLR, True)


# --- drawings -------------------------------------------------------------------------------

def png(pixels):
    """An 8-bit straight RGBA PNG, rows top to bottom."""
    h, w = len(pixels), len(pixels[0])
    raw = b"".join(b"\0" + bytes(v for px in row for v in px) for row in pixels)

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


def drawing(inside, ink, paper):
    return [[ink if inside(x, y) else paper for x in range(W)] for y in range(H)]


WHITE, BLACK = (255, 255, 255, 255), (0, 0, 0, 255)
RED, BLUE = (255, 0, 0, 255), (0, 0, 255, 255)
NONE = (0, 0, 0, 0)
SKIN, SKIN2 = (250, 214, 186, 255), (244, 208, 180, 255)  # six apart in every colour


def stair(x, y):
    """A one-pixel line going down one row every three columns, from (0, 1) to (11, 4)."""
    return y == 1 + x // 3


DRAWINGS = {
    # White above, black below, with one step in the middle of row 4.
    "step": drawing(lambda x, y: y >= 5 or (y == 4 and x >= 6), BLACK, WHITE),
    "stair": drawing(stair, BLACK, WHITE),
    # A box six by four: straight edges and four corners, every edge 4 pixels or more.
    "box": drawing(lambda x, y: 3 <= x <= 8 and 2 <= y <= 5, BLACK, WHITE),
    # A box three by three: its corners are shorter than 4.
    "dot": drawing(lambda x, y: 4 <= x <= 6 and 2 <= y <= 4, BLACK, WHITE),
    "red_on_blue": drawing(stair, RED, BLUE),
    "black_on_nothing": drawing(stair, BLACK, NONE),
    "red_on_nothing": drawing(stair, RED, NONE),
    # Two fills six apart, meeting at the step of "step".
    "near": drawing(lambda x, y: y >= 5 or (y == 4 and x >= 6), SKIN2, SKIN),
}


def decoded(name):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    out = []
    for row in DRAWINGS[name]:
        for r, g, b, a in row:
            a = a / 255
            out.append([srgb_to_linear(r / 255) * a, srgb_to_linear(g / 255) * a,
                        srgb_to_linear(b / 255) * a, a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(name, softness=50, threshold=10, shift=0):
    return {"drawing": name, "softness": softness, "threshold": threshold, "shift": shift}


def render(c, frame_no):
    soft = min(100.0, max(0.0, value_at(c["softness"], frame_no)))
    thr = min(255.0, max(0.0, value_at(c["threshold"], frame_no)))
    out = smooth(decoded(c["drawing"]), soft, thr)
    s = c["shift"]
    return [out[y * W + x - s] if x >= s else [0.0] * 4 for y in range(H) for x in range(W)]


CASES = {
    "FX-SMOOTH-001": ("One step between white and black, softness 50: row 4 turns into an even "
                      "slope across the whole row, and no other row changes.",
                      case("step"), [0]),
    "FX-SMOOTH-002": ("The same at softness 0: the drawing, untouched.",
                      case("step", softness=0), [0]),
    "FX-SMOOTH-003": ("The same at softness 100: the slope falls half as fast, so it is still "
                      "part of the way down at both ends of the row; row 4 only.",
                      case("step", softness=100), [0]),
    "FX-SMOOTH-004": ("A one-pixel black line stepping down every three pixels: each step is "
                      "softened, and a pixel two rows from the line is untouched.",
                      case("stair"), [0]),
    "FX-SMOOTH-005": ("A black box six by four: straight edges and corners of 4 or more, so "
                      "nothing changes.",
                      case("box"), [0]),
    "FX-SMOOTH-006": ("A black box three by three: its corners are shorter than 4 and are "
                      "rounded.",
                      case("dot"), [0]),
    "FX-SMOOTH-007": ("A red line on blue: every pixel is red, blue or a mix of the two, never "
                      "darker.",
                      case("red_on_blue"), [0]),
    "FX-SMOOTH-008": ("A black line on nothing: the softened pixels are black, partly covering, "
                      "with no grey or white fringe.",
                      case("black_on_nothing"), [0]),
    "FX-SMOOTH-009": ("A red line on nothing: every pixel that shows is the line's red.",
                      case("red_on_nothing"), [0]),
    "FX-SMOOTH-010": ("Two skin colours six apart, threshold 10: one colour to the rule, so "
                      "nothing changes.",
                      case("near"), [0]),
    "FX-SMOOTH-011": ("The same at threshold 0: two colours, and their step is softened.",
                      case("near", threshold=0), [0]),
    "FX-SMOOTH-012": ("Softness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                      "FX-SMOOTH-002, frame 2 is FX-SMOOTH-001, frame 4 is FX-SMOOTH-003.",
                      case("step", softness=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-SMOOTH-013": ("FX-SMOOTH-001 moved two pixels right: the same smoothed drawing, moved; "
                      "the smoothing is done on the drawing's own pixels before it is moved.",
                      case("step", shift=2), [0, 3]),
}

# Out of range in a file. D-46: the file is read, the effect is kept as written and left out of
# every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SMOOTH-020": ("Softness 150, above 100.", case("step", softness=150)),
    "FX-SMOOTH-021": ("Threshold -1, below 0.", case("step", threshold=-1)),
    "FX-SMOOTH-022": ("Softness keyed to 120 at frame 4.",
                      case("step", softness=keyed((0, 0), (4, 120)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    d = c["drawing"]
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-" + d,
                "enabled": True, "locked": False, "in_frame": 0, "out_frame": FRAMES,
                "source_offset_frames": 0,
                "transform": {
                    "anchor": prop([W / 2, H / 2]),
                    "position": prop([W / 2 + c["shift"], H / 2]),
                    "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
                },
                "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": "core.line_smooth",
                             "enabled": True,
                             "parameters": {"softness": setting_json(c["softness"]),
                                            "threshold": setting_json(c["threshold"])}}],
            }],
        }],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        print(f"{fx}: {says}\n")
        for f, px in rendered.items():
            changed = [(i % W, i // W) for i in range(W * H)
                       if px[i] != render(case(c["drawing"], softness=0, shift=c["shift"]), 0)[i]]
            print(f"frame {f}: {len(changed)} pixels changed")
        print()
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        plain = render(case(c["drawing"], softness=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: {says}\n")

    (OUT / "expected_smooth.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    enc = lambda frame: [encoded(p) for p in frame]  # noqa: E731
    step = decoded("step")

    # 001, by hand. Each half of row 4 is a run of 6 reaching the edge of the picture, so its
    # slope falls from a half at the step to nothing at the edge: half a pixel over 6. The pixel
    # k places from the step is covered by the other colour over the area between heights
    # 0.5 - k/12 and 0.5 - (k+1)/12, which is 0.5 - (2k+1)/24.
    one = c["FX-SMOOTH-001"]["0"]
    row = enc(one)[4 * W:5 * W]
    for k in range(6):
        area = 0.5 - (2 * k + 1) / 24
        assert abs(row[5 - k][0] - (1 - area)) < 1e-12, (k, row[5 - k])  # white, darkened
        assert abs(row[6 + k][0] - area) < 1e-12, (k, row[6 + k])        # black, lightened
    assert all(one[i] == step[i] for i in range(W * H) if i // W != 4)

    assert c["FX-SMOOTH-002"]["0"] == step
    # 003, by hand: the slope falls half as fast, a quarter of a pixel over the 6, so the area
    # is 0.5 - (2k+1)/48 and never reaches nothing.
    three = c["FX-SMOOTH-003"]["0"]
    row = enc(three)[4 * W:5 * W]
    for k in range(6):
        area = 0.5 - (2 * k + 1) / 48
        assert abs(row[5 - k][0] - (1 - area)) < 1e-12 and abs(row[6 + k][0] - area) < 1e-12, k
    assert all(three[i] == step[i] for i in range(W * H) if i // W != 4) and three != one

    # 004: nothing two rows or more from the line changes.
    four, stair_ = c["FX-SMOOTH-004"]["0"], decoded("stair")
    assert four != stair_
    for i in range(W * H):
        x, y = i % W, i // W
        if abs(y - (1 + x // 3)) >= 2:
            assert four[i] == stair_[i], (x, y)

    assert c["FX-SMOOTH-005"]["0"] == decoded("box")
    # Without the guard the box would have been rounded: the guard is what keeps it.
    global CORNER
    CORNER, kept = 10 ** 9, CORNER
    assert smooth(decoded("box"), 50, 10) != decoded("box")
    CORNER = kept
    assert c["FX-SMOOTH-006"]["0"] != decoded("dot")

    # 007: in the encoded values, green stays 0 and red and blue share one pixel's worth.
    for p in enc(c["FX-SMOOTH-007"]["0"]):
        assert abs(p[1]) < 1e-12 and abs(p[0] + p[2] - 1) < 1e-12 and p[3] == 1, p
    assert c["FX-SMOOTH-007"]["0"] != decoded("red_on_blue")

    # 008 and 009: every pixel that shows is the line's own colour, straight.
    for fx, colour in (("FX-SMOOTH-008", (0, 0, 0)), ("FX-SMOOTH-009", (1, 0, 0))):
        frame = c[fx]["0"]
        partial = [p for p in frame if 0 < p[3] < 1]
        assert partial, fx
        for p in frame:
            if p[3] > 0:
                assert all(abs(p[i] / p[3] - colour[i]) < 1e-12 for i in range(3)), (fx, p)

    assert c["FX-SMOOTH-010"]["0"] == decoded("near")
    assert c["FX-SMOOTH-011"]["0"] != decoded("near")

    assert c["FX-SMOOTH-012"] == {"0": step, "2": one, "4": three}
    moved = c["FX-SMOOTH-013"]["0"]
    assert moved == c["FX-SMOOTH-013"]["3"]
    for y in range(H):
        assert moved[y * W:y * W + 2] == [[0.0] * 4] * 2
        assert moved[y * W + 2:(y + 1) * W] == one[y * W:(y + 1) * W - 2]


if __name__ == "__main__":
    main()
