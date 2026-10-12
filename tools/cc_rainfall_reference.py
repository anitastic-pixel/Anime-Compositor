"""CC Rainfall, worked a second way.

D-582 (b), accepted by the owner on 2026-10-12 ("go with your recommendations on all six"),
adds CC Rainfall (`core.cc_rainfall`), after CycoreFX's CC Rainfall (Simulation): rain with
depth, drawn by the effect itself. The CycoreFX manual names its controls and says what each is
for; it publishes no formula, so the rule, the ranges and the settings when added are ours.
`core.rain` (D-163) and its fixtures are unchanged.

At composition frame n, at r frames a second, the settings held inside their ranges at n, on a
drawing w by h (its own size, before anything an earlier effect grew):

1. The drops are i = 0 .. floor(`drops`) - 1. Each takes five numbers u_k = (U(seed, i, 0, 0,
   k) + 1) / 2, k = 0 .. 4, U Noise's hash (D-119, `tools/noise_reference.py`), seed
   floor(`random_seed`).
2. Depth d = `scene_depth` u_0, so the drops fill a volume that deep behind the picture. With
   the focal length F = w, a drop's scale is s = F / (F + d): 1 at the front, smaller behind.
3. Its tilt is theta = `spread` (u_1 - 1/2) degrees and its own wind `wind` (1 +
   `wind_variation` / 100 (2 u_2 - 1)). On the picture it moves (vx, vy) = s (wind + `speed`
   tan theta, `speed`) pixels a second: far drops are slower.
4. Its streak is what it travels in half a frame, L = (vx, vy) / (2 r); its width ws = `size` s
   and its strength `opacity` / 100 s.
5. The ground is at g = `ground_level` / 100 h. A drop's head falls from top = -(L_y + e), e =
   max(ws / 2 + 1/2, 1), where its streak is wholly above the picture, to the ground, R = g -
   top, and starts again: p = frac(u_3 + vy (n / r) / R), head y = top + p R. Across, it starts
   from x0 = w (3/2 u_4 - 1/4) and drifts with its slope through the fall, centred on x0: head x
   = x0 + (vx / vy)(p R - R / 2), or x0 when vy is 0. The tail is the head less L.
6. At a pixel centre whose y is at most g, delta is its distance from the segment tail to head;
   the cover is clamp(ws / 2 + 1/2 - delta, 0, 1) when ws is at least 1, ws clamp(1 - delta, 0,
   1) when it is less (a hairline, fainter), and nothing when ws is 0 or a drop's R is 0.
7. The drops are laid far to near (d largest first; equal depths by i), each at its strength
   times its cover, in `color` (linear), normal, onto clear: the rain.
8. `composite_with_original` `off`: the rain alone. `on`: the rain over the drawing; with
   `transfer_mode` `lighten` each colour channel is then the larger of that and the drawing's,
   so the rain never darkens it; `composite` keeps it as laid.

`drops` 0 to 50000, 5000 when added; `size` 0 to 20 pixels, 3; `scene_depth` 0 to 20000 pixels,
5000; `speed` 0 to 20000 pixels a second, 4000; `wind` -5000 to 5000 pixels a second, 0;
`wind_variation` 0 to 100 per cent, 0; `spread` 0 to 90 degrees, 15; `color` `#rrggbb`,
#b4bec8; `opacity` 0 to 100, 25; `transfer_mode` `composite` or `lighten`, lighten;
`composite_with_original` `off` or `on`, on; `ground_level` 0 to 100 per cent, 100;
`random_seed` 0 to 100000, 0. Every number is keyable. The layer does not grow. A draft scales
size, scene depth, speed and wind with the picture.

Not built (gaps): Background Reflection, the Refracting appearance (every drop is drawn as a
soft solid), Offset and Embed Depth.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 48 pixels by 32 holding one drawing the same size, unmoved: a night
of #1e1a24, a pale wall of #d0d0d0, or the night with its right half clear. The drawings go into
`Fixtures/cc_rainfall/media`, the projects into `Fixtures/cc_rainfall`, and the expected frames
into `Fixtures/cc_rainfall/expected_cc_rainfall.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_rainfall_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
from noise_reference import u as U  # noqa: E402

W, H, FRAMES = 48, 32, 5
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_rainfall"
TOLERANCE = 2e-5  # document 25's default for a filter
WRAP = 1e-9       # no drop any case draws is this close to starting its fall again
TYPE = "core.cc_rainfall"
RANGES = {"drops": (0, 50000), "size": (0, 20), "scene_depth": (0, 20000),
          "speed": (0, 20000), "wind": (-5000, 5000), "wind_variation": (0, 100),
          "spread": (0, 90), "opacity": (0, 100), "ground_level": (0, 100),
          "random_seed": (0, 100000)}
WORDS = {"transfer_mode": ("composite", "lighten"), "composite_with_original": ("off", "on")}
NAMES = ("drops", "size", "scene_depth", "speed", "wind", "wind_variation", "spread", "color",
         "opacity", "transfer_mode", "composite_with_original", "ground_level", "random_seed")
NEAR = []  # the smallest distance to a wrap each case's drops come, for the printout


# --- the rule -------------------------------------------------------------------------------

def hex_linear(s):
    return [srgb_to_linear(int(s[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def segment_distance(px, py, ax, ay, bx, by):
    dx, dy = bx - ax, by - ay
    ll = dx * dx + dy * dy
    t = 0.0 if ll == 0 else min(1.0, max(0.0, ((px - ax) * dx + (py - ay) * dy) / ll))
    return math.hypot(px - (ax + t * dx), py - (ay + t * dy))


def drops_of(n, rate, v, w, h):
    """Steps 1 to 5: each drop's depth, index, head, tail, width and strength."""
    seed = math.floor(v["random_seed"])
    F = w
    g = v["ground_level"] / 100 * h
    tau = n / rate
    out = []
    for i in range(math.floor(v["drops"])):
        u = [(U(seed, i, 0, 0, k) + 1) / 2 for k in range(5)]
        d = v["scene_depth"] * u[0]
        s = F / (F + d)
        theta = math.radians(v["spread"] * (u[1] - 0.5))
        wind = v["wind"] * (1 + v["wind_variation"] / 100 * (2 * u[2] - 1))
        vx, vy = s * (wind + v["speed"] * math.tan(theta)), s * v["speed"]
        lx, ly = vx / (2 * rate), vy / (2 * rate)
        ws = v["size"] * s
        e = max(ws / 2 + 0.5, 1.0)
        top = -(ly + e)
        R = g - top
        if ws == 0 or R <= 0:
            continue
        q = u[3] + vy * tau / R
        NEAR.append(abs(q - round(q)))
        p = q - math.floor(q)
        hy = top + p * R
        x0 = w * (1.5 * u[4] - 0.25)
        hx = x0 + (vx / vy) * (p * R - R / 2) if vy > 0 else x0
        out.append((d, i, (hx, hy), (hx - lx, hy - ly), ws, v["opacity"] / 100 * s))
    out.sort(key=lambda t: (-t[0], t[1]))
    return out, g


