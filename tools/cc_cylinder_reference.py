"""CC Cylinder, worked a second way.

`core.cc_cylinder`, modelled on CycoreFX's CC Cylinder (Perspective), which wraps the layer
round a lit cylinder. The CycoreFX HD manual names the controls: Radius, Position X, Y and Z
("positive Z values move the cylinder farther away"), Rotation X, Y and Z, Render (Full,
Outside, Inside), and the Light group (Light Intensity, Light Color, Light Height, which puts the
light behind when negative, and Light Direction) and the Shading group (Ambient, Diffuse,
Specular, Roughness, the opposite of shininess, and Metal, at 100 the highlight takes the
layer's colour and at 0 the light's), with an ambient light always there. It publishes no
formula and no settings when added, so the rule and those settings are ours (document 21,
D-530). It draws its own fake 3D: the composition's camera and 3D layers are not used
(P0-12, a gap).

The settings, every number keyable. `radius`, 0 to 1000 per cent of half the drawing's width,
50 when added; `position`, x then y, -1000 to 1000 per cent of the drawing's width and height,
[50, 50] when added; `position_z`, -1000 to 1000 per cent of the drawing's width, 0 when added;
`rotation_x`, `rotation_y` and `rotation_z`, -3600 to 3600 degrees, 0 when added;
`light_intensity`, 0 to 1000 per cent, 100 when added; `light_height`, -100 to 100, 50 when
added; `light_direction`, -3600 to 3600 degrees, 0 up and 90 right, -45 when added; `ambient`
40, `diffuse` 60, `specular` 50, `roughness` 10 and `metal` 100 when added, each 0 to 100.
`light_color`, `#rrggbb`, "#ffffff" when added; `render`, "full" (when added), "outside" or
"inside", exact.

The rule, in the drawing's own W by H space, y down, z away from the viewer. A fixed eye sits at
E = (W/2, H/2, -Z0), Z0 = W 50/36 (a 50 mm lens on a 36 mm frame), and looks through the pixel's
centre P = (x + 1/2, y + 1/2, 0) along d = P - E. The cylinder's centre is
C = (px W/100, py H/100, pz W/100), its radius R = radius W/200, its axis the y axis turned by
M = Rz Ry Rx (x first), with Rx: (y cos - z sin, y sin + z cos), Ry: (x cos + z sin,
-x sin + z cos), Rz: (x cos - y sin, x sin + y cos). In the cylinder's own space the eye is
o = M^T (E - C) and the ray's direction is v = M^T d. The ray meets x^2 + z^2 = R^2 at
t = (-b -+ sqrt(b^2 - a c)) / a, a = vx^2 + vz^2, b = ox vx + oz vz, c = ox^2 + oz^2 - R^2: the
nearer root is the outside, the farther the inside, each only when t > 0 and the hit h = o + t v
has |hy| <= H/2 (the ends are open). Nothing is hit when R is 0, a is 0 or b^2 - a c < 0.

The picture on the cylinder: phi = atan2(hx, -hz), the drawing at
(W/2 + phi W / (2 pi), hy + H/2) by document 21's bilinear, each tap's column taken round the
drawing's width and its row outside the drawing transparent; the middle column faces the eye.

The light, from the direction L = (cos e sin g, -cos e cos g, -sin e), g the direction and
e = height 90/100 degrees (100 is from the eye's side, -100 from behind), turned into the
cylinder's own space. The normal n = (hx/R, 0, hz/R), turned round for the inside; V the unit
vector back to the eye. With N.L = n . L, the half vector Hv = (L + V) / |L + V| and
p = 2 / (roughness/100)^2, the highlight s = max(n . Hv, 0)^p, 0 when roughness is 0, when
N.L <= 0 or when L + V is 0. With i = light_intensity/100, Lc the light's linear colour and
A, D, S, Mt the shading numbers over 100, on the sampled premultiplied (c, alpha):

    rgb = c (A + D i Lc max(N.L, 0)) + S i s (Mt c + (1 - Mt) Lc alpha),  alpha unchanged.

"outside" draws the outside hit, "inside" the inside hit, "full" the outside over the inside.
The layer never grows: the cylinder is drawn inside the layer's own picture. A draft changes
nothing, every distance being a share of the drawing. One ray a pixel, so the cylinder's edges
are not smoothed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. The rule steps where a ray grazes the cylinder, at its open
ends and where the highlight is cut at N.L = 0; every case is checked that no pixel centre lies
near a step.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with a
transparent column and a half-covered one. The projects go into `Fixtures/cc_cylinder`, the
expected frames into `Fixtures/cc_cylinder/expected_cc_cylinder.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_cylinder_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_cylinder"
NUMBERS = ("radius", "position", "position_z", "rotation_x", "rotation_y", "rotation_z",
           "light_intensity", "light_height", "light_direction", "ambient", "diffuse",
           "specular", "roughness", "metal")
WORDS = ("render", "light_color")


# --- the rule -------------------------------------------------------------------------------

def turned(k):
    """M = Rz Ry Rx as rows."""
    cx, sx = math.cos(math.radians(k["rotation_x"])), math.sin(math.radians(k["rotation_x"]))
    cy, sy = math.cos(math.radians(k["rotation_y"])), math.sin(math.radians(k["rotation_y"]))
    cz, sz = math.cos(math.radians(k["rotation_z"])), math.sin(math.radians(k["rotation_z"]))
    rx = [[1, 0, 0], [0, cx, -sx], [0, sx, cx]]
    ry = [[cy, 0, sy], [0, 1, 0], [-sy, 0, cy]]
    rz = [[cz, -sz, 0], [sz, cz, 0], [0, 0, 1]]
    mul = lambda a, b: [[sum(a[i][j] * b[j][n] for j in range(3)) for n in range(3)] for i in range(3)]  # noqa: E731
    return mul(rz, mul(ry, rx))


def own(m, v):
    """M^T v: a world vector in the cylinder's own space."""
    return [sum(m[j][i] * v[j] for j in range(3)) for i in range(3)]


