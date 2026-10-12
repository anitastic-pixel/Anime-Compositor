"""Star Burst, worked a second way.

D-677 adds Star Burst (`core.star_burst`), after CycoreFX's CC Star Burst (Simulation): the layer
broken into a field of stars, each the colour of the picture where it was, flying through space
towards the viewer and coming round again from far away. The CycoreFX manual names the controls
and says what each is for (Scatter, 0 keeping the stars where they were put; Speed, towards the
viewer, below 0 away; Phase, lining the field up with the layer as it was; Grid Spacing, how
close the stars are, which also sizes them; Size; Blend w. Original, 100 the layer alone). It
publishes no formula, so the rule, the units, the ranges and the settings when added are ours.
The field is 3D, seen through the composition's camera when the file has one (as Particle
Space, D-610), else through an eye of its own. Nothing is carried from frame to frame
(document 20): every frame sums the speed from the in point.

On a drawing w by h, with the settings held inside their ranges, read at frame n (the keyed
Speed at every frame from the in point i_0 to n):

1. The grid: g = `grid_spacing` h / 270 pixels; C = ceil(w / g) columns and R = ceil(h / g) rows of
   stars, star (a, b) put at (x_0, y_0) = (w / 2 + (a - (C - 1) / 2) g, h / 2 + (b - (R - 1) / 2) g)
   on the layer's plane, its number b C + a. It takes the picture's colour there (the bilinear
   read below); a star where the picture is clear is not drawn.
2. Scatter: s = `scatter` / 100, u_k = (U(0, a, b, 0, k) + 1) / 2, U Noise's hash (D-119) at seed
   0. The star is at x = x_0 + s g (u_0 - 1/2), y = y_0 + s g (u_1 - 1/2), and at depth z_0 = s (3 F
   u_2 - 0.9 F) behind the plane (z above 0 away from the viewer), F = 1.5 h.
3. Flight: T = h (S + `phase`), S the trapezoid sum of Speed from i_0 to n divided by the frame
   rate (Speed in picture heights a second, Phase in picture heights). The star is at depth z =
   -0.9 F + ((z_0 + 0.9 F - T) mod 3 F): it flies towards the viewer and, once 0.9 F in front of
   the plane, comes round again 2.1 F behind it. With T = 0 and Scatter 0 every star is on the
   plane.
4. The eye: with the composition's camera, a its place in the layer's own pixels and A its
   distance from the layer's plane (compose works these out, as for Particle Space); otherwise
   a = (w / 2, h / 2) and A = F. A star with A + z <= h / 100 is not drawn; the others are drawn at
   a + ((x, y) - a) k, k = A / (A + z), radius r = (`size` / 100) (g / 4) k, so at Size 100 a star
   on the plane is half as wide as the grid is spaced.
5. Drawing, Particle Space's disc (D-610): when r < 0.7 the star is drawn with r = 0.7 and its
   opacity times (r / 0.7)^2; it covers a pixel whose centre is e from it by clamp(r - e + 1/2,
   0, 1). The stars go onto a clear layer farthest first (by z, then by number), each its
   straight colour at its alpha times its opacity times its covering, laid over: s + d (1 -
   s_alpha). Last the result is mixed with the picture, by m = `blend_with_original` / 100: out =
   stars (1 - m) + picture m.

The bilinear read, premultiplied and linear: fx = clamp(X - 1/2, 0, w - 1), likewise fy; the
four pixels round it, the last row and column repeated at the edge (Bubbles', D-671).

`scatter` 0 to 100, 100 when added; `speed` -100 to 100, 1; `phase` -1000 to 1000, 0;
`grid_spacing` 0.5 to 100, 4; `size` 0 to 1000, 100; `blend_with_original` 0 to 100, 0. Every
number keyable. Every length is a share of the drawing's height, so a draft needs no scaling.
The layer does not grow: stars past its edge are cut there.

**This file never runs the build's code path.** It lays every star onto the pixels round it,
one star after another, where the build bins the stars into tiles and works a pixel at a time,
in double precision from the drawings' 8-bit values.

Every case is a composition 48 pixels by 32 holding one drawing the same size: Bubbles' colour
field, or the field with its right half clear. The projects go into `Fixtures/star_burst`, the
expected frames into `Fixtures/star_burst/expected_star_burst.json`. Every case is "not built
yet" until B-556 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/star_burst_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
import smooth_reference as S  # noqa: E402
import bubbles_reference as BU  # noqa: E402
import hair_reference as HR  # noqa: E402

W, H, FPS = 48, 32, 24
assert (BU.W, BU.H) == (W, H)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "star_burst"
TOLERANCE = 2e-5  # document 25's default for a filter
TYPE = "core.star_burst"
SETTINGS = [("scatter", 0, 100, 100), ("speed", -100, 100, 1), ("phase", -1000, 1000, 0),
            ("grid_spacing", 0.5, 100, 4), ("size", 0, 1000, 100),
            ("blend_with_original", 0, 100, 0)]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
NEAREST = []  # how near each star came to the field's wrap, for check()


# --- the rule -------------------------------------------------------------------------------

def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def settings(c, f):
    return {n: clamp(value_at(c[n], f), *RANGES[n]) for n in NAMES}


def summed(c, n):
    """Step 3's S: the speed summed from the in point, in seconds."""
    i0 = c["in_frame"]
    s = [settings(c, i0 + j)["speed"] for j in range(max(0, n - i0) + 1)]
    return sum((s[j] + s[j + 1]) / 2 for j in range(len(s) - 1)) / c["rate"]


