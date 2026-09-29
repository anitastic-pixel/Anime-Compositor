"""Radio Waves, worked a second way.

D-200 adds `core.radio_waves`, After Effects' Radio Waves by a rule of our own: rings sent out
from a point one after another, each growing as it ages, turning and drifting if asked, fading in
and out and thinning or thickening, the shockwave of an anime hit, a sonar ping or a radio mast's
signal. Each ring is a regular polygon, 64 sides by default, which looks round. They are painted
over the layer, empty parts and all, inside the layer's rectangle; the layer does not grow. It is
modelled on After Effects' Radio Waves and not claimed to match it; nothing is ported. Document
21 is the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an
earlier effect may have grown. The producer is P = (producer_point_x / 100 W, producer_point_y /
100 H). With f the composition frame, a wave is born at each frame k * interval, k = 0, 1, 2 and
on, and is alive at f when its age t = f - k * interval is at least 0 and below `lifespan`. A wave
of age t has its centre at C = P + velocity * t * u(direction), u the unit vector that many
degrees clockwise from up, exact at quarter turns; its corners R = expansion * t out, the first at
theta = orientation + spin * t degrees clockwise from up and the rest every a = 360 / n degrees,
n = floor(sides); its width w = start_width + (end_width - start_width) * t / lifespan; and its
strength g = opacity / 100 * min(1, t / fade_in_time) * min(1, (lifespan - t) / fade_out_time),
each fade's factor 1 when its time is 0. At a pixel whose centre is X in the drawing's own space,
with q = X - C, phi the angle of q in degrees clockwise from up, atan2(q.x, -q.y), and delta = phi
- theta - a (floor((phi - theta) / a) + 0.5), the pixel's distance from the wave's outline is
s = | |q| cos(delta) - R cos(180 / n) |, its distance from the edge it faces, so corners are
mitred. With h = w / 2, the wave covers c = g * the profile: "square", clamp(min(s + 0.5, h) -
max(s - 0.5, -h), 0, 1), the share of a pixel-wide box a line w wide covers; "triangle", max(0, 1 -
s / h); "sine", cos(90 s / h degrees) when s < h, else 0; and 0 when w is 0. With T the product
of (1 - c) over the waves alive, K the colour's linear value and O the pixel, premultiplied, the
output is T O + (1 - T) (K, 1): the waves painted over the layer. A pixel no wave reaches, T = 1,
is left exactly as it is. A draft scales expansion, velocity and both widths as distances.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Lightning Bolt's night, imported from `tools/lightning_bolt_reference.py`, its
left half a night sky and its right half empty. The drawing goes into
`Fixtures/radio_waves/media`, the projects into `Fixtures/radio_waves`, and the expected frames
into `Fixtures/radio_waves/expected_radio_waves.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/radio_waves_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as RC  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from rim_light_reference import toward  # noqa: E402
from lightning_bolt_reference import DRAWINGS  # noqa: E402

W, H = RC.W, RC.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "radio_waves"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"producer_point": (-1000, 1000), "sides": (3, 64), "interval": (1, 1000),
          "expansion": (0, 1000), "orientation": (-3600, 3600), "direction": (-3600, 3600),
          "velocity": (0, 1000), "spin": (-360, 360), "lifespan": (1, 1000), "opacity": (0, 100),
          "fade_in_time": (0, 1000), "fade_out_time": (0, 1000), "start_width": (0, 1000),
          "end_width": (0, 1000)}
NUMBERS = tuple(RANGES)
WORDS = ("profile", "color")
FLOORED = ("sides",)
ORANGE = "#ffb040"


# --- the rule -------------------------------------------------------------------------------

def ages(frame_no, interval, lifespan):
    """The ages of the waves alive at the frame, the oldest first."""
    out, k = [], 0
    while k * interval <= frame_no:
        t = frame_no - k * interval
        if t < lifespan:
            out.append(t)
        k += 1
    return out


def gap(q, R, n, theta):
    """How far q, from the wave's centre, lies from the outline of the regular n-gon whose
    corners are R out, the first theta degrees clockwise from up: from the edge it faces."""
    a = 360 / n
    phi = math.degrees(math.atan2(q[0], -q[1]))
    delta = phi - theta - a * (math.floor((phi - theta) / a) + 0.5)
    return abs(math.hypot(q[0], q[1]) * math.cos(math.radians(delta)) - R * math.cos(math.pi / n))


def profile_at(profile, s, w):
    h = w / 2
    if h == 0:
        return 0.0
    if profile == "square":
        return min(1.0, max(0.0, min(s + 0.5, h) - max(s - 0.5, -h)))
    if profile == "triangle":
        return max(0.0, 1 - s / h)
    return math.cos(math.radians(90 * s / h)) if s < h else 0.0


def fade(t, time):
    return min(1.0, t / time) if time > 0 else 1.0


def alive(s, frame_no, dw, dh):
    """The waves alive at the frame: (centre, R, theta, width, strength), the empty ones left
    out."""
    P = (s["producer_point"][0] / 100 * dw, s["producer_point"][1] / 100 * dh)
    u = toward(s["direction"])
    out = []
    for t in ages(frame_no, s["interval"], s["lifespan"]):
        w = s["start_width"] + (s["end_width"] - s["start_width"]) * t / s["lifespan"]
        g = (s["opacity"] / 100 * fade(t, s["fade_in_time"])
             * fade(s["lifespan"] - t, s["fade_out_time"]))
        if w == 0 or g == 0:
            continue
        d = s["velocity"] * t
        out.append(((P[0] + d * u[0], P[1] + d * u[1]), s["expansion"] * t,
                    s["orientation"] + s["spin"] * t, w, g))
    return out


def radio_waves(layer, s, profile, color, frame_no, dw, dh):
    """The layer with the waves painted over it; its rectangle kept. The drawing's own space is
    the layer's, its corner at the layer's (0, 0) and the layer's `left` and `top` the growth of
    earlier effects, negated. `s` holds the numbers, held and floored."""
    K = [srgb_to_linear(v / 255) for v in RC.hex_color(color.lower())]
    waves = alive(s, frame_no, dw, dh)
    n = s["sides"]
    out = []
    for i, o in enumerate(layer["px"]):
        X = (layer["left"] + i % layer["w"] + 0.5, layer["top"] + i // layer["w"] + 0.5)
        T = 1.0
        for C, R, theta, w, g in waves:
            T *= 1 - g * profile_at(profile, gap((X[0] - C[0], X[1] - C[1]), R, n, theta), w)
        if T == 1:
            out.append(list(o))
        else:
            out.append([T * o[c] + (1 - T) * K[c] for c in range(3)] + [T * o[3] + (1 - T)])
    return dict(layer, px=out)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, tile=False, producer_point=(50, 50), sides=64, interval=24, expansion=5,
         orientation=0, direction=90, velocity=0, spin=0, lifespan=96, opacity=100,
         fade_in_time=0, fade_out_time=48, start_width=5, end_width=5, profile="square",
         color="#ffffff"):
    return {"drawing": "night", "shift": shift, "tile": tile, "producer_point": producer_point,
            "sides": sides, "interval": interval, "expansion": expansion,
            "orientation": orientation, "direction": direction, "velocity": velocity,
            "spin": spin, "lifespan": lifespan, "opacity": opacity, "fade_in_time": fade_in_time,
            "fade_out_time": fade_out_time, "start_width": start_width, "end_width": end_width,
            "profile": profile, "color": color}


def ring(**kw):
    """One wave, a square standing on its side, corners 2 pixels further out each frame, a
    pixel wide and never fading: at frame 2 its sides run 2.83 pixels from the middle."""
    return case(**{"sides": 4, "orientation": 45, "interval": 10, "expansion": 2, "lifespan": 10,
                   "fade_out_time": 0, "start_width": 1, "end_width": 1, **kw})


def circles(**kw):
    """As `ring`, round, 64 sides, a new wave every other frame."""
    return ring(**{"sides": 64, "orientation": 0, "interval": 2, "lifespan": 5, **kw})


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (tuple, list)):
        return tuple(min(hi, max(lo, e)) for e in v)
    v = min(hi, max(lo, v))
    return math.floor(v) if k in FLOORED else v


def drawn_layer(name):
    return {"px": [RC.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def render(c, frame_no):
    layer = drawn_layer(c["drawing"])
    if c["tile"]:
        layer = motion_tile(layer, 300, 300, "off")
    s = {k: held(c, k, frame_no) for k in NUMBERS}
    return frame(radio_waves(layer, s, c["profile"], c["color"], frame_no, W, H), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-RWAVE-001": ("The settings as they start: producer point (50, 50), 64 sides, a wave "
                     "every 24 frames growing 5 pixels a frame, orientation 0, direction 90, "
                     "velocity 0, spin 0, lifespan 96, opacity 100, fade-in 0, fade-out 48, "
                     "widths 5 and 5, square, white. At frame 0 the first wave is born, a white "
                     "dot 5 pixels across in the middle; at frame 2 it is a ring 10 pixels out "
                     "that only the frame's corners reach; by frame 4 it has left the frame.",
                     case(), [0, 2, 4]),
    "FX-RWAVE-002": ("Opacity 0: no waves, the drawing untouched.", case(opacity=0), [0, 2]),
    "FX-RWAVE-003": ("Start width 0 and end width 0: no waves, the drawing untouched.",
                     case(start_width=0, end_width=0), [0, 2]),
    "FX-RWAVE-004": ("\"The ring\": four sides, orientation 45, a wave every 10 frames growing "
                     "2 pixels a frame, lifespan 10, no fades, widths 1 and 1. At frame 2 it is "
                     "a square round the middle, (8, 5), its sides 2.83 pixels out, each side's "
                     "pixel-wide line shared between two rows or columns, 0.672 and 0.328, over "
                     "the night and the empty half alike.",
                     ring(), [2]),
    "FX-RWAVE-005": ("As the ring, three sides, orientation 0: a triangle, its top corner at "
                     "(8, 1) and its foot along y = 7, half in row 6 and half in row 7.",
                     ring(sides=3, orientation=0), [2]),
    "FX-RWAVE-006": ("As the ring, orientation 0 and spin 22.5 degrees a frame: at frame 2 the "
                     "wave has turned 45 degrees, FX-RWAVE-004 exactly.",
                     ring(orientation=0, spin=22.5), [2]),
    "FX-RWAVE-007": ("As the ring, velocity 1 toward direction 90: at frame 2 the wave's middle "
                     "has drifted 2 pixels right, the same as the ring sent from (62.5, 50).",
                     ring(velocity=1), [2]),
    "FX-RWAVE-008": ("Round, 64 sides, a wave every other frame, lifespan 5, otherwise the "
                     "ring: at frame 4 three waves, a dot in the middle born this frame, a ring "
                     "4 pixels out and one 8 pixels out that touches the frame's left and right "
                     "edges.",
                     circles(), [4]),
    "FX-RWAVE-009": ("As FX-RWAVE-008, lifespan 3: the wave born at frame 0 has died by frame "
                     "4, and only the dot and the ring 4 pixels out are left.",
                     circles(lifespan=3), [4]),
    "FX-RWAVE-010": ("As the ring, start width 4, end width 0, lifespan 4: born 4 pixels wide, "
                     "at frame 2, halfway through its life, it is 2 pixels wide.",
                     ring(start_width=4, end_width=0, lifespan=4), [0, 2]),
    "FX-RWAVE-011": ("As the ring, lifespan 4, fade-in 2, fade-out 2: at half strength at frame "
                     "1, whole at frame 2, FX-RWAVE-004, and at half again at frame 3.",
                     ring(lifespan=4, fade_in_time=2, fade_out_time=2), [1, 2, 3]),
    "FX-RWAVE-012": ("As the ring, widths 4 and 4, triangle: each side brightest on its line, "
                     "fading to nothing 2 pixels either side.",
                     ring(start_width=4, end_width=4, profile="triangle"), [2]),
    "FX-RWAVE-013": ("As FX-RWAVE-012, sine: softer at the middle, brighter than the triangle "
                     "wherever either reaches.",
                     ring(start_width=4, end_width=4, profile="sine"), [2]),
    "FX-RWAVE-014": ("As the ring in orange #ffb040 at opacity 50: the square painted half "
                     "strength, orange.",
                     ring(color=ORANGE, opacity=50), [2]),
    "FX-RWAVE-015": ("As FX-RWAVE-008, the producer point keyed from (25, 50) at frame 0 to "
                     "(75, 50) at frame 4, linear: every wave alive is sent from where the point "
                     "is now, so frame 4's three are round (12, 5).",
                     circles(producer_point=keyed((0, (25, 50)), (4, (75, 50)))), [0, 4]),
    "FX-RWAVE-016": ("As the ring, the opacity eased from 0 at frame 0 to 100 at frame 4 past "
                     "its end: frame 0 is the drawing, and frame 2 is held at 100, "
                     "FX-RWAVE-004.",
                     ring(opacity=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-RWAVE-017": ("As the ring, orientation 405, a turn and 45 degrees: FX-RWAVE-004.",
                     ring(orientation=405), [2]),
    "FX-RWAVE-018": ("As the ring, the layer moved three pixels right: the square moves with it, "
                     "round (11, 5).",
                     ring(shift=3), [2]),
    "FX-RWAVE-019": ("As the ring, round, after a Motion Tile at 300% by 300%, the producer "
                     "point at (0, 50), the drawing's left edge, the layer moved eight pixels "
                     "right: a ring round (8, 5) of the frame, drawn across the tile on the "
                     "left and the drawing on the right alike.",
                     ring(sides=64, orientation=0, tile=True, producer_point=(0, 50), shift=8),
                     [2]),
    "FX-RWAVE-020": ("As the ring, 4.7 sides, counted as 4: FX-RWAVE-004 exactly.",
                     ring(sides=4.7), [2]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RWAVE-021": ("Sides 2, below 3.", ring(sides=2)),
    "FX-RWAVE-022": ("Interval 0.5, below 1.", ring(interval=0.5)),
    "FX-RWAVE-023": ("Lifespan 0, below 1.", ring(lifespan=0)),
    "FX-RWAVE-024": ("Opacity keyed to 150 at frame 4.",
                     ring(opacity=keyed((0, 100), (4, 150)))),
    "FX-RWAVE-025": ("Start width -1, below 0.", ring(start_width=-1)),
    "FX-RWAVE-026": ("Profile \"bell\", which is not one.", ring(profile="bell")),
    "FX-RWAVE-027": ("Colour \"#fff\", written in three digits, not six.", ring(color="#fff")),
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
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "off"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.radio_waves",
                    "enabled": True,
                    "parameters": {k: setting_json(c[k]) for k in NUMBERS + WORDS}})
    comp["layers"][0]["effects"] = effects
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

    (OUT / "expected_radio_waves.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g: all(close(p, q) for p, q in zip(f, g))  # noqa: E731
    orange = [srgb_to_linear(v / 255) for v in RC.hex_color(ORANGE)]

    def lit(f, x, y, base=drawn, K=(1.0, 1.0, 1.0)):
        """How much of the waves' colour a pixel took: 1 - T."""
        o, p = base[at(x, y)], f[at(x, y)]
        return p[3] - o[3] if o[3] == 0 else (p[2] - o[2]) / (K[2] - o[2])

    def painted(f, base=drawn, K=(1.0, 1.0, 1.0)):
        """Every pixel is T O + (1 - T) (K, 1) for its own T."""
        for x in range(W):
            for y in range(H):
                A, o = lit(f, x, y, base, K), base[at(x, y)]
                assert -1e-12 <= A <= 1 + 1e-12
                assert close(f[at(x, y)], [(1 - A) * o[i] + A * K[i] for i in range(3)]
                             + [(1 - A) * o[3] + A]), (x, y)

    # The rule's own pieces.
    assert ages(4, 2, 5) == [4, 2, 0] and ages(4, 2, 3) == [2, 0] and ages(1, 10, 10) == [1]
    assert profile_at("square", 0.5, 1) == 0.5 and profile_at("square", 0, 1) == 1
    assert profile_at("square", 1, 1) == 0 and profile_at("square", 0, 0.5) == 0.5
    assert profile_at("triangle", 1, 4) == 0.5 and profile_at("triangle", 2, 4) == 0
    assert abs(profile_at("sine", 1, 4) - math.sqrt(0.5)) < 1e-15
    assert all(profile_at(p, 0, 0) == 0 for p in ("square", "triangle", "sine"))
    assert abs(gap((3, 0), 4, 4, 45) - (3 - 2 * math.sqrt(2))) < 1e-12
    assert abs(gap((0, -3), 4, 4, 45) - (3 - 2 * math.sqrt(2))) < 1e-12
    assert abs(gap((0, 0), 4, 3, 0) - 2) < 1e-12 and abs(gap((0, -4), 4, 3, 0)) < 1e-12
    assert fade(1, 2) == 0.5 and fade(5, 0) == 1 and fade(3, 2) == 1

    for name, frames in c.items():
        if name not in ("FX-RWAVE-014", "FX-RWAVE-018", "FX-RWAVE-019") and "warning" not in \
                expected["cases"][name]:
            for f in frames.values():
                painted(f)

    one = c["FX-RWAVE-001"]
    mid = [(7, 4), (8, 4), (7, 5), (8, 5)]
    assert all(abs(lit(one["0"], x, y) - 1) < 1e-12 for x, y in mid) and lit(one["0"], 11, 5) == 0
    assert lit(one["2"], 0, 0) > 0.5 and lit(one["2"], 15, 9) > 0.5
    assert all(lit(one["2"], x, y) == 0 for x in range(3, 13) for y in range(10))
    assert one["4"] == drawn
    for name in ("FX-RWAVE-002", "FX-RWAVE-003"):
        assert all(f == drawn for f in c[name].values())

    four = c["FX-RWAVE-004"]["2"]
    for x in range(W):
        for y in range(H):
            assert abs(lit(four, x, y) - lit(four, 15 - x, y)) < 1e-12
            assert abs(lit(four, x, y) - lit(four, x, 9 - y)) < 1e-12
    for y in (4, 5):
        assert abs(lit(four, 5, y) + lit(four, 4, y) - 1) < 1e-12
        assert abs(lit(four, 5, y) - (3.5 - 2 * 2 ** 0.5)) < 1e-12  # 0.672
    assert abs(lit(four, 7, 2) + lit(four, 7, 1) - 1) < 1e-12
    assert all(lit(four, x, y) == 0 for x in range(6, 10) for y in range(3, 7))
    assert all(lit(four, x, y) == 0 for x in (0, 1, 2, 13, 14, 15) for y in range(10))

    five = c["FX-RWAVE-005"]["2"]
    for x in range(W):
        for y in range(H):
            assert abs(lit(five, x, y) - lit(five, 15 - x, y)) < 1e-12
    assert all(abs(lit(five, x, y) - 0.5) < 1e-12 for x in (6, 7, 8, 9) for y in (6, 7))
    assert lit(five, 7, 0) > 0.3 and lit(five, 7, 1) > 0.8 and lit(five, 7, 8) == 0
    assert lit(five, 7, 4) == 0

    assert c["FX-RWAVE-006"]["2"] == four
    assert c["FX-RWAVE-007"]["2"] == render(ring(producer_point=(62.5, 50)), 2)

    eight = c["FX-RWAVE-008"]["4"]
    assert all(lit(eight, x, y) > 0.29 for x, y in mid)            # the dot
    assert all(lit(eight, x, 4) > 0.45 for x in (0, 15))           # 8 out
    assert lit(eight, 3, 4) > 0.45 and lit(eight, 12, 4) > 0.45    # 4 out
    assert lit(eight, 5, 4) == 0 and lit(eight, 2, 1) == 0
    nine = c["FX-RWAVE-009"]["4"]
    near = [(x, y) for x in range(W) for y in range(H) if math.hypot(x - 7.5, y - 4.5) < 6]
    assert all(nine[at(x, y)] == eight[at(x, y)] for x, y in near)
    assert all(lit(nine, x, y) == 0 for x, y in [(0, 4), (0, 5), (15, 4), (15, 5)])

    ten = c["FX-RWAVE-010"]
    assert ten["2"] == render(ring(start_width=2, end_width=2, lifespan=4), 2)
    assert all(abs(lit(ten["0"], x, y) - 1) < 1e-12 for x, y in mid) and lit(ten["0"], 10, 5) < 1e-12

    eleven = c["FX-RWAVE-011"]
    assert eleven["2"] == four
    for f in ("1", "3"):
        whole = render(ring(lifespan=4), int(f))
        assert any(lit(whole, x, y) > 0 for x in range(W) for y in range(H))
        assert all(abs(lit(eleven[f], x, y) - 0.5 * lit(whole, x, y)) < 1e-12
                   for x in range(W) for y in range(H))

    twelve, thirteen = c["FX-RWAVE-012"]["2"], c["FX-RWAVE-013"]["2"]
    s5 = 2 * 2 ** 0.5 - 2.5  # column 5's distance from the square's left side
    assert abs(lit(twelve, 5, 4) - (1 - s5 / 2)) < 1e-12
    assert abs(lit(thirteen, 5, 4) - math.cos(math.radians(90 * s5 / 2))) < 1e-12
    for x in range(W):
        for y in range(H):
            t, s = lit(twelve, x, y), lit(thirteen, x, y)
            assert (t == 0) == (s == 0) and s >= t - 1e-15

    fourteen = c["FX-RWAVE-014"]["2"]
    painted(fourteen, K=orange)
    assert all(abs(lit(fourteen, x, y, K=orange) - 0.5 * lit(four, x, y)) < 1e-12
               for x in range(W) for y in range(H))

    fifteen = c["FX-RWAVE-015"]
    assert fifteen["0"] == render(circles(producer_point=(25, 50)), 0)
    assert fifteen["4"] == render(circles(producer_point=(75, 50)), 4)
    sixteen = c["FX-RWAVE-016"]
    assert ease(OVERSHOOT, 0.5) > 1 and sixteen["0"] == drawn and sixteen["2"] == four
    assert same(c["FX-RWAVE-017"]["2"], four)

    eighteen = c["FX-RWAVE-018"]["2"]
    moved = plain(ring(shift=3))
    painted(eighteen, base=moved)
    assert all(abs(lit(eighteen, x + 3, y, base=moved) - lit(four, x, y)) < 1e-12
               for x in range(W - 3) for y in range(H))

    nineteen = c["FX-RWAVE-019"]["2"]
    tiled = frame(motion_tile(drawn_layer("night"), 300, 300, "off"), 8)
    painted(nineteen, base=tiled)
    assert all(abs(lit(nineteen, x, y, base=tiled) - lit(nineteen, 15 - x, y, base=tiled))
               < 1e-12 for x in range(W) for y in range(H))
    assert lit(nineteen, 4, 4, base=tiled) > 0.45 and lit(nineteen, 11, 4, base=tiled) > 0.45
    assert lit(nineteen, 8, 4, base=tiled) == 0

    assert c["FX-RWAVE-020"]["2"] == four
    print("checked")


if __name__ == "__main__":
    main()
