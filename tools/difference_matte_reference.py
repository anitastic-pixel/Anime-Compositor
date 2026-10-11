"""Difference Matte, worked a second way.

D-501 adds Difference Matte (`core.difference_matte`), after After Effects' Difference Matte
(Keying, EFFECTS.md row 304): a layer's pixels are compared with a clean plate, another layer of
the composition named by D-189's layer setting (`tools/effect_layer_reference.py`), and where
they match the pixel is keyed out. Adobe's help describes the settings (View, Difference Layer,
If Layer Sizes Differ, Matching Tolerance, Matching Softness, Blur Before Difference) but
publishes no formula and no defaults we could read, so the rule below and the settings when added
are ours, the "rule ours" precedent.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy) however far earlier effects grew it:

1. With no layer named (""), or a layer named that is not in the composition (D-189's warning),
   the output is O.
2. Otherwise the layer's map P is D-189's, fitted to the drawing's own W by H by `fit`.
3. With `blur_before_difference` sigma above 0, O and P are each blurred by document 21's
   Gaussian at sigma (transparent outside, the kernel cut at ceil(3 sigma)); call them O' and P'.
   At 0 they are O and P.
4. At pixel (x, y) of O, S = O'(x, y) and M = P'(x - ox, y - oy), transparent outside. Each is
   taken to straight encoded colour: c = sRGB(clamp(rgb / a, 0, 1)) when a > 0, else 0, and the
   alpha a as it is.
5. d = 100 max(|dr|, |dg|, |db|, |da|), the largest difference in per cent.
6. With softness above 0, m = clamp((d - tolerance) / softness, 0, 1); at softness 0, m is 0
   where d <= tolerance and 1 elsewhere. m is how much of the pixel is kept.
7. `view` `final`: the output is O m, every channel. `matte`: the matte as a grey, the colour
   linear(m) O.a in every channel and the alpha O.a.

`view` `final` when added, or `matte`; `layer` D-189's word, "" when added; `fit` `center` when
added or `stretch` (After Effects' "If Layer Sizes Differ"); `matching_tolerance` 0 to 100, 15
when added; `matching_softness` 0 to 100, 0 when added; `blur_before_difference` a Gaussian sigma
in pixels, 0 to 100, 0 when added. The three numbers are keyable.

**This file never runs the build's code path.** It works in double precision from the drawings'
8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/difference_matte/`:
the plates, every one switched off, and on top of them the layer `holder`, showing the drawing
`shot`, with the effect. The expected pixels are in
`Fixtures/difference_matte/expected_difference_matte.json`, with a project whose layer settings
go round in a circle.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/difference_matte_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "difference_matte"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
GREEN = (40, 200, 60, 255)

PLATE = K.DRAWINGS["holder"]  # red, blue and cream squares, the bottom right 4 by 3 clear


def shifted(rows, by):
    """Every channel of every pixel that shows moved by `by(x, y)` levels."""
    return [[p if p[3] == 0 else tuple(min(255, max(0, v + by(x, y))) for v in p[:3]) + (p[3],)
             for x, p in enumerate(row)] for y, row in enumerate(rows)]


DRAWINGS = {
    # The plate with a green subject 4 by 3 at (4, 3) to (7, 5).
    "shot": [[GREEN if 4 <= x <= 7 and 3 <= y <= 5 else PLATE[y][x] for x in range(W)]
             for y in range(H)],
    "plate": PLATE,
    # The plate 10 levels darker in every channel.
    "dim": shifted(PLATE, lambda x, y: -10),
    # The plate 8 levels lighter and darker by turns, a fine grain.
    "grain": shifted(PLATE, lambda x, y: 8 if (x + y) % 2 == 0 else -8),
    # The plate's middle 8 by 6, (4, 2) to (11, 7): centred on 16 by 10 it lies where it was cut.
    "patch": [row[4:12] for row in PLATE[2:8]],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = {"plate": "plate", "dim": "dim", "grain": "grain", "patch": "patch", "late": "plate"}


# --- the rule -------------------------------------------------------------------------------

def straight(p):
    a = p[3]
    if a <= 0:
        return [0.0, 0.0, 0.0, 0.0]
    return [S.linear_to_srgb(min(1.0, max(0.0, p[c] / a))) for c in range(3)] + [min(1.0, a)]


def difference(o, m, sigma):
    """Steps 3 to 5: d at every pixel of o, m already fitted to it."""
    if sigma > 0:
        o, m = K.gaussian(o, sigma, "transparent"), K.gaussian(m, sigma, "transparent")
    out = []
    for y in range(o["h"]):
        for x in range(o["w"]):
            s, q = straight(L.at(o, x, y)), straight(L.at(m, x, y))
            out.append(100 * max(abs(s[c] - q[c]) for c in range(4)))
    return out


def kept(d, tol, soft):
    if soft > 0:
        return min(1.0, max(0.0, (d - tol) / soft))
    return 0.0 if d <= tol else 1.0


def matte(o, m, tol, soft, sigma, view):
    """Steps 1 to 7 on the picture o; m is the map already fitted to it, or None."""
    if m is None:
        return o
    out = []
    for p, d in zip(o["px"], difference(o, m, sigma)):
        k = kept(d, tol, soft)
        out.append([srgb_to_linear(k) * p[3]] * 3 + [p[3]] if view == "matte" else [v * k for v in p])
    return L.pic(o["w"], o["h"], out)


# --- the cases ------------------------------------------------------------------------------

def case(view="final", layer="", fit="center", tol=15, soft=0, blur=0, shift=(0, 0), on="holder"):
    return {"view": view, "layer": layer, "fit": fit, "matching_tolerance": tol,
            "matching_softness": soft, "blur_before_difference": blur, "shift": shift, "on": on}


def the_map(c, frame):
    if c["layer"] == "":
        return None
    if c["layer"] == "holder":
        return decoded("shot")  # steps 1 and 2 only: its own drawing, no effects
    if c["layer"] not in MAPS:
        return None
    p = L.EMPTY if c["layer"] == "late" and frame < 4 else decoded(MAPS[c["layer"]])
    return L.fit(p, c["fit"], W, H)


def render(c, frame):
    """The composition's pixels at frame: the holder with its effect, moved by its shift; or,
    with the effect on an adjustment layer above, the frame below through it."""
    out = matte(decoded("shot"), the_map(c, frame), value_at(c["matching_tolerance"], frame),
                value_at(c["matching_softness"], frame),
                value_at(c["blur_before_difference"], frame), c["view"])
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


CASES = {
    "FX-DMATTE-001": ("As added: no layer named, so nothing changes.", case(), (0,)),
    "FX-DMATTE-002": ("The plate as the difference layer, tolerance 15: every pixel the shot "
                      "shares with the plate keyed out, the green subject kept whole, the clear "
                      "corner clear.", case(layer="plate"), (0,)),
    "FX-DMATTE-003": ("FX-DMATTE-002 as the matte: the subject white, the rest black, the clear "
                      "corner clear.", case("matte", "plate"), (0,)),
    "FX-DMATTE-004": ("A plate 10 levels darker, tolerance 3: every difference is 3.92 per cent, "
                      "above 3, so nothing is keyed and nothing changes.",
                      case(layer="dim", tol=3), (0,)),
    "FX-DMATTE-005": ("The darker plate, tolerance 5: 3.92 is within 5, so the background is "
                      "keyed and only the subject stays.", case(layer="dim", tol=5), (0,)),
    "FX-DMATTE-006": ("The darker plate, tolerance 2, softness 4: the background kept at "
                      "(3.92 - 2) / 4, about 0.48, the subject whole.",
                      case(layer="dim", tol=2, soft=4), (0,)),
    "FX-DMATTE-007": ("A grainy plate, 8 levels lighter and darker by turns, tolerance 2, no "
                      "blur: every difference is 3.14 per cent, so nothing is keyed.",
                      case(layer="grain", tol=2), (0,)),
    "FX-DMATTE-008": ("The grainy plate, tolerance 2, softness 4, blurred at 1 before the "
                      "difference: the blur smooths the grain away, so most of the background "
                      "is keyed, while the subject and its edge stay.",
                      case(layer="grain", tol=2, soft=4, blur=1), (0,)),
    "FX-DMATTE-009": ("The plate, tolerance 5, softness 10, blurred at 1: the subject kept whole "
                      "and a soft halo of the background around it, the rest keyed.",
                      case(layer="plate", tol=5, soft=10, blur=1), (0,)),
    "FX-DMATTE-010": ("The plate's middle 8 by 6 centred: it lies exactly where it was cut, so "
                      "the background inside it is keyed; outside it the plate is clear, which "
                      "differs from everything that shows, so that is kept.",
                      case(layer="patch"), (0,)),
    "FX-DMATTE-011": ("The middle 8 by 6 stretched to 16 by 10: it no longer lies on the shot, "
                      "so far less is keyed.", case(layer="patch", fit="stretch"), (0,)),
    "FX-DMATTE-012": ("The holder names itself, tolerance 0: every pixel matches itself, so "
                      "everything is keyed.", case(layer="holder", tol=0), (0,)),
    "FX-DMATTE-013": ("A layer that is not in the composition, `gone`: nothing changes, and the "
                      "warning every frame.", case(layer="gone"), (0, 4)),
    "FX-DMATTE-014": ("The plate with its in point at frame 4: before it the plate is empty, "
                      "clear, which differs from everything that shows, so at frame 0 nothing "
                      "changes; at frame 4 it is FX-DMATTE-002.", case(layer="late"), (0, 4)),
    "FX-DMATTE-015": ("The darker plate, tolerance keyed from 0 at frame 0 to 10 at frame 4: "
                      "frame 0 unchanged, frames 2 and 4 the background keyed.",
                      case(layer="dim", tol=keyed((0, 0), (4, 10))), (0, 2, 4)),
    "FX-DMATTE-016": ("FX-DMATTE-006 on the holder moved 2 right and 1 down: the same picture "
                      "moved, since the plate lies on the layer.",
                      case(layer="dim", tol=2, soft=4, shift=(2, 1)), (0,)),
    "FX-DMATTE-017": ("The effect on an adjustment layer above the holder, the plate as its "
                      "difference layer: the plate lies on the frame, which here is the "
                      "holder's own rectangle, so this is FX-DMATTE-002.",
                      case(layer="plate", on="adjust"), (0,)),
}

INVALID = {
    "FX-DMATTE-018": ("Tolerance 101, above 100.", case(layer="plate", tol=101)),
    "FX-DMATTE-019": ("Tolerance -1, below 0.", case(layer="plate", tol=-1)),
    "FX-DMATTE-020": ("Softness 101, above 100.", case(layer="plate", soft=101)),
    "FX-DMATTE-021": ("Blur 101, above 100.", case(layer="plate", blur=101)),
    "FX-DMATTE-022": ("Blur -1, below 0.", case(layer="plate", blur=-1)),
    "FX-DMATTE-023": ("A view written \"alpha\".", case("alpha", "plate")),
    "FX-DMATTE-024": ("A fit written \"tile\", which After Effects' sizes do not offer.",
                      case(layer="plate", fit="tile")),
    "FX-DMATTE-025": ("A layer written as the number 3, not a word.", case(layer=3)),
    "FX-DMATTE-026": ("Tolerance keyed to 150 at frame 4.",
                      case(layer="plate", tol=keyed((0, 15), (4, 150)))),
}

KEYS = ("view", "layer", "fit", "matching_tolerance", "matching_softness",
        "blur_before_difference")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.difference_matte", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEYS}}


def map_layers():
    return [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES)
            for n in ("plate", "dim", "grain", "patch")] + [
        L.raster("late", "asset-plate", enabled=False, in_frame=4, out_frame=FRAMES)]


def project(pid, layers):
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    """Layers are drawn first to last, so the holder is written after the plates and an
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


