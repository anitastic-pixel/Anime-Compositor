"""Lightning Bolt, worked a second way.

D-190 adds `core.lightning_bolt`: a jagged, forking bolt of light drawn from one point of the
layer to another, a white-hot core in a coloured glow, redrawn as a new bolt every few frames. It
is After Effects' Lightning and Advanced Lightning in purpose, and this program's own rule: a
bolt made by halving its line again and again and pushing each middle aside by a number from
Noise's hash (D-119), with forks that thin to nothing. Nothing is ported. Document 21 is the rule
in words; this file is the reference for the numbers document 25 pins against it.

The rule. `start` and `end` are points in per cent of the drawing's own size, S and E in its own
space, the top-left corner (0, 0). With `opacity` 0, or `width` and `glow` both 0, the output is
the input. Otherwise, with m = floor(f / hold) for the composition frame f, the bolt is a list of
segments (P, Q, wP, wQ, depth, bolt, key), starting as the one (S, E, 1, 1, 0, 0, 1), halved
`detail` times. Each halving replaces every segment, in order, with L = |Q - P|:

- L = 0: the segment, kept whole.
- Otherwise, with r_j = U(seed, key, bolt, m, j) for j = 0 to 4, U Noise's hash, n = (-(Q - P).y,
  (Q - P).x) / L and the middle M = (P + Q) / 2 + n r_0 jagged / 100 L / 2, weight wM = (wP + wQ)
  / 2: the two halves (P, M, wP, wM, depth, bolt, 2 key) and (M, Q, wM, wQ, depth, bolt, 2 key +
  1); then, when depth < 3 and (r_1 + 1) / 2 < branches / 100, a fork (M, B, wM / 2, 0, depth +
  1, 1024 bolt + key, 1), where B = M + L (0.3 + 0.15 (r_4 + 1)) turn((Q - P) / L, a), a = (15 +
  15 (r_2 + 1)) degrees, negative when r_3 < 0, and turn(v, a) = (v.x cos a - v.y sin a, v.x sin
  a + v.y cos a).

A segment's key and bolt name it for good: a fork, or a finer halving, never moves what was
already drawn. At a point X, for each segment, t = clamp((X - P) . (Q - P) / L^2, 0, 1) (0 when L
= 0), d = |X - P - t (Q - P)|, its weight w = wP + t (wQ - wP), its core k = clamp(min(d + 0.5,
w width / 2) - max(d - 0.5, -w width / 2), 0, 1), the share of a pixel-wide box a line w width
thick covers, and its glow g = w (1 - d / (w glow))^2 where d < w glow, else 0. C and G are the
largest core and glow of any segment at the output pixel's centre, and the light is C core_colour
+ (1 - C) G glow_colour, both linear. With s = opacity / 100 and O the input, the output is O.rgb
+ s light and O.a + s (C + (1 - C) G) (1 - O.a): added as light, not clamped. Nothing grows: the
bolt is drawn inside the layer. A draft scales width and glow as distances.

A fork is a yes-or-no choice from a number, and so is its side: `check` asserts that no segment
any case reaches has (r_1 + 1) / 2 within 1e-5 of branches / 100, nor a fork's r_3 within 1e-5 of
0.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build writes single precision into its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says, drawn below. The drawing goes into `Fixtures/lightning_bolt/media`, the projects into
`Fixtures/lightning_bolt`, and the expected frames into
`Fixtures/lightning_bolt/expected_lightning_bolt.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_bolt_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from noise_reference import u as U  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_bolt"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-5      # the spec's margin: no reached segment this close to a fork's cliff
RANGES = {"start": (-1000, 1000), "end": (-1000, 1000), "jagged": (0, 100), "detail": (1, 8),
          "branches": (0, 100), "width": (0, 100), "glow": (0, 500), "opacity": (0, 100),
          "hold": (1, 100), "seed": (0, 100000)}
FLOORED = ("detail", "hold", "seed")
NUMBERS = ("start", "end", "jagged", "detail", "branches", "width", "glow", "opacity", "hold",
           "seed")
NAMES = NUMBERS + ("color", "glow_color")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def turn(v, a):
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return (v[0] * c - v[1] * s, v[0] * s + v[1] * c)


def bolt(S_, E, jagged, detail, branches, seed, m, reached=None):
    """The segments, (P, Q, wP, wQ, depth, bolt, key). `reached`, when given, collects each
    split segment's (r_1, fork or not, r_3) for the cliff check."""
    segs = [(S_, E, 1.0, 1.0, 0, 0, 1)]
    for _ in range(detail):
        out = []
        for P, Q, wp, wq, depth, b, key in segs:
            dx, dy = Q[0] - P[0], Q[1] - P[1]
            L = math.hypot(dx, dy)
            if L == 0:
                out.append((P, Q, wp, wq, depth, b, key))
                continue
            r = [U(seed, key, b, m, j) for j in range(5)]
            a = r[0] * jagged / 100 * L / 2
            M = ((P[0] + Q[0]) / 2 - dy / L * a, (P[1] + Q[1]) / 2 + dx / L * a)
            wm = (wp + wq) / 2
            out += [(P, M, wp, wm, depth, b, 2 * key), (M, Q, wm, wq, depth, b, 2 * key + 1)]
            fork = depth < 3 and (r[1] + 1) / 2 < branches / 100
            if reached is not None and depth < 3:
                reached.append((r[1], fork, r[3]))
            if fork:
                v = turn((dx / L, dy / L), (15 + 15 * (r[2] + 1)) * (-1 if r[3] < 0 else 1))
                lb = L * (0.3 + 0.15 * (r[4] + 1))
                out.append((M, (M[0] + v[0] * lb, M[1] + v[1] * lb), wm / 2, 0.0, depth + 1,
                            1024 * b + key, 1))
        segs = out
    return segs


