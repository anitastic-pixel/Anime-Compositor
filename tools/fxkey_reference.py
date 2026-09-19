"""Keyframed effect settings, worked a second way.

D-68 proposes that an effect's setting may be animated as a layer's position is: in the file the
setting is either a plain number (or, for a colour, three), which is constant, or a property
record `{"base", "keyframes"}` exactly as document 19 writes a transform property. Its value at a
composition frame is document 20's: the first key's value before it, the last key's after it,
and between two keys hold, linear, or the eased fraction of the way. A colour's three numbers
all take the same fraction, in linear RGB. The value is then held inside the setting's own range
(`sigma_px >= 0`, `amount` within 0 and 1), because an ease may overshoot and the effect's
arithmetic has no meaning outside it. The effect then runs as document 21 says, with that value.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel through
`tools/adjust_reference.py`, and solves an ease by `tools/ease_reference.py`'s bisection where
the build uses Newton's method.

Every case is a composition 6 pixels by 2 and five frames long. The projects and drawings go
into `Fixtures/fxkey`, and the expected frames into `Fixtures/fxkey/expected_fxkey.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fxkey_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as adj  # noqa: E402
from adjust_reference import W, H, BG, HALF, DOT, decoded, over, fmt  # noqa: E402
from ease_reference import ease, EASE_IN_OUT  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fxkey"
TOLERANCE = 1e-6
FRAMES = 5

# The y handles are past 1, so the eased fraction passes 1 before the segment ends.
OVERSHOOT = (0.3, 1.6, 0.7, 1.6)


def keyed(*keys):
    """An animated setting: keys of (frame, value) or (frame, value, interp), where interp is
    "hold", "linear" (the default) or the four numbers of an ease."""
    return {"keys": [(k[0], k[1], k[2] if len(k) > 2 else "linear") for k in keys]}


def value_at(setting, frame):
    """Document 20, for a number or a list of numbers."""
    if not isinstance(setting, dict):
        return setting
    keys = setting["keys"]
    if frame <= keys[0][0]:
        return keys[0][1]
    if frame >= keys[-1][0]:
        return keys[-1][1]
    a, b = next((a, b) for a, b in zip(keys, keys[1:]) if a[0] <= frame < b[0])
    if a[2] == "hold":
        return a[1]
    f = (frame - a[0]) / (b[0] - a[0])
    if a[2] != "linear":
        f = ease(a[2], f)
    if isinstance(a[1], (list, tuple)):
        return [x + (y - x) * f for x, y in zip(a[1], b[1])]
    return a[1] + (b[1] - a[1]) * f


def in_range(kind, args):
    """The value is held inside the setting's own range after it is worked out."""
    if kind == "blur":
        return [max(0.0, args[0])]
    if kind == "tint":
        return [args[0], min(1.0, max(0.0, args[1]))]
    return args


def stack_at(effects, frame):
    return [(kind, *in_range(kind, [value_at(a, frame) for a in args]))
            for kind, *args in effects]


def render(case, frame_no):
    """Bottom layer first; a drawing's own effects in its own space, an adjustment layer's on
    the frame beneath it. Every layer here is the composition's size and unmoved."""
    frame = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        stack = stack_at(layer.get("effects", []), frame_no)
        if layer["kind"] == "raster":
            frame = [over(s, d) for s, d in
                     zip(adj.run_stack(decoded(layer["drawing"]), stack), frame)]
        else:
            frame = adj.run_stack(frame, stack)
    return frame


# --- the project files ----------------------------------------------------------------------

def setting_json(setting):
    """A plain number stays a plain number; an animated one is document 19's property record."""
    if not isinstance(setting, dict):
        return list(setting) if isinstance(setting, tuple) else setting
    keys = []
    for frame, value, interp in setting["keys"]:
        key = {"frame": frame, "value": list(value) if isinstance(value, tuple) else value,
               "interp": interp if isinstance(interp, str) else "ease"}
        if not isinstance(interp, str):
            key["ease"] = list(interp)
        keys.append(key)
    return {"base": keys[0]["value"], "keyframes": keys}


NAMES = {"exposure": ["stops"], "tint": ["color", "amount"], "blur": ["sigma_px"]}


def project_json(fx, case):
    constant = {"layers": [{**l, "effects": [(k, *[value_at(a, 0) for a in args])
                                             for k, *args in l.get("effects", [])]}
                           for l in case["layers"]]}
    project = adj.project_json(fx, constant)
    comp = project["compositions"][0]
    comp["duration_frames"] = FRAMES
    comp["work_area"]["end_frame_exclusive"] = FRAMES
    for record, layer in zip(comp["layers"], case["layers"]):
        record["out_frame"] = FRAMES
        for written, (kind, *args) in zip(record["effects"], layer.get("effects", [])):
            written["parameters"] = {n: setting_json(a) for n, a in zip(NAMES[kind], args)}
    return project


# --- the cases ------------------------------------------------------------------------------

def adjustment(*effects):
    return {"id": "adj", "kind": "adjustment", "effects": list(effects)}