# Layer settings that go round in a circle: a whole project, refused on opening.
LOADS = {
    "cycle_two.json": ("`a` and `b`, each the shot, each with a Difference Matte whose difference "
                       "layer is the other: refused, `EFFECT_LAYER_CYCLE`.",
                       [drawing("a", effect("fx-a-0", case(layer="b"))),
                        drawing("b", effect("fx-b-0", case(layer="a")))]),
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

    (OUT / "expected_difference_matte.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-9):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    clear = [0.0] * 4
    subject = {y * W + x for y in range(3, 6) for x in range(4, 8)}
    shows = [i for i, p in enumerate(drawn) if p[3] > 0]
    assert shows and len(shows) < W * H, "the shot has a clear corner"
    background = [i for i in shows if i not in subject]
    # Soft-edged cases aside, no difference at a pixel that shows lies within 0.01 of the
    # tolerance, so no rounding on the card can turn a keep into a key. The one exception is the
    # holder against itself, where every difference is exactly 0, on the card as here.
    for fx, (_, k, frames) in CASES.items():
        if value_at(k["matching_softness"], 0) == 0:
            for f in frames:
                m = the_map(k, f)
                if m is not None:
                    tol = value_at(k["matching_tolerance"], f)
                    ds = difference(decoded("shot"), m, value_at(k["blur_before_difference"], f))
                    if k["layer"] == "holder":
                        assert all(ds[i] == 0 for i in shows), fx
                    else:
                        assert all(abs(ds[i] - tol) > 0.01 for i in shows), (fx, f)
    for fx in ("FX-DMATTE-001", "FX-DMATTE-004", "FX-DMATTE-007"):
        assert c[fx]["0"] == drawn, fx
    assert c["FX-DMATTE-013"]["0"] == drawn and c["FX-DMATTE-013"]["4"] == drawn
    assert c["FX-DMATTE-014"]["0"] == drawn and c["FX-DMATTE-014"]["4"] == c["FX-DMATTE-002"]["0"]
    # The plate: the subject whole, every other pixel clear.
    keyed_out = [drawn[i] if i in subject else clear for i in range(W * H)]
    for fx in ("FX-DMATTE-002", "FX-DMATTE-005", "FX-DMATTE-017"):
        assert c[fx]["0"] == keyed_out, fx
    assert c["FX-DMATTE-015"]["0"] == drawn
    assert c["FX-DMATTE-015"]["2"] == keyed_out and c["FX-DMATTE-015"]["4"] == keyed_out
    # The matte view: white over the subject, black elsewhere, clear stays clear.
    for i, p in enumerate(c["FX-DMATTE-003"]["0"]):
        assert p == ([1.0] * 4 if i in subject else [0.0, 0.0, 0.0, drawn[i][3]]), i
    # Softness: the background at (3.92 - 2) / 4 of itself.
    share = (100 * 10 / 255 - 2) / 4
    assert abs(share - 0.4804) < 1e-4
    for i in range(W * H):
        want = drawn[i] if i in subject else [v * share for v in drawn[i]]
        assert near([c["FX-DMATTE-006"]["0"][i]], [want], 1e-12), i
    # The grain: without the blur nothing keyed; with it most of the background keyed, and the
    # subject untouched.
    blurred = c["FX-DMATTE-008"]["0"]
    gone = sum(blurred[i][3] == 0 for i in background)
    assert gone > len(background) // 2, gone
    assert all(blurred[i] == drawn[i] for i in subject)
    # The halo: the subject whole, some background pixels part kept, the far corners keyed.
    halo = c["FX-DMATTE-009"]["0"]
    assert all(halo[i] == drawn[i] for i in subject)
    assert any(0 < halo[i][3] < drawn[i][3] for i in background)
    assert halo[0] == clear
    # The centred patch: keyed inside it but for the subject, kept outside it.
    inside = {y * W + x for y in range(2, 8) for x in range(4, 12)}
    assert c["FX-DMATTE-010"]["0"] == [clear if i in inside and i not in subject else drawn[i]
                                       for i in range(W * H)]
    stretched = c["FX-DMATTE-011"]["0"]
    assert sum(p[3] == 0 for p in stretched) < sum(p[3] == 0 for p in c["FX-DMATTE-010"]["0"])
    # Itself: everything keyed.
    assert c["FX-DMATTE-012"]["0"] == [clear] * (W * H)
    # Moved: the same picture, two right and one down.
    moved = c["FX-DMATTE-016"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-DMATTE-006"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    print("checks passed")


if __name__ == "__main__":
    main()