def reach(seg, width, glow):
    """How far past its line a segment can light anything."""
    w = max(seg[2], seg[3])
    return max(w * width / 2 + 0.5, w * glow)


def light(seg, width, glow, X, Y):
    """A segment's (core, glow) at the point (X, Y)."""
    P, Q, wp, wq = seg[:4]
    dx, dy = Q[0] - P[0], Q[1] - P[1]
    L2 = dx * dx + dy * dy
    t = 0.0 if L2 == 0 else min(1.0, max(0.0, ((X - P[0]) * dx + (Y - P[1]) * dy) / L2))
    d = math.hypot(X - P[0] - t * dx, Y - P[1] - t * dy)
    w = wp + t * (wq - wp)
    half = w * width / 2
    k = min(1.0, max(0.0, min(d + 0.5, half) - max(d - 0.5, -half)))
    r = w * glow
    g = w * (1 - d / r) ** 2 if d < r else 0.0
    return k, g


def field(segs, width, glow, w, h, ox=0.0, oy=0.0):
    """C and G at every pixel centre of a w by h buffer whose drawing starts at (ox, oy)."""
    C, G = [0.0] * (w * h), [0.0] * (w * h)
    for seg in segs:
        e = reach(seg, width, glow)
        (px, py), (qx, qy) = seg[0], seg[1]
        for y in range(max(0, math.floor(min(py, qy) - e + oy)), min(h, math.ceil(max(py, qy) + e + oy) + 1)):
            for x in range(max(0, math.floor(min(px, qx) - e + ox)), min(w, math.ceil(max(px, qx) + e + ox) + 1)):
                k, g = light(seg, width, glow, x - ox + 0.5, y - oy + 0.5)
                i = y * w + x
                C[i], G[i] = max(C[i], k), max(G[i], g)
    return C, G


def lightning_bolt(pixels, start, end, jagged, detail, branches, width, glow, opacity, hold, seed,
                   color, glow_color, frame_no, size_px=(W, H)):
    """The output pixels. Numbers are already held; detail, hold and seed already floored."""
    w, h = size_px
    working = [R.working(p) for p in pixels]
    if opacity == 0 or (width == 0 and glow == 0):
        return working
    m = frame_no // hold
    S_ = (start[0] / 100 * w, start[1] / 100 * h)
    E = (end[0] / 100 * w, end[1] / 100 * h)
    C, G = field(bolt(S_, E, jagged, detail, branches, seed, m), width, glow, w, h)
    core = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    halo = [srgb_to_linear(v / 255) for v in R.hex_color(glow_color.lower())]
    s = opacity / 100
    out = []
    for o, c, g in zip(working, C, G):
        lit = [c * core[ch] + (1 - c) * g * halo[ch] for ch in range(3)]
        out.append([o[ch] + s * lit[ch] for ch in range(3)] + [o[3] + s * (c + (1 - c) * g) * (1 - o[3])])
    return out


