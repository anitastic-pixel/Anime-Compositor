"""Rolling Shutter Repair, worked a second way.

D-654 proposes Rolling Shutter Repair (`core.rolling_shutter_repair`), after After Effects' Rolling
Shutter Repair (Distort): the slant and wobble a rolling-shutter camera puts in moving footage,
where each row (or column) of a frame is taken a little later than the one before, taken out by
moving each row back to the frame's middle moment along the motion this program's own estimate
finds (`tools/flow_reference.py`). Adobe's page names the controls and says what each is for; it
publishes no formula, and the timing of the rows, the two methods' rule and the settings when added
are ours (P0-13, D-600: "Blur + retiming only", Rolling Shutter Repair "then try").

At the holder's composition frame n, with n0 its in point, m1 its last frame (out point - 1) and
P(m) the holder at composition frame m with the effects before Rolling Shutter Repair, by the
layer's own timing, W by H, linear and premultiplied, O = P(n):

1. On an adjustment layer the output is O, and the frame says TEMPORAL_SMOOTHING_SKIPPED: the
   frames beneath at other times are not to hand.
2. r = Rolling Shutter Rate / 100, the part of a frame's time the scan takes. With r = 0 the output
   is O. Each pixel (x, y) is scanned at u: (y + 0.5) / H with `top_to_bottom`, 1 - (y + 0.5) / H
   with `bottom_to_top`, (x + 0.5) / W with `left_to_right`, 1 - (x + 0.5) / W with
   `right_to_left`; it was taken tau = r (u - 0.5) frames after the frame's middle moment.
3. The motion either side, d+ toward frame n + 1 and d- toward frame n - 1, in pixels at each
   pixel:
   - `pixel_motion`: d+ = the estimate from O to P(n + 1), d- = the estimate from O to P(n - 1),
     both at Vector Detail = Pixel Motion Detail, the estimate's other settings as added.
   - `warp` (as added): the camera's whole-frame motion. M+ = the map from O to P(n + 1) fitted
     as Warp Stabilizer's `position_scale_rotation` pair model with Preserve Scale off
     (`tools/warp_stabilizer_reference.py`, its step 3: Vector Detail 50, or 100 with Detailed
     Analysis `on`), and M- likewise to P(n - 1); d+ at pixel p = M+(p) - p and d- likewise, with
     points as Warp Stabilizer's (centred, divided by D = hypot(W, H) / 2), then times D.
   A side is still when its frame is outside n0 to m1 or is the same picture as O (a drawing
   held); a still side takes the other side's motion turned round (d- = -d+, or d+ = -d-); with
   both sides still the output is O.
4. The output at (x, y) is O read bilinearly by document 21 (transparent outside) at
   (x + 0.5, y + 0.5) + tau d+ where tau >= 0, and + (-tau) d- where tau < 0: what that row would
   have shown at the frame's middle moment.

Settings: `rolling_shutter_rate` 0 to 100 per cent, 50 when added; `scan_direction`
`top_to_bottom`, `bottom_to_top`, `left_to_right` or `right_to_left`, `top_to_bottom`; `method`
`warp` or `pixel_motion`, `warp`; `detailed_analysis` `off` or `on`, `off`; `pixel_motion_detail`
0 to 100, 20. The numbers are keyable and read at frame n, the words not.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 32 by 32, six frames, in
`Fixtures/rolling_shutter_repair/`: the drawing `holder`, a textured card panning 3 right and 0.5
down a frame filmed by a camera scanning top to bottom in 80 per cent of a frame (`skew`), the
same card panning 3 down and 0.5 right scanned left to right (`side`), or Pixel Motion Blur's soft
disc, with the effect. The expected pixels are in
`Fixtures/rolling_shutter_repair/expected_rolling_shutter_repair.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/rolling_shutter_repair_reference.py
"""

import json
import sys
from math import hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import flow_reference as flow  # noqa: E402
import pixel_motion_blur_reference as P  # noqa: E402
import warp_stabilizer_reference as WS  # noqa: E402
from pixel_motion_blur_reference import W, H, FRAMES, ONES, TWOS  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "rolling_shutter_repair"
TOLERANCE = 2e-5  # document 25's default for a filter
D = hypot(W, H) / 2
CAMERA = 0.8  # the drawings' own scan: 80 per cent of a frame