def rainfall(pixels, v, n, rate, w=W, h=H):
    """`pixels` 8-bit straight RGBA rows; `v` the held settings. The frame, premultiplied."""
    G = hex_linear(v["color"].lower())
    rain = [[0.0] * 4 for _ in range(w * h)]
    drops, g = drops_of(n, rate, v, w, h)
    for _, _, (hx, hy), (tx, ty), ws, strength in drops:
        reach = max(ws / 2 + 0.5, 1.0)
        x_lo, x_hi = math.floor(min(hx, tx) - reach), math.ceil(max(hx, tx) + reach)
        y_lo, y_hi = math.floor(min(hy, ty) - reach), math.ceil(max(hy, ty) + reach)
        for y in range(max(0, y_lo), min(h, y_hi + 1)):
            if y + 0.5 > g:
                continue
            for x in range(max(0, x_lo), min(w, x_hi + 1)):
                delta = segment_distance(x + 0.5, y + 0.5, hx, hy, tx, ty)
                if ws >= 1:
                    cover = min(1.0, max(0.0, ws / 2 + 0.5 - delta))
                else:
                    cover = ws * min(1.0, max(0.0, 1 - delta))
                a = strength * cover
                if a == 0:
                    continue
                r = rain[y * w + x]
                rain[y * w + x] = [G[c] * a + r[c] * (1 - a) for c in range(3)] + [a + r[3] * (1 - a)]
    out = []
    for i, p in enumerate(pixels):
        r = rain[i]
        if v["composite_with_original"] == "off":
            out.append(r)
            continue
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) * a for c in range(3)] + [a]
        over = [r[c] + b[c] * (1 - r[3]) for c in range(4)]
        if v["transfer_mode"] == "lighten":
            over = [max(over[c], b[c]) for c in range(3)] + [over[3]]
        out.append(over)
    return out


