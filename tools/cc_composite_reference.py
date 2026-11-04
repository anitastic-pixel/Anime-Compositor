"""CC Composite, worked a second way.

D-1049 adds `core.cc_composite`, after After Effects' CC Composite (Channel): the layer's own
pixels, as they were before its first effect, laid back on what the effects above it made. Lane
w12g's long-shadow tutorial (wWtZXibe2QI at 1:55) uses it to put the logo back on top of the
shadow its blur, Matte Choker and Fill made. It is this program's own method, modelled on After
Effects' CC Composite; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. `opacity`, 0..100 and keyable, held inside 0..100 at each frame; `composite_original`,
"in_front", "behind", "add", "multiply" or "screen"; `rgb_only`, "off" or "on". Starting at 100,
"in_front", "off". With o = opacity / 100, the original g (the layer's premultiplied pixels before
its first effect, masks applied) and the effects' result r, each pixel becomes document 21's layer
blend (Solid Composite's, tools/solid_composite_reference.py): in_front, o g laid normal over r;
behind, r laid normal over o g; add, multiply and screen, o g laid by that mode on r. With
rgb_only "on" the alpha is r's and the colour the straight colour of that blend times r's alpha
(transparent where the blend has none). The effect does not grow the layer.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Channel Mixer's swatches (tools/channel_mixer_reference.py). Where a case says so, a
Solid Composite (D-312) comes first in the stack, so there is something for the original to be
laid on. The drawing goes into `Fixtures/cc_composite/media`, the projects into
`Fixtures/cc_composite`, and the expected frames into
`Fixtures/cc_composite/expected_cc_composite.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_composite_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import channel_mixer_reference as CM  # noqa: E402
import solid_composite_reference as SC  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_composite"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("opacity", "composite_original", "rgb_only")
START = {"opacity": 100, "composite_original": "in_front", "rgb_only": "off"}
SOLID_NAMES = ("source_opacity", "color", "opacity", "blend")


# --- the rule -------------------------------------------------------------------------------

def cc_composite(original, result, opacity, mode, rgb_only):
    """Both lists premultiplied working pixels, the original first."""
    o = opacity / 100
    out = []
    for g, r in zip(original, result):
        g = [v * o for v in g]
        if mode == "in_front":
            p = SC.blend("normal", g, r)
        elif mode == "behind":
            p = SC.blend("normal", r, g)
        else:
            p = SC.blend(mode, g, r)
        if rgb_only == "on":
            p = [v * r[3] for v in SC.straight(p)] + [r[3]] if p[3] != 0 else [0.0] * 4
        out.append(p)
    return out


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = CM.DRAWINGS  # the swatches


def case(shift=0, solid=None, **settings):
    """`solid`: the Solid Composite before it, its settings over Solid Composite's start."""
    return {"drawing": "swatches", "shift": shift,
            "solid": None if solid is None else {**SC.START, **solid}, **START, **settings}


def held(c, k, frame_no):
    return min(100, max(0, value_at(c[k], frame_no)))


def stack(c, frame_no=0):
    """The original and what the Solid Composite (if any) made of it."""
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    original = [R.working(p) for p in pixels]
    s = c["solid"]
    result = original if s is None else SC.solid_composite(
        pixels, s["source_opacity"], s["color"], s["opacity"], s["blend"])
    return original, result


def render(c, frame_no=0):
    original, result = stack(c, frame_no)
    return R.frame(cc_composite(original, result, held(c, "opacity", frame_no),
                                c["composite_original"], c["rgb_only"]), c["shift"])


def before_it(c):
    """The frame without the CC Composite: the drawing, or what the Solid Composite made."""
    return R.frame(stack(c)[1], c["shift"])


HALF_BLUE = {"source_opacity": 0, "color": "#3366cc", "opacity": 50}
BLUE = {"source_opacity": 0, "color": "#3366cc"}

