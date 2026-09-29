"""Mix on every effect, worked a second way.

D-202 gives every effect a Mix, After Effects' "Compositing Options: Effect Opacity": 0 to 100
per cent (100), keyable, how much of the effect's result is kept. It is written on the effect's
record beside `enabled`, not in its parameters, as `"mix"`, a plain number or document 19's
property record; a file with none, every file written before D-202, reads as 100. It is this
program's own rule, modelled on After Effects' Effect Opacity; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. m = mix / 100, the mix at the frame held inside 0 to 100. Where an effect in the stack
takes the picture `before` and gives `after`, the next effect takes

    out = before + m (after - before)

at every sample of `after`, all four premultiplied channels, where `before` is placed where it
was: when the effect grew the layer by (dx, dy) on each side, `before`'s pixel (x, y) is `after`'s
(x + dx, y + dy), and `before` is transparent outside its own rectangle. At 100 the effect's
result is taken as it is, exactly; at 0 its input, exactly. A Light Wrap mixes the placed layer
the same way, before and after its light is laid on (D-132); on an adjustment layer the stack
runs on the frame beneath, so the mix is its adjustment's. Posterize Time holds a whole layer in
time and has no picture to mix: its mix must be 100.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, where the build works in single precision on its buffers.

The cases use three other effects' own fixtures: Invert's bands (`tools/invert_reference.py`),
16 pixels by 10, for Invert, Gaussian Blur, Exposure and an adjustment layer; Light Wrap's box
over bright bands (`tools/light_wrap_reference.py`); and Posterize Time's running ball
(`tools/posterize_time_reference.py`). The drawings go into `Fixtures/effect_mix/media`, the
projects into `Fixtures/effect_mix`, and the expected frames into
`Fixtures/effect_mix/expected_effect_mix.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/effect_mix_reference.py
"""

import copy
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as A  # noqa: E402
from adjust_reference import over, exposure, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from edges_reference import gaussian  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import invert_reference as I  # noqa: E402
import light_wrap_reference as LW  # noqa: E402
import posterize_time_reference as P  # noqa: E402

W, H = R.W, R.H
FRAMES = S.FRAMES
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "effect_mix"
TOLERANCE = 2e-5  # document 25's default for a filter
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def mixed(before, after, mix):
    """`after` with `before` mixed back in, each a layer {"px", "left", "top", "w", "h"} in the
    layer's own space; `after` holds `before`'s rectangle, as an effect only grows outward."""
    m = mix / 100
    if m == 1:
        return after
    out = []
    for y in range(after["h"]):
        for x in range(after["w"]):
            bx, by = x + after["left"] - before["left"], y + after["top"] - before["top"]
            inside = 0 <= bx < before["w"] and 0 <= by < before["h"]
            b = before["px"][by * before["w"] + bx] if inside else EMPTY
            a = after["px"][y * after["w"] + x]
            out.append([b[c] + m * (a[c] - b[c]) for c in range(4)])
    return dict(after, px=out)


# --- the cases ------------------------------------------------------------------------------

def fx(kind, mix=None, enabled=True, **settings):
    """One effect: kind "invert", "blur", "exposure", "wrap" or "posterize"; mix None when the
    file does not say."""
    return {"kind": kind, "mix": mix, "enabled": enabled, **settings}


def invert(mix=None, enabled=True):
    return fx("invert", mix, enabled, channel="rgb", amount=100)


def case(*effects, adjust=False):
    """The bands drawing with the effects on it, or, with `adjust`, on an adjustment layer above
    it."""
    return {"scene": "bands", "effects": list(effects), "adjust": adjust}


def held(v):
    return min(100.0, max(0.0, v))


def run(layer, effects, frame_no):
    """The layer's stack at a frame, each switched-on effect mixed by its own mix."""
    for e in effects:
        if not e["enabled"]:
            continue
        if e["kind"] == "invert":
            after = dict(layer, px=negative(layer["px"], e["amount"]))
        elif e["kind"] == "blur":
            after = gaussian(layer, e["sigma"], "transparent")
        else:
            after = dict(layer, px=exposure(layer["px"], e["stops"]))
        mix = 100 if e["mix"] is None else held(value_at(e["mix"], frame_no))
        layer = mixed(layer, after, mix)
    return layer


def negative(px, amount):
    """Invert, channel rgb, on working values: `invert_reference.invert`'s rule, which starts from
    8-bit pixels, the same numbers."""
    t = amount / 100
    out = []
    for w in px:
        a = w[3]
        w = list(w)
        if a > 0 and t > 0:
            for c in range(3):
                e = S.linear_to_srgb(min(1.0, max(0.0, w[c] / a)))
                w[c] = A.srgb_to_linear(min(1.0, max(0.0, e + t * (1 - 2 * e)))) * a
        out.append(w)
    return out


