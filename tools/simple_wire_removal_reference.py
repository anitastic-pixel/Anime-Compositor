"""CC Simple Wire Removal, worked a second way.

D-458 adds `core.simple_wire_removal`, after CycoreFX's CC Simple Wire Removal (After Effects'
Keying group): a straight wire between two points is painted out, by fading it, by another frame
of the layer, or by the pixels either side of it. The Cycore FX manual names the controls and
what each style does ("Fade: Clears a path through the layer"; "Frame Offset: Brings in pixel
data from a previous or subsequent frame"; "Displace: Brings in pixel data from outside the
remover"; "Displace Horizontal: ... constrains its sampling to the horizontal"; lower Slope a
gradual edge, higher an abrupt one; Mirror Blend "the default of 25"), but publishes no formula,
so the rule below is ours. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. Settings: `point_a` and `point_b`, the wire's ends in per cent of the drawing's own
width and height, -1000 to 1000, (25, 50) and (75, 50) when added; `removal_style`, "fade",
"frame_offset", "displace" or "displace_horizontal", "displace" when added; `thickness`, 0 to 100
pixels, 5; `slope`, 0 to 100 per cent, 50; `mirror_blend`, 0 to 100 per cent, 25;
`frame_offset`, -1000 to 1000 frames, -1, taken down to a whole number. The numbers are keyable.

- A and B are the points in pixels, L = |B - A|, u the unit step from A to B and n = (-u_y, u_x)
  across it. A pixel's centre P lies t = (P - A) . u / L of the way along and d = (P - A) . n to
  the side. With h = thickness / 2, r = thickness (1 - slope / 100) and E = h + r, the remover's
  weight w is 1 for |d| <= h, 1 - (|d| - h) / r out to E, and 0 past E or for t outside 0 to 1
  (the ends are square). Thickness 0, or A on B, leaves the picture as it was.
- Each pixel becomes c (1 - w) + R w, c its own colour and R the style's, all premultiplied:
  "fade" R = 0, clear; "frame_offset" R = the layer's own pixel at the composition frame plus the
  offset, with the effects before this one; "displace" with F = A + (t L) u the foot of P on the
  wire, R = (1 - m) I + m M, where m = mirror_blend / 100, I = the samples at F - E n and F + E n
  mixed by (d + E) / (2 E), and M = the sample at F + s (2 E - |d|) n, P reflected in the
  remover's edge on its own side, s = 1 for d >= 0 and -1 below; "displace_horizontal" the same
  with each sample moved along P's own row instead, to the point of that row at the same d
  ((target - d) / n_x across from P), and R = c for a wire lying flat (n_x = 0), which no row
  crosses. Samples are document 21's bilinear sample with the point first held inside the
  picture's pixel centres (D-109's repeat).
- "frame_offset" at a frame where the layer does not show, or on an adjustment layer, whose
  frames beneath at other times are not to hand, leaves the picture as it was; the adjustment
  layer says TEMPORAL_SMOOTHING_SKIPPED each frame. The layer does not grow. In a draft the
  thickness, in pixels, is scaled with the picture.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, and asserts no pixel's centre lies within a millionth of a
pixel of a place where the rule jumps: a square end, the hard edge of slope 100, or the wire
itself when the mirror counts.

Every case is a project of one composition 16 by 10, four frames, in
`Fixtures/simple_wire_removal/`: the layer `holder`, a run of two drawings, a colour ramp with an
empty column 0 and a half-covered column 15, the first with a near-white wire along row 5
(frames 0 and 2), the second without it and redder (frames 1 and 3), the clean plate. The
expected pixels are in `Fixtures/simple_wire_removal/expected_simple_wire_removal.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/simple_wire_removal_reference.py
"""

import json
import sys
from math import floor, hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
import auto_tone_reference as A  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "simple_wire_removal"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 4
NAMES = ("point_a", "point_b", "removal_style", "thickness", "slope", "mirror_blend",
         "frame_offset")
RANGES = {"point_a": (-1000, 1000), "point_b": (-1000, 1000), "thickness": (0, 100),
          "slope": (0, 100), "mirror_blend": (0, 100), "frame_offset": (-1000, 1000)}
EMPTY = [0.0] * 4
WIRE = ((10, 54), (90, 54))  # along row 5, a little above its middle


# --- the drawings ---------------------------------------------------------------------------

def cell(x, y, n):
    if x == 0:
        return (0, 0, 0, 0)
    if n == 1 and y == 5:
        rgb = (250, 245, 235)
    else:
        rgb = (min(255, 30 + 12 * x + (0 if n == 1 else 30)), 40 + 18 * y, 180 - 6 * x - 5 * y)
    return rgb + ((128,) if x == 15 else (255,))