# --- the drawings ---------------------------------------------------------------------------

NIGHT = (30, 26, 36, 255)   # #1e1a24
WALL = (208, 208, 208, 255)  # #d0d0d0
DRAWINGS = {"night": [[NIGHT] * W for _ in range(H)],
            "wall": [[WALL] * W for _ in range(H)],
            "half": [[NIGHT if x < W // 2 else S.NONE for x in range(W)] for _ in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(drawing="night", rate=24, drops=5000, size=3, scene_depth=5000, speed=4000, wind=0,
         wind_variation=0, spread=15, color="#b4bec8", opacity=25, transfer_mode="lighten",
         composite_with_original="on", ground_level=100, random_seed=0):
    c = dict(locals())
    return c


# A few plain drops at the front, 10 pixels a frame down, a 5-pixel streak, fully strong; seed 6
# puts the first one near the middle.
FEW = dict(size=2, scene_depth=0, speed=240, spread=0, color="#c8d8ff", opacity=100,
           random_seed=6)


def few(drops=1, **kw):
    return case(**{**FEW, "drops": drops, **kw})


def held(c, k, n):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], n)))


def valid(c):
    for k, (lo, hi) in RANGES.items():
        v = c[k]
        values = [x[1] for x in v["keys"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    if not all(c[k] in words for k, words in WORDS.items()):
        return False
    s = c["color"]
    return len(s) == 7 and s[0] == "#" and all(ch in "0123456789abcdefABCDEF" for ch in s[1:])


def render(c, n):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    if not valid(c):
        return rainfall(pixels, {**case(), "drops": 0}, n, c["rate"])
    v = {k: held(c, k, n) for k in RANGES}
    v.update({k: c[k] for k in ("color", "transfer_mode", "composite_with_original")})
    return rainfall(pixels, v, n, c["rate"])


def plain(c):
    return render({**c, "drops": 0, "composite_with_original": "on"}, 0)


DARK = dict(color="#202020", drops=6)

CASES = {
    "FX-RAIN3D-001": ("As added: 5000 drops, size 3, scene depth 5000, speed 4000, wind 0, "
                      "variation 0, spread 15, colour #b4bec8, opacity 25, Lighten, composited "
                      "with the night. In a picture 48 wide the volume is a hundred times "
                      "deeper than the focal length, so most drops are far, faint and fine; "
                      "the few near ones are long pale streaks.", case(), (0, 2)),
    "FX-RAIN3D-002": ("One drop at the front, size 2, falling 240 pixels a second, 10 a frame "
                      "at 24, a streak 5 long, opacity 100: frame 1 is frame 0's streak 10 "
                      "pixels lower, frame 4 has it 40 lower, wrapped back to the top.",
                      few(), (0, 1, 4)),
    "FX-RAIN3D-003": ("The same drop, size 0.5: a hairline at half strength.", few(size=0.5),
                      (1,)),
    "FX-RAIN3D-004": ("Size 0: no drop is drawn, so the night is untouched.", few(size=0),
                      (1,)),
    "FX-RAIN3D-005": ("Drops 0: the night, untouched.", few(drops=0), (1,)),
    "FX-RAIN3D-006": ("Wind 120 pixels a second: the drop drifts 5 pixels right a frame as it "
                      "falls 10, its streak slanting.", few(wind=120), (0, 1)),
    "FX-RAIN3D-007": ("Six drops, wind 120, variation 50: each its own wind, between 60 and "
                      "180.", few(6, wind=120, wind_variation=50), (0, 2)),
    "FX-RAIN3D-008": ("Six drops, spread 60: each tilted its own way, up to 30 degrees either "
                      "side of straight down.", few(6, spread=60), (1,)),
    "FX-RAIN3D-009": ("Six drops in a volume 48 deep, the focal length: the far ones down to "
                      "half size, half speed, half strength, and laid first.",
                      few(6, scene_depth=48), (1,)),
    "FX-RAIN3D-010": ("Six drops, ground level 50: nothing below row 16.",
                      few(6, ground_level=50), (1,)),
    "FX-RAIN3D-011": ("Six drops, speed 0: each a round dot, still, so frame 4 is frame 0.",
                      few(6, speed=0), (0, 4)),
    "FX-RAIN3D-012": ("Six drops, seed 7: drops of their own.", few(6, random_seed=7), (1,)),
    "FX-RAIN3D-013": ("Seed 7.9 counts as 7: FX-RAIN3D-012.", few(6, random_seed=7.9), (1,)),
    "FX-RAIN3D-014": ("Six drops at opacity 40.", few(6, opacity=40), (1,)),
    "FX-RAIN3D-015": ("Six dark drops (#202020) on the pale wall, Composite: they darken it.",
                      few(drawing="wall", transfer_mode="composite", **DARK), (1,)),
    "FX-RAIN3D-016": ("The same with Lighten: rain never darkens, so the wall is untouched.",
                      few(drawing="wall", **DARK), (1,)),
    "FX-RAIN3D-017": ("Six drops, Composite With Original off: the rain alone, on clear.",
                      few(6, composite_with_original="off"), (1,)),
    "FX-RAIN3D-018": ("Six drops on the night with its right half clear: the rain falls over "
                      "the clear half too.", few(6, drawing="half"), (1,)),
    "FX-RAIN3D-019": ("FX-RAIN3D-007 at 30 frames a second: on frame 2 the drops have fallen "
                      "for a fifteenth of a second, not a twelfth, and their streaks are a "
                      "fifth shorter.", few(6, rate=30, wind=120, wind_variation=50), (2,)),
    "FX-RAIN3D-020": ("Drops keyed from 0 at frame 0 to 12 at frame 4: six on frame 2, "
                      "FX-RAIN3D-007's six, the same drops.",
                      few(keyed((0, 0), (4, 12)), wind=120, wind_variation=50), (0, 2)),
    "FX-RAIN3D-021": ("Forty drops with everything: depth 96, wind -60 with variation 80, "
                      "spread 30, size 3, opacity 70, ground level 90, seed 3.",
                      few(40, scene_depth=96, wind=-60, wind_variation=80, spread=30, size=3,
                          opacity=70, ground_level=90, random_seed=3), (0, 3)),
    "FX-RAIN3D-022": ("FX-RAIN3D-012 with its colour written in capitals, #C8D8FF: the same.",
                      few(6, random_seed=7, color="#C8D8FF"), (1,)),
}

# D-46: the file is read, the effect is kept as written and left out of every frame.
INVALID = {
    "FX-RAIN3D-023": ("Drops -1, below 0.", few(drops=-1)),
    "FX-RAIN3D-024": ("Drops 50001, above 50000.", few(drops=50001)),
    "FX-RAIN3D-025": ("Size 21, above 20.", few(size=21)),
    "FX-RAIN3D-026": ("Scene depth -1, below 0.", few(scene_depth=-1)),
    "FX-RAIN3D-027": ("Speed 20001, above 20000.", few(speed=20001)),
    "FX-RAIN3D-028": ("Wind 5001, above 5000.", few(wind=5001)),
    "FX-RAIN3D-029": ("Variation 101, above 100.", few(wind_variation=101)),
    "FX-RAIN3D-030": ("Spread 91, above 90.", few(spread=91)),
    "FX-RAIN3D-031": ("Opacity 101, above 100.", few(opacity=101)),
    "FX-RAIN3D-032": ("Ground level 101, above 100.", few(ground_level=101)),
    "FX-RAIN3D-033": ("Random seed -1, below 0.", few(random_seed=-1)),
    "FX-RAIN3D-034": ("Transfer mode written \"screen\".", few(transfer_mode="screen")),
    "FX-RAIN3D-035": ("Composite With Original written \"maybe\".",
                      few(composite_with_original="maybe")),
    "FX-RAIN3D-036": ("A colour written \"#c8d8f\", one digit short.", few(color="#c8d8f")),
    "FX-RAIN3D-037": ("Speed keyed to 30000 at frame 4, above 20000.",
                      few(speed=keyed((0, 240), (4, 30000)))),
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
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": c["rate"], "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-" + d,
                "enabled": True, "locked": False, "in_frame": 0, "out_frame": FRAMES,
                "source_offset_frames": 0,
                "transform": {
                    "anchor": prop([0, 0]), "position": prop([0, 0]),
                    "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
                },
                "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-1", "type_id": TYPE, "enabled": True,
                             "parameters": {k: setting_json(c[k]) for k in NAMES}}],
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
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        NEAR.clear()
        rendered = {str(f): render(c, f) for f in frames}
        assert not NEAR or min(NEAR) > WRAP, fx
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        assert not valid(c), fx
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_cc_rainfall.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    night = plain(few())
    wall = plain(few(drawing="wall"))
    changed = lambda a, b: [i for i in range(W * H) if a[i] != b[i]]  # noqa: E731
    assert len(changed(c["FX-RAIN3D-001"]["0"], night)) > 100
    # One drop, 10 pixels a frame: frame 1 is frame 0 ten rows lower.
    f0, f1 = c["FX-RAIN3D-002"]["0"], c["FX-RAIN3D-002"]["1"]
    rows0 = sorted({i // W for i in changed(f0, night)})
    rows1 = sorted({i // W for i in changed(f1, night)})
    assert rows0 and [r + 10 for r in rows0 if r + 10 < H] == [r for r in rows1 if r >= rows0[0] + 10]
    for x in range(W):
        for y in range(H - 10):
            assert all(abs(p - q) < 1e-12 for p, q in zip(f0[y * W + x], f1[(y + 10) * W + x]))
    hair = c["FX-RAIN3D-003"]["1"]
    assert max(p[2] - night[i][2] for i, p in enumerate(hair)) < 0.5 * (hex_linear("#c8d8ff")[2] - night[0][2]) + 1e-12
    for fx in ("FX-RAIN3D-004", "FX-RAIN3D-005"):
        assert c[fx]["1"] == night, fx
    wind = c["FX-RAIN3D-006"]
    cols0 = sorted({i % W for i in changed(wind["0"], night)})
    cols1 = sorted({i % W for i in changed(wind["1"], night)})
    assert sum(cols1) / len(cols1) > sum(cols0) / len(cols0) + 3
    assert all(i // W < 16 for i in changed(c["FX-RAIN3D-010"]["1"], night))
    assert c["FX-RAIN3D-011"]["0"] == c["FX-RAIN3D-011"]["4"]
    assert c["FX-RAIN3D-013"]["1"] == c["FX-RAIN3D-012"]["1"] == c["FX-RAIN3D-022"]["1"]
    assert c["FX-RAIN3D-012"]["1"] != c["FX-RAIN3D-008"]["1"]
    dark = c["FX-RAIN3D-015"]["1"]
    assert changed(dark, wall) and all(dark[i][0] < wall[i][0] for i in changed(dark, wall))
    assert c["FX-RAIN3D-016"]["1"] == wall
    alone = c["FX-RAIN3D-017"]["1"]
    assert any(p[3] == 0 for p in alone) and any(p[3] > 0 for p in alone)
    half = c["FX-RAIN3D-018"]["1"]
    assert any(half[y * W + x][3] > 0 for y in range(H) for x in range(W // 2, W))
    assert c["FX-RAIN3D-020"]["2"] == c["FX-RAIN3D-007"]["2"]
    assert c["FX-RAIN3D-020"]["0"] == night
    assert c["FX-RAIN3D-019"]["2"] != c["FX-RAIN3D-007"]["2"]
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