# --- the drawing ----------------------------------------------------------------------------

NIGHT = (30, 40, 80, 255)           # #1e2850, a night sky
NONE = S.NONE


def night(x, y):
    """The left half, columns 0 to 7, a night sky; the right half empty."""
    return NIGHT if x < 8 else NONE


DRAWINGS = {"night": [[night(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(start=(40, 0), end=(60, 100), jagged=40, detail=6, branches=30, width=3, glow=24,
         opacity=100, hold=2, seed=0, color="#ffffff", glow_color="#6e8cff", shift=0):
    return {"drawing": "night", "start": start, "end": end, "jagged": jagged, "detail": detail,
            "branches": branches, "width": width, "glow": glow, "opacity": opacity, "hold": hold,
            "seed": seed, "color": color, "glow_color": glow_color, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    v = min(hi, max(lo, v))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    out = lightning_bolt(pixels, *[held(c, k, frame_no) for k in NUMBERS], c["color"],
                         c["glow_color"], frame_no)
    return [out[y * W + x - c["shift"]] if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(opacity=0, shift=c["shift"]), 0)


def segments(c, frame_no, reached=None):
    """The case's bolt at a frame, in the drawing's own space."""
    h = {k: held(c, k, frame_no) for k in NUMBERS}
    S_ = (h["start"][0] / 100 * W, h["start"][1] / 100 * H)
    E = (h["end"][0] / 100 * W, h["end"][1] / 100 * H)
    return bolt(S_, E, h["jagged"], h["detail"], h["branches"], h["seed"],
                frame_no // h["hold"], reached)


# A straight line across row 4, through the sky and the clear, its ends outside the frame.
LINE = {"start": (-10, 45), "end": (110, 45), "jagged": 0, "branches": 0, "width": 1, "glow": 0}
# A small jagged bolt down the middle.
SMALL = {"start": (50, 0), "end": (50, 100), "branches": 0, "width": 1, "glow": 2}

CASES = {
    "FX-BOLT-001": ("The settings as they start: from (40, 0) to (60, 100) per cent, jaggedness "
                    "40, detail 6, branches 30, width 3, glow 24, white in a blue glow #6e8cff, "
                    "opacity 100, hold 2, seed 0. On a frame this small the glow reaches every "
                    "pixel. Frames 0 and 1 are one bolt, held for two frames; frame 2 is a new "
                    "one.",
                    case(), [0, 1, 2]),
    "FX-BOLT-002": ("Opacity 0: the drawing, untouched.",
                    case(opacity=0), [0]),
    "FX-BOLT-003": ("Width 0 and glow 0: the drawing, untouched.",
                    case(width=0, glow=0), [0]),
    "FX-BOLT-004": ("Jaggedness 0, branches 0, width 1, glow 0, from (-10, 45) to (110, 45): a "
                    "straight white line one pixel thick along the middle of row 4, its ends "
                    "outside the frame. Row 4 is white added to the sky on the left, and white "
                    "on the clear on the right; nothing else changes.",
                    case(**LINE), [0]),
    "FX-BOLT-005": ("FX-BOLT-004 at 50 per cent down, on the edge between rows 4 and 5: each "
                    "row half covered.",
                    case(**{**LINE, "start": (-10, 50), "end": (110, 50)}), [0]),
    "FX-BOLT-006": ("FX-BOLT-005 two pixels wide: rows 4 and 5 both fully covered, rows 3 and 6 "
                    "untouched.",
                    case(**{**LINE, "start": (-10, 50), "end": (110, 50), "width": 2}), [0]),
    "FX-BOLT-007": ("FX-BOLT-004 with width 0 and glow 3: only the glow, full on row 4, "
                    "(2/3)^2 of it on rows 3 and 5, (1/3)^2 on rows 2 and 6, nothing further.",
                    case(**{**LINE, "width": 0, "glow": 3}), [0]),
    "FX-BOLT-008": ("FX-BOLT-004 with glow 3: row 4 the white core, rows 2, 3, 5 and 6 "
                    "FX-BOLT-007's glow.",
                    case(**{**LINE, "glow": 3}), [0]),
    "FX-BOLT-009": ("Jaggedness 40, detail 6, no branches, width 1, glow 2, from the middle of "
                    "the top edge to the middle of the bottom: a jagged line with both ends "
                    "fixed. Frames 0 and 1 the same bolt, frame 2 a new one.",
                    case(**SMALL), [0, 1, 2]),
    "FX-BOLT-010": ("FX-BOLT-009 with branches 100: the same bolt, with forks that thin to "
                    "nothing at their tips added to it.",
                    case(**{**SMALL, "branches": 100}), [0]),
    "FX-BOLT-011": ("FX-BOLT-009 with detail 1: the line is halved once, two straight pieces "
                    "meeting at one bend.",
                    case(**{**SMALL, "detail": 1}), [0]),
    "FX-BOLT-012": ("FX-BOLT-009 with seed 7: another bolt between the same ends.",
                    case(**{**SMALL, "seed": 7}), [0]),
    "FX-BOLT-013": ("FX-BOLT-009 with hold 1: a new bolt on every frame, frame 1 different from "
                    "frame 0.",
                    case(**{**SMALL, "hold": 1}), [0, 1]),
    "FX-BOLT-014": ("FX-BOLT-008 in colour, an orange core #ff8000 in a green glow #00ff00: row "
                    "4 orange, the glow's rows green; the covering as FX-BOLT-008's.",
                    case(**{**LINE, "glow": 3, "color": "#ff8000", "glow_color": "#00ff00"}),
                    [0]),
    "FX-BOLT-015": ("FX-BOLT-014 with its colours written in capitals: the same.",
                    case(**{**LINE, "glow": 3, "color": "#FF8000", "glow_color": "#00FF00"}),
                    [0]),
    "FX-BOLT-016": ("FX-BOLT-008 at opacity 50: half the light.",
                    case(**{**LINE, "glow": 3, "opacity": 50}), [0]),
    "FX-BOLT-017": ("FX-BOLT-004 with both points keyed down from 5 per cent at frame 0 to 85 "
                    "at frame 4, linear: the line on row 0 at frame 0, on row 4 at frame 2, as "
                    "FX-BOLT-004, and on row 8 at frame 4.",
                    case(**{**LINE, "start": keyed((0, [-10, 5]), (4, [-10, 85])),
                            "end": keyed((0, [110, 5]), (4, [110, 85]))}), [0, 2, 4]),
    "FX-BOLT-018": ("FX-BOLT-008 with opacity eased from 0 at frame 0 to 100 at frame 4 on a "
                    "curve that overshoots: at frame 2 it would pass 100, is held at 100, and "
                    "is FX-BOLT-008, as frame 4 is.",
                    case(**{**LINE, "glow": 3, "opacity": keyed((0, 0, OVERSHOOT), (4, 100))}),
                    [0, 2, 4]),
    "FX-BOLT-019": ("FX-BOLT-009 moved three pixels right: the bolt moves with the drawing, and "
                    "nothing is drawn outside the layer, so the three columns left of it stay "
                    "empty.",
                    case(**{**SMALL, "shift": 3}), [0]),
    "FX-BOLT-020": ("Start and end both at the middle, (50, 50), width 4, glow 0: a round dot "
                    "4 pixels across.",
                    case(start=(50, 50), end=(50, 50), width=4, glow=0), [0]),
    "FX-BOLT-021": ("FX-BOLT-010 with detail 8, the most: finer steps and more forks.",
                    case(**{**SMALL, "branches": 100, "detail": 8}), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-BOLT-022": ("Jaggedness 101, above 100.", case(jagged=101)),
    "FX-BOLT-023": ("Detail 0, below 1.", case(detail=0)),
    "FX-BOLT-024": ("Detail 9, above 8.", case(detail=9)),
    "FX-BOLT-025": ("Width 101, above 100.", case(width=101)),
    "FX-BOLT-026": ("Glow 501, above 500.", case(glow=501)),
    "FX-BOLT-027": ("Hold 0, below 1.", case(hold=0)),
    "FX-BOLT-028": ("Seed 100001, above 100000.", case(seed=100001)),
    "FX-BOLT-029": ("A start at (1001, 0), past 1000.", case(start=(1001, 0))),
    "FX-BOLT-030": ("Branches keyed to 150 at frame 4.", case(branches=keyed((0, 30), (4, 150)))),
    "FX-BOLT-031": ("A colour written \"#12345\", one digit short.", case(color="#12345")),
    "FX-BOLT-032": ("A glow colour written \"blue\".", case(glow_color="blue")),
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
        "instance_id": "fx-0-0", "type_id": "core.lightning_bolt", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
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

    (OUT / "expected_lightning_bolt.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    sky, white = R.working(NIGHT), [1.0, 1.0, 1.0]
    blue = [srgb_to_linear(v / 255) for v in R.hex_color("#6e8cff")]
    row = lambda px, y: [px[at(x, y)] for x in range(W)]  # noqa: E731

    # No reached segment sits near a fork's cliff, nor a fork near its side's.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            reached = []
            segments(cs, f, reached)
            b = held(cs, "branches", f)
            for r1, fork, r3 in reached:
                assert b in (0, 100) or abs((r1 + 1) / 2 - b / 100) > CLIFF, fx
                assert not fork or abs(r3) > CLIFF, fx
    # Everywhere: nothing is taken away, the covering stays inside 0 to 1.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-BOLT-019" else 0))
        for px in frames.values():
            for p, q in zip(px, base):
                assert 0 <= p[3] <= 1 + 1e-15 and p[3] >= q[3], fx
                assert all(p[ch] >= q[ch] - 1e-15 for ch in range(3)), fx
    # The bolt's ends stay fixed; a finer detail keeps every coarser point; forks add, never move.
    six = segments(case(**SMALL), 0)
    main = [s for s in six if s[5] == 0]
    assert main[0][0] == (8.0, 0.0) and main[-1][1] == (8.0, 10.0) and len(main) == 64
    assert all(main[i][1] == main[i + 1][0] for i in range(63))
    five = [s for s in segments(case(**{**SMALL, "detail": 5}), 0) if s[5] == 0]
    assert [s[0] for s in five] == [s[0] for s in main[::2]]
    forked = segments(case(**{**SMALL, "branches": 100}), 0)
    assert [s for s in forked if s[5] == 0] == main and len(forked) > len(main)
    assert all(s[3] == 0 for s in forked if s[5] != 0 and s[6] % 2 == 1 and
               all(t[0] != s[1] for t in forked))
    assert max(s[4] for s in segments(case(**{**SMALL, "branches": 100, "detail": 8}), 0)) == 3
    one = segments(case(**{**SMALL, "detail": 1}), 0)
    assert len(one) == 2 and one[0][1] == one[1][0] and one[0][1][1] == 5.0
    assert abs(one[0][1][0] - 8.0) <= 0.4 * 10 / 2

    # 001: every pixel lit; frames 0 and 1 one bolt, frame 2 another.
    first = c["FX-BOLT-001"]
    assert all(first["0"][i] != drawn[i] for i in range(W * H))
    assert first["1"] == first["0"] and first["2"] != first["0"]
    for fx in ("FX-BOLT-002", "FX-BOLT-003"):
        assert c[fx]["0"] == drawn, fx
    # 004: row 4 exactly white added, nothing else.
    four = c["FX-BOLT-004"]["0"]
    assert row(four, 4) == [[sky[ch] + 1 for ch in range(3)] + [1.0]] * 8 + [white + [1.0]] * 8
    assert all(four[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in range(H) if y != 4)
    # 005: half on rows 4 and 5; 006: whole.
    five_ = c["FX-BOLT-005"]["0"]
    for y in (4, 5):
        assert row(five_, y) == [[sky[ch] + 0.5 for ch in range(3)] + [1.0]] * 8 + [[0.5] * 4] * 8
    six_ = c["FX-BOLT-006"]["0"]
    assert row(six_, 4) == row(six_, 5) == row(four, 4)
    assert row(six_, 3) == row(drawn, 3) and row(six_, 6) == row(drawn, 6)
    # 007: the glow alone, (1 - d / 3)^2; 008: the core over it.
    seven, eight = c["FX-BOLT-007"]["0"], c["FX-BOLT-008"]["0"]
    for y, g in ((4, 1.0), (3, 4 / 9), (5, 4 / 9), (2, 1 / 9), (6, 1 / 9), (1, 0.0), (7, 0.0)):
        assert near([seven[at(12, y)]], [[g * blue[ch] for ch in range(3)] + [g]]), y
        assert near([seven[at(2, y)]], [[sky[ch] + g * blue[ch] for ch in range(3)] + [1.0]]), y
        assert row(eight, y) == (row(four, 4) if y == 4 else row(seven, y)), y
    # 009: a bolt from the top edge's middle to the bottom's, jagged; held for two frames.
    nine = c["FX-BOLT-009"]
    assert nine["1"] == nine["0"] and nine["2"] != nine["0"]
    assert any(nine["0"][at(x, 0)][3] > 0 for x in (7, 8)) and any(nine["0"][at(x, 9)][3] > 0 for x in (7, 8))
    assert c["FX-BOLT-010"]["0"] != nine["0"]
    assert all(c["FX-BOLT-010"]["0"][i][3] >= nine["0"][i][3] for i in range(W * H))
    assert c["FX-BOLT-011"]["0"] != nine["0"] and c["FX-BOLT-012"]["0"] != nine["0"]
    assert c["FX-BOLT-013"]["1"] != c["FX-BOLT-013"]["0"] and c["FX-BOLT-013"]["0"] == nine["0"]
    # 014 and 015: orange core, green glow; 016: half.
    fourteen = c["FX-BOLT-014"]["0"]
    orange = [1.0, srgb_to_linear(128 / 255), 0.0]
    assert near([fourteen[at(12, 4)]], [orange + [1.0]])
    assert near([fourteen[at(12, 3)]], [[0.0, 4 / 9, 0.0, 4 / 9]])
    assert all(fourteen[i][3] == eight[i][3] for i in range(W * H))
    assert c["FX-BOLT-015"]["0"] == fourteen
    sixteen = c["FX-BOLT-016"]["0"]
    for i in range(W * H):
        assert all(abs(sixteen[i][ch] - drawn[i][ch] - 0.5 * (eight[i][ch] - drawn[i][ch])) < 1e-12
                   for ch in range(4))
    # The keyed cases meet the plain ones at their frames.
    k = c["FX-BOLT-017"]
    assert k["2"] == four and row(k["0"], 0) == row(four, 4) and row(k["4"], 8) == row(four, 4)
    k = c["FX-BOLT-018"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert k["0"] == drawn and k["2"] == k["4"] == eight
    # 019: moved, clipped to the layer.
    moved = c["FX-BOLT-019"]["0"]
    assert all(moved[at(x, y)] == nine["0"][at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == EMPTY for x in (0, 1, 2) for y in range(H))
    # 020: a round dot, the same turned a quarter.
    dot = c["FX-BOLT-020"]["0"]
    lit = [(x, y) for x in range(W) for y in range(H) if dot[at(x, y)] != drawn[at(x, y)]]
    assert sorted((15 - x, 9 - y) for x, y in lit) == sorted(lit) and len(lit) == 16
    assert all((x - 7.5) ** 2 + (y - 4.5) ** 2 < 6.5 for x, y in lit)
    assert near([dot[at(8, 4)], dot[at(8, 5)]], [[1.0] * 4] * 2)
    # 021: finer still, and more lit than 010.
    assert sum(p[3] for p in c["FX-BOLT-021"]["0"]) > sum(p[3] for p in nine["0"])
    print("checked")


if __name__ == "__main__":
    main()
