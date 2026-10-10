"""Threads, worked a second way.

D-425 adds `core.threads`, our name for CycoreFX's CC Threads: the layer woven into a cloth of
threads, each thread taking the layer's colours where it lies. CycoreFX's manual (CycoreFX HD
1.8.9, pages 45 and 46) says what each control does in a sentence and publishes no formula,
ranges or defaults; the rule below is this program's own reading of it, and nothing is ported.

With the holder's input O (linear, premultiplied), W by H its drawing, and p a pixel's centre:

1. The cloth's frame. c = (`center_x` / 100 W, `center_y` / 100 H) (the manual: "the offset of
   the threads, and the rotation point"); d = p - c turned by -`direction` degrees (on screen,
   clockwise for a positive direction) to (u, v): u = dx cos t + dy sin t, v = -dx sin t + dy cos t.
2. The threads. The warp runs down v, threads `width` pixels apart across u: thread i =
   floor(u / width), its middle at (i + 1/2) width, e = u - (i + 1/2) width. The weft runs along u,
   threads `height` pixels apart down v: thread j = floor(v / height), f = v - (j + 1/2) height.
   Each thread is `coverage` per cent of its spacing wide (the manual: at 100 the threads cover
   almost all; at 50, half, a quarter left clear each side): half widths hw = coverage / 100
   width / 2, hh = coverage / 100 height / 2. A pixel's share of a thread, m = the part of the
   pixel's width [e - 1/2, e + 1/2] inside [-hw, hw]: mw for the warp, mf for the weft (by f and
   hh). Only the nearest thread each way counts, so at coverage 100 a pixel on the seam between
   two threads takes half of one.
3. The weave. With n = `overlaps` taken whole, the warp is over the weft where (i + j) mod 2n < n,
   under elsewhere: n = 1 a plain weave, each thread over one and under the next; n = 2 over two
   and under two, a diagonal twill.
4. Texture (the manual: "a textured feeling", more contrast between crossing threads): each
   thread is shaded round, its colour times 1 - T min(1, (e / hw)^2) (by f and hh for the weft),
   T = `texture` / 100: full in its middle, darker towards its sides. A thread of no width is not
   shaded.
5. Shadowing (the manual: the shadow the threads cast on the threads beneath, "a kind of ambient
   illumination"): the thread beneath is darkened by S (1 + g) / 2, S = `shadowing` / 100, g =
   1 - (|e_o| - h_o) / h_o held to 0..1, where e_o and h_o are the thread above's offset and half
   width: half of S everywhere beneath, all of S under the upper thread and at its edge, fading
   over its half width beside it. With the thread above of no width, g is 0.
6. The cloth: the upper thread is O mw (its colour shaded) and the lower O mf (its colour shaded
   and shadowed), the upper over the lower: colour U + L (1 - m_upper), covering O.a m_upper +
   O.a m_lower (1 - m_upper). Where there is no thread the cloth is clear. The layer does not
   grow. For a draft the width and height are distances; the centre is a share.

`width` and `height` 1 to 1000 pixels, 50; `overlaps` 1 to 10, 1, taken whole (CC's is not
keyable; ours is, read whole at each frame); `direction` -3600 to 3600 degrees, 0; `center` -1000
to 1000 per cent each way, 50, 50; `coverage` 0 to 100, 90; `shadowing` 0 to 100, 50; `texture` 0
to 100, 0. Every number keyable. The values when added are chosen here; the manual gives none.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values, where the build works in single precision on its buffers.

Every case is a project of one composition 16 by 10, eight frames at 24 a second, in
`Fixtures/threads/`: Blobbylize's drawings, the holder at the top with the effect. The expected
pixels are in `Fixtures/threads/expected_threads.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/threads_reference.py
"""

import json
import sys
from math import cos, floor, radians, sin
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import blobbylize_reference as B  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "threads"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 8
assert (B.W, B.H) == (W, H)
RANGES = {"width": (1, 1000), "height": (1, 1000), "overlaps": (1, 10),
          "direction": (-3600, 3600), "center": (-1000, 1000), "coverage": (0, 100),
          "shadowing": (0, 100), "texture": (0, 100)}
