"""CC Wide Time, worked a second way.

D-520 proposes CC Wide Time (`core.cc_wide_time`), after CycoreFX's CC Wide Time (Time): a time
effect that averages its layer over a span of frames before and after the present one, a
smearing kind of motion blur. This file pins the rule. The CycoreFX manual names the three
controls and says the layer's own effects should be pre-composed first; it publishes no formula
and no defaults, so the rule and the settings when added are ours.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is W by
H at n:

1. On an adjustment layer the output is O. Otherwise F = floor(forward_steps) and
   B = floor(backward_steps), each held inside its range at frame n, as every setting is.
2. For m = n - B to n + F, P_m is the holder's own picture at composition frame m by document
   21's steps 1 and 2 alone: its drawing, solid, shapes or composition, and its masks, at that
   frame by the layer's own timing. Its effects are not run, the ones before CC Wide Time
   included. A frame outside the layer's in and out points gives an empty picture, which still
   counts. P_m lies on the drawing corner to corner and is cut to W by H, transparent outside it.
3. R = (sum of P_m) / (B + F + 1), all four numbers, each frame weighted the same.
4. The output is R with its top-left pixel at (ox, oy), transparent elsewhere: what effects
   before CC Wide Time made is replaced. The bounds do not grow.
5. `native_motion_blur` `off`, as added: compositions nested inside the holder are drawn with
   their own motion blur switched off, so the frames are not blurred twice. `on`: they are drawn
   as they are. A project with no composition blur draws the same either way.
6. For a draft each P_m is made at the holder's divisor, as Echo's are; no setting is a distance.

`forward_steps` and `backward_steps`, 0 to 100 whole frames, 3 when added, both keyable;
`native_motion_blur`, `off` or `on`, `off` when added.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values, summing the frames at once, where the build folds each into the last.

Every case is a project of one composition 16 by 10, eight frames, in `Fixtures/cc_wide_time/`:
the drawing `holder`, a ball running right across five drawings, with the effect. The expected
pixels are in `Fixtures/cc_wide_time/expected_cc_wide_time.json`. Step 5's nested composition
is checked by the build's own test, not here.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_wide_time_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_wide_time"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 8
DW, DH = 14, 5  # every drawing of the run
CLEAR = [0.0, 0.0, 0.0, 0.0]


def ball(k):
    """Drawing k, 1 to 5: a ball three pixels square in rows 1 to 3, its left column at 2k - 1,
    a colour of its own, and a half-covering smear in the column behind it."""
    colour = (230, 40 + 40 * k, 210 - 40 * k)
    x0 = 2 * k - 1
    rows = []
    for y in range(DH):
        row = []
        for x in range(DW):
            if 1 <= y <= 3 and x0 <= x < x0 + 3:
                row.append(colour + (255,))
            elif 1 <= y <= 3 and x == x0 - 1:
                row.append(colour + (128,))
            else:
                row.append((0, 0, 0, 0))
        rows.append(row)
    return rows


DRAWINGS = {f"run_{k}": ball(k) for k in range(1, 6)}
ONES = [{"start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1},
        {"start_frame": 1, "end_frame_exclusive": 2, "drawing_number": 2},
        {"start_frame": 2, "end_frame_exclusive": 3, "drawing_number": 3},
        {"start_frame": 3, "end_frame_exclusive": 4, "drawing_number": 4},
        {"start_frame": 4, "end_frame_exclusive": 8, "drawing_number": 5}]
TWOS = [{"start_frame": 2 * i, "end_frame_exclusive": 2 * i + 2, "drawing_number": i + 1}
        for i in range(4)]


def decoded(number):
    L.DRAWINGS[f"run_{number}"] = DRAWINGS[f"run_{number}"]
    return L.decoded(f"run_{number}")


# --- the rule -------------------------------------------------------------------------------

def picture(c, m):
    """P at composition frame m: the holder's drawing through its mask, or empty."""
    local = m - c["in"]
    if not c["in"] <= m < FRAMES:
        return L.EMPTY
    number = next(s["drawing_number"] for s in c["spans"]
                  if s["start_frame"] <= local < s["end_frame_exclusive"])
    return L.masked(decoded(number), c["mask"], 1)