CASES = {
    "FX-CCCOMP-001": ("The settings as they start, opacity 100, in front, RGB only off, as the "
                      "only effect: the drawing laid over itself: every solid pixel and the "
                      "empty border as they were, a soft edge thicker (covering a becomes "
                      "a + a(1 - a)).",
                      case(), [0]),
    "FX-CCCOMP-002": ("After a Solid Composite that leaves only a half-covering blue (#3366cc at "
                      "opacity 50, source opacity 0): in front, the drawing laid back over the "
                      "blue: every solid pixel the drawing's, the border the half blue.",
                      case(solid=HALF_BLUE), [0]),
    "FX-CCCOMP-003": ("The same, behind: the half blue over the drawing: every solid pixel half "
                      "blue, the border the half blue.",
                      case(solid=HALF_BLUE, composite_original="behind"), [0]),
    "FX-CCCOMP-004": ("FX-CCCOMP-002 at opacity 50: the drawing at half strength over the half "
                      "blue.",
                      case(solid=HALF_BLUE, opacity=50), [0]),
    "FX-CCCOMP-005": ("FX-CCCOMP-002 at opacity 0: nothing laid back: the half blue alone, as "
                      "the Solid Composite left it.",
                      case(solid=HALF_BLUE, opacity=0), [0]),
    "FX-CCCOMP-006": ("After a solid blue (#3366cc, source opacity 0), add: every solid pixel "
                      "the drawing plus the blue, in linear light, held at 1; the border the "
                      "blue.",
                      case(solid=BLUE, composite_original="add"), [0]),
    "FX-CCCOMP-007": ("After the solid blue, multiply: every solid pixel the drawing times the "
                      "blue; the border the blue.",
                      case(solid=BLUE, composite_original="multiply"), [0]),
    "FX-CCCOMP-008": ("After the solid blue, screen: every solid pixel lighter than both; the "
                      "border the blue.",
                      case(solid=BLUE, composite_original="screen"), [0]),
    "FX-CCCOMP-009": ("FX-CCCOMP-002 with RGB only on: the colours of FX-CCCOMP-002 but the "
                      "Solid Composite's alpha, half covering everywhere.",
                      case(solid=HALF_BLUE, rgb_only="on"), [0]),
    "FX-CCCOMP-010": ("FX-CCCOMP-001 with RGB only on: the alpha the drawing's own, so a soft "
                      "edge keeps its covering; every pixel the drawing's.",
                      case(rgb_only="on"), [0]),
    "FX-CCCOMP-011": ("FX-CCCOMP-002 with opacity keyed from 0 at frame 0 to 100 at frame 4, "
                      "linear: frame 0 FX-CCCOMP-005, frame 2 FX-CCCOMP-004, frame 4 "
                      "FX-CCCOMP-002.",
                      case(solid=HALF_BLUE, opacity=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-CCCOMP-012": ("FX-CCCOMP-002 moved three pixels right: the same, moved; columns 0 to 2 "
                      "of the frame stay empty.",
                      case(shift=3, solid=HALF_BLUE), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CCCOMP-013": ("Opacity 101: above 100.", case(opacity=101)),
    "FX-CCCOMP-014": ("Composite original \"stencil\": this program's CC Composite lays the "
                      "original in front, behind, or by add, multiply or screen.",
                      case(composite_original="stencil")),
    "FX-CCCOMP-015": ("RGB only \"yes\": it is \"off\" or \"on\".", case(rgb_only="yes")),
    "FX-CCCOMP-016": ("Opacity keyed to 150 at frame 4.",
                      case(opacity=keyed((0, 100), (4, 150)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = SC.prop([W / 2, H / 2])
    t["position"] = SC.prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["solid"] is not None:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.solid_composite",
                        "enabled": True,
                        "parameters": {k: setting_json(c["solid"][k]) for k in SOLID_NAMES}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.cc_composite",
                    "enabled": True, "parameters": {k: setting_json(c[k]) for k in NAMES}})
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
        before = before_it(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = before_it(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_cc_composite.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    f = {fx: v["frames"] for fx, v in expected["cases"].items()}
    c = {fx: v["0"] for fx, v in f.items()}
    drawn = before_it(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda g, h: all(near(p, q) for p, q in zip(g, h))  # noqa: E731
    lin = lambda a, *e8: [SC.srgb_to_linear(v / 255) * a for v in e8] + [a]  # noqa: E731
    line, skin, red, white, grey, black = (2, 4), (3, 4), (5, 4), (11, 4), (12, 4), (13, 4)
    solid = (line, skin, red, (7, 4), (9, 4), white, grey, black)
    soft = (1, 2)
    empty = [(0, 4), (15, 4), (4, 0), (4, 9)]
    blue = (0x33, 0x66, 0xcc)
    half_blue = lin(0.5, *blue)

    # Every case keeps colour inside its covering.
    for frames in f.values():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                      for v in p[:3])

    one = c["FX-CCCOMP-001"]
    s = drawn[at(*soft)]
    assert 0 < s[3] < 1
    assert all(near(one[at(*xy)], drawn[at(*xy)]) for xy in solid + tuple(empty))
    assert abs(one[at(*soft)][3] - (s[3] + s[3] * (1 - s[3]))) < 1e-12
    two = c["FX-CCCOMP-002"]
    assert all(near(two[at(*xy)], drawn[at(*xy)]) for xy in solid)
    assert all(near(two[at(*xy)], half_blue) for xy in empty)
    three = c["FX-CCCOMP-003"]
    assert all(near(three[at(*xy)], [half_blue[k] + 0.5 * drawn[at(*xy)][k] for k in range(4)])
               for xy in solid)
    assert all(near(three[at(*xy)], half_blue) for xy in empty)
    four = c["FX-CCCOMP-004"]
    assert near(four[at(*white)], [0.5 + 0.5 * half_blue[k] for k in range(3)] + [0.75])
    assert like(c["FX-CCCOMP-005"], before_it(case(solid=HALF_BLUE)))
    b = lin(1, *blue)
    six, seven, eight = c["FX-CCCOMP-006"], c["FX-CCCOMP-007"], c["FX-CCCOMP-008"]
    for xy in solid:
        d = drawn[at(*xy)]
        assert near(six[at(*xy)], [min(1, d[k] + b[k]) for k in range(3)] + [1])
        assert near(seven[at(*xy)], [d[k] * b[k] for k in range(3)] + [1])
        assert all(eight[at(*xy)][k] >= max(d[k], b[k]) - 1e-12 for k in range(3))
    for g in (six, seven, eight):
        assert all(near(g[at(*xy)], b) for xy in empty)
    nine = c["FX-CCCOMP-009"]
    assert all(abs(p[3] - 0.5) < 1e-12 for p in nine)
    assert near(nine[at(*white)], [0.5, 0.5, 0.5, 0.5])
    ten = c["FX-CCCOMP-010"]
    assert like(ten, drawn)
    eleven = f["FX-CCCOMP-011"]
    assert like(eleven["0"], c["FX-CCCOMP-005"]) and like(eleven["2"], four)
    assert like(eleven["4"], two)
    moved = f["FX-CCCOMP-012"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
        assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3))
    for fx in INVALID:
        assert c[fx] == drawn
    print("checked")


if __name__ == "__main__":
    main()
