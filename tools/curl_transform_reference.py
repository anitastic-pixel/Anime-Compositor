"""Curl Noise's Transform (Rotation, Scale, Perspective Offset, and D-750's Uniform Scaling,
Scale Width and Scale Height), worked a second way.

D-646 adds to `core.curl_noise` (D-446, D-583; `tools/curl_noise_reference.py`, whose rule, step
2, this file uses) the Transform controls After Effects' Curl Noise lists beside Size and Offset.
Adobe publishes no formula; the rule is this program's own, after the same controls of Fractal
Noise, and nothing is ported.

With c the drawing's middle (its width and height halved, in the drawing's own pixels),
theta = rotation in degrees, mx = my = 100 / scale with Uniform Scaling on, else
mx = 100 / scale_width and my = 100 / scale_height (D-750), the noise is read at

    q = c + (mx (d_x cos + d_y sin), my (-d_x sin + d_y cos)),  d = p - (offset + drift) - c,

so it is turned clockwise by Rotation and made Scale per cent as big (Scale Width per cent as
wide and Scale Height per cent as tall) round the drawing's middle, then slid on the picture by
Offset and the drift. The trace (D-749) runs in q, and Curl Generation's direction is brought
back to the picture's directions. With Perspective Offset on, octave o of the value noise is
read at q + (1 - 0.5^o) T(offset + drift), T the same turn and scale without c, so each finer
octave slides half as far as the one before, as if further away.

With Source This Layer or Other Layer the picture is read at p - (offset + drift) as D-583 says,
unturned and at its own size, and the Transform does nothing (D-749: the seed it turned before
is gone).

Settings: `rotation` -100000 to 100000 degrees, 0 when added; `scale` 1 to 10000 per cent,
100; `perspective_offset` `off` (when added) or `on`; D-750: `uniform_scaling` `on` (when added)
or `off`, `scale_width` and `scale_height` 1 to 10000 per cent, 100. A file without them reads
them as added, so a file from before D-750 reads Uniform Scaling on and is drawn by its Scale;
they are written only when they are not as added (D-304). The numbers can be keyed.

**This file never runs the build's code path.** Cases as in `tools/curl_noise_reference.py`:
a 16 by 10 composition holding Noise's card. The expected frames are in
`Fixtures/curl_noise/expected_curl_transform.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/curl_transform_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import curl_noise_reference as C  # noqa: E402
from fxkey_reference import keyed  # noqa: E402

W, H = C.W, C.H
OUT = C.OUT
TOLERANCE = 2e-5
STARTS = C.TRANSFORM_KEYS
small, render, setup, point, curl_at, project_json = (C.small, C.render, C.setup, C.point,
                                                      C.curl_at, C.project_json)


def numbers(c):
    n = {k: C.held(c, k, 0) for k in C.RANGES}
    n.update(perspective_offset=c["perspective_offset"], uniform_scaling=c["uniform_scaling"])
    return n


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
    "FX-CURL-069": ("Source This Layer, Rotation 90, still: the picture is read unturned, so it "
                    "is Rotation 0's frame (D-749: the seed Rotation turned is gone).",
                    small(source="this_layer", rotation=90, **STILL), [0]),
    "FX-CURL-070": ("Rotation keyed from 0 at frame 0 to 90 at frame 4, still, Input Noise: frame "
                    "0 is the unturned noise, frame 4 FX-CURL-061.",
                    small(rotation=keyed((0, 0), (4, 90)), view="input_noise", **STILL), [0, 4]),
    "FX-CURL-071": ("Rotation 30, still, after a Motion Tile that grows the layer: turned round "
                    "the drawing's middle, not the grown buffer's, so it is FX-CURL-072.",
                    small(rotation=30, tile=True, **STILL), [0]),
    "FX-CURL-072": ("Rotation 30, still.", small(rotation=30, **STILL), [0]),
    "FX-CURL-073": ("Rotation 360, still: a whole turn, the noise as unturned (within the "
                    "tolerance).", small(rotation=360, **STILL), [0]),
    # D-750 (row 15).
    "FX-CURL-079": ("Uniform Scaling off, Scale Width 200, Scale Height 100, still, Input Noise: "
                    "the noise twice as wide round the card's middle, as tall as before.",
                    small(uniform_scaling="off", scale_width=200, view="input_noise", **STILL),
                    [0]),
    "FX-CURL-080": ("Uniform Scaling off, Scale Width 200, Scale Height 200, still, Input Noise: "
                    "FX-CURL-063's Scale 200.",
                    small(uniform_scaling="off", scale_width=200, scale_height=200,
                          view="input_noise", **STILL), [0]),
    "FX-CURL-081": ("Uniform Scaling on, Scale Width 300 and Scale Height 50 written, still, "
                    "Input Noise: with Uniform Scaling on the two are not read, so it is Scale "
                    "100's frame.",
                    small(scale_width=300, scale_height=50, view="input_noise", **STILL), [0]),
    "FX-CURL-082": ("Uniform Scaling off, Scale Width 7500, Scale Height 5, still (a tutorial's "
                    "long streaks): the noise stretched far across and squeezed down, so the "
                    "greys change down the card far more than across it.",
                    small(uniform_scaling="off", scale_width=7500, scale_height=5, **STILL), [0]),
}

INVALID = {
    "FX-CURL-074": ("Scale 0.5, below 1.", small(scale=0.5)),
    "FX-CURL-075": ("Rotation 200000, above 100000.", small(rotation=200000)),
    "FX-CURL-076": ("Perspective Offset \"yes\", not on or off.", small(perspective_offset="yes")),
    "FX-CURL-077": ("Scale keyed to 20000 at frame 4, above 10000.",
                    small(scale=keyed((0, 100), (4, 20000)))),
    "FX-CURL-083": ("Scale Width 0.5, below 1.", small(uniform_scaling="off", scale_width=0.5)),
    "FX-CURL-084": ("Uniform Scaling \"yes\", not on or off.", small(uniform_scaling="yes")),
    "FX-CURL-085": ("Scale Height keyed to 20000 at frame 4, above 10000.",
                    small(uniform_scaling="off", scale_height=keyed((0, 100), (4, 20000)))),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    with open(OUT / name, "w", encoding="utf-8", newline="") as f:
        f.write(json.dumps(project_json(fx, c), indent=2) + "\n")
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
    with open(OUT / "expected_curl_transform.json", "w", encoding="utf-8", newline="") as f:
        f.write(json.dumps(expected, indent=1) + "\n")
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
    # The flow is turned with the noise: the direction at p is R(90) of the unturned one at q,
    # and the grey is the unturned one's.
    k0 = setup(numbers(small(**STILL)), (0, 0, 0))
    k90 = setup(numbers(small(rotation=90, **STILL)), (0, 0, 0))
    for px, py in ((2.5, 3.5), (11.5, 7.5), (8.5, 0.5)):
        qx, qy = point(k90, px, py)
        a, b = curl_at(k90, px, py), curl_at(k0, qx, qy)
        assert near(a[:2], [-b[1], b[0]], 1e-6), (a, b)
        assert abs(a[2] - b[2]) < 1e-6 and abs(a[3] - b[3]) < 1e-6
    assert same(c["FX-CURL-063"]["0"], c["FX-CURL-064"]["0"], 1e-9)
    assert differ(c["FX-CURL-065"]["0"], render(small(**STILL), 0))
    k66 = c["FX-CURL-066"]
    assert same(k66["0"], old["FX-CURL-011"]["frames"]["0"], 0)
    assert differ(k66["1"], old["FX-CURL-011"]["frames"]["1"])
    assert differ(c["FX-CURL-067"]["0"], c["FX-CURL-067"]["2"])
    assert same(c["FX-CURL-068"]["0"], old["FX-CURL-045"]["frames"]["0"], 0)
    assert same(c["FX-CURL-069"]["0"], render(small(source="this_layer", **STILL), 0), 0)
    assert same(c["FX-CURL-070"]["0"], still_input, 0)
    assert same(c["FX-CURL-070"]["4"], c["FX-CURL-061"]["0"], 0)
    assert same(c["FX-CURL-071"]["0"], c["FX-CURL-072"]["0"], 0)
    assert same(c["FX-CURL-073"]["0"], render(small(**STILL), 0), 1e-6)
    # D-750.
    assert differ(c["FX-CURL-079"]["0"], still_input)
    assert same(c["FX-CURL-080"]["0"], c["FX-CURL-063"]["0"], 0)
    assert same(c["FX-CURL-081"]["0"], still_input, 0)
    enc = lambda p: C.S.linear_to_srgb(p[0] / p[3])  # noqa: E731
    k82 = c["FX-CURL-082"]["0"]
    inside = lambda x, y: drawn[at(x, y)][3] == 1  # noqa: E731
    across = sum(abs(enc(k82[at(x + 1, y)]) - enc(k82[at(x, y)])) for x in range(W - 1)
                 for y in range(H) if inside(x, y) and inside(x + 1, y))
    down = sum(abs(enc(k82[at(x, y + 1)]) - enc(k82[at(x, y)])) for x in range(W)
               for y in range(H - 1) if inside(x, y) and inside(x, y + 1))
    assert across * 10 < down, (across, down)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), name
    print("checked")


if __name__ == "__main__":
    main()
