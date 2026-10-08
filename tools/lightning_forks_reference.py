"""D-338's long forks for Lightning Bolt, worked a second way.

From P-26's tutorial 2. The tutorial turns Advanced Lightning's Decay down "for them to touch down
on the ground": its forks run on down beside the main bolt until the ground mask (Alpha Obstacle)
stops them, so three long strands each reach the ground. D-190's forks are short twigs, a third to
three fifths of the piece they leave. `core.lightning_bolt` gains `forks`: "short", what a file
without it means, D-190's and D-324's forks exactly; or "long".

The rule, on top of D-324's (`tools/lightning_extras_reference.py`), with k_d = 1 - decay / 100.
Each of the type's bolts the halving starts from, (O_b, D_b, ...), is a main bolt with its own way
u = (D_b - O_b) / |D_b - O_b|. With "long", a fork that leaves a main bolt (depth 0) in halving
0, 1 or 2 at its pushed middle M is long, when |D_b - O_b| > 0 and what is left of the way past M,
left = (D_b - M) . u, is above 0. The fork's way is v = turn(u, (10 + 10 (r_2 + 1)) s), s the
side D-190 picks from r_3 (-1 when r_3 < 0, otherwise 1), so it turns 10 to 30 degrees; its length
is k_d left / (v . u), so it ends where it has covered k_d of what is left of the main bolt's way;
and it is the segment (M, M + v length, w0, w0 k_d, depth 1, 1024 b + key, 1, b), w0 as D-324
(the half weight wM / 2, breaking's full wM). It is then halved, pushed and forked as every fork
is. Every other fork is D-324's. D-324's stopping (Alpha Obstacle) then ends each strand where it
meets what blocks, so on the ground mask the strands all stop at the ground.

**This file never runs the build's code path.** It works in double precision on lists.

Every case is D-190's 16 by 10 composition and drawing. The projects go into
`Fixtures/lightning_forks`, the expected frames into
`Fixtures/lightning_forks/expected_lightning_forks.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_forks_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import lightning_bolt_reference as LB  # noqa: E402
import lightning_extras_reference as LX  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = LB.W, LB.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_forks"
TOLERANCE = LB.TOLERANCE
CLIFF = LB.CLIFF


# --- the rule -------------------------------------------------------------------------------

def bolt(kind, O, D, bottom, jagged, detail, branches, seed, m, turbulence, decay, conductivity,
         forks, reached=None, born=None):
    """D-324's segments before any obstacle, with D-338's long forks when `forks` is "long",
    and D-339's, starting at the main bolt's full weight, when it is "full"; `born` gathers each
    fork as it starts, before it is halved."""
    r = LX.numbers(seed, conductivity)
    kd = 1 - decay / 100
    segs = LX.roots(kind, O, D, bottom, kd, r, m)
    ways = {s[5]: (s[0], s[1]) for s in segs}
    for i in range(detail):
        out = []
        for P, Q, wp, wq, depth, b, key, parent in segs:
            dx, dy = Q[0] - P[0], Q[1] - P[1]
            L = math.hypot(dx, dy)
            if L == 0:
                out.append((P, Q, wp, wq, depth, b, key, parent))
                continue
            rr = [r(key, b, m, j) for j in range(5)]
            a = rr[0] * jagged / 100 * L / 2 * (1 + turbulence / 100 * i / 2)
            Mid = ((P[0] + Q[0]) / 2 - dy / L * a, (P[1] + Q[1]) / 2 + dx / L * a)
            wm = (wp + wq) / 2
            out += [(P, Mid, wp, wm, depth, b, 2 * key, parent),
                    (Mid, Q, wm, wq, depth, b, 2 * key + 1, parent)]
            chance = branches / 100 * (1 + turbulence / 100)
            if not (depth < 3 and (rr[1] + 1) / 2 < chance):
                continue
            if reached is not None:
                reached.append(rr[3])
            side = -1 if rr[3] < 0 else 1
            w0 = wm if kind == "breaking" else wm / 2
            run = None
            if forks in ("long", "full") and depth == 0 and i < 3:
                (o, e) = ways[b]
                ul = math.hypot(e[0] - o[0], e[1] - o[1])
                if ul > 0:
                    u = ((e[0] - o[0]) / ul, (e[1] - o[1]) / ul)
                    left = (e[0] - Mid[0]) * u[0] + (e[1] - Mid[1]) * u[1]
                    if left > 0:
                        run = (u, left)
            if run:
                u, left = run
                v = LB.turn(u, (10 + 10 * (rr[2] + 1)) * side)
                lb = kd * left / (v[0] * u[0] + v[1] * u[1])
                ws = wm if forks == "full" else w0
                out.append((Mid, (Mid[0] + v[0] * lb, Mid[1] + v[1] * lb), ws, ws * kd,
                            depth + 1, 1024 * b + key, 1, b))
                if born is not None:
                    born.append(out[-1])
                continue
            way = (dx / L, dy / L)
            if kind == "strike":
                tx, ty = D[0] - Mid[0], D[1] - Mid[1]
                tl = math.hypot(tx, ty)
                if tl > 0:
                    way = (tx / tl, ty / tl)
            v = LB.turn(way, (15 + 15 * (rr[2] + 1)) * side)
            lb = L * (0.3 + 0.15 * (rr[4] + 1))
            out.append((Mid, (Mid[0] + v[0] * lb, Mid[1] + v[1] * lb), w0, 0.0, depth + 1,
                        1024 * b + key, 1, b))
            if born is not None:
                born.append(out[-1])
        segs = out
    return segs


def segments(c, frame_no, reached=None, born=None):
    """The case's bolt at a frame, in the drawing's own space, any obstacle applied."""
    h = {k: LX.held(c, k, frame_no) for k in LB.NUMBERS + LX.EXTRA}
    O = (h["start"][0] / 100 * W, h["start"][1] / 100 * H)
    D = (h["end"][0] / 100 * W, h["end"][1] / 100 * H)
    segs = bolt(c["kind"], O, D, H, h["jagged"], h["detail"], h["branches"], h["seed"],
                frame_no // h["hold"], h["turbulence"], h["decay"], h["conductivity"], c["forks"],
                reached, born)
    if h["obstacle"] > 0:
        pixels = [p for row in LX.DRAWINGS[c["drawing"]] for p in row]
        segs = LX.stop(segs, LX.blocker(pixels, h["obstacle"]))
    return segs


def render(c, frame_no):
    pixels = [p for row in LX.DRAWINGS[c["drawing"]] for p in row]
    working = [R.working(p) for p in pixels]
    opacity, width, glow = (LX.held(c, k, frame_no) for k in ("opacity", "width", "glow"))
    if opacity == 0 or (width == 0 and glow == 0):
        return working
    C, G = LB.field([s[:7] for s in segments(c, frame_no)], width, glow, W, H)
    core = [srgb_to_linear(v / 255) for v in R.hex_color(c["color"].lower())]
    halo = [srgb_to_linear(v / 255) for v in R.hex_color(c["glow_color"].lower())]
    s = opacity / 100
    out = []
    for o, k, g in zip(working, C, G):
        lit = [k * core[ch] + (1 - k) * g * halo[ch] for ch in range(3)]
        out.append([o[ch] + s * lit[ch] for ch in range(3)] +
                   [o[3] + s * (k + (1 - k) * g) * (1 - o[3])])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(forks="long", **settings):
    c = LX.case(**settings)
    c["forks"] = forks
    return c


# Down the middle, from the top edge's middle to the bottom's, thin and glowless, every halving
# forking, so each of the seven first forks is long.
DOWN = {**LX.DOWN, "branches": 100}

CASES = {
    "FX-LFORK-001": ("A jagged bolt from the middle of the top edge to the middle of the bottom, "
                     "width 1, no glow, branches 100, forks Long: the seven forks of the first "
                     "three halvings each run on to the bottom edge beside the main bolt, turned "
                     "10 to 30 degrees from straight down.",
                     case(**DOWN), [0, 1]),
    "FX-LFORK-002": ("FX-LFORK-001 with forks written \"short\": D-324's forks, exactly "
                     "FX-LIGHTX-005's bolt with branches 100.", case(forks="short", **DOWN), [0]),
    "FX-LFORK-003": ("FX-LFORK-001 with decay 50: each long fork goes half of what is left of "
                     "the way down and thins to half its starting weight.",
                     case(decay=50, **DOWN), [0]),
    "FX-LFORK-004": ("FX-LFORK-001 over a drawing whose rows 7 to 9 are ground, Alpha Obstacle "
                     "50: every strand stops where it reaches the ground, several of them on row "
                     "7, and row 9 is untouched. Tutorial 2's strands touching down.",
                     case(drawing="ground", obstacle=50, **DOWN), [0, 1]),
    "FX-LFORK-005": ("FX-LFORK-001 as Strike: the long forks leave along the main bolt's own "
                     "way too, so they are FX-LFORK-001's; only the short ones turn from the way "
                     "to the end point.", case(kind="strike", **DOWN), [0]),
    "FX-LFORK-006": ("FX-BOLT-001's settings (branches 30) with forks Long: the same main bolt, "
                     "the forks of its first three halvings longer.",
                     case(), [0, 2]),
}

INVALID = {
    "FX-LFORK-007": ("Forks \"Long\": the word is exact, so a capital is not it.",
                     case(forks="Long")),
    "FX-LFORK-008": ("Forks \"many\", which is not one.", case(forks="many")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = LX.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["forks"] = c["forks"]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in LX.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in frames}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = render(case(opacity=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_lightning_forks.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    ground = render(case(drawing="ground", opacity=0), 0)

    # "short" is D-324's bolt, segment for segment, in every one of its cases.
    for fx, (_, xc, frames) in LX.CASES.items():
        for f in frames:
            assert segments({**xc, "forks": "short"}, f) == LX.segments(xc, f), fx
    assert c["FX-LFORK-002"]["0"] == LX.render(LX.case(**DOWN), 0)
    # No reached fork near its side's cliff.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            reached = []
            segments(cs, f, reached)
            assert all(abs(r3) > CLIFF for r3 in reached), fx
    # 001: seven long forks, each from the main bolt, each ending on the bottom edge, y = 10,
    # and each 10 to 30 degrees from straight down.
    for f in (0, 1):
        born = []
        segs = segments(CASES["FX-LFORK-001"][1], f, born=born)
        long_ = [s for s in born if s[3] > 0]
        assert len(long_) == 7, f
        for P, Q, wp, wq, *_ in long_:
            assert abs(Q[1] - H) < 1e-9 and wq == wp
            deg = math.degrees(math.atan2(abs(Q[0] - P[0]), Q[1] - P[1]))
            assert 10 - 1e-9 <= deg <= 30 + 1e-9, deg
        ends = [s[1] for s in segs if s[4] == 1 and abs(s[1][1] - H) < 1e-9]
        assert len({round(e[0], 6) for e in ends}) >= 3
    assert c["FX-LFORK-001"]["0"] != c["FX-LFORK-002"]["0"]
    # 003: halfway down what is left, thinning to half.
    born = []
    segments(CASES["FX-LFORK-003"][1], 0, born=born)
    assert sum(s[3] > 0 for s in born) == 7
    for P, Q, wp, wq, d, b, key, parent in born:
        if wq > 0:
            assert abs((Q[1] - P[1]) - 0.5 * (H - P[1])) < 1e-9 and abs(wq - wp / 2) < 1e-12
    # 004: every strand stops at the ground; row 9 untouched; row 7 lit in several places.
    for px in c["FX-LFORK-004"].values():
        assert all(px[at(x, 9)] == ground[at(x, 9)] for x in range(W))
        assert sum(px[at(x, 7)] != ground[at(x, 7)] for x in range(W)) >= 3
    assert max(s[1][1] for s in segments(CASES["FX-LFORK-004"][1], 0)) < 8
    # 005: the long forks the same as 001's.
    b1, b5 = [], []
    one = segments(CASES["FX-LFORK-001"][1], 0, born=b1)
    five = segments(CASES["FX-LFORK-005"][1], 0, born=b5)
    assert [s for s in one if s[4] == 0] == [s for s in five if s[4] == 0]
    assert [s for s in b1 if s[3] > 0] == [s for s in b5 if s[3] > 0]
    # 006: the main bolt as FX-BOLT-001's, and some fork longer.
    b6 = []
    six = segments(CASES["FX-LFORK-006"][1], 0, born=b6)
    bolt1 = LB.segments(LB.case(), 0)
    assert [s[:7] for s in six if s[4] == 0] == [s for s in bolt1 if s[4] == 0]
    assert any(s[3] > 0 for s in b6) and c["FX-LFORK-006"]["0"] != LB.render(LB.case(), 0)
    print("checked")


if __name__ == "__main__":
    main()