def wide(c, n):
    """Steps 1 to 3 at frame n: R, W by H."""
    f = floor(value_at(c["forward_steps"], n))
    b = floor(value_at(c["backward_steps"], n))
    base = picture(c, n)
    frames = [picture(c, m) for m in range(n - b, n + f + 1)]
    out = []
    for y in range(base["h"]):
        for x in range(base["w"]):
            px = [L.at(p, x, y) for p in frames]
            out.append([sum(v) / len(frames) for v in zip(*px)])
    return L.pic(base["w"], base["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(forward_steps=3, backward_steps=3, native_motion_blur="off", spans=ONES, start=0,
         shift=(0, 0), mask=None, before=None, after=None, on="holder"):
    return {"forward_steps": forward_steps, "backward_steps": backward_steps,
            "native_motion_blur": native_motion_blur, "spans": spans, "in": start,
            "shift": shift, "mask": mask, "before": before, "after": after, "on": on}


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        out = picture(c, frame)
    else:
        out = wide(c, frame)
        if c["after"] is not None:
            out = L.exposure(out, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    dx, dy = c["shift"]
    p = picture(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-WIDE-001": ("As added: three frames before and three after, seven averaged. At frame 3 "
                    "drawings 1 to 4 and drawing 5 three times; at frame 0 the three frames "
                    "before the layer are empty but still counted, so the ball is faint; at "
                    "frame 6 frame 8 is past the out point.", case(), (0, 3, 6)),
    "FX-WIDE-002": ("No steps either way: the layer exactly as it is.",
                    case(forward_steps=0, backward_steps=0), (0, 3)),
    "FX-WIDE-003": ("Two forward steps only: frames 1, 2 and 3 averaged at frame 1.",
                    case(forward_steps=2, backward_steps=0), (1,)),
    "FX-WIDE-004": ("Two backward steps only: frames 2, 3 and 4 averaged at frame 4.",
                    case(forward_steps=0, backward_steps=2), (4,)),
    "FX-WIDE-005": ("One step each way, the layer's in point at frame 2: at frame 2 the frame "
                    "before is outside the layer and counts empty.",
                    case(forward_steps=1, backward_steps=1, start=2), (2, 3)),
    "FX-WIDE-006": ("One step each way, drawings on twos: at frame 2 drawings 1, 2 and 2; at "
                    "frame 3 drawings 2, 2 and 3.",
                    case(forward_steps=1, backward_steps=1, spans=TWOS), (2, 3)),
    "FX-WIDE-007": ("Forward steps keyed from 0 at frame 0 to 4 at frame 4, no backward steps: "
                    "2 at frame 2.", case(forward_steps=keyed((0, 0), (4, 4)), backward_steps=0),
                    (0, 2, 4)),
    "FX-WIDE-008": ("Steps 1.5 forward and 1.9 back count as one each.",
                    case(forward_steps=1.5, backward_steps=1.9), (3,)),
    "FX-WIDE-009": ("An Exposure of +1 before CC Wide Time is not seen: it reads the drawing, "
                    "not what came before it. FX-WIDE-001.", case(before=1), (3,)),
    "FX-WIDE-010": ("An Exposure of -1 after CC Wide Time darkens the smear too.",
                    case(after=-1), (3,)),
    "FX-WIDE-011": ("A mask keeping columns 0 to 6: every frame is masked.",
                    case(mask=L.rect(0, 0, 7, 5)), (3,)),
    "FX-WIDE-012": ("The holder moved 2 right and 3 down: FX-WIDE-001 moved, since the frames "
                    "lie on the layer.", case(shift=(2, 3)), (3,)),
    "FX-WIDE-013": ("The effect on an adjustment layer above the holder: nothing changes.",
                    case(on="adjust"), (3,)),
    "FX-WIDE-014": ("Native Motion Blur on, with no composition inside: FX-WIDE-001.",
                    case(native_motion_blur="on"), (3,)),
    "FX-WIDE-015": ("Ten steps each way, past both ends of the layer: 21 frames counted, eight "
                    "of them drawn.", case(forward_steps=10, backward_steps=10), (4,)),
}

INVALID = {
    "FX-WIDE-016": ("Forward steps 101, above 100.", case(forward_steps=101)),
    "FX-WIDE-017": ("Backward steps -1, below 0.", case(backward_steps=-1)),
    "FX-WIDE-018": ("Backward steps keyed to 150 at frame 4.",
                    case(backward_steps=keyed((0, 3), (4, 150)))),
    "FX-WIDE-019": ("Native Motion Blur written \"maybe\".", case(native_motion_blur="maybe")),
}

KEYS = ("forward_steps", "backward_steps", "native_motion_blur")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.cc_wide_time", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-run", spans=c["spans"], position=c["shift"],
                      in_frame=c["in"], out_frame=FRAMES, mask=c["mask"])
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-run", "kind": "image_sequence", "name": "run",
                        "pattern": "run_####.png",
                        "frames": {str(k): f"media/run_{k}.png" for k in range(1, 6)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


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

    (OUT / "expected_cc_wide_time.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = lambda f: plain(case(), f)  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b) for p, q in zip(u, v))  # noqa: E731
    mean = lambda *ps: [[sum(v) / len(ps) for v in zip(*px)] for px in zip(*ps)]  # noqa: E731
    empty = [CLEAR] * (W * H)
    # Frame 3: frames 0 to 6, drawings 1, 2, 3, 4 and 5 three times.
    assert near(c["FX-WIDE-001"]["3"], mean(*[drawn(m) for m in range(7)]))
    assert near(c["FX-WIDE-001"]["0"], mean(empty, empty, empty, *[drawn(m) for m in range(4)]))
    assert near(c["FX-WIDE-001"]["6"], mean(*[drawn(m) for m in range(3, 8)], empty, empty))
    assert c["FX-WIDE-002"]["0"] == drawn(0) and c["FX-WIDE-002"]["3"] == drawn(3)
    assert near(c["FX-WIDE-003"]["1"], mean(drawn(1), drawn(2), drawn(3)))
    assert near(c["FX-WIDE-004"]["4"], mean(drawn(2), drawn(3), drawn(4)))
    late = lambda f: plain(case(start=2), f)  # noqa: E731
    assert near(c["FX-WIDE-005"]["2"], mean(empty, late(2), late(3)))
    assert near(c["FX-WIDE-005"]["3"], mean(late(2), late(3), late(4)))
    twos = lambda f: plain(case(spans=TWOS), f)  # noqa: E731
    assert twos(1) != twos(2) and twos(2) == twos(3)
    assert near(c["FX-WIDE-006"]["2"], mean(twos(1), twos(2), twos(3)))
    assert near(c["FX-WIDE-006"]["3"], mean(twos(2), twos(3), twos(4)))
    assert c["FX-WIDE-007"]["0"] == drawn(0)
    assert near(c["FX-WIDE-007"]["2"], mean(drawn(2), drawn(3), drawn(4)))
    assert near(c["FX-WIDE-007"]["4"], mean(drawn(4), drawn(5), drawn(6), drawn(7), empty))
    assert near(c["FX-WIDE-008"]["3"], mean(drawn(2), drawn(3), drawn(4)))
    assert c["FX-WIDE-009"]["3"] == c["FX-WIDE-001"]["3"]
    assert near(c["FX-WIDE-010"]["3"], [[p[0] / 2, p[1] / 2, p[2] / 2, p[3]]
                                        for p in c["FX-WIDE-001"]["3"]])
    assert all(p == CLEAR for y in range(H) for x in range(7, W)
               for p in [c["FX-WIDE-011"]["3"][y * W + x]])
    assert c["FX-WIDE-011"]["3"] != c["FX-WIDE-001"]["3"]
    moved = c["FX-WIDE-012"]["3"]
    assert all(moved[(y + 3) * W + x + 2] == c["FX-WIDE-001"]["3"][y * W + x]
               for y in range(H - 3) for x in range(W - 2))
    assert c["FX-WIDE-013"]["3"] == drawn(3)
    assert c["FX-WIDE-014"]["3"] == c["FX-WIDE-001"]["3"]
    assert near(c["FX-WIDE-015"]["4"], mean(*[empty] * 6, *[drawn(m) for m in range(8)],
                                            *[empty] * 7))
    # Every pixel a picture can hold: numbers from 0 to 1, colour no more than covering.
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