def filmed(k, sideways):
    """Drawing k + 1: the card at time k + tau, tau the pixel's scan time, as the camera saw it."""
    rows = []
    for y in range(H):
        row = []
        for x in range(W):
            u = (x + 0.5) / W if sideways else (y + 0.5) / H
            t = k + CAMERA * (u - 0.5)
            ox, oy = (0.5 * t, 3.0 * t) if sideways else (3.0 * t, 0.5 * t)
            row.append(WS.texture(x + 0.5 - ox, y + 0.5 - oy))
        rows.append(row)
    return rows


DRAWINGS = {**{f"skew_{k + 1}": filmed(k, False) for k in range(FRAMES)},
            **{f"side_{k + 1}": filmed(k, True) for k in range(FRAMES)},
            **{f"disc_{k + 1}": P.disc(k) for k in range(FRAMES)}}


def drawn(c, m):
    """The holder at composition frame m with the effects before Rolling Shutter Repair."""
    local = m - c["in"]
    number = next(s["drawing_number"] for s in c["spans"]
                  if s["start_frame"] <= local < s["end_frame_exclusive"])
    name = f"{c['asset']}_{number}"
    L.DRAWINGS[name] = DRAWINGS[name]
    p = L.decoded(name)
    return L.exposure(p, c["before"]) if c["before"] is not None else p


# --- the rule -------------------------------------------------------------------------------

def fitted_map(a, b, vd):
    """Warp Stabilizer's step 3 pair model, position_scale_rotation, from a to b."""
    u = flow.estimate(a, b, {"vector_detail": vd})
    points, base = [], []
    for y in range(H):
        for x in range(W):
            i = y * W + x
            px, py = (x + 0.5 - W / 2) / D, (y + 0.5 - H / 2) / D
            points.append((px, py, px + u[0][i] / D, py + u[1][i] / D))
            base.append(a["px"][i][3])
    m, weights = None, base
    for _ in range(3):
        m = WS.fitted("position_scale_rotation", False, points, weights)
        if m is None:
            break
        weights = []
        for w, (x, y, X, Y) in zip(base, points):
            fx, fy = WS.apply(m, x, y)
            r = hypot(fx - X, fy - Y) * D
            weights.append(w / (1 + r * r))
    return m if m is not None else WS.IDENTITY


def motion(c, n, o, m):
    """Step 3's d toward frame m, as two lists, or None when that side is still."""
    if not (c["in"] <= m <= FRAMES - 1):
        return None
    q = drawn(c, m)
    if q["px"] == o["px"]:
        return None
    if c["method"] == "pixel_motion":
        u = flow.estimate(o, q, {"vector_detail": value_at(c["pixel_motion_detail"], n)})
        return [list(u[0]), list(u[1])]
    t = fitted_map(o, q, 100 if c["detailed_analysis"] == "on" else 50)
    dx, dy = [], []
    for y in range(H):
        for x in range(W):
            px, py = (x + 0.5 - W / 2) / D, (y + 0.5 - H / 2) / D
            X, Y = WS.apply(t, px, py)
            dx.append((X - px) * D)
            dy.append((Y - py) * D)
    return [dx, dy]


def repaired(c, n):
    o = drawn(c, n)
    r = value_at(c["rolling_shutter_rate"], n) / 100
    if r == 0:
        return o
    ahead, back = motion(c, n, o, n + 1), motion(c, n, o, n - 1)
    if ahead is None and back is None:
        return o
    if ahead is None:
        ahead = [[-v for v in back[0]], [-v for v in back[1]]]
    if back is None:
        back = [[-v for v in ahead[0]], [-v for v in ahead[1]]]
    scan = c["scan_direction"]
    out = []
    for y in range(H):
        for x in range(W):
            u = {"top_to_bottom": (y + 0.5) / H, "bottom_to_top": 1 - (y + 0.5) / H,
                 "left_to_right": (x + 0.5) / W, "right_to_left": 1 - (x + 0.5) / W}[scan]
            tau = r * (u - 0.5)
            i = y * W + x
            d, s = (ahead, tau) if tau >= 0 else (back, -tau)
            out.append(list(L.bilinear(o, x + 0.5 + s * d[0][i], y + 0.5 + s * d[1][i])))
    return L.pic(W, H, out)


# --- the cases ------------------------------------------------------------------------------

ADDED = {"rolling_shutter_rate": 50, "scan_direction": "top_to_bottom", "method": "warp",
         "detailed_analysis": "off", "pixel_motion_detail": 20}
KEYS = tuple(ADDED)


