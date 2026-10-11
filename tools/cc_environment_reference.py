"""CC Environment, worked a second way.

`core.cc_environment`, modelled on CycoreFX's CC Environment (Perspective), which shows an
environment picture (a Spherical Map, latitude and longitude; a Light Probe, an angular map; or a
Vertical Cross) around the viewer, "similar to 3D programs", looking about as the composition's
camera turns. The CycoreFX HD manual names the controls: Environment (the layer used as the
environment), Mapping (Spherical, Probe, Vertical Cross), Horizontal Pan, and Filter Environment
("filtered for increased image quality", used only with Spherical). It publishes no formula and
no settings when added, so the rule and those settings are ours (document 21, D-531).

It draws its own fake 3D. The composition's camera is not used (P0-12, a gap): in its place the
effect has its own view, Tilt, Roll and Field of View, beside the manual's Horizontal Pan, all
keyable, so the look about is animated on the effect, not on a camera.

The settings. `environment`, D-189's layer setting (`tools/effect_layer_reference.py`), ""
when added; the layer named is read whole, at its own size, never fitted to the holder.
`mapping`, "spherical" (when added), "probe" or "vertical_cross", exact. `horizontal_pan`,
`tilt` and `roll`, -3600 to 3600 degrees, 0 when added; `field_of_view`, 1 to 179 degrees across
the drawing's width, 40 when added (about a 50 mm lens on a 36 mm frame). `filter_environment`,
"on" (when added) or "off".

The rule. At composition frame n, with O the holder's input (linear, premultiplied), w by h,
whose drawing's own top-left pixel is at (ox, oy), its drawing W = w - 2 ox by H = h - 2 oy, and
E the environment's picture, Ew by Eh:

1. With `environment` "", or naming a layer not in the composition (D-189's warning), the
   output is O.
2. The ray through a point (x, y) of the drawing (y down, z away from the viewer):
   c = (x - W/2, y - H/2, F), F = (W/2) / tan(field_of_view / 2). Turned by roll r about z:
   (cx cos r - cy sin r, cx sin r + cy cos r, cz); then by tilt t about x, positive looking
   up: (x, y cos t - z sin t, y sin t + z cos t); then by pan p about y, positive looking
   right: (x cos p + z sin p, y, -x sin p + z cos p); d is that, made unit length.
3. The environment's colour along d, read from E by document 21's bilinear (pixel centres at
   halves) at a point (u, v) found by the mapping:
   - "spherical": u = Ew (1/2 + atan2(dx, dz) / (2 pi)), v = Eh (1/2 - asin(-dy) / pi), each
     tap's column taken round the width and its row held within the picture.
   - "probe": with s = sqrt(dx^2 + dy^2) and q = acos(dz) / (pi s) (q = 0 where s = 0, the
     middle), u = Ew (1 + dx q) / 2, v = Eh (1 + dy q) / 2, taps held within the picture: the
     view ahead at the middle, the view behind round the rim.
   - "vertical_cross": the picture three faces across and four down, each Ew/3 by Eh/4. The face
     is the axis d leans on most (x first, then y, then z, where they tie), with (a, b) on it,
     each -1 to 1: ahead (+z) column 1, row 1, (dx/dz, dy/dz); right (+x) column 2, row 1,
     (-dz/dx, dy/dx); left (-x) column 0, row 1, (dz/|dx|, dy/|dx|); up (-y) column 1, row 0,
     (dx/|dy|, dz/|dy|); down (+y) column 1, row 2, (dx/dy, -dz/dy); behind (-z) column 1, row 3,
     (dx/|dz|, -dy/|dz|), upside down as a vertical cross keeps it. u = Ew/3 (column +
     (a + 1)/2), v = Eh/4 (row + (b + 1)/2), each held within the face's pixel centres (from
     its first pixel's centre to its last's), then read with taps held within the picture.
   With `filter_environment` "on" and "spherical", the colour is the mean of four rays at
   (x +- 1/4, y +- 1/4) from the pixel's centre; otherwise the one ray through (x + 1/2, y + 1/2).
   An empty picture (the layer not yet begun) reads as transparent.
4. The output at buffer pixel (X, Y), x = X - ox, y = Y - oy: the environment's colour times
   O's alpha there, so the environment fills the layer's shape and nothing beyond it.

The layer never grows. A draft reads a drawing and an environment both smaller, every distance
being a share of them, so the same view, coarser.

**This file never runs the build's code path.** It works in double precision from the
drawings' 8-bit values. The rule steps only on a Vertical Cross where two axes tie and at the
point straight behind on a Probe: every case is checked that no ray lies near either.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/cc_environment/`:
the environment layers, every one switched off, and on top of them the layer `holder`, showing
the drawing `shot` (red, blue and cream squares, the bottom right 4 by 3 clear), with the
effect. The expected pixels are in `Fixtures/cc_environment/expected_cc_environment.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_environment_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_environment"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]


def sky(w, h):
    """A smooth picture: red rising across, green rising down, blue a slow wave across, a
    half-transparent pixel at (1, 1)."""
    return [[(255, 255, 255, 128) if (x, y) == (1, 1) else
             (round(20 + 215 * x / (w - 1)), round(30 + 200 * y / (h - 1)),
              round(128 + 100 * math.sin(2 * math.pi * x / w)), 255)
             for x in range(w)] for y in range(h)]


FACE = {(1, 0): (250, 250, 250), (0, 1): (220, 40, 40), (1, 1): (40, 200, 60),
        (2, 1): (40, 80, 220), (1, 2): (60, 40, 30), (1, 3): (240, 200, 40)}


def cross():
    """A vertical cross 6 by 8, faces 2 by 2: up white, left red, ahead green, right blue, down
    brown, behind yellow, each face's lower right pixel a little darker; the corners clear."""
    out = []
    for y in range(8):
        row = []
        for x in range(6):
            f = FACE.get((x // 2, y // 2))
            if f is None:
                row.append((0, 0, 0, 0))
            else:
                k = 0.7 if (x % 2, y % 2) == (1, 1) else 1.0
                row.append(tuple(round(v * k) for v in f) + (255,))
        out.append(row)
    return out


DRAWINGS = {
    "shot": K.DRAWINGS["holder"],
    "sphere": sky(16, 8),
    "probe": sky(8, 8),
    "cross": cross(),
}

MAPS = {"sphere": "sphere", "probe": "probe", "cross": "cross", "late": "sphere"}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


# --- the rule -------------------------------------------------------------------------------

def ray(k, x, y):
    """Step 2: the unit direction through the drawing's point (x, y)."""
    f = (W / 2) / math.tan(math.radians(k["field_of_view"]) / 2)
    cx, cy, cz = x - W / 2, y - H / 2, f
    r, t, p = (math.radians(k[n]) for n in ("roll", "tilt", "horizontal_pan"))
    cx, cy = cx * math.cos(r) - cy * math.sin(r), cx * math.sin(r) + cy * math.cos(r)
    cy, cz = cy * math.cos(t) - cz * math.sin(t), cy * math.sin(t) + cz * math.cos(t)
    cx, cz = cx * math.cos(p) + cz * math.sin(p), -cx * math.sin(p) + cz * math.cos(p)
    n = math.sqrt(cx * cx + cy * cy + cz * cz)
    return cx / n, cy / n, cz / n


def sample(e, u, v, wrap):
    """Document 21's bilinear at (u, v); columns taken round with `wrap`, else held; rows held."""
    w, h = e["w"], e["h"]
    if w == 0 or h == 0:
        return list(CLEAR)
    fx, fy = u - 0.5, v - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx = (x0 + dx) % w if wrap else min(max(x0 + dx, 0), w - 1)
            sy = min(max(y0 + dy, 0), h - 1)
            p = e["px"][sy * w + sx]
            for i in range(4):
                out[i] += p[i] * wx * wy
    return out


def face(d):
    """The Vertical Cross's face: (column, row, a, b)."""
    dx, dy, dz = d
    ax, ay, az = abs(dx), abs(dy), abs(dz)
    if ax >= ay and ax >= az:
        return (2, 1, -dz / dx, dy / dx) if dx > 0 else (0, 1, dz / ax, dy / ax)
    if ay >= az:
        return (1, 0, dx / ay, dz / ay) if dy < 0 else (1, 2, dx / dy, -dz / dy)
    return (1, 1, dx / dz, dy / dz) if dz > 0 else (1, 3, dx / az, -dy / az)


def look(e, mapping, d):
    """Step 3 for one ray."""
    dx, dy, dz = d
    ew, eh = e["w"], e["h"]
    if mapping == "spherical":
        u = ew * (0.5 + math.atan2(dx, dz) / (2 * math.pi))
        v = eh * (0.5 - math.asin(max(-1.0, min(1.0, -dy))) / math.pi)
        return sample(e, u, v, True)
    if mapping == "probe":
        s = math.sqrt(dx * dx + dy * dy)
        q = 0.0 if s == 0 else math.acos(max(-1.0, min(1.0, dz))) / (math.pi * s)
        return sample(e, ew * (1 + dx * q) / 2, eh * (1 + dy * q) / 2, False)
    col, row, a, b = face(d)
    fw, fh = ew / 3, eh / 4
    u = min(max(fw * (col + (a + 1) / 2), col * fw + 0.5), (col + 1) * fw - 0.5)
    v = min(max(fh * (row + (b + 1) / 2), row * fh + 0.5), (row + 1) * fh - 0.5)
    return sample(e, u, v, False)


def colour_at(e, k, x, y):
    """Step 3 at the drawing's pixel (x, y), with Filter Environment."""
    if k["filter_environment"] == "on" and k["mapping"] == "spherical":
        taps = [look(e, "spherical", ray(k, x + 0.5 + sx, y + 0.5 + sy))
                for sy in (-0.25, 0.25) for sx in (-0.25, 0.25)]
        return [sum(t[i] for t in taps) / 4 for i in range(4)]
    return look(e, k["mapping"], ray(k, x + 0.5, y + 0.5))


def environment(o, e, k):
    """Steps 2 to 4 on the drawing o (no effect before this one, so ox = oy = 0)."""
    out = []
    for y in range(H):
        for x in range(W):
            a = o["px"][y * W + x][3]
            out.append([c * a for c in colour_at(e, k, x, y)])
    return out


# --- the cases ------------------------------------------------------------------------------

NUMBERS = ("horizontal_pan", "tilt", "roll", "field_of_view")
WORDS = ("environment", "mapping", "filter_environment")


def case(env="", mapping="spherical", filt="on", shift=(0, 0), **numbers):
    c = {"environment": env, "mapping": mapping, "filter_environment": filt, "shift": shift,
         "horizontal_pan": 0, "tilt": 0, "roll": 0, "field_of_view": 40}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS}
    k.update(mapping=c["mapping"], filter_environment=c["filter_environment"])
    return k


def env_picture(name, frame):
    """Step 1: the environment's picture, "" for none, or None for a layer not there."""
    if name == "":
        return ""
    if name not in MAPS:
        return None
    return L.EMPTY if name == "late" and frame < 4 else decoded(MAPS[name])


def worked(c, frame):
    o = decoded("shot")
    e = env_picture(c["environment"], frame)
    if e == "" or e is None:
        return o["px"]
    return environment(o, e, settings(c, frame))


def render(c, frame):
    out = L.pic(W, H, worked(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-ENV-001": ("As added: no environment named, so nothing changes.", case(), (0,)),
    "FX-ENV-002": ("The spherical map, field of view 90: the middle of the map ahead, red rising "
                   "to the right, green to the bottom; the clear corner stays clear.",
                   case("sphere", field_of_view=90), (0,)),
    "FX-ENV-003": ("FX-ENV-002 with Filter Environment off: one ray a pixel.",
                   case("sphere", filt="off", field_of_view=90), (0,)),
    "FX-ENV-004": ("Horizontal Pan 90: the view turned a quarter right, so the map three "
                   "quarters across lies ahead.", case("sphere", horizontal_pan=90,
                                                       field_of_view=90), (0,)),
    "FX-ENV-005": ("Horizontal Pan 180: looking behind, across the map's left and right edges, "
                   "which meet with no seam.", case("sphere", horizontal_pan=180,
                                                    field_of_view=90), (0,)),
    "FX-ENV-006": ("Tilt 45: looking up, the map's upper rows.",
                   case("sphere", tilt=45, field_of_view=90), (0,)),
    "FX-ENV-007": ("Tilt 80: looking almost straight up, near the map's top row, which the "
                   "rows are held to.", case("sphere", tilt=80, field_of_view=90), (0,)),
    "FX-ENV-008": ("Roll 30: the view turned about the way it looks, the map's rows tilted.",
                   case("sphere", roll=30, field_of_view=90), (0,)),
    "FX-ENV-009": ("Field of view 20, as added otherwise: a narrow view, the map enlarged.",
                   case("sphere", field_of_view=20), (0,)),
    "FX-ENV-010": ("The probe, field of view 90: the probe's middle ahead.",
                   case("probe", "probe", field_of_view=90), (0,)),
    "FX-ENV-011": ("The probe, Horizontal Pan 150: looking nearly behind, toward the rim.",
                   case("probe", "probe", horizontal_pan=150, field_of_view=90), (0,)),
    "FX-ENV-012": ("The probe with Filter Environment off: the filter is for Spherical only, so "
                   "this is FX-ENV-010.", case("probe", "probe", filt="off", field_of_view=90),
                   (0,)),
    "FX-ENV-013": ("The vertical cross, field of view 90: the green face ahead, edges of red, "
                   "blue, white and brown at the sides, top and bottom.",
                   case("cross", "vertical_cross", field_of_view=90), (0,)),
    "FX-ENV-014": ("The cross, Horizontal Pan 90: the blue face ahead.",
                   case("cross", "vertical_cross", horizontal_pan=90, field_of_view=90), (0,)),
    "FX-ENV-015": ("The cross, Tilt 90: the white face ahead.",
                   case("cross", "vertical_cross", tilt=90, field_of_view=90), (0,)),
    "FX-ENV-016": ("The cross, Horizontal Pan 180: the yellow face behind, its darker pixel "
                   "turned to the upper left, as the cross keeps it upside down.",
                   case("cross", "vertical_cross", horizontal_pan=180, field_of_view=90), (0,)),
    "FX-ENV-017": ("Horizontal Pan keyed from 0 at frame 0 to 360 at frame 4: frame 0 is "
                   "FX-ENV-002, frame 2 is FX-ENV-005, and frame 4 FX-ENV-002 again.",
                   case("sphere", horizontal_pan=keyed((0, 0), (4, 360)), field_of_view=90),
                   (0, 2, 4)),
    "FX-ENV-018": ("FX-ENV-002 on the holder moved 3 right and 1 down: the same, moved.",
                   case("sphere", shift=(3, 1), field_of_view=90), (0,)),
    "FX-ENV-019": ("An environment layer that is not in the composition, `gone`: nothing "
                   "changes, and the warning every frame.", case("gone", field_of_view=90),
                   (0, 4)),
    "FX-ENV-020": ("The environment with its in point at frame 4: before it there is no "
                   "picture, so the layer is clear; at frame 4 it is FX-ENV-002.",
                   case("late", field_of_view=90), (0, 4)),
    "FX-ENV-021": ("Field of view 150 with Roll -20 and Tilt -30: wide, looking down, the map's "
                   "lower rows.", case("sphere", roll=-20, tilt=-30, field_of_view=150), (0,)),
}

INVALID = {
    "FX-ENV-022": ("Mapping \"Spherical\": the word is exact, so a capital is not it.",
                   case("sphere", "Spherical")),
    "FX-ENV-023": ("Filter Environment written \"yes\".", case("sphere", filt="yes")),
    "FX-ENV-024": ("Horizontal Pan 3601, above 3600.", case("sphere", horizontal_pan=3601)),
    "FX-ENV-025": ("Tilt -3601, below -3600.", case("sphere", tilt=-3601)),
    "FX-ENV-026": ("Roll 3601, above 3600.", case("sphere", roll=3601)),
    "FX-ENV-027": ("Field of view 0, below 1.", case("sphere", field_of_view=0)),
    "FX-ENV-028": ("Field of view 180, above 179.", case("sphere", field_of_view=180)),
    "FX-ENV-029": ("An environment written as the number 3, not a word.", case(3)),
    "FX-ENV-030": ("Field of view keyed to 200 at frame 4.",
                   case("sphere", field_of_view=keyed((0, 40), (4, 200)))),
}


def effect(fid, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return {"instance_id": fid, "type_id": "core.cc_environment", "enabled": True,
            "parameters": params}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES)
            for n in ("sphere", "probe", "cross")] + [
        L.raster("late", "asset-sphere", enabled=False, in_frame=4, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-shot", position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, map_layers() + [holder])]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        step_clear(fx, c, frames)
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if c["environment"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_cc_environment.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def step_clear(fx, c, frames):
    """No ray lies near a Vertical Cross's tie or straight behind on a Probe."""
    for f in frames:
        k = settings(c, f)
        offsets = ([(0.5 + sx, 0.5 + sy) for sy in (-0.25, 0.25) for sx in (-0.25, 0.25)]
                   if k["filter_environment"] == "on" and k["mapping"] == "spherical"
                   else [(0.5, 0.5)])
        for y in range(H):
            for x in range(W):
                for ox, oy in offsets:
                    d = ray(k, x + ox, y + oy)
                    a = sorted(abs(q) for q in d)
                    if k["mapping"] == "vertical_cross":
                        assert a[2] - a[1] > 1e-6, (fx, f, x, y)
                    if k["mapping"] == "probe":
                        assert math.hypot(d[0], d[1]) > 1e-6 or d[2] > 0, (fx, f, x, y)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    shot = decoded("shot")["px"]
    # As added and a missing layer: nothing changes, exactly.
    assert c["FX-ENV-001"]["0"] == drawn
    assert c["FX-ENV-019"]["0"] == drawn and c["FX-ENV-019"]["4"] == drawn
    # The clear corner stays clear; elsewhere the layer is opaque.
    for fx in ("FX-ENV-002", "FX-ENV-010", "FX-ENV-013"):
        for i, p in enumerate(c[fx]["0"]):
            assert (p[3] == 0.0) == (shot[i][3] == 0.0), (fx, i)
    # Red rises to the right and green down, on the spherical map ahead.
    s = c["FX-ENV-002"]["0"]
    assert s[4 * W + 10][0] > s[4 * W + 2][0] and s[6 * W + 4][1] > s[1 * W + 4][1]
    # The filter changes the picture a little, not much.
    assert c["FX-ENV-003"]["0"] != s and near(c["FX-ENV-003"]["0"], s, 0.05)
    # Pan 90 puts the map's three quarters ahead: redder than pan 0 in the middle.
    assert c["FX-ENV-004"]["0"][4 * W + 7][0] > s[4 * W + 7][0]
    # Looking up reads the upper rows, less green; looking down, the lower, more green.
    assert c["FX-ENV-006"]["0"][4 * W + 7][1] < s[4 * W + 7][1]
    assert c["FX-ENV-021"]["0"][4 * W + 7][1] > s[4 * W + 7][1]
    # The filter is for Spherical only.
    assert c["FX-ENV-012"]["0"] == c["FX-ENV-010"]["0"]
    # The cross's faces, read where the view is held to one face pixel: ahead green, pan 90
    # blue, tilt 90 white, behind yellow; behind, the darker pixel at the upper left.
    lin = lambda rgb: [srgb_to_linear(v / 255) for v in rgb]  # noqa: E731
    for fx, rgb, (x, y) in (("FX-ENV-013", FACE[(1, 1)], (1, 0)), ("FX-ENV-014", FACE[(2, 1)], (1, 0)),
                            ("FX-ENV-015", FACE[(1, 0)], (1, 0)), ("FX-ENV-016", FACE[(1, 3)], (14, 0))):
        p = c[fx]["0"][y * W + x]
        assert near([p[:3]], [lin(rgb)], 1e-12), (fx, p)
    dark = [round(v * 0.7) for v in FACE[(1, 3)]]
    assert near([c["FX-ENV-016"]["0"][1][:3]], [lin(dark)], 1e-12)
    # Keyed: frame 0 is pan 0, frame 2 pan 180, frame 4 a whole turn.
    assert c["FX-ENV-017"]["0"] == s
    assert near(c["FX-ENV-017"]["2"], c["FX-ENV-005"]["0"], 1e-12)
    assert near(c["FX-ENV-017"]["4"], s, 1e-9)
    # Late: clear before its in point, FX-ENV-002 at it.
    assert all(p == CLEAR for p in c["FX-ENV-020"]["0"])
    assert c["FX-ENV-020"]["4"] == s
    # Moved: the same picture, three right and one down.
    moved = c["FX-ENV-018"]["0"]
    assert all(moved[(y + 1) * W + x + 3] == s[y * W + x] for y in range(H - 1) for x in range(W - 3))
    print("checks passed")


if __name__ == "__main__":
    main()