def scene(k):
    z0 = W * 50 / 36
    eye = (W / 2, H / 2, -z0)
    c = (k["position"][0] * W / 100, k["position"][1] * H / 100, k["position_z"] * W / 100)
    m = turned(k)
    e, g = math.radians(k["light_height"] * 0.9), math.radians(k["light_direction"])
    light = [math.cos(e) * math.sin(g), -math.cos(e) * math.cos(g), -math.sin(e)]
    return {"m": m, "eye": eye, "o": own(m, [eye[i] - c[i] for i in range(3)]),
            "r": k["radius"] * W / 200, "light": own(m, light)}


def hits(k, sc, x, y):
    """The ray through pixel (x, y): its direction in own space and its [(t, inside)] hits, with
    the numbers the step checks need."""
    eye, o, r = sc["eye"], sc["o"], sc["r"]
    v = own(sc["m"], [x + 0.5 - eye[0], y + 0.5 - eye[1], -eye[2]])
    a = v[0] ** 2 + v[2] ** 2
    if r == 0 or a == 0:
        return v, [], None
    b = o[0] * v[0] + o[2] * v[2]
    c = o[0] ** 2 + o[2] ** 2 - r * r
    disc = b * b - a * c
    if disc < 0:
        return v, [], (disc, b * b + abs(a * c))
    sq = math.sqrt(disc)
    out = []
    for t, inside in (((-b - sq) / a, False), ((-b + sq) / a, True)):
        if t > 0 and abs(o[1] + t * v[1]) <= H / 2:
            out.append((t, inside))
    return v, out, (disc, b * b + abs(a * c))