def stars(src, v, S_, view, w=W, h=H):
    """Steps 1 to 4: the stars drawn, farthest first, as (z, number, centre, radius, colour)."""
    g = v["grid_spacing"] * h / 270
    cols, rows = math.ceil(w / g), math.ceil(h / g)
    s = v["scatter"] / 100
    F = 1.5 * h
    T = h * (S_ + v["phase"])
    (ax, ay), A = ((view[0], view[1]), view[2]) if view is not None else ((w / 2, h / 2), F)
    out = []
    for b in range(rows):
        for a in range(cols):
            x0, y0 = w / 2 + (a - (cols - 1) / 2) * g, h / 2 + (b - (rows - 1) / 2) * g
            col = BU.bilinear(src, x0, y0, w, h)
            if col[3] <= 0:
                continue
            u = [(U(0, a, b, 0, k) + 1) / 2 for k in range(3)]
            x, y = x0 + s * g * (u[0] - 0.5), y0 + s * g * (u[1] - 0.5)
            z0 = s * (3 * F * u[2] - 0.9 * F)
            m = (z0 + 0.9 * F - T) % (3 * F)
            NEAREST.append(min(m, 3 * F - m) / F)
            z = -0.9 * F + m
            if A + z <= h / 100:
                continue
            k = A / (A + z)
            out.append((z, b * cols + a, (ax + (x - ax) * k, ay + (y - ay) * k),
                        v["size"] / 100 * g / 4 * k, col))
    out.sort(key=lambda t: (-t[0], t[1]))
    return out


def star_burst(src, v, S_, view, w=W, h=H):
    out = [[0.0] * 4 for _ in range(w * h)]
    for _, _, (px, py), r, col in stars(src, v, S_, view, w, h):
        op = 1.0
        if r < 0.7:
            op, r = (r / 0.7) ** 2, 0.7
        reach = r + 0.5
        for Y in range(max(0, math.floor(py - reach - 1)), min(h, math.floor(py + reach) + 2)):
            for X in range(max(0, math.floor(px - reach - 1)), min(w, math.floor(px + reach) + 2)):
                cv = clamp(r - math.hypot(X + 0.5 - px, Y + 0.5 - py) + 0.5, 0, 1)
                if cv > 0:
                    sp = [q * op * cv for q in col]  # the straight colour times its alpha: premultiplied
                    d = out[Y * w + X]
                    out[Y * w + X] = [sp[k] + d[k] * (1 - sp[3]) for k in range(4)]
    m = v["blend_with_original"] / 100
    return [[o[k] * (1 - m) + p[k] * m for k in range(4)] for o, p in zip(out, src)]


