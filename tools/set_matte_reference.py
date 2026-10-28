"""Set Matte, worked a second way.

D-931 adds Set Matte (`core.set_matte`), after After Effects' Set Matte (Channel, EFFECTS.md
P0-3): a layer's matte, its alpha, is replaced by one channel of another layer of the
composition, named by D-189's layer setting (`tools/effect_layer_reference.py`). Adobe's help
names the settings (Take Matte From Layer, Use For Matte, Invert Matte, Stretch Matte To Fit,
Composite Matte With Original, Premultiply Matte Layer) and what each is for, but publishes no
formula, so the rule below is ours where the help is silent.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. With no layer named (""), or a layer named that is not in the composition (D-189's warning),
   the output is O.
2. Otherwise the layer's map P is D-189's, fitted to the drawing's own W by H: stretched with
   `stretch_matte_to_fit` on, centred with it off.
3. At pixel (x, y) of O, M = P(x - ox, y - oy), transparent outside. Its alpha is a and its
   straight encoded colour e = sRGB(clamp(rgb / a, 0, 1)) when a > 0, else 0. With
   `premultiply_matte_layer` on, e is taken over black first: e a.
4. m is what `use_for_matte` reads from e and a, Shift Channels' reading (D-142): red, green,
   blue or alpha as they are; luminance 0.2126 r + 0.7152 g + 0.0722 b; hue the HSV hue over
   360; lightness (max + min) / 2; saturation (max - min) / (1 - |2 lightness - 1|), 0 for a
   grey. m is clamped to 0 to 1, and with `invert_matte` on it is 1 - m.
5. The new alpha A is O.a m with `composite_matte_with_original` on, else m. The output is O's
   straight colour (rgb / O.a, 0 where O.a is 0) times A, and A.

`layer` D-189's word, "" when added; `use_for_matte` one of alpha (when added), red, green,
blue, luminance, hue, lightness, saturation; `invert_matte` off when added;
`stretch_matte_to_fit`, `composite_matte_with_original` and `premultiply_matte_layer` on when
added, After Effects' own. None is keyable.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/set_matte/`: the
matte layers, every one switched off, and on top of them the layer `holder`, showing the drawing
`shot`, with the effect. The expected pixels are in `Fixtures/set_matte/expected_set_matte.json`,
with a project whose layer settings go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/set_matte_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from shift_channels_reference import read  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "set_matte"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5


def key_pixel(x, y):
    """A colour ramp in every channel, solid left of x = 12, part covering to 14, clear after."""
    a = 255 if x < 12 else 160 if x < 14 else 0
    return (255 - 16 * x, 25 * y, (7 * x * y) % 256, a)


DRAWINGS = {
    "shot": K.DRAWINGS["holder"],  # red, blue and cream squares, the bottom right 4 by 3 clear
    "key": [[key_pixel(x, y) for x in range(W)] for y in range(H)],
    # The key's middle 8 by 6, (4, 2) to (11, 7): centred on 16 by 10 it lies where it was cut.
    "patch": [[key_pixel(x, y) for x in range(4, 12)] for y in range(2, 8)],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = {"key": "key", "patch": "patch", "late": "key"}


# --- the rule -------------------------------------------------------------------------------

def clamp01(v):
    return min(1.0, max(0.0, v))


def matte_value(q, c):
    """Steps 3 and 4 at one map pixel q."""
    a = q[3]
    e = [S.linear_to_srgb(clamp01(q[i] / a)) for i in range(3)] if a > 0 else [0.0] * 3
    if c["premultiply_matte_layer"] == "on":
        e = [v * a for v in e]
    m = clamp01(read(c["use_for_matte"], e, a))
    return 1 - m if c["invert_matte"] == "on" else m


def set_matte(o, m, c):
    """Steps 1 to 5 on the picture o; m is the map already fitted to it, or None."""
    if m is None:
        return o
    out = []
    for p, q in zip(o["px"], m["px"]):
        k = matte_value(q, c)
        a = p[3] * k if c["composite_matte_with_original"] == "on" else k
        out.append([p[i] / p[3] * a if p[3] > 0 else 0.0 for i in range(3)] + [a])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(layer="", use="alpha", invert="off", stretch="on", composite="on", premultiply="on",
         shift=(0, 0), on="holder"):
    return {"layer": layer, "use_for_matte": use, "invert_matte": invert,
            "stretch_matte_to_fit": stretch, "composite_matte_with_original": composite,
            "premultiply_matte_layer": premultiply, "shift": shift, "on": on}


def the_map(c, frame):
    if c["layer"] not in MAPS:
        return None
    p = L.EMPTY if c["layer"] == "late" and frame < 4 else decoded(MAPS[c["layer"]])
    return L.fit(p, "stretch" if c["stretch_matte_to_fit"] == "on" else "center", W, H)


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = set_matte(decoded("shot"), the_map(c, frame), c)
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-SETMATTE-001": ("As added: no layer named, so nothing changes.", case(), (0,)),
    "FX-SETMATTE-002": ("The key's alpha, composited with the original: solid left of x = 12, "
                        "part covering to 14, clear after; the shot's clear corner stays clear.",
                        case("key"), (0,)),
    "FX-SETMATTE-003": ("FX-SETMATTE-002 without compositing: the key's alpha replaces the "
                        "shot's, so the shot's clear corner shows black where the key covers.",
                        case("key", composite="off"), (0,)),
    "FX-SETMATTE-004": ("The key's red, falling left to right.", case("key", "red"), (0,)),
    "FX-SETMATTE-005": ("The key's green, rising top to bottom.", case("key", "green"), (0,)),
    "FX-SETMATTE-006": ("The key's blue.", case("key", "blue"), (0,)),
    "FX-SETMATTE-007": ("The key's luminance.", case("key", "luminance"), (0,)),
    "FX-SETMATTE-008": ("The key's hue.", case("key", "hue"), (0,)),
    "FX-SETMATTE-009": ("The key's lightness.", case("key", "lightness"), (0,)),
    "FX-SETMATTE-010": ("The key's saturation.", case("key", "saturation"), (0,)),
    "FX-SETMATTE-011": ("The key's luminance not premultiplied: the part-covering columns read "
                        "their full colour, so they keep more than in FX-SETMATTE-007.",
                        case("key", "luminance", premultiply="off"), (0,)),
    "FX-SETMATTE-012": ("The key's alpha inverted: only what the key leaves clear is kept.",
                        case("key", invert="on"), (0,)),
    "FX-SETMATTE-013": ("The key's middle 8 by 6, not stretched: centred, it lies where it was "
                        "cut, and outside it everything is clear.",
                        case("patch", stretch="off"), (0,)),
    "FX-SETMATTE-014": ("The middle 8 by 6's red stretched to 16 by 10: nothing the shot covers "
                        "turns clear, and the ramp is spread twice as wide.",
                        case("patch", "red"), (0,)),
    "FX-SETMATTE-015": ("A layer that is not in the composition, `gone`: nothing changes, and "
                        "the warning every frame.", case("gone"), (0, 4)),
    "FX-SETMATTE-016": ("The key with its in point at frame 4: before it the key is empty, so "
                        "at frame 0 everything is clear; at frame 4 it is FX-SETMATTE-002.",
                        case("late"), (0, 4)),
    "FX-SETMATTE-017": ("FX-SETMATTE-004 on the holder moved 2 right and 1 down: the same "
                        "picture moved, since the key lies on the layer.",
                        case("key", "red", shift=(2, 1)), (0,)),
    "FX-SETMATTE-018": ("The effect on an adjustment layer above the holder, the key's alpha: "
                        "the frame below is the holder's own rectangle, so this is "
                        "FX-SETMATTE-002.", case("key", on="adjust"), (0,)),
}

INVALID = {
    "FX-SETMATTE-019": ("A matte channel written \"chroma\".", case("key", "chroma")),
    "FX-SETMATTE-020": ("Invert written \"yes\".", case("key", invert="yes")),
    "FX-SETMATTE-021": ("Stretch written \"tile\".", case("key", stretch="tile")),
    "FX-SETMATTE-022": ("Composite written \"1\".", case("key", composite="1")),
    "FX-SETMATTE-023": ("Premultiply written \"maybe\".", case("key", premultiply="maybe")),
    "FX-SETMATTE-024": ("A layer written as the number 3, not a word.", case(3)),
}

KEYS = ("layer", "use_for_matte", "invert_matte", "stretch_matte_to_fit",
        "composite_matte_with_original", "premultiply_matte_layer")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.set_matte", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES) for n in ("key", "patch")] + [
        L.raster("late", "asset-key", enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    """Layers are drawn first to last, so the holder is written after the mattes and an
    adjustment layer after the holder."""
    holder = L.raster("holder", "asset-shot", position=c["shift"], out_frame=FRAMES)
    layers = map_layers() + [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = [effect("fx-1", c)]
    return project("proj-" + fx.lower(), layers)


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def drawing(id, fx):
    lay = L.raster(id, "asset-shot", out_frame=FRAMES)
    lay["effects"] = [fx]
    return lay


LOADS = {
    "cycle_two.json": ("`a` and `b`, each the shot, each with a Set Matte whose matte layer is "
                       "the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case("b"))),
                        drawing("b", effect("fx-b-0", case("a")))]),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "loads": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if c["layer"] == "gone":
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
    for name, (says, layers) in LOADS.items():
        write(name, project("proj-" + name[:-5].replace("_", "-"), map_layers() + layers))
        expected["loads"][name] = {"says": says, "refused": "EFFECT_LAYER_CYCLE"}

    (OUT / "expected_set_matte.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    clear = [0.0] * 4
    key = decoded("key")
    ka = [L.at(key, i % W, i // W)[3] for i in range(W * H)]
    assert c["FX-SETMATTE-001"]["0"] == drawn
    assert c["FX-SETMATTE-015"]["0"] == drawn and c["FX-SETMATTE-015"]["4"] == drawn
    # The alpha: the shot times the key's covering.
    assert near(c["FX-SETMATTE-002"]["0"], [[v * ka[i] for v in drawn[i]] for i in range(W * H)])
    assert c["FX-SETMATTE-018"]["0"] == c["FX-SETMATTE-002"]["0"]
    assert c["FX-SETMATTE-016"]["0"] == [clear] * (W * H)
    assert c["FX-SETMATTE-016"]["4"] == c["FX-SETMATTE-002"]["0"]
    # Inverted: the shot times what the key leaves.
    assert near(c["FX-SETMATTE-012"]["0"], [[v * (1 - ka[i]) for v in drawn[i]]
                                            for i in range(W * H)])
    # Not composited: every alpha is the key's, black where the shot was clear.
    for i, p in enumerate(c["FX-SETMATTE-003"]["0"]):
        assert abs(p[3] - ka[i]) < 1e-12, i
        if drawn[i][3] == 0:
            assert p[:3] == [0.0, 0.0, 0.0]
    assert any(drawn[i][3] == 0 and ka[i] > 0 for i in range(W * H)), "a clear pixel turns black"
    # Premultiplied or not: the same where the key is solid, more kept where it part covers.
    pre, raw = c["FX-SETMATTE-007"]["0"], c["FX-SETMATTE-011"]["0"]
    for i in range(W * H):
        if ka[i] == 1:
            assert near([pre[i]], [raw[i]]), i
        elif 0 < ka[i] < 1 and drawn[i][3] > 0:
            assert raw[i][3] > pre[i][3], i
    # Each colour channel gives a different matte.
    words = ["004", "005", "006", "007", "008", "009", "010"]
    assert len({json.dumps(c["FX-SETMATTE-" + n]["0"]) for n in words}) == len(words)
    # Red falls left to right along a row the shot covers.
    red = c["FX-SETMATTE-004"]["0"]
    row = [red[W + x][3] / drawn[W + x][3] for x in range(12) if drawn[W + x][3] > 0]
    assert all(a > b for a, b in zip(row, row[1:])), row
    # The centred patch: clear outside, the key inside.
    inside = {y * W + x for y in range(2, 8) for x in range(4, 12)}
    for i, p in enumerate(c["FX-SETMATTE-013"]["0"]):
        assert (p == clear) if i not in inside else near([p], [c["FX-SETMATTE-002"]["0"][i]]), i
    stretched = c["FX-SETMATTE-014"]["0"]
    assert all(stretched[i][3] > 0 for i in range(W * H) if drawn[i][3] > 0)
    # Moved: the same picture, two right and one down.
    moved = c["FX-SETMATTE-017"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == red[y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    print("checks passed")


if __name__ == "__main__":
    main()
