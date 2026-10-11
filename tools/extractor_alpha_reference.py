"""Pass Extract's alpha channel and UnMult, worked a second way.

D-552 (EFFECTS.md row 378, after fnord's EXtractoR, which ships with After Effects): D-349's named
pass took one channel name or three, and drew an opaque picture. EXtractoR also lets an alpha
channel be picked and has an UnMult checkbox, because most renderers write their channels
premultiplied. fnord's ProEXR manual (version 2.6.6) names the controls (Red, Green, Blue, Alpha
menus; Black Point and White Point "to re-scale the floating point values"; UnMult, "if you are
bringing in premultiplied RGB channels, you probably want to click the UnMult checkbox") but
publishes no formula, so the rule below is ours.

**The names.** Pass Extract's `channel`, with `pass` "named", may now hold four names with commas
between, each exactly as the file writes it: red, green, blue and alpha. One name or three, as
D-349, have no alpha. Any other count, or a name the file lacks, is no pass (nothing changes, and
`EFFECT_CHANNEL_MISSING` every frame). The alpha is read as the other channels are (D-348): the
display window, 0 outside the data window, not-a-number as 0 and an infinity as the largest
single-precision number of its sign, with `MEDIA_EXR_ADJUSTED` each frame that reads one.

**The new setting.** `unmult`, "off" or "on", "off" when added and left out of the file while
"off", so every project saved before reads and draws as it did.

**The rule.** With the pass's alpha A (1 when the names are one or three), a = A held to 0..1.
Where a is 0 the pixel is transparent. Elsewhere each of the three values v is first divided by a
when `unmult` is "on", then worked as D-348 works it (Black Point, White Point, Invert, then
Clamp) into o, and the pixel is (o_r a, o_g a, o_b a, a): After Effects holds its pixels straight,
so the extracted channels are taken as straight colour over the alpha, which this program, holding
its pixels premultiplied, stores multiplied by it. With one name or three, a is 1 and the picture
is D-349's, whatever `unmult` says. Outside the drawing the pixel is transparent, as before.

So with UnMult on and the points as added, a premultiplied render comes back as written (where
its colour is no brighter than its alpha); with UnMult off its soft edges are darkened twice, as
in After Effects.

**This file never runs the build's code path.** It writes the EXR files with OpenEXR's own library
(3.4.15), reads them back with it, and works each expected pixel in double precision. It borrows
`tools/depth_channel_reference.py`'s file writer and reader and its levels, pointed at this
folder.

Every case is a project of one composition 8 by 6 at 24 frames a second, five frames long, in
`Fixtures/extractor_alpha/`; the expected pixels are in
`Fixtures/extractor_alpha/expected_extractor_alpha.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/extractor_alpha_reference.py   (with OpenEXR 3.4.15 and numpy)
"""

import json
import sys
from math import isfinite
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
import depth_channel_reference as D  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "extractor_alpha"
D.OUT = OUT  # the borrowed writer and reader work in this folder
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = D.W, D.H, D.FRAMES, D.FPS
CLEAR = [0.0, 0.0, 0.0, 0.0]
RGBA = "Reflect.R, Reflect.G, Reflect.B, Reflect.A"


# --- the files ------------------------------------------------------------------------------

def reflect():
    """A render's premultiplied layer: straight colour by column and row over an alpha by column,
    0, 1/4, 1/2, 3/4, 1, 1.5 (past 1), -1/4 (below 0) and 0.6; and a mask by row, 0 to 1."""
    y, x = np.mgrid[0:H, 0:W].astype(np.float64)
    a = np.array([0.0, 0.25, 0.5, 0.75, 1.0, 1.5, -0.25, 0.6])[np.newaxis, :].repeat(H, 0)
    s = [0.2 + 0.1 * x, 0.9 - 0.1 * y, np.full((H, W), 0.5)]
    out = {f"Reflect.{k}": (v * a).astype("f4") for k, v in zip("RGB", s)}
    out["Reflect.A"] = a.astype("f4")
    out["Mask.Y"] = (y / 5).astype("f4")
    return out