def wrapped(px, x, y):
    """Document 21's bilinear sample, each tap's column taken round the drawing's width."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx, sy = (x0 + dx) % W, y0 + dy
            if 0 <= sy < H and wx * wy:
                p = px[sy * W + sx]
                for i in range(4):
                    out[i] += p[i] * wx * wy
    return out


def colour(hexa):
    return [S.srgb_to_linear(int(hexa[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def shade(px, k, sc, v, t, inside):
    """The lit colour of one hit, and its N.L and highlight for the step checks."""
    o, r, lo = sc["o"], sc["r"], sc["light"]
    h = [o[i] + t * v[i] for i in range(3)]
    phi = math.atan2(h[0], -h[2])
    c = wrapped(px, W / 2 + phi / (2 * math.pi) * W, h[1] + H / 2)
    n = [h[0] / r, 0.0, h[2] / r]
    if inside:
        n = [-q for q in n]
    lv = math.sqrt(sum(q * q for q in v))
    view = [-q / lv for q in v]
    nl = sum(n[i] * lo[i] for i in range(3))
    s = 0.0
    hv = [lo[i] + view[i] for i in range(3)]
    hl = math.sqrt(sum(q * q for q in hv))
    if k["roughness"] > 0 and nl > 0 and hl > 0:
        p = 2 / (k["roughness"] / 100) ** 2
        s = max(sum(n[i] * hv[i] for i in range(3)) / hl, 0.0) ** p
    i = k["light_intensity"] / 100
    lc = colour(k["light_color"])
    am, di, sp, mt = (k[n_] / 100 for n_ in ("ambient", "diffuse", "specular", "metal"))
    rgb = [c[j] * (am + di * i * lc[j] * max(nl, 0.0))
           + sp * i * s * (mt * c[j] + (1 - mt) * lc[j] * c[3]) for j in range(3)]
    return rgb + [c[3]], nl, sp * i * s


def over(top, under):
    return [top[j] + (1 - top[3]) * under[j] for j in range(4)]


def cylinder(px, k):
    sc = scene(k)
    out = []
    for y in range(H):
        for x in range(W):
            v, hs, _ = hits(k, sc, x, y)
            got = {inside: shade(px, k, sc, v, t, inside)[0] for t, inside in hs}
            none = [0.0] * 4
            if k["render"] == "outside":
                out.append(got.get(False, none))
            elif k["render"] == "inside":
                out.append(got.get(True, none))
            else:
                out.append(over(got.get(False, none), got.get(True, none)))
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(render="full", light_color="#ffffff", shift=0, **numbers):
    c = {"render": render, "light_color": light_color, "shift": shift, "radius": 50,
         "position": [50, 50], "position_z": 0, "rotation_x": 0, "rotation_y": 0,
         "rotation_z": 0, "light_intensity": 100, "light_height": 50, "light_direction": -45,
         "ambient": 40, "diffuse": 60, "specular": 50, "roughness": 10, "metal": 100}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    k = {n: value_at(c[n], frame_no) for n in NUMBERS}
    k.update(render=c["render"], light_color=c["light_color"])
    return k


def render(c, frame_no):
    return R.frame(cylinder(working(), settings(c, frame_no)), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.cc_cylinder", params, c["shift"])


CASES = {
    "FX-CYL-001": ("The settings as they start: the drawing wrapped round an upright cylinder "
                   "half the drawing's half width in radius, in the middle, its middle column "
                   "facing the viewer, lit from the upper left; its sides fall away round the "
                   "curve, the corners beside it are transparent, and the inside's far wall is "
                   "hidden behind the front.", case(), [0]),
    "FX-CYL-002": ("Render outside: the outside alone.", case("outside"), [0]),
    "FX-CYL-003": ("Render inside: the inside alone, the far wall, the drawing seen from behind "
                   "and so mirrored, its middle at the drawing's two side edges.",
                   case("inside"), [0]),
    "FX-CYL-004": ("Rotation Y 90: the cylinder turned a quarter about its upright axis, so the "
                   "drawing three quarters of the way across faces the viewer.",
                   case(rotation_y=90), [0]),
    "FX-CYL-005": ("Rotation X 30: the cylinder tipped, its top towards the viewer, so the open "
                   "end shows the inside.", case(rotation_x=30), [0]),
    "FX-CYL-006": ("Rotation Z 90: the cylinder lying on its side, the drawing's columns "
                   "wrapped round it from top to bottom and its rows running along it.", case(rotation_z=90), [0]),
    "FX-CYL-007": ("Radius 100: a cylinder as wide as the drawing, nearer the viewer and so "
                   "running past the drawing's sides.", case(radius=100), [0]),
    "FX-CYL-008": ("Radius 0: no cylinder, so nothing is drawn and the layer is transparent.",
                   case(radius=0), [0]),
    "FX-CYL-009": ("Position Z 100: the cylinder a drawing's width farther away, smaller.",
                   case(position_z=100), [0]),
    "FX-CYL-010": ("Position [25, 50]: the cylinder a quarter of the way across, seen a little "
                   "from the side.", case(position=[25, 50]), [0]),
    "FX-CYL-011": ("Light intensity 0: the ambient light alone, every pixel the wrapped "
                   "drawing at 40 per cent.", case(light_intensity=0), [0]),
    "FX-CYL-012": ("Ambient 100, diffuse 0, specular 0: no shading, the wrapped drawing as it "
                   "is.", case(ambient=100, diffuse=0, specular=0), [0]),
    "FX-CYL-013": ("An orange light, metal 0: the lit side and the highlight orange; with no "
                   "blue in the light, blue is the ambient's alone.",
                   case(light_color="#ff8000", metal=0), [0]),
    "FX-CYL-014": ("Roughness 50, specular 100: a larger, duller highlight.",
                   case(roughness=50, specular=100), [0]),
    "FX-CYL-015": ("Light height -50: the light from behind, so the side facing the viewer is "
                   "darker.", case(light_height=-50), [0]),
    "FX-CYL-016": ("Light direction 90: the light from the right.",
                   case(light_direction=90), [0]),
    "FX-CYL-017": ("Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole "
                   "turn, half way at frame 2.", case(rotation_y=keyed((0, 0), (4, 360))),
                   [0, 2, 4]),
    "FX-CYL-018": ("FX-CYL-001 moved three pixels right: the same, moved, cylinder and all.",
                   case(shift=3), [0, 3]),
}

INVALID = {
    "FX-CYL-019": ("Radius -1, below 0.", case(radius=-1)),
    "FX-CYL-020": ("Radius 1001, above 1000.", case(radius=1001)),
    "FX-CYL-021": ("Position Z 1001, above 1000.", case(position_z=1001)),
    "FX-CYL-022": ("Rotation Y 3601, above 3600.", case(rotation_y=3601)),
    "FX-CYL-023": ("Render \"Full\": the word is exact, so a capital is not it.", case("Full")),
    "FX-CYL-024": ("Light color \"white\": not a #rrggbb colour.", case(light_color="white")),
    "FX-CYL-025": ("Ambient 101, above 100.", case(ambient=101)),
    "FX-CYL-026": ("Roughness -1, below 0.", case(roughness=-1)),
    "FX-CYL-027": ("Light height 101, above 100.", case(light_height=101)),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "cc_cylinder", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """No pixel centre may lie within a hair of a graze, an open end or the highlight's cut."""
    px = working()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            sc = scene(k)
            for y in range(H):
                for x in range(W):
                    v, hs, d = hits(k, sc, x, y)
                    if d is not None:
                        assert abs(d[0]) > 1e-4 * d[1], (fx, f, x, y, "graze")
                    if d is None or d[0] < 0:
                        continue
                    sq = math.sqrt(d[0])
                    a = v[0] ** 2 + v[2] ** 2
                    b = sc["o"][0] * v[0] + sc["o"][2] * v[2]
                    for t in ((-b - sq) / a, (-b + sq) / a):
                        if t > 0:
                            assert abs(abs(sc["o"][1] + t * v[1]) - H / 2) > 1e-4, (fx, f, x, y, "end")
                    for t, inside in hs:
                        _, nl, s = shade(px, k, sc, v, t, inside)
                        assert abs(nl) > 1e-4 or s < 1e-7, (fx, f, x, y, "cut")


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    covered = lambda fr: [n for n, p in enumerate(fr) if p[3] > 0]  # noqa: E731
    one, two, three = c["FX-CYL-001"]["0"], c["FX-CYL-002"]["0"], c["FX-CYL-003"]["0"]
    assert all(near(p, over(q, r)) for p, q, r in zip(one, two, three))
    assert one[0][3] == 0 and one[W - 1][3] == 0 and one[(H // 2) * W + W // 2][3] == 1
    assert any(p[3] > 0 for p in three)
    assert all(near(p, q, 1e-9) for p, q in zip(one, two) if q[3] > 1 - 1e-9)
    assert one != c["FX-CYL-004"]["0"] and one != c["FX-CYL-005"]["0"]
    assert one != c["FX-CYL-006"]["0"] and one != c["FX-CYL-016"]["0"]
    assert len(covered(c["FX-CYL-007"]["0"])) > len(covered(one))
    assert all(p == [0.0] * 4 for p in c["FX-CYL-008"]["0"])
    assert len(covered(c["FX-CYL-009"]["0"])) < len(covered(one))
    mid = lambda fr: sum(n % W for n in covered(fr)) / len(covered(fr))  # noqa: E731
    assert mid(c["FX-CYL-010"]["0"]) < mid(one) - 2
    eleven, twelve = c["FX-CYL-011"]["0"], c["FX-CYL-012"]["0"]
    assert all(near(p, [0.4 * v for v in q[:3]] + [q[3]]) for p, q in zip(eleven, twelve))
    assert all(near(p[2:], q[2:]) for p, q in zip(c["FX-CYL-013"]["0"], eleven))
    assert c["FX-CYL-013"]["0"] != one and c["FX-CYL-014"]["0"] != one
    lit = lambda fr: sum(sum(p[:3]) for p in fr)  # noqa: E731
    assert lit(c["FX-CYL-015"]["0"]) < lit(one)
    seventeen = c["FX-CYL-017"]
    assert seventeen["0"] == one
    assert seventeen["2"] == render(case(rotation_y=180), 0)
    assert all(near(p, q, 1e-9) for p, q in zip(seventeen["4"], one))
    moved = c["FX-CYL-018"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == one[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
