"""Curl Noise's Transform (Rotation, Scale, Perspective Offset), worked a second way.

D-646 adds to `core.curl_noise` (D-446, D-583; `tools/curl_noise_reference.py`, whose rule this
file reuses unchanged) the three Transform controls After Effects' Curl Noise lists beside Size
and Offset. Adobe publishes no formula; the rule below is this program's own, after the same
controls of Fractal Noise, and nothing is ported.

With c the drawing's middle (its width and height halved, in the drawing's own pixels),
m = 100 / scale and theta = rotation in degrees, octave o of the noise is read at

    p_o = p - w_o (offset + drift frame),  w_o = 0.5^o with Perspective Offset on, else 1,
    q_o = c + m R(-theta) (p_o - c),  R(-theta) (x, y) = (x cos + y sin, -x sin + y cos),

in place of step 1's p'; so the noise is turned clockwise by Rotation and made Scale per cent
as big round the drawing's middle, then slid on the picture by Offset and the drift, and with
Perspective Offset on each finer octave slides half as far as the one before, as if further
away. The slope, worked in q, is brought back to the picture by m R(theta), before Swirl and
Vertical Bias, which stay in the picture's own directions. The seed (step 4) is read at q_0.
With Rotation 0 and Scale 100 q_o is p_o exactly, and with Perspective Offset off p_o is p',
so every file before D-646 draws as before.

With Source This Layer or Other Layer the picture is read at p' as D-583 says, unturned and
at its own size (as Size does not scale it); Rotation and Scale turn and scale only the seed,
and Perspective Offset does nothing.

Settings: `rotation` -100000 to 100000 degrees, 0 when added; `scale` 1 to 10000 per cent,
100; `perspective_offset` `off` (when added) or `on`. A file without them reads them as added,
and they are written only when they are not (D-304). Both numbers can be keyed.

**This file never runs the build's code path.** Cases as in `tools/curl_noise_reference.py`:
a 16 by 10 composition holding Noise's card. The expected frames are in
`Fixtures/curl_noise/expected_curl_transform.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/curl_transform_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import curl_noise_reference as C  # noqa: E402
from fxkey_reference import keyed, setting_json  # noqa: E402

W, H = C.W, C.H
OUT = C.OUT
TOLERANCE = 2e-5
C.RANGES.update({"rotation": (-100000, 100000), "scale": (1, 10000)})
STARTS = {"rotation": 0, "scale": 100, "perspective_offset": "off"}

_setup, _field_at = C.setup, C.field_at


def setup(n, frame_no):
    k = _setup(n, frame_no)
    t = math.radians(n["rotation"])
    k["turn"] = (None if n["rotation"] == 0 and n["scale"] == 100
                 else (math.cos(t), math.sin(t), 100 / n["scale"], (W / 2, H / 2)))
    k["persp"] = n["perspective_offset"] == "on"
    return k


def point(k, px, py, o):
    """q_o: the point octave o of the noise is read at."""
    w = 0.5 ** o if k["persp"] else 1.0
    x, y = px - w * k["shift"][0], py - w * k["shift"][1]
    if k["turn"] is None:
        return x, y
    c, s, m, (cx, cy) = k["turn"]
    dx, dy = x - cx, y - cy
    return cx + (dx * c + dy * s) * m, cy + (-dx * s + dy * c) * m


def field_at(k, swirl, px, py, src=None):
    """`curl_noise_reference.field_at` with the Transform."""
    s = k["s"]
    if src is None:
        nv = gx = gy = total = 0.0
        fine = 1.0
        for o in range(4):
            x, y = point(k, px, py, o)
            v, a, b = C.value_slope(8 * o, x / s * fine, y / s * fine, k["z"] * fine)
            amp = k["amps"][o]
            nv += amp * v
            gx += amp * fine * a
            gy += amp * fine * b
            total += amp
            fine *= 2
        nv, gx, gy = nv / total, gx / total / s, gy / total / s
        if k["turn"] is not None:
            c, sn, m, _ = k["turn"]
            gx, gy = (c * gx - sn * gy) * m, (sn * gx + c * gy) * m
    else:
        bx = px - k["shift"][0] + src["origin"][0]
        by = py - k["shift"][1] + src["origin"][1]
        nv = 2 * C.plane_at(src, bx, by) - 1
        gx = C.plane_at(src, bx + 1, by) - C.plane_at(src, bx - 1, by)
        gy = C.plane_at(src, bx, by + 1) - C.plane_at(src, bx, by - 1)
        if math.hypot(gx, gy) < 1e-5:
            gx = gy = 0.0
    t = math.radians(-90 + swirl * nv)
    c, sn = math.cos(t), math.sin(t)
    vx = (c * gx - sn * gy) * k["bias"][0]
    vy = (sn * gx + c * gy) * k["bias"][1]
    length = max(math.hypot(vx, vy), 1 / s)
    d = (vx / length, vy / length)
    cell, e = s / 50, k["edge"]
    x0, y0 = point(k, px, py, 0)
    sx, sy = x0 / cell, y0 / cell
    if e == 1:
        seed = C.value(7, sx, sy, k["z"], True)
    elif e == 0:
        seed = C.value(7, sx, sy, k["z"], False)
    else:
        seed = e * C.value(7, sx, sy, k["z"], True) + (1 - e) * C.value(7, sx, sy, k["z"], False)
    return [C.f32(d[0]), C.f32(d[1]), C.f32(nv), C.f32(seed)]


C.setup, C.field_at = setup, field_at


def small(rotation=0, scale=100, perspective_offset="off", **k):
    c = C.small(**k)
    c.update(rotation=rotation, scale=scale, perspective_offset=perspective_offset)
    return c


def render(c, frame_no):
    n = {k: C.held(c, k, frame_no) for k in C.RANGES}
    n["perspective_offset"] = c["perspective_offset"]
    words = {k: c[k] for k in C.WORDS + ("mirror_at_edge",)}
    return C.frame(C.curl_noise(C.layer_of(c), n, words, frame_no, c["float_depth"], C.the_map(c)),
                   c["shift"])


STILL = C.STILL
MOVING = {"speed": 20, "direction": 90, "turbulence_speed": 0}

CASES = {
    "FX-CURL-061": ("Rotation 90, still, Input Noise: the noise turned a quarter clockwise round "
                    "the card's middle.", small(rotation=90, view="input_noise", **STILL), [0]),
    "FX-CURL-062": ("Rotation 90, still, Swirl 0, Curl Generation: the flow turned with the "
                    "noise, a quarter clockwise.",
                    small(rotation=90, swirl=0, view="curl_generation", **STILL), [0]),
    "FX-CURL-063": ("Scale 200, still, Input Noise: the noise twice as big round the card's "
                    "middle, so it is Size 12 with Offset (-8, -5), FX-CURL-064.",
                    small(scale=200, view="input_noise", **STILL), [0]),
    "FX-CURL-064": ("Size 12, Offset (-8, -5), still, Input Noise.",
                    small(size=12, offset=(-8, -5), view="input_noise", **STILL), [0]),
    "FX-CURL-065": ("Scale 50, still: flow lines through cells half the size.",
                    small(scale=50, **STILL), [0]),
    "FX-CURL-066": ("Perspective Offset on, Speed 20, Direction 90, Turbulence Speed 0, Input "
                    "Noise: at frame 0 nothing has slid, so it is FX-CURL-011's frame 0; at frame "
                    "1 the finer octaves have slid less than the coarse one.",
                    small(perspective_offset="on", view="input_noise", **MOVING), [0, 1]),
    "FX-CURL-067": ("Rotation 45, Scale 150, Perspective Offset on, Offset (3, 2), Speed 20, "
                    "Direction 90: the Transform together, flow lines.",
                    small(rotation=45, scale=150, perspective_offset="on", offset=(3, 2),
                          **MOVING), [0, 2]),
    "FX-CURL-068": ("Source This Layer, Rotation 90, Scale 200, Perspective Offset on, still, "
                    "Input Noise: the picture is read unturned, so it is FX-CURL-045.",
                    small(source="this_layer", rotation=90, scale=200, perspective_offset="on",
                          view="input_noise", **STILL), [0]),
    "FX-CURL-069": ("Source This Layer, Rotation 90, still: only the seed is turned, so the lines "
                    "differ from Rotation 0's.", small(source="this_layer", rotation=90, **STILL),
                    [0]),
    "FX-CURL-070": ("Rotation keyed from 0 at frame 0 to 90 at frame 4, still, Input Noise: frame "
                    "0 is the unturned noise, frame 4 FX-CURL-061.",
                    small(rotation=keyed((0, 0), (4, 90)), view="input_noise", **STILL), [0, 4]),
    "FX-CURL-071": ("Rotation 30, still, after a Motion Tile that grows the layer: turned round "
                    "the drawing's middle, not the grown buffer's, so it is FX-CURL-072.",
                    small(rotation=30, tile=True, **STILL), [0]),
    "FX-CURL-072": ("Rotation 30, still.", small(rotation=30, **STILL), [0]),
    "FX-CURL-073": ("Rotation 360, still: a whole turn, the noise as unturned (within the "
                    "tolerance).", small(rotation=360, **STILL), [0]),
}

INVALID = {
    "FX-CURL-074": ("Scale 0.5, below 1.", small(scale=0.5)),
    "FX-CURL-075": ("Rotation 200000, above 100000.", small(rotation=200000)),
    "FX-CURL-076": ("Perspective Offset \"yes\", not on or off.", small(perspective_offset="yes")),
    "FX-CURL-077": ("Scale keyed to 20000 at frame 4, above 10000.",
                    small(scale=keyed((0, 100), (4, 20000)))),
}


def project_json(fx, c):
    p = C.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][-1]["parameters"]
    for k, v in STARTS.items():
        if c[k] != v:
            params[k] = c[k] if k == "perspective_offset" else setting_json(c[k])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = C.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = C.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_curl_transform.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((OUT / "expected_curl_noise.json").read_text(encoding="utf-8"))["cases"]
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = C.plain(small())
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    near = lambda p, q, e=1e-9: all(abs(a - b) <= e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(f[i], g[i], e) for i in range(W * H))  # noqa: E731
    differ = lambda f, g: sum(not near(f[i], g[i]) for i in shown) > len(shown) * 0.5  # noqa: E731

    # The defaults guard: with the Transform as added every earlier case is drawn as it was.
    for fx in ("FX-CURL-002", "FX-CURL-004", "FX-CURL-011", "FX-CURL-032", "FX-CURL-033",
               "FX-CURL-049"):
        src = old[fx]
        proj = json.loads((OUT / src["project"]).read_text(encoding="utf-8"))
        fxp = proj["compositions"][0]["layers"][0]["effects"][-1]["parameters"]
        assert not any(k in fxp for k in STARTS), fx
    assert same(render(small(), 0), old["FX-CURL-002"]["frames"]["0"], 0)
    assert same(render(small(source="this_layer"), 4), old["FX-CURL-033"]["frames"]["4"], 0)
    assert same(render(small(**MOVING, view="input_noise"), 1), old["FX-CURL-011"]["frames"]["1"], 0)

    still_input = render(small(view="input_noise", **STILL), 0)
    assert differ(c["FX-CURL-061"]["0"], still_input)
    # The slope is turned with the noise: the flow at p is R(90) of the unturned flow at q.
    k0 = setup({**{n: C.held(small(**STILL), n, 0) for n in C.RANGES}, "perspective_offset": "off"}, 0)
    k90 = setup({**{n: C.held(small(rotation=90, **STILL), n, 0) for n in C.RANGES},
                 "perspective_offset": "off"}, 0)
    for px, py in ((2.5, 3.5), (11.5, 7.5), (8.5, 0.5)):
        qx, qy = point(k90, px, py, 0)
        a = field_at(k90, 0, px, py)
        b = field_at(k0, 0, qx, qy)
        assert near(a[:2], [-b[1], b[0]], 1e-6), (a, b)
        assert abs(a[2] - b[2]) < 1e-6
    assert same(c["FX-CURL-063"]["0"], c["FX-CURL-064"]["0"], 1e-9)
    assert differ(c["FX-CURL-065"]["0"], render(small(**STILL), 0))
    k66 = c["FX-CURL-066"]
    assert same(k66["0"], old["FX-CURL-011"]["frames"]["0"], 0)
    assert differ(k66["1"], old["FX-CURL-011"]["frames"]["1"])
    assert differ(c["FX-CURL-067"]["0"], c["FX-CURL-067"]["2"])
    assert same(c["FX-CURL-068"]["0"], old["FX-CURL-045"]["frames"]["0"], 0)
    assert differ(c["FX-CURL-069"]["0"], render(small(source="this_layer", **STILL), 0))
    assert same(c["FX-CURL-070"]["0"], still_input, 0)
    assert same(c["FX-CURL-070"]["4"], c["FX-CURL-061"]["0"], 0)
    assert same(c["FX-CURL-071"]["0"], c["FX-CURL-072"]["0"], 0)
    assert same(c["FX-CURL-073"]["0"], render(small(**STILL), 0), 1e-6)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), name
    print("checked")


if __name__ == "__main__":
    main()