def make_files():
    D.write("media/render.exr", dict(D.colour(), **reflect()))
    inner = {k: v[1:5, 2:7] for k, v in dict(D.colour(), **reflect()).items()}
    D.write("media/window.exr", inner, data=(2, 1, 6, 4))
    odd = reflect()
    odd["Reflect.A"] = odd["Reflect.A"].copy()
    odd["Reflect.A"][2, 3], odd["Reflect.A"][2, 4], odd["Reflect.A"][2, 5] = np.nan, np.inf, -np.inf
    D.write("media/odd.exr", dict(D.colour(), **odd))


def named(names, text):
    """One name, three, or four (the fourth the alpha), each exactly as written: the three colour
    channels and the alpha's (None), or None."""
    wanted = [t.strip() for t in text.split(",")]
    if len(wanted) not in (1, 3, 4) or any(t not in names for t in wanted):
        return None
    return (wanted * 3 if len(wanted) == 1 else wanted[:3]), (wanted[3] if len(wanted) == 4 else None)


def passes(rel, text):
    """Each pixel's three values and its raw alpha (1 with no alpha named), or None."""
    c = D.read(rel)
    found = named(sorted(c), text)
    if found is None:
        return None
    rgb, alpha = found
    return [[D.finite(c[n][y, x]) for n in rgb] + [D.finite(c[alpha][y, x]) if alpha else 1.0]
            for y in range(H) for x in range(W)]


def non_finite(rel, text):
    c = D.read(rel)
    found = named(sorted(c), text)
    if found is None:
        return 0
    chosen = set(found[0]) | ({found[1]} if found[1] else set())
    return sum(1 for n in chosen for v in c[n].flat if not isfinite(v))


# --- the rule -------------------------------------------------------------------------------

def extract(q, black, white, invert, clamp, unmult):
    a = min(1.0, max(0.0, q[3]))
    if a <= 0:
        return list(CLEAR)
    return [D.extract(v / a if unmult == "on" else v, black, white, invert, clamp) * a
            for v in q[:3]] + [a]


def works(c):
    return c.get("works", True) and passes(c["file"], c["channel"]) is not None


def effected(c, n):
    p = D.picture(c["file"])
    if not works(c):
        return p
    pv = passes(c["file"], c["channel"])
    b, w = value_at(c["black_point"], n), value_at(c["white_point"], n)
    out = L.pic(W, H, [extract(q, b, w, c["invert"], c["clamp"], c.get("unmult", "off"))
                       for q in pv])
    if c["after"] is not None:
        out = L.exposure(out, c["after"])
    return out