BANDS_PX = [R.working(p) for row in I.DRAWINGS["bands"] for p in row]


def placed(layer):
    """The layer on the W by H frame, unmoved."""
    return [layer["px"][(y - layer["top"]) * layer["w"] + x - layer["left"]]
            if 0 <= x - layer["left"] < layer["w"] and 0 <= y - layer["top"] < layer["h"]
            else list(EMPTY) for y in range(H) for x in range(W)]


def render(c, frame_no):
    if c["scene"] == "wrap":
        return wrapped(c, frame_no)
    drawing = {"px": BANDS_PX, "left": 0, "top": 0, "w": W, "h": H}
    if c["adjust"]:
        # D-65: the stack runs on the frame beneath, which is the drawing alone, and the
        # adjustment layer covers the whole frame at full opacity.
        return placed(run(dict(drawing, px=placed(drawing)), c["effects"], frame_no))
    return placed(run(drawing, c["effects"], frame_no))


def plain(c):
    if c["scene"] == "wrap":
        return wrapped(dict(c, effects=[]), 0)
    return placed({"px": BANDS_PX, "left": 0, "top": 0, "w": W, "h": H})


# Light Wrap's own scene: its bright bands behind, its box in front with the Light Wrap.

def wrap_case(mix):
    return {"scene": "wrap", "effects": [fx("wrap", mix, width=10, intensity=100)],
            "adjust": False}


def wrapped(c, frame_no):
    bands = [R.working(p) for row in LW.DRAWINGS["bands"] for p in row]
    L = [R.working(p) for row in LW.DRAWINGS["box"] for p in row]
    for e in c["effects"]:
        lit = LW.light_wrap(L, bands, e["width"], e["intensity"], "screen")
        m = held(value_at(e["mix"], frame_no)) / 100 if e["mix"] is not None else 1
        L = lit if m == 1 else [[p[k] + m * (q[k] - p[k]) for k in range(4)]
                                for p, q in zip(L, lit)]
    return [over(p, b) for p, b in zip(L, bands)]


# Posterize Time's own scene, for the one effect whose mix must be 100.

PTIME = P.case()