NAMES = ("width", "height", "overlaps", "direction", "center", "coverage", "shadowing", "texture")


# --- the rule -------------------------------------------------------------------------------

def held(c, k, frame):
    v = value_at(c[k], frame)
    lo, hi = RANGES[k]
    return [min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple)) else min(hi, max(lo, v))


def share(e, half):
    """Step 2: the part of a pixel's width about e inside a thread of half width `half`."""
    return max(0.0, min(e + 0.5, half) - max(e - 0.5, -half))


def shade(e, half, t):
    """Step 4."""
    return 1 - t * min(1.0, (e / half) ** 2) if half > 0 else 1.0


def shadow(e, half, s):
    """Step 5: what is left of the thread beneath, by the thread above's offset and half width."""
    g = min(1.0, max(0.0, 1 - (abs(e) - half) / half)) if half > 0 else 0.0
    return 1 - s * (1 + g) / 2


def threads(o, s):
    w, h = o["w"], o["h"]
    cx, cy = s["center"][0] / 100 * w, s["center"][1] / 100 * h
    t = radians(s["direction"])
    ct, st = cos(t), sin(t)
    n = floor(s["overlaps"])
    hw, hh = s["coverage"] / 100 * s["width"] / 2, s["coverage"] / 100 * s["height"] / 2
    tex, sh = s["texture"] / 100, s["shadowing"] / 100
    out = []
    for y in range(h):
        for x in range(w):
            dx, dy = x + 0.5 - cx, y + 0.5 - cy
            u, v = dx * ct + dy * st, -dx * st + dy * ct
            i, j = floor(u / s["width"]), floor(v / s["height"])
            e, f = u - (i + 0.5) * s["width"], v - (j + 0.5) * s["height"]
            warp = (share(e, hw), shade(e, hw, tex), e, hw)
            weft = (share(f, hh), shade(f, hh, tex), f, hh)
            up, down = (warp, weft) if (i + j) % (2 * n) < n else (weft, warp)
            dark = shadow(up[2], up[3], sh)
            p = o["px"][y * w + x]
            out.append([p[ch] * (up[0] * up[1] + down[0] * down[1] * dark * (1 - up[0]))
                        for ch in range(3)] + [p[3] * (up[0] + down[0] * (1 - up[0]))])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

def case(holder="photo", shift=(0, 0), **kw):
    c = {"width": 50, "height": 50, "overlaps": 1, "direction": 0, "center": (50, 50),
         "coverage": 90, "shadowing": 50, "texture": 0}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def settings(c, frame):
    return {k: held(c, k, frame) for k in NAMES}