def render(c, frame):
    dx, dy = c["move"]
    p = effected(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    return render(dict(c, works=False, after=None), frame)


# --- the cases ------------------------------------------------------------------------------

def case(channel=RGBA, black_point=0, white_point=1, invert="off", clamp="on", file="render.exr",
         move=(0, 0), after=None, **kw):
    c = {"channel": channel, "black_point": black_point, "white_point": white_point,
         "invert": invert, "clamp": clamp, "file": "media/" + file, "move": move, "after": after}
    c.update(kw)  # `unmult` only where a case writes it
    return c


ON = {"unmult": "on"}
CASES = {
    "FX-EXA-001": ("Red, green, blue and alpha named, Reflect.R, .G, .B, .A, UnMult left out (off), "
                   "the points as added: the stored colour taken as straight and laid over its "
                   "alpha, so the soft columns are darkened twice; clear where the alpha is 0 or "
                   "below.", case(), (0,)),
    "FX-EXA-002": ("The same, UnMult on: the render back as it was written, its colour divided by "
                   "its alpha and multiplied again; the column whose alpha is past 1 is held at 1.",
                   case(**ON), (0,)),
    "FX-EXA-003": ("UnMult on, Black 0.2, White 0.8: the straight colour stretched, then laid over "
                   "the alpha.", case(black_point=0.2, white_point=0.8, **ON), (0,)),
    "FX-EXA-004": ("UnMult on, Invert on: the straight colour turned over, the alpha kept.",
                   case(invert="on", **ON), (0,)),
    "FX-EXA-005": ("UnMult on, Black 0.25, White 0.5, Clamp off: values past 1 kept, times the "
                   "alpha.", case(black_point=0.25, white_point=0.5, clamp="off", **ON), (0,)),
    "FX-EXA-006": ("UnMult off, Black 0.25, White 0.5, Clamp off.",
                   case(black_point=0.25, white_point=0.5, clamp="off", unmult="off"), (0,)),
    "FX-EXA-007": ("UnMult on, Black and White both 0.5: a cut on the straight colour.",
                   case(black_point=0.5, white_point=0.5, **ON), (0,)),
    "FX-EXA-008": ("The alpha from another channel, Mask.Y (0 at the top to 1 at the bottom), "
                   "UnMult on: the colour fades in down the picture, the top row clear.",
                   case(channel="Reflect.R, Reflect.G, Reflect.B, Mask.Y", **ON), (0,)),
    "FX-EXA-009": ("One channel three times and an alpha, Reflect.G x3, Reflect.A, UnMult on: a "
                   "grey over the alpha.",
                   case(channel="Reflect.G, Reflect.G, Reflect.G, Reflect.A", **ON), (0,)),
    "FX-EXA-010": ("Three names, UnMult on: no alpha, so the picture is opaque and UnMult changes "
                   "nothing (D-349's picture, as FX-EXA-011).",
                   case(channel="Reflect.R, Reflect.G, Reflect.B", **ON), (0,)),
    "FX-EXA-011": ("Three names, UnMult off: D-349's opaque picture.",
                   case(channel="Reflect.R, Reflect.G, Reflect.B"), (0,)),
    "FX-EXA-012": ("One name, UnMult on: opaque, as UnMult off.", case(channel="Reflect.G", **ON),
                   (0,)),
    "FX-EXA-013": ("The data window only columns 2 to 6 of rows 1 to 4, UnMult on: outside it the "
                   "alpha is 0, clear.", case(file="window.exr", **ON), (0,)),
    "FX-EXA-014": ("Not-a-number and infinities in the alpha, UnMult on: not-a-number (0) and "
                   "minus infinity clear, plus infinity held at 1; the warning every frame.",
                   case(file="odd.exr", **ON), (0,)),
    "FX-EXA-015": ("The layer moved 2 right and 1 down: FX-EXA-002 moved.",
                   case(move=(2, 1), **ON), (0,)),
    "FX-EXA-016": ("White Point keyed from 1 at frame 0 to 0.5 at frame 4, UnMult on: frame 0 is "
                   "FX-EXA-002.", case(white_point=keyed((0, 1), (4, 0.5)), **ON), (0, 4)),
    "FX-EXA-017": ("An Exposure of -1 after it: the colour halved, the alpha kept, the picture "
                   "being premultiplied.", case(after=-1, **ON), (0,)),
    "FX-EXA-018": ("Four names, the alpha one the file lacks (Reflect.Alpha): nothing changes, "
                   "the warning every frame.",
                   case(channel="Reflect.R, Reflect.G, Reflect.B, Reflect.Alpha", **ON), (0,)),
    "FX-EXA-019": ("Five names: nothing changes, the warning every frame.",
                   case(channel=RGBA + ", Mask.Y", **ON), (0,)),
}

INVALID = {
    "FX-EXA-020": ("UnMult written \"yes\".", case(unmult="yes")),
    "FX-EXA-021": ("UnMult written \"On\" (the words are lower case).", case(unmult="On")),
}

SETTINGS = ("black_point", "white_point", "invert", "clamp")


def project_json(fx, c):
    holder = L.raster("scene", "asset-scene", position=c["move"], out_frame=FRAMES)
    parameters = {"pass": "named", **{k: setting_json(c[k]) for k in SETTINGS},
                  "channel": c["channel"]}
    if "unmult" in c:
        parameters["unmult"] = c["unmult"]
    holder["effects"] = [{"instance_id": "fx-1", "type_id": "core.pass_extract", "enabled": True,
                          "parameters": parameters}]
    if c["after"] is not None:
        holder["effects"].append(D.exposure("fx-2", c["after"]))
    comp = {"id": "comp-main", "name": "comp-main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": FPS, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": ["scene"], "layers": [holder]}
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [D.exr("scene", c["file"])], "compositions": [comp]}


def write_project(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def warning_at_frame(c):
    if passes(c["file"], c["channel"]) is None:
        return "EFFECT_CHANNEL_MISSING"
    if non_finite(c["file"], c["channel"]):
        return "MEDIA_EXR_ADJUSTED"
    return None


def main():
    make_files()
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write_project(fx, c), "frames": rendered}
        said = warning_at_frame(c)
        if said:
            entry["frame_warning"] = said
        expected["cases"][fx] = entry
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()) + (f", {said}" if said else ""))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write_project(fx, c),
                                 "frames": {"0": plain(c, 0), "4": plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_extractor_alpha.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    four = {fx: v["frames"].get("4") for fx, v in expected["cases"].items()}
    near = lambda a, b, e=1e-6: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                    for p, q in zip(u, v))
    r = D.read("media/render.exr")
    stored = [[r[f"Reflect.{k}"][i // W, i % W] for k in "RGBA"] for i in range(W * H)]
    scene = plain(case(), 0)
    alpha = [0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 0.0, 0.6]
    for i in range(W * H):
        a = alpha[i % W]
        # 001, 002: the alpha held to 0..1, clear at 0 and below.
        assert abs(c["FX-EXA-001"][i][3] - a) < 1e-6 and abs(c["FX-EXA-002"][i][3] - a) < 1e-6
        if a == 0:
            assert c["FX-EXA-001"][i] == CLEAR == c["FX-EXA-002"][i]
            continue
        # 002: the render as written where its alpha is at most 1; 001: darkened twice.
        if stored[i][3] <= 1:
            assert near([c["FX-EXA-002"][i][:3]], [stored[i][:3]]), i
            assert near([c["FX-EXA-001"][i][:3]], [[v * a for v in stored[i][:3]]]), i
    assert c["FX-EXA-001"][1] != c["FX-EXA-002"][1]
    # 008: the top row clear, the bottom row the render's straight colour, opaque.
    assert all(c["FX-EXA-008"][x] == CLEAR for x in range(W))
    assert all(abs(c["FX-EXA-008"][5 * W + x][3] - 1) < 1e-9 for x in range(W))
    # 009: grey.
    assert all(p[0] == p[1] == p[2] for p in c["FX-EXA-009"])
    # 010, 011, 012: opaque, UnMult changing nothing without an alpha.
    assert c["FX-EXA-010"] == c["FX-EXA-011"] and all(p[3] == 1 for p in c["FX-EXA-010"])
    assert all(p[3] == 1 for p in c["FX-EXA-012"])
    # 013: clear outside the data window.
    for i, p in enumerate(c["FX-EXA-013"]):
        inside = 2 <= i % W <= 6 and 1 <= i // W <= 4
        assert (p == c["FX-EXA-002"][i]) if inside else (p == CLEAR), i
    # 014: not-a-number and minus infinity clear, plus infinity opaque.
    odd = c["FX-EXA-014"]
    assert odd[2 * W + 3] == CLEAR == odd[2 * W + 5] and odd[2 * W + 4][3] == 1.0
    assert all(odd[i] == c["FX-EXA-002"][i] for i in range(W * H) if i // W != 2)
    assert all(c["FX-EXA-015"][(y + 1) * W + x + 2] == c["FX-EXA-002"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert near(c["FX-EXA-016"], c["FX-EXA-002"]) and four["FX-EXA-016"] != c["FX-EXA-016"]
    assert near([[v * (0.5 if k < 3 else 1) for k, v in enumerate(p)] for p in c["FX-EXA-002"]],
                c["FX-EXA-017"])
    for fx in ("FX-EXA-018", "FX-EXA-019"):
        assert c[fx] == scene, fx
    # 005: past 1 kept.
    assert max(p[0] for p in c["FX-EXA-005"]) > 1
    print("checks passed")


if __name__ == "__main__":
    main()