def case(asset="skew", spans=ONES, start=0, shift=(0, 0), before=None, after=None,
         on="holder", **settings):
    c = dict(ADDED)
    c.update(settings)
    c.update({"asset": asset, "spans": spans, "in": start, "shift": shift, "before": before,
              "after": after, "on": on})
    return c


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        out = drawn(c, frame)
    else:
        out = repaired(c, frame)
        if c["after"] is not None:
            out = L.exposure(out, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    dx, dy = c["shift"]
    p = drawn(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


PM = {"method": "pixel_motion"}
CASES = {
    "FX-RSR-001": ("As added: Warp at 50 per cent, scanned top to bottom, on the card filmed "
                   "scanning in 80 per cent of a frame. Frame 0 is the layer's first, its motion "
                   "before taken as the motion after turned round; frame 5 its last.", case(),
                   (0, 2, 5)),
    "FX-RSR-002": ("Rate 80, the camera's own: the slant taken out.",
                   case(rolling_shutter_rate=80), (2,)),
    "FX-RSR-003": ("Rate 0: the layer exactly as it is.", case(rolling_shutter_rate=0), (2,)),
    "FX-RSR-004": ("Rate 100.", case(rolling_shutter_rate=100), (2,)),
    "FX-RSR-005": ("Bottom to Top on the card scanned top to bottom: the slant doubled, not taken "
                   "out.", case(rolling_shutter_rate=80, scan_direction="bottom_to_top"), (2,)),
    "FX-RSR-006": ("Left to Right on the card panning down and scanned left to right.",
                   case(asset="side", rolling_shutter_rate=80, scan_direction="left_to_right"),
                   (2,)),
    "FX-RSR-007": ("Right to Left on the same card.",
                   case(asset="side", rolling_shutter_rate=80, scan_direction="right_to_left"),
                   (2,)),
    "FX-RSR-008": ("Pixel Motion at 80 per cent.", case(rolling_shutter_rate=80, **PM), (2,)),
    "FX-RSR-009": ("Pixel Motion, Pixel Motion Detail 100.",
                   case(rolling_shutter_rate=80, pixel_motion_detail=100, **PM), (2,)),
    "FX-RSR-010": ("Pixel Motion Detail keyed from 20 at frame 0 to 100 at frame 4: 60 at "
                   "frame 2.", case(rolling_shutter_rate=80,
                                    pixel_motion_detail=keyed((0, 20), (4, 100)), **PM), (2,)),
    "FX-RSR-011": ("Warp with Detailed Analysis on.",
                   case(rolling_shutter_rate=80, detailed_analysis="on"), (2,)),
    "FX-RSR-012": ("Rate keyed from 0 at frame 0 to 100 at frame 4: 50 at frame 2, as added.",
                   case(rolling_shutter_rate=keyed((0, 0), (4, 100))), (2,)),
    "FX-RSR-013": ("Pixel Motion Blur's disc running over nothing, Pixel Motion at 100: only its "
                   "own pixels move.", case(asset="disc", rolling_shutter_rate=100, **PM), (2,)),
    "FX-RSR-014": ("An Exposure of +1 before Rolling Shutter Repair: the frames it reads are "
                   "brightened too.", case(before=1), (2,)),
    "FX-RSR-015": ("An Exposure of -1 after Rolling Shutter Repair darkens its picture.",
                   case(after=-1), (2,)),
    "FX-RSR-016": ("The holder moved 3 right and 2 down: FX-RSR-001 moved.", case(shift=(3, 2)),
                   (2,)),
    "FX-RSR-017": ("The layer's in point at frame 2: frame 2 is its first, its motion before "
                   "taken as the motion after turned round.", case(start=2), (2, 3)),
    "FX-RSR-018": ("The effect on an adjustment layer above the holder: nothing changes, and the "
                   "frame says why.", case(on="adjust"), (2,)),
    "FX-RSR-019": ("The card on twos: each frame's held side takes the other side's motion "
                   "turned round.", case(spans=TWOS), (2, 3)),
}

INVALID = {
    "FX-RSR-020": ("Rolling Shutter Rate 101, above 100.", case(rolling_shutter_rate=101)),
    "FX-RSR-021": ("Rolling Shutter Rate -1, below 0.", case(rolling_shutter_rate=-1)),
    "FX-RSR-022": ("Scan Direction written \"diagonal\".", case(scan_direction="diagonal")),
    "FX-RSR-023": ("Method written \"optical\".", case(method="optical")),
    "FX-RSR-024": ("Pixel Motion Detail 101, above 100.", case(pixel_motion_detail=101)),
    "FX-RSR-025": ("Rolling Shutter Rate keyed to 150 at frame 4.",
                   case(rolling_shutter_rate=keyed((0, 50), (4, 150)))),
    "FX-RSR-026": ("Detailed Analysis written \"yes\".", case(detailed_analysis="yes")),
}


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.rolling_shutter_repair", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def project_json(fx, c):
    asset = c["asset"]
    holder = L.raster("holder", "asset-" + asset, spans=c["spans"], position=c["shift"],
                      in_frame=c["in"], out_frame=FRAMES)
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    comp = L.composition("comp-main", W, H, FRAMES, layers)
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-" + asset, "kind": "image_sequence", "name": asset,
                        "pattern": asset + "_####.png",
                        "frames": {str(k): f"media/{asset}_{k}.png" for k in range(1, FRAMES + 1)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [comp]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)), "frames": rendered}
        if c["on"] == "adjust":
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": plain(c, 0), "4": plain(c, 4)}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_rolling_shutter_repair.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def slant(px):
    """How far the card's stripes lean: the mean, over the middle columns, of the best shift
    between the top quarter's and the bottom quarter's green, in pixels (sub-pixel by parabola)."""
    def band(y0, y1):
        return [sum(px[y * W + x][1] for y in range(y0, y1)) / (y1 - y0) for x in range(W)]
    top, bottom = band(4, 10), band(22, 28)
    score = lambda s: sum((top[x] - bottom[x + s]) ** 2 for x in range(8, 24))  # noqa: E731
    shifts = list(range(-6, 7))
    best = min(shifts, key=score)
    a, b, c = score(best - 1), score(best), score(best + 1)
    return best + (a - c) / (2 * (a - 2 * b + c)) if a - 2 * b + c > 0 else float(best)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    d = lambda k, **kw: plain(case(**kw), k)  # noqa: E731
    # The drawings lean by the pan times the scan: 3 x 0.8 x (18 / 32) between the bands.
    lean = slant(d(2))
    assert abs(lean - 3 * CAMERA * 18 / 32) < 0.4, lean
    # The camera's own rate takes most of the lean out; Bottom to Top makes it worse.
    for fx in ("FX-RSR-002", "FX-RSR-008"):
        assert abs(slant(c[fx]["2"])) < abs(lean) / 3, (fx, slant(c[fx]["2"]))
    assert abs(slant(c["FX-RSR-005"]["2"])) > abs(lean) * 1.5
    assert abs(slant(c["FX-RSR-001"]["2"])) < abs(lean)
    assert c["FX-RSR-001"]["0"] != d(0) and c["FX-RSR-001"]["5"] != d(5)
    # Rate 0 and the adjustment layer are exactly as drawn.
    assert c["FX-RSR-003"]["2"] == d(2) and c["FX-RSR-018"]["2"] == d(2)
    assert near(c["FX-RSR-012"]["2"], c["FX-RSR-001"]["2"])
    assert c["FX-RSR-006"]["2"] != c["FX-RSR-007"]["2"]
    assert c["FX-RSR-009"]["2"] != c["FX-RSR-008"]["2"]
    assert c["FX-RSR-010"]["2"] not in (c["FX-RSR-008"]["2"], c["FX-RSR-009"]["2"])
    assert c["FX-RSR-011"]["2"] != c["FX-RSR-002"]["2"]
    assert c["FX-RSR-004"]["2"] != c["FX-RSR-002"]["2"]
    assert c["FX-RSR-014"]["2"] != c["FX-RSR-001"]["2"]
    assert near(c["FX-RSR-015"]["2"], [[p[0] / 2, p[1] / 2, p[2] / 2, p[3]]
                                       for p in c["FX-RSR-001"]["2"]])
    moved = c["FX-RSR-016"]["2"]
    assert all(moved[(y + 2) * W + x + 3] == c["FX-RSR-001"]["2"][y * W + x]
               for y in range(H - 2) for x in range(W - 3))
    assert c["FX-RSR-017"]["2"] != plain(case(start=2), 2)
    assert c["FX-RSR-019"]["2"] != d(2, spans=TWOS)
    # The disc moves only where it is: far corners stay clear.
    disc = c["FX-RSR-013"]["2"]
    assert disc != d(2, asset="disc") and disc[0][3] == 0 and disc[-1][3] == 0
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(a >= -1e-12 for a in p) and p[3] <= 1 + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