def valid(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        values = [k[1] for k in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    return True


def view_of(c):
    """What compose works out for the composition camera, in the layer's own pixels."""
    if c["camera"] is None:
        return None
    x, y, depth, zoom = c["camera"]
    assert zoom == -depth, "only cameras that draw the layer's plane 1:1"
    return (x - c["shift"], y, 0 - depth)


def move(c):
    m = c["shift"]
    if c["camera"] is not None:
        assert c["camera"][1] == H / 2 and c["camera"][0] == int(c["camera"][0])
        m += int(W / 2 - c["camera"][0])
    return m


def render(c, f):
    src = BU.working(c["drawing"])
    if not valid(c) or f < c["in_frame"]:
        return BU.frame(src, move(c))
    return BU.frame(star_burst(src, settings(c, f), summed(c, f), view_of(c)), move(c))


def plain(c):
    return BU.frame(BU.working(c["drawing"]), move(c))


# --- the cases ------------------------------------------------------------------------------

def case(drawing="field", rate=FPS, in_frame=0, shift=0, camera=None, **given):
    """Every setting as added, then the case's; `camera` is (x, y, depth, zoom) for a
    composition camera whose plane at depth 0 is drawn 1:1."""
    c = {s[0]: s[3] for s in SETTINGS}
    for key, v in given.items():
        assert key in c, key
        c[key] = v
    c.update(drawing=drawing, rate=rate, in_frame=in_frame, shift=shift, camera=camera)
    return c


# A close look: a grid of about 21 by 14 stars, each about 1.2 pixels across on the plane.
BIG = dict(grid_spacing=20)
# The picture as dots: no scatter, standing still.
DOTS = dict(BIG, scatter=0, speed=0)
F = 1.5 * H


def big(**kw):
    return case(**{**BIG, **kw})


def dots_(**kw):
    return case(**{**DOTS, **kw})


CASES = {
    "FX-SBURST-001": ("As added: a fine field of about 7000 faint star dots, the picture's colours, "
                      "scattered through the depth and flying towards the viewer.", case(),
                      (0, 12)),
    "FX-SBURST-002": ("A close look: about 300 stars, nearer ones larger and further out from the "
                      "middle.", big(), (12,)),
    "FX-SBURST-003": ("Scatter 0, Speed 0: the picture as a grid of dots on its own plane.",
                      dots_(), (12,)),
    "FX-SBURST-004": ("Scatter 0, Speed 1 at frame 12: the plane of dots half a picture height "
                      "nearer, so half as large again and spread from the middle.",
                      dots_(speed=1), (12,)),
    "FX-SBURST-005": ("Scatter 0, Speed 0, Phase 0.5: the same as FX-SBURST-004.",
                      dots_(phase=0.5), (12,)),
    "FX-SBURST-006": ("Scatter 0, Speed -1 at frame 12: the plane of dots further off, smaller "
                      "and drawn in towards the middle.", dots_(speed=-1), (12,)),
    "FX-SBURST-007": ("Size 300: the close look's stars three times as wide.", big(size=300),
                      (12,)),
    "FX-SBURST-008": ("Blend w. Original 50: the close look half over the picture.",
                      big(blend_with_original=50), (12,)),
    "FX-SBURST-009": ("Blend w. Original 100: the picture alone.", big(blend_with_original=100),
                      (12,)),
    "FX-SBURST-010": ("The picture with its right half clear: stars only from the left half.",
                      big(drawing="half"), (12,)),
    "FX-SBURST-011": ("Speed keyed from 0 at frame 0 to 2 at frame 12: the field speeding up.",
                      big(speed=keyed((0, 0), (12, 2))), (0, 6, 12)),
    "FX-SBURST-012": ("FX-SBURST-002 moved 3 pixels right: the same, moved.", big(shift=3),
                      (12,)),
    "FX-SBURST-013": ("The composition's camera in the middle, 48 pixels off, as the effect's own "
                      "eye: the same as FX-SBURST-002.", big(camera=(W // 2, H / 2, -F, F)), (12,)),
    "FX-SBURST-014": ("The camera moved to x 20: the field seen from the left of its middle.",
                      big(camera=(20, H / 2, -F, F)), (12,)),
    "FX-SBURST-015": ("The camera dollied in to 24 pixels off: the near stars nearer, those "
                      "behind it gone.", big(camera=(W // 2, H / 2, -24, 24)), (12,)),
    "FX-SBURST-016": ("In point at frame 6: at frame 12 the same as the close look at frame 6.",
                      big(in_frame=6), (12,)),
    "FX-SBURST-017": ("Grid Spacing 100: a few large stars, 5 by 3.", case(grid_spacing=100),
                      (12,)),
    "FX-SBURST-018": ("Scatter 0, Speed 3 at frame 24: the plane of dots has flown past the "
                      "viewer and come round again behind, half its size.", dots_(speed=3),
                      (24,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-SBURST-019": ("Scatter 101, above 100.", big(scatter=101)),
    "FX-SBURST-020": ("Speed 101, above 100.", big(speed=101)),
    "FX-SBURST-021": ("Phase 1001, above 1000.", big(phase=1001)),
    "FX-SBURST-022": ("Grid Spacing 0.4, below 0.5.", big(grid_spacing=0.4)),
    "FX-SBURST-023": ("Size -1, below 0.", big(size=-1)),
    "FX-SBURST-024": ("Blend w. Original 101, above 100.", big(blend_with_original=101)),
    "FX-SBURST-025": ("Speed keyed to 200 at frame 4, above 100.",
                      big(speed=keyed((0, 0), (4, 200)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = HR.project_json(fx, HR.case(drawing=c["drawing"], rate=c["rate"], in_frame=c["in_frame"],
                                    shift=c["shift"]))
    fx0 = p["compositions"][0]["layers"][0]["effects"][0]
    fx0["type_id"], fx0["parameters"] = TYPE, {n: setting_json(c[n]) for n in NAMES}
    if c["camera"] is not None:
        x, y, depth, zoom = c["camera"]
        p["compositions"][0]["camera"] = {"position": prop([x, y]), "depth": prop(depth),
                                          "zoom": prop(zoom)}
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
        src = BU.working(c["drawing"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {len(stars(src, settings(c, int(f)), summed(c, int(f)), view_of(c)))} stars"
            for f in rendered))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        assert not valid(c), fx
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_star_burst.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def near(a, b, tol=1e-12):
    return all(abs(x - y) <= tol for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    # No star sat within a millionth of F of the wrap, where the two ways of working could
    # disagree about which side it falls.
    assert min(NEAREST) > 1e-6, min(NEAREST)
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    two = c["FX-SBURST-002"]["12"]
    assert near(c["FX-SBURST-005"]["12"], c["FX-SBURST-004"]["12"])
    assert near(c["FX-SBURST-013"]["12"], two)
    assert near(c["FX-SBURST-009"]["12"], plain(case()))
    k = c["FX-SBURST-011"]
    assert k["0"] != k["6"] != k["12"]
    for n in (3, 4, 6, 7, 8, 10, 14, 15, 17):
        assert c["FX-SBURST-%03d" % n]["12"] != two, n
    # The in point: frame 12 from an in point at 6 is frame 6 from an in point at 0.
    src = BU.working("field")
    assert near(c["FX-SBURST-016"]["12"],
                BU.frame(star_burst(src, settings(big(), 6), summed(big(), 6), None), 0))
    # FX-SBURST-004's plane is at z = -16, drawn 1.5 times as large; FX-SBURST-018's at z = 48,
    # drawn half as large.
    for cs, f, zz in ((dots_(speed=1), 12, -16), (dots_(speed=3), 24, 48)):
        zs = {round(t[0], 9) for t in stars(src, settings(cs, f), summed(cs, f), None)}
        assert zs == {zz}, (zs, zz)
    # The half picture's stars all come from its left half.
    half = stars(BU.working("half"), settings(big(scatter=0), 12), 0, None)
    assert half and all(t[2][0] < W / 2 + 1 for t in half)
    moved = c["FX-SBURST-012"]["12"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two[y * W:y * W + W - 3]
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