def render(c, frame):
    out = threads(B.decoded(c["holder"]), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = B.decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


# Threads 4 pixels apart each way, unshadowed: the weave plain to see on the photo's 2-pixel
# squares.
SMALL = dict(width=4, height=4, shadowing=0)
CASES = {
    "FX-THREADS-001": ("As added: threads 50 pixels apart, coverage 90, shadowing 50, centred on "
                       "the drawing, so its middle falls between threads: the gap between two "
                       "warp threads and the gap between two weft threads cross in the middle, "
                       "columns 6 to 9 and rows 3 to 6 clear, half clear beside them; elsewhere "
                       "the photo, the thread above hiding the one beneath, which shows "
                       "shadowed only at the upper thread's soft edge.", case(), (0,)),
    "FX-THREADS-002": ("Threads 4 pixels apart, unshadowed: a plain weave, each thread 3.6 "
                       "pixels wide in the photo's colours, the edges of each pixel-wide gap "
                       "between them soft.", case(**SMALL), (0,)),
    "FX-THREADS-003": ("FX-THREADS-002 at coverage 100: the threads meet edge to edge, their "
                       "seams on pixel edges, so the photo shows whole, untouched.", case(**dict(SMALL, coverage=100)), (0,)),
    "FX-THREADS-004": ("FX-THREADS-002 at coverage 50: threads 2 pixels wide with 2-pixel gaps; "
                       "where the gaps cross, clear.", case(**dict(SMALL, coverage=50)), (0,)),
    "FX-THREADS-005": ("Coverage 0: no threads, everything clear.",
                       case(**dict(SMALL, coverage=0)), (0,)),
    "FX-THREADS-006": ("FX-THREADS-002 with Overlaps 2: each thread over two and under two, a "
                       "diagonal twill.", case(**dict(SMALL, overlaps=2, shadowing=100)), (0,)),
    "FX-THREADS-007": ("FX-THREADS-006 with Overlaps 3.", case(**dict(SMALL, overlaps=3,
                                                                    shadowing=100)), (0,)),
    "FX-THREADS-008": ("FX-THREADS-002 turned 30 degrees clockwise about the middle.",
                       case(**dict(SMALL, direction=30)), (0,)),
    "FX-THREADS-009": ("Shadowing 100: where the threads cross, the one beneath darkened, "
                       "fully beside the one above, half where it is further off; the one "
                       "above as in FX-THREADS-002.", case(**dict(SMALL, shadowing=100)), (0,)),
    "FX-THREADS-010": ("Texture 100: each thread full in its middle and black at its edges, "
                       "round.", case(**dict(SMALL, texture=100)), (0,)),
    "FX-THREADS-011": ("Width 6, height 3, coverage 80, shadowing 60, texture 40: wide warp "
                       "threads crossing narrow weft threads.",
                       case(width=6, height=3, coverage=80, shadowing=60, texture=40), (0,)),
    "FX-THREADS-012": ("Centre 25, 50, the point (4, 5): the cloth slides a whole thread left, so "
                       "each crossing swaps which thread is on top.",
                       case(**dict(SMALL, center=(25, 50), shadowing=100)), (0,)),
    "FX-THREADS-013": ("On the shapes drawing, clear between its blocks: the threads take the "
                       "blocks' colours, and are clear where the drawing is.",
                       case(holder="shapes", **dict(SMALL, shadowing=100, texture=50)), (0,)),
    "FX-THREADS-014": ("FX-THREADS-009 on the holder moved 2 right and 1 down: the same, moved; "
                       "the cloth is woven on the drawing before it moves.",
                       case(shift=(2, 1), **dict(SMALL, shadowing=100)), (0,)),
    "FX-THREADS-015": ("Direction keyed from 0 at frame 0 to 60 at frame 4, linear: frame 2 is "
                       "FX-THREADS-008, frame 0 FX-THREADS-002.",
                       case(**dict(SMALL, direction=keyed((0, 0), (4, 60)))), (0, 2, 4)),
    "FX-THREADS-016": ("Overlaps keyed from 1 at frame 0 to 3 at frame 4, linear: read whole, "
                       "frame 1 is plain still, frame 2 is FX-THREADS-006.",
                       case(**dict(SMALL, shadowing=100, overlaps=keyed((0, 1), (4, 3)))),
                       (1, 2, 4)),
    "FX-THREADS-017": ("Width and height 1, coverage 50: threads half a pixel wide, every pixel "
                       "half warp and half weft.", case(width=1, height=1, coverage=50), (0,)),
    "FX-THREADS-018": ("Coverage eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                       "overshoots: at frame 2 it would pass 100, is held at 100, as frame 4 "
                       "is.", case(**dict(SMALL, coverage=keyed((0, 0, OVERSHOOT), (4, 100)))),
                       (0, 2, 4)),
    "FX-THREADS-019": ("Width 5, height 3, Overlaps 2, direction -20, coverage 85, shadowing 70, "
                       "texture 60, centre 40, 60, on the ramp: the controls together.",
                       case(holder="ramp", width=5, height=3, overlaps=2, direction=-20,
                            coverage=85, shadowing=70, texture=60, center=(40, 60)), (0,)),
}

INVALID = {
    "FX-THREADS-020": ("Width 0.5, below 1.", case(width=0.5)),
    "FX-THREADS-021": ("Height 1001, above 1000.", case(height=1001)),
    "FX-THREADS-022": ("Overlaps 0, below 1.", case(overlaps=0)),
    "FX-THREADS-023": ("Overlaps 11, above 10.", case(overlaps=11)),
    "FX-THREADS-024": ("Direction 3601, past ten turns.", case(direction=3601)),
    "FX-THREADS-025": ("A centre 50, -1001, past ten heights.", case(center=(50, -1001))),
    "FX-THREADS-026": ("Coverage 101, above 100.", case(coverage=101)),
    "FX-THREADS-027": ("Shadowing -1, below 0.", case(shadowing=-1)),
    "FX-THREADS-028": ("Texture 101, above 100.", case(texture=101)),
}


def effect(c):
    return {"instance_id": "fx-1", "type_id": "core.threads", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-" + c["holder"], position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect(c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in ("photo", "shapes", "ramp")],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [holder])]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("photo", "shapes", "ramp"):
        (OUT / "media" / f"{name}.png").write_bytes(png(B.DRAWINGS[name]))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_threads.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    photo, shapes = plain(case()), plain(case(holder="shapes"))
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for u, v in zip(p, q)  # noqa: E731
                                    for a, b in zip(u, v))
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= u <= p[3] + 1e-12
                                                           for u in p[:3]), (fx, p)

    # The shares: a whole pixel inside, half on the edge, none outside, a thin thread's width.
    assert share(0.5, 1.8) == 1 and abs(share(1.8, 1.8) - 0.5) < 1e-12 and abs(share(2.3, 1.8)) < 1e-12
    assert share(0, 0.25) == 0.5 and share(1, 0) == 0
    assert shadow(0, 2, 1) == 0 and shadow(3, 2, 1) == 0.25 and shadow(5, 2, 1) == 0.5

    one = c["FX-THREADS-001"]["0"]
    assert all(one[at(x, y)] == [0.0] * 4 for x in range(6, 10) for y in range(3, 7))
    assert one[at(10, 4)][3] == 0.5 and one[at(1, 1)] == photo[at(1, 1)]
    assert one[at(5, 1)] != photo[at(5, 1)]  # the weft beneath, shadowed, by a half-wide warp
    two = c["FX-THREADS-002"]["0"]
    # Inside both threads where they cross, unshadowed and untextured: the photo itself.
    assert two[at(9, 6)] == photo[at(9, 6)] and two[at(8, 5)] != photo[at(8, 5)]
    alpha = lambda px: sum(p[3] for p in px)  # noqa: E731
    three, four = c["FX-THREADS-003"]["0"], c["FX-THREADS-004"]["0"]
    assert alpha(four) < alpha(two) < alpha(three) and three == photo
    assert four[at(8, 5)] == [0.0] * 4
    assert all(p == [0.0] * 4 for p in c["FX-THREADS-005"]["0"])
    six, seven = c["FX-THREADS-006"]["0"], c["FX-THREADS-007"]["0"]
    nine = c["FX-THREADS-009"]["0"]
    assert six not in (nine, seven) and alpha(six) == alpha(nine) == alpha(two)
    eight = c["FX-THREADS-008"]["0"]
    assert eight != two
    # Shadowing and texture darken and never uncover; the covering stays.
    for other in (nine, c["FX-THREADS-010"]["0"]):
        assert all(abs(p[3] - q[3]) < 1e-15 and all(p[i] <= q[i] + 1e-15 for i in range(3))
                   for p, q in zip(other, two)) and other != two
    assert c["FX-THREADS-012"]["0"] != nine
    thirteen = c["FX-THREADS-013"]["0"]
    assert all(thirteen[i] == [0.0] * 4 for i in range(W * H) if shapes[i][3] == 0)
    assert any(p[3] > 0 for p in thirteen)
    moved = c["FX-THREADS-014"]["0"]
    assert all(moved[at(x, y)] == nine[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    k = c["FX-THREADS-015"]
    assert k["0"] == two and near(k["2"], eight) and k["4"] != k["2"]
    k = c["FX-THREADS-016"]
    assert k["1"] == nine and k["2"] == six and k["4"] == seven
    seventeen = c["FX-THREADS-017"]["0"]
    assert all(abs(p[3] - 0.75 * q[3]) < 1e-12 for p, q in zip(seventeen, photo))
    k = c["FX-THREADS-018"]
    assert k["0"] == c["FX-THREADS-005"]["0"] and k["2"] == k["4"] == three
    print("checked")


if __name__ == "__main__":
    main()