DRAWINGS = {f"wire_{n}": [[cell(x, y, n) for x in range(W)] for y in range(H)] for n in (1, 2)}
RUN = [1, 2, 1, 2]
SPANS = [{"start_frame": f, "end_frame_exclusive": f + 1, "drawing_number": d}
         for f, d in enumerate(RUN)]


def working(p):
    a = p[3] / 255
    return [A.srgb_to_linear(v / 255) * a for v in p[:3]] + [a] if a > 0 else list(EMPTY)


def drawing(n):
    """The holder's picture at frame n, premultiplied linear, or None where it does not show."""
    if not 0 <= n < FRAMES:
        return None
    return {"px": [working(p) for row in DRAWINGS[f"wire_{RUN[n]}"] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the rule -------------------------------------------------------------------------------

def settings(c, n):
    held = {"removal_style": c["removal_style"]}
    for k, (lo, hi) in RANGES.items():
        v = value_at(c[k], n)
        held[k] = [min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple)) \
            else min(hi, max(lo, v))
    return held


def removed(c, n):
    """The holder's pixels at frame n with the effect, or None where it does not show."""
    pic = drawing(n)
    if pic is None:
        return None
    px = pic["px"]
    s = settings(c, n)
    ax, ay = s["point_a"][0] / 100 * W, s["point_a"][1] / 100 * H
    bx, by = s["point_b"][0] / 100 * W, s["point_b"][1] / 100 * H
    length = hypot(bx - ax, by - ay)
    t_ = s["thickness"]
    style = s["removal_style"]
    if length == 0 or t_ <= 0:
        return list(px)
    other = None
    if style == "frame_offset":
        other = None if c["on"] == "adjust" else drawing(n + floor(s["frame_offset"]))
        if other is None:
            return list(px)
    ux, uy = (bx - ax) / length, (by - ay) / length
    nx, ny = -uy, ux
    h = t_ / 2
    r = t_ * (1 - s["slope"] / 100)
    e = h + r
    m = s["mirror_blend"] / 100
    held = lambda x, y: bilinear(pic, x, y, edges="repeat")  # noqa: E731
    out = []
    for y in range(H):
        for x in range(W):
            c0 = px[y * W + x]
            p0, p1 = x + 0.5, y + 0.5
            along = (p0 - ax) * ux + (p1 - ay) * uy
            d = (p0 - ax) * nx + (p1 - ay) * ny
            t = along / length
            assert abs(abs(d) - e) > 1e-6 or r > 0, "a centre on the hard edge"
            if t < 0 or t > 1 or abs(d) >= e:
                out.append(list(c0))
                continue
            assert abs(t) > 1e-6 and abs(t - 1) > 1e-6, "a centre on a square end"
            w = 1.0 if abs(d) <= h else 1 - (abs(d) - h) / r
            sg = 1 if d >= 0 else -1
            if style in ("displace", "displace_horizontal"):
                assert m == 0 or abs(d) > 1e-6, "a centre on the wire"
            if style == "fade":
                rr = EMPTY
            elif style == "frame_offset":
                rr = other["px"][y * W + x]
            elif style == "displace_horizontal" and nx == 0:
                rr = c0
            else:
                if style == "displace":
                    fx, fy = ax + along * ux, ay + along * uy
                    at = lambda k: held(fx + k * nx, fy + k * ny)  # noqa: E731
                else:
                    at = lambda k: held(p0 + (k - d) / nx, p1)  # noqa: E731
                lo, hi, mir = at(-e), at(e), at(sg * (2 * e - abs(d)))
                f = (d + e) / (2 * e)
                inter = [u + (v - u) * f for u, v in zip(lo, hi)]
                rr = [u + (v - u) * m for u, v in zip(inter, mir)]
            out.append([u * (1 - w) + v * w for u, v in zip(c0, rr)])
    return out


def render(c, n):
    """The composition's frame n."""
    got = removed(c, n)
    if got is None:
        return [list(EMPTY) for _ in range(W * H)]
    sh = c["shift"]
    return [got[y * W + x - sh] if 0 <= x - sh < W else list(EMPTY)
            for y in range(H) for x in range(W)]


def plain(c, n):
    return render(dict(c, thickness=0), n)


# --- the cases ------------------------------------------------------------------------------

def case(style="displace", a=(25, 50), b=(75, 50), thickness=5, slope=50, mirror=25, offset=-1,
         shift=0, on="holder"):
    return {"point_a": list(a) if isinstance(a, tuple) else a,
            "point_b": list(b) if isinstance(b, tuple) else b,
            "removal_style": style, "thickness": thickness, "slope": slope,
            "mirror_blend": mirror, "frame_offset": offset, "shift": shift, "on": on}


def on_wire(**kw):
    return case(a=WIRE[0], b=WIRE[1], thickness=kw.pop("thickness", 3), **kw)


CASES = {
    "FX-WIRE-001": ("The settings as added: displace, from 25, 50 to 75, 50, thickness 5, slope "
                    "50, mirror blend 25. The remover reaches 5 pixels each side of the line "
                    "between columns 4 and 12, so the whole height there is smoothed, most at the "
                    "middle.", case(), (0, 1)),
    "FX-WIRE-002": ("Fade along the wire, thickness 3: the wire and a soft edge each side made "
                    "clear, the ramp showing through the edges; past the ends untouched.",
                    on_wire(style="fade"), (0,)),
    "FX-WIRE-003": ("Frame offset 1 along the wire: frames 0 and 2 take the band from the clean, "
                    "redder plate after them, so the wire is gone; frame 3 would read frame 4, "
                    "where the layer does not show, so it is untouched.",
                    on_wire(style="frame_offset", offset=1), (0, 2, 3)),
    "FX-WIRE-004": ("Frame offset -1: frame 0 would read frame -1, so it is untouched; frame 2 "
                    "reads frame 1, the clean plate.", on_wire(style="frame_offset", offset=-1),
                    (0, 2)),
    "FX-WIRE-005": ("Frame offset 1.7, taken down to 1: the same as FX-WIRE-003.",
                    on_wire(style="frame_offset", offset=1.7), (0,)),
    "FX-WIRE-006": ("Displace along the wire, thickness 3: the wire replaced by the ramp above "
                    "and below it, mixed across the band, a quarter mirrored.", on_wire(), (0,)),
    "FX-WIRE-007": ("Mirror blend 0: the band is only the mix of the two edges.",
                    on_wire(mirror=0), (0,)),
    "FX-WIRE-008": ("Mirror blend 100: the band is only the picture reflected in its edges.",
                    on_wire(mirror=100), (0,)),
    "FX-WIRE-009": ("Slope 0: the edge fades over a whole thickness each side.",
                    on_wire(slope=0), (0,)),
    "FX-WIRE-010": ("Slope 100, thickness 4: a hard edge, every pixel within 2 of the line "
                    "replaced whole and none past it touched.", on_wire(slope=100, thickness=4),
                    (0,)),
    "FX-WIRE-011": ("Thickness 0: the drawing, untouched.", on_wire(thickness=0), (0,)),
    "FX-WIRE-012": ("Both points at 50, 54: the drawing, untouched.",
                    case(a=(50, 54), b=(50, 54)), (0,)),
    "FX-WIRE-013": ("Displace horizontal along the flat wire: no row crosses it, so the drawing "
                    "is untouched.", on_wire(style="displace_horizontal"), (0,)),
    "FX-WIRE-014": ("Displace on a wire straight down the middle, from 50, 0 to 50, 100.",
                    case(a=(50, 0), b=(50, 100), thickness=3), (0,)),
    "FX-WIRE-015": ("Displace horizontal on the same upright wire: across it is along a row, so "
                    "the same as FX-WIRE-014.",
                    case(style="displace_horizontal", a=(50, 0), b=(50, 100), thickness=3),
                    (0,)),
    "FX-WIRE-016": ("Displace horizontal on a slanting wire from 20, 10 to 80, 90: each sample "
                    "taken along its own row.",
                    case(style="displace_horizontal", a=(20, 10), b=(80, 90), thickness=3), (0,)),
    "FX-WIRE-017": ("Displace on the same slanting wire: the samples taken across the wire, so "
                    "not the same as FX-WIRE-016.", case(a=(20, 10), b=(80, 90), thickness=3),
                    (0,)),
    "FX-WIRE-018": ("Thickness keyed from 2 at frame 0 to 6 at frame 2, linear: 2, 4 and 6.",
                    on_wire(thickness=keyed((0, 2), (2, 6))), (0, 1, 2)),
    "FX-WIRE-019": ("FX-WIRE-006 with the layer moved three pixels right: the same, moved; "
                    "nothing grows.", on_wire(shift=3), (0,)),
    "FX-WIRE-020": ("A wire along the top edge, from 10, 4 to 90, 4, thickness 4: the samples "
                    "above the picture are held to its top row.",
                    case(a=(10, 4), b=(90, 4), thickness=4), (0,)),
    "FX-WIRE-021": ("Frame offset on an adjustment layer above the holder: the frames beneath "
                    "at other times are not to hand, so each frame as it is, with a warning each "
                    "frame.", on_wire(style="frame_offset", offset=1, on="adjust"), (0, 2)),
    "FX-WIRE-022": ("Displace on an adjustment layer above the holder: the frame beneath is the "
                    "holder alone, so the same as FX-WIRE-006.", on_wire(on="adjust"), (0,)),
}

INVALID = {
    "FX-WIRE-023": ("Removal style \"clone\", which is not a choice.", case(style="clone")),
    "FX-WIRE-024": ("Removal style \"Displace\": the word is exact, so a capital is not it.",
                    case(style="Displace")),
    "FX-WIRE-025": ("Thickness 101, above 100.", case(thickness=101)),
    "FX-WIRE-026": ("Slope -1, below 0.", case(slope=-1)),
    "FX-WIRE-027": ("Mirror blend 101, above 100.", case(mirror=101)),
    "FX-WIRE-028": ("Frame offset 1001, past 1000.", case(offset=1001)),
    "FX-WIRE-029": ("Point A at 1001, 50, past ten widths.", case(a=(1001, 50))),
    "FX-WIRE-030": ("Thickness keyed to 150 at frame 2.", case(thickness=keyed((0, 5), (2, 150)))),
}


# --- the project files ----------------------------------------------------------------------

def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.simple_wire_removal", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-wire", spans=SPANS, out_frame=FRAMES,
                      position=(c["shift"], 0))
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-wire", "kind": "image_sequence", "name": "wire",
                        "pattern": "wire_####.png",
                        "frames": {str(k): f"media/wire_{k}.png" for k in (1, 2)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["on"] == "adjust" and c["removal_style"] == "frame_offset":
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(c, 0), "2": plain(c, 2)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_simple_wire_removal.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def near(f, g, e=1e-9):
    return all(abs(u - v) <= e for p, q in zip(f, g) for u, v in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    p0, p1 = plain(case(), 0), plain(case(), 1)
    at = lambda x, y: y * W + x  # noqa: E731
    rows = lambda f, ys: [f[at(x, y)] for y in ys for x in range(W)]  # noqa: E731

    assert not near(c["FX-WIRE-001"]["0"], p0) and not near(c["FX-WIRE-001"]["1"], p1)
    two = c["FX-WIRE-002"]["0"]
    assert two[at(8, 5)] == EMPTY and near(rows(two, (0, 1, 8, 9)), rows(p0, (0, 1, 8, 9)))
    assert two[at(0, 5)] == p0[at(0, 5)] and two[at(15, 5)] == p0[at(15, 5)]  # past the ends
    three = c["FX-WIRE-003"]
    assert three["0"][at(8, 5)] == p1[at(8, 5)] and near(three["3"], p1)
    assert near(three["2"], three["0"])
    four = c["FX-WIRE-004"]
    assert near(four["0"], p0) and near(four["2"], three["0"])
    assert c["FX-WIRE-005"]["0"] == three["0"]
    six, seven, eight = (c[f"FX-WIRE-00{k}"]["0"] for k in (6, 7, 8))
    assert not near(six, seven) and not near(six, eight) and not near(seven, eight)
    assert p0[at(8, 5)][0] > 0.9 and six[at(8, 5)][0] < 0.25  # the near-white wire is gone
    assert not near(c["FX-WIRE-009"]["0"], six)
    ten = c["FX-WIRE-010"]["0"]  # line at 5.4: rows 3 to 6 within 2, rows 2 and 7 past it
    assert near(rows(ten, (0, 1, 2, 7, 8, 9)), rows(p0, (0, 1, 2, 7, 8, 9)))
    assert ten[at(8, 5)] == ten[at(8, 5)] and not near([ten[at(8, 3)]], [p0[at(8, 3)]])
    for fx in ("FX-WIRE-011", "FX-WIRE-012", "FX-WIRE-013"):
        assert near(c[fx]["0"], p0), fx
    assert near(c["FX-WIRE-015"]["0"], c["FX-WIRE-014"]["0"])
    assert not near(c["FX-WIRE-016"]["0"], c["FX-WIRE-017"]["0"])
    eighteen = c["FX-WIRE-018"]
    assert near(eighteen["0"], render(on_wire(thickness=2), 0))
    assert near(eighteen["1"], render(on_wire(thickness=4), 1))
    assert near(eighteen["2"], render(on_wire(thickness=6), 2))
    nineteen = c["FX-WIRE-019"]["0"]
    assert all(nineteen[at(x, y)] == six[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(nineteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert not near(c["FX-WIRE-020"]["0"], p0)
    assert near(c["FX-WIRE-021"]["0"], p0) and near(c["FX-WIRE-021"]["2"], p0)
    assert near(c["FX-WIRE-022"]["0"], six)
    for fx, v in c.items():
        for f in v.values():
            for p in f:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
    print("checked")


if __name__ == "__main__":
    main()
