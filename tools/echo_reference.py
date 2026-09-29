"""Echo, worked a second way.

D-195 proposes Echo (`core.echo`): a time effect that lays copies of its layer from other frames
over it, as trails. This file pins the rule.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it, and whose drawing is W by
H at n:

1. On an adjustment layer the output is O. Otherwise t = floor(echo_time) and N = floor(echoes),
   each held inside its range at frame n, as every setting is.
2. For k = 0 to N, P_k is the holder's own picture at composition frame n + k t by document 21's
   steps 1 and 2 alone: its drawing, solid, shapes or composition, and its masks, at that frame
   by the layer's own timing. Its effects are not run, the ones before Echo included. A frame
   outside the layer's in and out points gives an empty picture. P_k lies on the drawing corner
   to corner and is cut to W by H, transparent outside it.
3. Q_k = I D^k P_k, all four numbers, with I = intensity and D = decay (D^0 = 1).
4. The operator makes R from Q_0 to Q_N, all four numbers each: `add` min(1, sum); `maximum` and
   `minimum` the largest and smallest; `screen` 1 - product of (1 - clamp(Q, 0, 1));
   `composite_in_back` Q_0 over Q_1 over ... over Q_N, the present on top; `composite_in_front`
   Q_N over ... over Q_0, the furthest echo on top; `blend` sum / (N + 1). Here a over b is
   a + (1 - a's covering) b.
5. The output is R with its top-left pixel at (ox, oy), transparent elsewhere: what effects
   before Echo made is replaced. The bounds do not grow.
6. For a draft each P_k is made at the holder's divisor, as D-189's map is (step 3); no setting
   is a distance.

`echo_time`, -120 to 120 frames, -1 when added; `echoes`, 0 to 30, 1 when added; `intensity`
and `decay`, 0 to 1, 1 when added; all four keyable. `operator`, one of the seven words above,
`add` when added.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values, one echo after another, where the build folds each into the last.

Every case is a project of one composition 16 by 10, eight frames, in `Fixtures/echo/`: the
drawing `holder`, a ball running right across five drawings, with the effect. The expected pixels
are in `Fixtures/echo/expected_echo.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/echo_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "echo"
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


def over(a, b):
    return [p + (1 - a[3]) * q for p, q in zip(a, b)]


def combine(qs, operator):
    out = []
    for px in zip(*qs):
        if operator == "add":
            r = [min(1.0, sum(v)) for v in zip(*px)]
        elif operator == "maximum":
            r = [max(v) for v in zip(*px)]
        elif operator == "minimum":
            r = [min(v) for v in zip(*px)]
        elif operator == "screen":
            r = []
            for v in zip(*px):
                keep = 1.0
                for q in v:
                    keep *= 1 - min(1.0, max(0.0, q))
                r.append(1 - keep)
        elif operator == "composite_in_back":
            r = list(px[-1])
            for q in reversed(px[:-1]):
                r = over(q, r)
        elif operator == "composite_in_front":
            r = list(px[0])
            for q in px[1:]:
                r = over(q, r)
        elif operator == "blend":
            r = [sum(v) / len(px) for v in zip(*px)]
        else:
            raise AssertionError(operator)
        out.append(r)
    return out


def echo(c, n):
    """Steps 1 to 4 at frame n: R, W by H."""
    t = floor(value_at(c["echo_time"], n))
    count = floor(value_at(c["echoes"], n))
    i, d = value_at(c["intensity"], n), value_at(c["decay"], n)
    base = picture(c, n)
    qs = []
    for k in range(count + 1):
        p = picture(c, n + k * t)
        g = i * d ** k
        qs.append([[v * g for v in L.at(p, x, y)] for y in range(base["h"]) for x in range(base["w"])])
    return L.pic(base["w"], base["h"], combine(qs, c["operator"]))


# --- the cases ------------------------------------------------------------------------------

def case(echo_time=-1, echoes=1, intensity=1, decay=1, operator="add", spans=ONES, start=0,
         shift=(0, 0), mask=None, before=None, after=None, on="holder"):
    return {"echo_time": echo_time, "echoes": echoes, "intensity": intensity, "decay": decay,
            "operator": operator, "spans": spans, "in": start, "shift": shift, "mask": mask,
            "before": before, "after": after, "on": on}


def render(c, frame):
    """The composition's pixels at frame: the holder with its effects, moved by its shift; or,
    with the effect on an adjustment layer above, the holder as it is."""
    if c["on"] == "adjust":
        out = picture(c, frame)
    else:
        out = echo(c, frame)
        if c["after"] is not None:
            out = L.exposure(out, c["after"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    dx, dy = c["shift"]
    p = picture(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-ECHO-001": ("As added: one echo, one frame back, starting intensity and decay 1, Add. At "
                    "frame 0 the frame before is outside the layer, so only the ball; at frame 2 "
                    "drawings 3 and 2 added, brighter where they overlap; at frame 5 drawing 5, "
                    "held, added to itself.", case(), (0, 2, 5)),
    "FX-ECHO-002": ("No echoes: the layer exactly as it is.", case(echoes=0), (0, 3)),
    "FX-ECHO-003": ("No echoes, starting intensity 0.5: the ball at half strength.",
                    case(echoes=0, intensity=0.5), (3,)),
    "FX-ECHO-004": ("Three echoes one frame apart, decay 0.5, Add: drawing 5 whole, 4 at a half, 3 "
                    "at a quarter, 2 at an eighth.", case(echoes=3, decay=0.5), (4,)),
    "FX-ECHO-005": ("Echo time +1, two echoes: the frames after. At frame 1 drawings 2, 3 and 4; "
                    "at frame 6 drawing 5 twice and frame 8, past the out point, empty.",
                    case(echo_time=1, echoes=2), (1, 6)),
    "FX-ECHO-006": ("Echo time -2, two echoes: drawings 5, 3 and 1 at frame 4.",
                    case(echo_time=-2, echoes=2), (4,)),
    "FX-ECHO-007": ("Three echoes, decay 0.7, Maximum.",
                    case(echoes=3, decay=0.7, operator="maximum"), (4,)),
    "FX-ECHO-008": ("One echo, Minimum: only where drawings 4 and 3 overlap is anything left.",
                    case(operator="minimum"), (3,)),
    "FX-ECHO-009": ("Three echoes, decay 0.7, Screen.",
                    case(echoes=3, decay=0.7, operator="screen"), (4,)),
    "FX-ECHO-010": ("Three echoes, decay 0.7, Composite In Back: the present ball on top.",
                    case(echoes=3, decay=0.7, operator="composite_in_back"), (4,)),
    "FX-ECHO-011": ("Three echoes, decay 0.7, Composite In Front: the furthest echo on top.",
                    case(echoes=3, decay=0.7, operator="composite_in_front"), (4,)),
    "FX-ECHO-012": ("Three echoes, Blend: the four drawings averaged.",
                    case(echoes=3, operator="blend"), (4,)),
    "FX-ECHO-013": ("The layer's in point at frame 2, three echoes: at frame 3 its drawings 2 and "
                    "1; frames 1 and 0 are before it, empty.", case(echoes=3, start=2), (3,)),
    "FX-ECHO-014": ("Drawings on twos, one echo: at frame 2 drawings 2 and 1; at frame 3 drawing 2 "
                    "added to itself.", case(spans=TWOS), (2, 3)),
    "FX-ECHO-015": ("Number of echoes keyed from 0 at frame 0 to 3 at frame 4, decay 0.5: 1.5 at "
                    "frame 2 counts as 1.", case(echoes=keyed((0, 0), (4, 3)), decay=0.5),
                    (0, 2, 4)),
    "FX-ECHO-016": ("Echo time -1.5 counts as -2: FX-ECHO-006.",
                    case(echo_time=-1.5, echoes=2), (4,)),
    "FX-ECHO-017": ("An Exposure of +1 before the Echo is not seen: Echo reads the drawing, not "
                    "what came before it. FX-ECHO-001.", case(before=1), (2,)),
    "FX-ECHO-018": ("An Exposure of -1 after the Echo darkens the trail too.", case(after=-1),
                    (2,)),
    "FX-ECHO-019": ("A mask keeping columns 0 to 6, two echoes: every echo is masked.",
                    case(echoes=2, mask=L.rect(0, 0, 7, 5)), (3,)),
    "FX-ECHO-020": ("The holder moved 2 right and 3 down: FX-ECHO-001 moved, since the echoes lie "
                    "on the layer.", case(shift=(2, 3)), (2,)),
    "FX-ECHO-021": ("The effect on an adjustment layer above the holder: nothing changes.",
                    case(on="adjust"), (2,)),
    "FX-ECHO-022": ("Echo time 0, two echoes, decay 0.5: the same drawing three times, 1.75 of it, "
                    "held at 1.", case(echo_time=0, echoes=2, decay=0.5), (2,)),
    "FX-ECHO-023": ("Starting intensity keyed from 1 at frame 0 to 0 at frame 4: at frame 4 "
                    "nothing is left.", case(intensity=keyed((0, 1), (4, 0))), (0, 2, 4)),
}

INVALID = {
    "FX-ECHO-024": ("Number of echoes 31, above 30.", case(echoes=31)),
    "FX-ECHO-025": ("Number of echoes -1, below 0.", case(echoes=-1)),
    "FX-ECHO-026": ("Starting intensity 1.5, above 1.", case(intensity=1.5)),
    "FX-ECHO-027": ("Decay -0.1, below 0.", case(decay=-0.1)),
    "FX-ECHO-028": ("Echo time 121, above 120.", case(echo_time=121)),
    "FX-ECHO-029": ("Echo time keyed to -200 at frame 4.",
                    case(echo_time=keyed((0, -1), (4, -200)))),
    "FX-ECHO-030": ("An operator written \"multiply\".", case(operator="multiply")),
}

KEYS = ("echo_time", "echoes", "intensity", "decay", "operator")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.echo", "enabled": True,
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

    (OUT / "expected_echo.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = lambda f: plain(case(), f)  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b) for p, q in zip(u, v))  # noqa: E731
    add = lambda *ps: [[min(1.0, sum(v)) for v in zip(*px)] for px in zip(*ps)]  # noqa: E731
    scaled = lambda p, g: [[v * g for v in px] for px in p]  # noqa: E731
    assert c["FX-ECHO-001"]["0"] == drawn(0)
    assert near(c["FX-ECHO-001"]["2"], add(drawn(2), drawn(1)))
    assert near(c["FX-ECHO-001"]["5"], add(drawn(5), drawn(4)))
    assert c["FX-ECHO-001"]["2"] != add(drawn(2))  # the overlap is held at 1
    assert c["FX-ECHO-002"]["0"] == drawn(0) and c["FX-ECHO-002"]["3"] == drawn(3)
    assert near(c["FX-ECHO-003"]["3"], scaled(drawn(3), 0.5))
    assert near(c["FX-ECHO-004"]["4"], add(drawn(4), scaled(drawn(3), 0.5), scaled(drawn(2), 0.25),
                                           scaled(drawn(1), 0.125)))
    assert near(c["FX-ECHO-005"]["1"], add(drawn(1), drawn(2), drawn(3)))
    assert near(c["FX-ECHO-005"]["6"], add(drawn(6), drawn(7)))
    assert near(c["FX-ECHO-006"]["4"], add(drawn(4), drawn(2), drawn(0)))
    assert c["FX-ECHO-016"]["4"] == c["FX-ECHO-006"]["4"]
    # Minimum: only the two columns where drawings 4 and 3 meet keep anything.
    kept = {x for y in range(H) for x in range(W) if c["FX-ECHO-008"]["3"][y * W + x] != CLEAR}
    assert kept == {6, 7}, kept
    # Composite In Back: the present ball whole on top. In Front: the oldest echo, frame 1's
    # drawing 2, alone in columns 2 and 3, and over frame 2's in column 5, where In Back has frame
    # 2's over it.
    for i, p in enumerate(drawn(4)):
        if p[3] == 1.0:
            assert c["FX-ECHO-010"]["4"][i] == p
    oldest, next_ = scaled(drawn(1), 0.7 ** 3), scaled(drawn(2), 0.7 ** 2)
    for y in range(1, 4):
        for x in (2, 3):
            assert oldest[y * W + x] != CLEAR
            assert near([c["FX-ECHO-011"]["4"][y * W + x]], [oldest[y * W + x]])
        i = y * W + 5
        assert oldest[i][3] > 0.3 and next_[i][3] > 0.4
        assert near([c["FX-ECHO-011"]["4"][i]], [over(oldest[i], next_[i])])
        assert near([c["FX-ECHO-010"]["4"][i]], [over(next_[i], oldest[i])])
    assert near(c["FX-ECHO-012"]["4"], [[sum(v) / 4 for v in zip(*px)]
                                        for px in zip(drawn(4), drawn(3), drawn(2), drawn(1))])
    late = lambda f: plain(case(start=2), f)  # noqa: E731
    assert near(c["FX-ECHO-013"]["3"], add(late(3), late(2)))
    twos = lambda f: plain(case(spans=TWOS), f)  # noqa: E731
    assert near(c["FX-ECHO-014"]["2"], add(twos(2), twos(1)))
    assert near(c["FX-ECHO-014"]["3"], add(twos(3), twos(3)))
    assert c["FX-ECHO-015"]["0"] == drawn(0)
    assert near(c["FX-ECHO-015"]["2"], add(drawn(2), scaled(drawn(1), 0.5)))
    assert c["FX-ECHO-015"]["4"] == c["FX-ECHO-004"]["4"]
    assert c["FX-ECHO-017"]["2"] == c["FX-ECHO-001"]["2"]
    assert near(c["FX-ECHO-018"]["2"], [[p[0] / 2, p[1] / 2, p[2] / 2, p[3]]
                                        for p in c["FX-ECHO-001"]["2"]])
    assert all(p == CLEAR for y in range(H) for x in range(7, W)
               for p in [c["FX-ECHO-019"]["3"][y * W + x]])
    assert c["FX-ECHO-019"]["3"] != c["FX-ECHO-004"]["4"]
    moved = c["FX-ECHO-020"]["2"]
    assert all(moved[(y + 3) * W + x + 2] == c["FX-ECHO-001"]["2"][y * W + x]
               for y in range(H - 3) for x in range(W - 2))
    assert c["FX-ECHO-021"]["2"] == drawn(2)
    assert near(c["FX-ECHO-022"]["2"], add(scaled(drawn(2), 1.75)))
    assert c["FX-ECHO-023"]["0"] == c["FX-ECHO-001"]["0"]
    assert near(c["FX-ECHO-023"]["2"], add(scaled(drawn(2), 0.5), scaled(drawn(1), 0.5)))
    assert all(p == CLEAR for p in c["FX-ECHO-023"]["4"])
    # Every pixel a picture can hold: numbers from 0 to 1, colour no more than covering.
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