CASES = {
    "FX-FXK-001": ("Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the "
                   "first key's value before it, the last key's after it, one stop between.",
                   {"layers": [BG, adjustment(("exposure", keyed((1, 0), (3, 2))))]},
                   [0, 1, 2, 3, 4]),
    "FX-FXK-002": ("A hold key: 0 stops until the next key, then 1 stop.",
                   {"layers": [BG, adjustment(("exposure", keyed((0, 0, "hold"), (2, 1))))]},
                   [1, 2]),
    "FX-FXK-003": ("An eased key, 0 to 2 stops over four frames on the ease-in-out curve: "
                   "slow, then fast, then slow.",
                   {"layers": [BG, adjustment(("exposure", keyed((0, 0, EASE_IN_OUT), (4, 2))))]},
                   [1, 2, 3]),
    "FX-FXK-004": ("A tint's colour keyed from red to blue: each of its three numbers goes the "
                   "same fraction of the way, in linear light.",
                   {"layers": [HALF, adjustment(("tint", keyed((0, (1, 0, 0)), (4, (0, 0, 1))),
                                                 1.0))]},
                   [0, 1, 2]),
    "FX-FXK-005": ("A tint's amount eased from 0 to 1 on a curve that overshoots: the amount "
                   "stops at 1 and goes no further.",
                   {"layers": [BG, adjustment(("tint", (0, 0, 1),
                                               keyed((0, 0, OVERSHOOT), (4, 1))))]},
                   [1, 2]),
    "FX-FXK-006": ("A blur on a drawing's own layer, keyed from no blur to 2 pixels.",
                   {"layers": [{**DOT, "effects": [("blur", keyed((0, 0), (4, 2)))]}]},
                   [0, 2]),
    "FX-FXK-007": ("A blur eased from 1 pixel to none on the overshooting curve: a blur of "
                   "less than nothing is no blur.",
                   {"layers": [DOT, adjustment(("blur", keyed((0, 1, OVERSHOOT), (4, 0))))]},
                   [2]),
    "FX-FXK-008": ("Two settings of one effect keyed at once, and a second effect left plain.",
                   {"layers": [BG, adjustment(("tint", keyed((0, (0, 0, 1)), (4, (0, 1, 0))),
                                               keyed((0, 0), (4, 1))),
                                              ("exposure", 1))]},
                   [2]),
}

# FX-FXK-009: a tint amount keyed to 1.5 is outside its range. D-46: the file is read, the
# effect is kept as written and bypassed on every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {"layers": [BG, adjustment(("tint", (0, 0, 1), keyed((0, 0), (4, 1.5))))]}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("bg", "half", "dot"):
        (OUT / "media" / f"{name}.png").write_bytes(adj.png(adj.DRAWINGS[name]))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, case, frames) in CASES.items():
        rendered = {str(f): render(case, f) for f in frames}
        project = f"{fx.lower().replace('-', '_')}.json"
        (OUT / project).write_text(json.dumps(project_json(fx, case), indent=2) + "\n",
                                   encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": project, "frames": rendered}

        print(f"{fx}: {says}\n")
        print("| frame, row | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
        print("| --- " * (W + 1) + "|")
        for f, px in rendered.items():
            for y in range(H):
                cells = (" ".join(fmt(v) for v in px[y * W + x]) for x in range(W))
                print(f"| {f}, {y} | " + " | ".join(cells) + " |")
        print()

    says = ("A tint amount keyed to 1.5, outside 0 to 1: the file is read, the effect is kept as "
            "written and left out of every frame, with a warning.")
    below = adj.render({"layers": [BG]})
    (OUT / "fx_fxk_009.json").write_text(json.dumps(project_json("FX-FXK-009", INVALID), indent=2)
                                         + "\n", encoding="utf-8")
    expected["cases"]["FX-FXK-009"] = {"says": says, "project": "fx_fxk_009.json",
                                       "frames": {"0": below, "4": below},
                                       "warning": "EFFECT_PARAMETER_INVALID"}
    print(f"FX-FXK-009: {says}\n")

    (OUT / "expected_fxkey.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v.get("frames") for fx, v in expected["cases"].items()}
    bg = adj.render({"layers": [BG]})
    stops = lambda s: adj.exposure(bg, s)  # noqa: E731
    assert c["FX-FXK-001"] == {"0": bg, "1": bg, "2": stops(1), "3": stops(2), "4": stops(2)}
    assert c["FX-FXK-002"] == {"1": bg, "2": stops(1)}
    three = [p[0][0] for p in c["FX-FXK-003"].values()]  # the red pixel, 2^stops
    assert 1 < three[0] < 2 ** 0.5 and three[1] == 2 and 2 ** 1.5 < three[2] < 4, three
    assert c["FX-FXK-004"]["2"] == adj.tint(decoded("half"), (0.5, 0, 0.5), 1.0)
    assert ease(OVERSHOOT, 0.5) > 1, "the curve does not overshoot at the middle"
    assert c["FX-FXK-005"]["2"] == adj.tint(bg, (0, 0, 1), 1.0)
    assert c["FX-FXK-005"]["1"] != c["FX-FXK-005"]["2"]
    dot = decoded("dot")
    assert c["FX-FXK-006"] == {"0": dot, "2": adj.blur(dot, 1.0)}
    assert c["FX-FXK-007"]["2"] == dot
    assert c["FX-FXK-008"]["2"] == adj.exposure(adj.tint(bg, (0, 0.5, 0.5), 0.5), 1)


if __name__ == "__main__":
    main()