CASES = {
    "FX-MIX-001": ("Invert, channel rgb, amount 100, at mix 50: every sample of the drawing half "
                   "way between its own value and its negative's, in linear light, so black and "
                   "white both land on the same light grey, 0.5 in linear light, #bcbcbc but for "
                   "rounding (187.5), and the skin #f6d6be on #b59f91; each pixel's covering is "
                   "kept. It is not Invert at "
                   "amount 50, FX-INVERT-003, which meets in the encoded middle, #808080.",
                   case(invert(50)), [0]),
    "FX-MIX-002": ("The same Invert with no mix written, as every file before D-202: read as "
                   "100, so the frame is FX-INVERT-001 exactly.",
                   case(invert()), [0]),
    "FX-MIX-003": ("Mix 100 written: the same as no mix, FX-MIX-002.",
                   case(invert(100)), [0]),
    "FX-MIX-004": ("Mix 0: the drawing, exactly, as if the Invert were switched off.",
                   case(invert(0)), [0]),
    "FX-MIX-005": ("Mix 25: a quarter of the way from the drawing to its negative.",
                   case(invert(25)), [0]),
    "FX-MIX-006": ("Gaussian Blur, sigma 1, edges transparent, at mix 50: the layer grows by 3 "
                   "pixels on each side, and the sharp drawing is laid back in where it was, not "
                   "at the grown layer's corner: each pixel half the blurred drawing and half the "
                   "sharp one, so the empty rows and column round the drawing take half the "
                   "blur's spill.",
                   case(fx("blur", 50, sigma=1)), [0]),
    "FX-MIX-007": ("Mix keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the "
                   "drawing, frame 2 FX-MIX-001 and frame 4 FX-MIX-002.",
                   case(invert(keyed((0, 0), (4, 100)))), [0, 2, 4]),
    "FX-MIX-008": ("Mix eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: "
                   "at frame 2 it would pass 100, is held at 100, and is FX-MIX-002, as is frame "
                   "4; frame 0 is the drawing.",
                   case(invert(keyed((0, 0, OVERSHOOT), (4, 100)))), [0, 2, 4]),
    "FX-MIX-009": ("Invert at mix 50, then Exposure -1 at no mix: the Exposure takes the mixed "
                   "picture, so the frame is FX-MIX-001 at half its light.",
                   case(invert(50), fx("exposure", stops=-1)), [0]),
    "FX-MIX-010": ("Exposure -1, then Exposure +1 at mix 50: the second is mixed with what it "
                   "was given, the darkened drawing, not with the drawing, so each colour is "
                   "three quarters of the drawing's, not all of it.",
                   case(fx("exposure", stops=-1), fx("exposure", 50, stops=1)), [0]),
    "FX-MIX-011": ("Invert at mix 50, switched off: the drawing.",
                   case(invert(50, enabled=False)), [0]),
    "FX-MIX-012": ("Invert at mix 50 on an adjustment layer above the drawing: the frame "
                   "beneath is the drawing alone, so this is FX-MIX-001.",
                   case(invert(50), adjust=True), [0]),
    "FX-MIX-013": ("Light Wrap, as it starts, at mix 50, on Light Wrap's box over its bright "
                   "bands: the box takes half the light FX-WRAP-001 lays on it, and the bands "
                   "round it are untouched.",
                   wrap_case(50), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-MIX-014": ("Invert at mix 101, above 100.", case(invert(101))),
    "FX-MIX-015": ("Invert at mix -1, below 0.", case(invert(-1))),
    "FX-MIX-016": ("Invert with its mix keyed to 150 at frame 4.",
                   case(invert(keyed((0, 50), (4, 150))))),
}

# Posterize Time with a mix below 100: kept as written and left out, so nothing is held.
PTIME_INVALID = ("FX-MIX-017", "Posterize Time, 12 a second, at mix 50: it holds the layer in "
                 "time and has no picture to mix, so its mix must be 100.")


# --- the project files ----------------------------------------------------------------------

TYPES = {"invert": "core.invert", "blur": "core.gaussian_blur", "exposure": "core.exposure",
         "wrap": "core.light_wrap"}


def effect_json(e, i, n):
    """The record as the build writes it: the mix after `enabled`, and only when written."""
    params = {"invert": lambda: {"channel": e.get("channel"), "amount": e.get("amount")},
              "blur": lambda: {"sigma_px": e.get("sigma"), "edges": "transparent"},
              "exposure": lambda: {"stops": e.get("stops")},
              "wrap": lambda: {"width": e.get("width"), "intensity": e.get("intensity"),
                               "blend": "screen"}}[e["kind"]]()
    rec = {"instance_id": f"fx-{i}-{n}", "type_id": TYPES[e["kind"]], "enabled": e["enabled"]}
    if e["mix"] is not None:
        rec["mix"] = setting_json(e["mix"])
    rec["parameters"] = params
    return rec


def layer_json(l, i):
    kind = l.get("kind", "raster")
    rec = A.layer_json({"id": l["id"], "kind": kind, "drawing": l.get("drawing"),
                        "out": FRAMES}, i)
    t = rec["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2, H / 2])
    rec["effects"] = [effect_json(e, i, n) for n, e in enumerate(l.get("effects", []))]
    return rec


def project_json(fx_id, c):
    if c["scene"] == "wrap":
        layers = [{"id": "bg", "drawing": "wrap_bands"},
                  {"id": "art", "drawing": "wrap_box", "effects": c["effects"]}]
    elif c["adjust"]:
        layers = [{"id": "art", "drawing": "bands"},
                  {"id": "adj", "kind": "adjustment", "effects": c["effects"]}]
    else:
        layers = [{"id": "art", "drawing": "bands", "effects": c["effects"]}]
    p = S.project_json(fx_id, {"drawing": "bands", "shift": 0, "softness": 0, "threshold": 0})
    template = p["assets"][0]
    used = [l["drawing"] for l in layers if "drawing" in l]
    p["assets"] = [dict(template, id="asset-" + d, name=d, path=f"media/{d}.png") for d in used]
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    comp["layer_order"] = [l["id"] for l in layers]  # bottom first, as document 19 draws
    comp["layers"] = [layer_json(l, i) for i, l in enumerate(layers)]
    return p


def ptime_json(fx_id):
    p = P.project_json(fx_id, PTIME)
    p["project_id"] = "proj-" + fx_id.lower()
    holder = p["compositions"][0]["layers"][0]
    e = holder["effects"][0]
    holder["effects"][0] = {"instance_id": e["instance_id"], "type_id": e["type_id"],
                            "enabled": e["enabled"], "mix": 50, "parameters": e["parameters"]}
    return p


def write(fx_id, p):
    name = f"{fx_id.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bands.png").write_bytes(S.png(I.DRAWINGS["bands"]))
    (OUT / "media" / "wrap_bands.png").write_bytes(S.png(LW.DRAWINGS["bands"]))
    (OUT / "media" / "wrap_box.png").write_bytes(S.png(LW.DRAWINGS["box"]))
    for name, pixels in P.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(P.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx_id, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx_id] = {"says": says, "project": write(fx_id, project_json(fx_id, c)),
                                    "frames": rendered}
        before = plain(c)
        print(f"{fx_id}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    tail = (" The file is read, the effect is kept as written and left out of every frame, with "
            "a warning.")
    for fx_id, (says, c) in INVALID.items():
        before = plain(c)
        expected["cases"][fx_id] = {"says": says + tail,
                                    "project": write(fx_id, project_json(fx_id, c)),
                                    "frames": {"0": before, "4": before},
                                    "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx_id}: invalid")
    fx_id, says = PTIME_INVALID
    expected["cases"][fx_id] = {"says": says + tail, "project": write(fx_id, ptime_json(fx_id)),
                                "frames": {"1": P.plain(PTIME, 1), "3": P.plain(PTIME, 3)},
                                "warning": "EFFECT_PARAMETER_INVALID"}
    print(f"{fx_id}: invalid")

    (OUT / "expected_effect_mix.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(json.loads(json.dumps(expected)))


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {k: v["frames"] for k, v in expected["cases"].items()}
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    drawing = plain(case())
    negative_frame = R.frame(I.invert([p for row in I.DRAWINGS["bands"] for p in row], "rgb",
                                      100), 0)

    # The rule's own pieces: 100 is the effect's result, 0 its input, placed where it was.
    one = {"px": [[0.2, 0.4, 0.6, 1.0]], "left": 0, "top": 0, "w": 1, "h": 1}
    two = {"px": [[1.0, 1.0, 1.0, 1.0]] * 9, "left": -1, "top": -1, "w": 3, "h": 3}
    assert mixed(one, two, 100) is two
    assert mixed(one, two, 0)["px"] == [EMPTY] * 4 + [one["px"][0]] + [EMPTY] * 4
    assert near([mixed(one, two, 50)["px"][4]], [[0.6, 0.7, 0.8, 1.0]])
    assert near(negative(BANDS_PX, 100), negative_frame)
    assert negative(BANDS_PX, 0) == BANDS_PX

    # Every case keeps every pixel premultiplied, its covering in 0..1.
    for frames in c.values():
        for f in frames.values():
            assert all(-1e-12 <= p[3] <= 1 + 1e-12 and all(v <= p[3] + 1e-12 for v in p[:3])
                       for p in f)

    half = c["FX-MIX-001"]["0"]
    assert near(half, [[(p[k] + q[k]) / 2 for k in range(4)]
                       for p, q in zip(drawing, negative_frame)])
    assert all(p[3] == q[3] for p, q in zip(half, drawing))
    black, white = half[1 * W + 1], half[1 * W + 13]
    assert abs(black[0] - 0.5) < 1e-12 and abs(white[0] - 0.5) < 1e-12
    amount50 = R.frame(I.invert([p for row in I.DRAWINGS["bands"] for p in row], "rgb", 50), 0)
    assert not near(half, amount50, 1e-3)
    assert c["FX-MIX-002"]["0"] == negative_frame == c["FX-MIX-003"]["0"]
    assert c["FX-MIX-004"]["0"] == drawing
    assert near(c["FX-MIX-005"]["0"], [[p[k] + (q[k] - p[k]) / 4 for k in range(4)]
                                       for p, q in zip(drawing, negative_frame)])
    blurred = placed(gaussian({"px": BANDS_PX, "left": 0, "top": 0, "w": W, "h": H}, 1,
                              "transparent"))
    six = c["FX-MIX-006"]["0"]
    assert near(six, [[(p[k] + q[k]) / 2 for k in range(4)] for p, q in zip(drawing, blurred)])
    assert all(0 < six[x][3] < 0.5 for x in range(1, W))  # row 0, empty, takes half the spill
    seven = c["FX-MIX-007"]
    assert seven["0"] == drawing and near(seven["2"], half) and seven["4"] == negative_frame
    eight = c["FX-MIX-008"]
    assert eight["0"] == drawing and eight["2"] == eight["4"] == negative_frame
    assert near(c["FX-MIX-009"]["0"], exposure(half, -1))
    assert near(c["FX-MIX-010"]["0"], [[v * 0.75 for v in p[:3]] + [p[3]] for p in drawing])
    assert not near(c["FX-MIX-010"]["0"], drawing, 1e-3)
    assert c["FX-MIX-011"]["0"] == drawing
    assert near(c["FX-MIX-012"]["0"], half)
    lit = wrapped(wrap_case(None), 0)
    unlit = wrapped(dict(wrap_case(None), effects=[]), 0)
    thirteen = c["FX-MIX-013"]["0"]
    assert lit == LW.render(LW.std(), 0)
    assert near(thirteen, [[(p[k] + q[k]) / 2 for k in range(4)] for p, q in zip(lit, unlit)])
    assert thirteen != unlit and thirteen != lit
    for k in list(INVALID) + [PTIME_INVALID[0]]:
        assert expected["cases"][k]["warning"] == "EFFECT_PARAMETER_INVALID"
    assert c["FX-MIX-017"]["3"] != P.render(PTIME, 3)  # held, it would show frame 2's drawing
    print("checked")


if __name__ == "__main__":
    main()
