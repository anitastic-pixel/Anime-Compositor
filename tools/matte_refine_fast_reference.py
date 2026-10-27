"""Refine Hard Matte's and Refine Soft Matte's Fast quality, worked a second way.

D-903 (wave 11, pick 9): a Quality setting on the two Refine effects, "exact" (as they have
always been drawn, and the default, never saved) or "fast", which the user chooses and which
exports draw too. Fast works each guided filter the way its own authors published for speed:
He and Sun, "Fast Guided Filter" (2015, arXiv:1505.00996). Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it (FX-MREF-036 to 041).
Everything else of the two effects is `matte_refine_reference.py`'s, which this file imports
and does not change.

**The fast guided filter** of a matte p by a guide I at radius r: s = min(4, floor(r / 4)).
Where s is below 2 it is the guided filter itself. Otherwise the picture is cut into blocks
s by s from its top left corner (the last column and row of blocks as wide as is left); the low
picture holds each block's mean of I and of p. On the low picture, at radius rl = floor(r / s),
a and b are worked as the guided filter works them, and their box means at rl taken. Each full
pixel (x, y) reads those means bilinearly at u = (x + 0.5) / s - 0.5, v = (y + 0.5) / s - 0.5,
each held inside 0 and the low picture's last column or row: x0 = floor(u), x1 = x0 + 1 held
to the last column, weight u - x0, and the same down the rows. q = A . I + B with the full
picture's own guide I, so the edge stays as sharp as the picture.

**This file never runs the build's code path**, as `matte_refine_reference.py` says of itself.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/matte_refine_fast_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import matte_refine_reference as M  # noqa: E402

EXPECTED = M.OUT / "expected_matte_refine_fast.json"
GUIDED = M.guided  # the exact filter, kept before refine_fast swaps it


def step(r):
    return min(4, r // 4)


def guided_fast(I, p, r, eps=M.EPS):
    s = step(r)
    if s < 2:
        return GUIDED(I, p, r, eps)
    h, w = p.shape
    lh, lw = -(-h // s), -(-w // s)

    def low(f):
        out = np.zeros((lh, lw) + f.shape[2:])
        for Y in range(lh):
            for X in range(lw):
                out[Y, X] = f[Y * s:(Y + 1) * s, X * s:(X + 1) * s].mean(axis=(0, 1))
        return out

    Il, pl = low(I), low(p)
    rl = r // s
    mI, mp = M.mean(Il, rl), M.mean(pl, rl)
    S_ = M.mean(Il[..., :, None] * Il[..., None, :], rl) - mI[..., :, None] * mI[..., None, :]
    cov = M.mean(Il * pl[..., None], rl) - mI * mp[..., None]
    a = np.linalg.solve(S_ + eps * np.eye(3), cov[..., None])[..., 0]
    b = mp - (a * mI).sum(-1)
    ma, mb = M.mean(a, rl), M.mean(b, rl)

    def at(n, ln):
        u = np.clip((np.arange(n) + 0.5) / s - 0.5, 0, ln - 1)
        i0 = np.floor(u).astype(int)
        return i0, np.minimum(i0 + 1, ln - 1), u - i0

    y0, y1, fy = at(h, lh)
    x0, x1, fx = at(w, lw)

    def up(f):
        fx_ = fx.reshape((1, w) + (1,) * (f.ndim - 2))
        fy_ = fy.reshape((h, 1) + (1,) * (f.ndim - 2))
        top = f[y0][:, x0] * (1 - fx_) + f[y0][:, x1] * fx_
        bottom = f[y1][:, x0] * (1 - fx_) + f[y1][:, x1] * fx_
        return top * (1 - fy_) + bottom * fy_

    return (up(ma) * I).sum(-1) + up(mb)


def refine_fast(P, c, n):
    """Refine with the guided filter worked the fast way; nothing else of it changes."""
    M.guided = guided_fast
    try:
        return M.refine(P, c, n)
    finally:
        M.guided = GUIDED


def render(c, n, P=None):
    P = M.drawn() if P is None else P
    return M.moved(refine_fast(P, c, n), c["shift"])


def fast(c):
    return dict(c, quality="fast")


CASES = {
    "FX-MREF-036": ("FX-MREF-023 (Refine Soft Matte as added, edge radius 10) at Quality Fast: "
                    "the fit worked on blocks 2 by 2 at radius 5 and read back smoothly, so "
                    "the covering is close to FX-MREF-023's and not the same.",
                    fast(M.soft())),
    "FX-MREF-037": ("Refine Hard Matte, feather 8, contrast 0, at Quality Fast: blocks 2 by 2 "
                    "at radius 4.", fast(M.hard(feather=8, contrast=0))),
    "FX-MREF-038": ("FX-MREF-013 (Refine Hard Matte as added, feather 2) at Quality Fast: "
                    "below radius 8 Fast is the exact filter, so FX-MREF-013's frame.",
                    fast(M.hard())),
    "FX-MREF-039": ("Refine Soft Matte, edge radius 12, feather 13, decontaminate off, at "
                    "Quality Fast: blocks 3 by 3 at radius 4 for both, the last row of blocks "
                    "one pixel tall (16 rows).",
                    fast(M.soft(edge_radius=12, feather=13, decontaminate="off"))),
    "FX-MREF-040": ("Refine Soft Matte, edge radius 20, at Quality Fast: blocks 4 by 4 (the "
                    "most) at radius 5.", fast(M.soft(edge_radius=20))),
}

INVALID = {
    "FX-MREF-041": ("Refine Soft Matte, quality written \"medium\".",
                    dict(M.soft(), quality="medium")),
}


def write(fx, c):
    j = M.project_json(fx, c)
    j["compositions"][0]["layers"][0]["effects"][0]["parameters"]["quality"] = c["quality"]
    name = f"{fx.lower().replace('-', '_')}.json"
    (M.OUT / name).write_text(json.dumps(j, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": M.TOLERANCE, "width": M.W, "height": M.H, "cases": {}}
    rounded = M.drawn().astype(np.float32).astype(np.float64)
    for fx, (says, c) in sorted(CASES.items()):
        got = render(c, 0)
        moved_by = float(np.abs(render(c, 0, rounded) - got).max())
        assert moved_by < M.TOLERANCE / 10, (fx, moved_by)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": M.as_list(got)}}
        exact = M.render(c, 0)
        print(f"{fx}: largest difference from exact {float(np.abs(got - exact).max()):.4f}")
    for fx, (says, c) in sorted(INVALID.items()):
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = M.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    EXPECTED.write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    P = M.drawn()
    I = M.to_srgb(M.straight(P))
    # A constant matte comes back constant at every block size.
    for r in (8, 12, 16):
        assert np.allclose(guided_fast(I, np.full((M.H, M.W), 0.7), r), 0.7, atol=1e-12)
    # Below radius 8 it is the exact filter, to the last digit.
    assert (guided_fast(I, P[..., 3], 7) == GUIDED(I, P[..., 3], 7)).all()
    exact13 = json.loads((M.OUT / "expected_matte_refine.json").read_text(encoding="utf-8"))
    assert c["FX-MREF-038"] == exact13["cases"]["FX-MREF-013"]["frames"]["0"]
    # Fast is not exact where it works on blocks, and stays near it.
    alpha = lambda f: np.array([p[3] for p in f]).reshape(M.H, M.W)  # noqa: E731
    e23 = exact13["cases"]["FX-MREF-023"]["frames"]["0"]
    d = np.abs(alpha(c["FX-MREF-036"]) - alpha(e23))
    assert 1e-3 < d.max() < 0.5, d.max()
    for fx in ("FX-MREF-037", "FX-MREF-039", "FX-MREF-040"):
        assert c[fx] != M.as_list(M.render(CASES[fx][1], 0)), fx
    for name, px in c.items():
        for q in px:
            assert -1e-12 <= q[3] <= 1 + 1e-12 and all(-1e-12 <= v <= q[3] + 1e-12
                                                      for v in q[:3]), (name, q)
    print("checked")


if __name__ == "__main__":
    main()
