"""D-763: in 8 bpc (After Effects) colours mix in display values, worked a second way.

After Effects in an 8-bit project has no linear working space: it lays one layer on another,
fades a layer by its opacity, cuts it by a mask's feather and averages a Mosaic block on the
display values, the numbers 0 to 255 a picture is saved with. This program mixed them in linear
light in every depth, so in 8 bpc a black Drop Shadow at 50% over white came out light grey
(188) where After Effects shows mid grey (128 less a step), and a Screen came out darker.

From D-763 on, in a composition in 8 bpc (After Effects) (D-330), and in the compositions inside
it, which follow the outermost one's depth:

- each layer's picture is taken to display values (its straight colour, held to 0..1, through
  the sRGB curve, premultiplied), its opacity, matte and mask coverage applied as before, laid on
  the frame in its blend mode on those values, and the frame taken back to linear light. The
  modes that already worked on encoded colours (D-301, D-748) work on them as they are;
- Drop Shadow, Fill and Mosaic work on display values, the blurs, Solid Composite and Glow as
  D-330 and D-337 already had them; Echo mixes its copies in display values;
- Display, Float and 32 bpc (After Effects) are unchanged.

Every case is a composition 6 by 2 (mask_reference's size). Each is written twice: in 8 bpc,
and the same scene in Display, whose frame is the linear-light mix as before D-763. The projects
go into `Fixtures/display_mix`, the expected frames into
`Fixtures/display_mix/expected_display_mix.json`.

**This file never runs the build's code path.** It works in double precision on lists. Where a
layer has an effect, 8 bpc rounds its pixels to 8 bits (D-330), and every value rounded is
checked to lie clear of a rounding step, or exactly on a half that both precisions hold exactly.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/display_mix_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import mask_reference as M  # noqa: E402
import mosaic_reference as MO  # noqa: E402
import smooth_reference as S  # noqa: E402
from adjust_reference import prop, srgb_to_linear  # noqa: E402

W, H = M.W, M.H  # 6 by 2
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "display_mix"
TOLERANCE = 2e-5  # document 25's default for a filter
CLEAR = 1e-3      # how far, in 255ths, every rounded value lies from a rounding step

near = []


def lin(v):
    return srgb_to_linear(min(max(v, 0.0), 1.0))


def enc(v):
    return S.linear_to_srgb(min(max(v, 0.0), 1.0))


def to_display(p):
    """Linear premultiplied to display premultiplied, the straight colour held to 0..1."""
    a = p[3]
    if a <= 0:
        return [0.0] * 4
    return [enc(p[c] / a) * a for c in range(3)] + [a]


def to_linear(p):
    a = p[3]
    if a <= 0:
        return [0.0] * 4
    return [lin(p[c] / a) * a for c in range(3)] + [a]


def q(v):
    """A value 0..1 rounded to a 255th, a half up, as the build's f32 `round` does for these."""
    s = min(max(v, 0.0), 1.0) * 255
    off = abs(s - math.floor(s) - 0.5)
    if off != 0.0:  # an exact half is held exactly by both precisions and rounds up in both
        near.append(off)
    return math.floor(s + 0.5) / 255


def eight(px):
    """D-330: a layer's pixels after an effect, held to 8 bits in display values (linear in and
    out)."""
    out = []
    for p in px:
        a = q(p[3])
        if a <= 0 or p[3] <= 0:
            out.append([0.0] * 4)
            continue
        out.append([lin(q(enc(min(max(p[c] / p[3], 0.0), 1.0)))) * a for c in range(3)] + [a])
    return out


# --- blending, on straight colours 0..1 ----------------------------------------------------

MODES = {
    "normal": None,
    "screen": lambda s, d: s + d - s * d,
    "add": lambda s, d: min(s + d, 1.0),
}


def blend(mode, src, dst):
    """Document 21's layer mix of premultiplied `src` on `dst`, in whatever values they hold."""
    a_s, a_d = src[3], dst[3]
    if MODES[mode] is None:
        return [src[c] + dst[c] * (1 - a_s) for c in range(4)]
    cs = [src[c] / a_s if a_s > 0 else 0.0 for c in range(3)]
    cd = [dst[c] / a_d if a_d > 0 else 0.0 for c in range(3)]
    out = [(1 - a_s) * dst[c] + (1 - a_d) * src[c] + a_s * a_d * MODES[mode](cs[c], cd[c])
           for c in range(3)]
    return out + [a_s + a_d - a_s * a_d]


def composite(layers, display):
    """Layers bottom first, each a list of W*H linear premultiplied pixels (opacity, mask
    already in) and a mode. In 8 bpc they are laid in display values and the frame decoded."""
    frame = [[0.0] * 4 for _ in range(W * H)]
    for px, mode in layers:
        if display:
            px = [to_display(p) for p in px]
        frame = [blend(mode, s, d) for s, d in zip(px, frame)]
    return [to_linear(p) for p in frame] if display else frame


# --- the scenes -----------------------------------------------------------------------------

UNDER = [(0.20, 0.45, 0.80), (0.90, 0.60, 0.15)]  # encoded: columns 0-2, columns 3-5
OVER = [(0.70, 0.25, 0.55), (0.35, 0.80, 0.40)]   # encoded: row 0, row 1
WHITE = (1.0, 1.0, 1.0)
INK = (40 / 255, 90 / 255, 200 / 255)              # Drop Shadow's drawing, whole 255ths
A = (250, 60, 10, 255)                              # Mosaic's two colours, columns turn about
B = (0, 120, 200, 255)


def solid_px(color, left, top, w, h, opacity=1.0, coverage=None):
    out = []
    for y in range(H):
        for x in range(W):
            inside = left <= x < left + w and top <= y < top + h
            k = opacity * (coverage[y * W + x] if coverage else 1.0) if inside else 0.0
            out.append([lin(v) * k for v in color] + [k])
    return out


def under_px():
    return [[lin(v) for v in UNDER[x // 3]] + [1.0] for y in range(H) for x in range(W)]


def over_px(opacity_row1):
    return [[lin(v) * (opacity_row1 if y else 1.0) for v in OVER[y]] +
            [opacity_row1 if y else 1.0] for y in range(H) for x in range(W)]


def stripes():
    return [[A if x % 2 == 0 else B for x in range(W)] for y in range(H)]


def shadowed(display):
    """INK's 3 by 2 solid at the left with a black Drop Shadow at 50%, distance 1 to the right,
    softness 0: the shadow alone covers column 3, at alpha 0.5. In 8 bpc the layer's pixels are
    then rounded to 8 bits (D-330)."""
    px = []
    for y in range(H):
        for x in range(W):
            if x < 3:
                px.append([lin(v) for v in INK] + [1.0])
            elif x == 3:
                # the shadow mixed behind nothing: black at 0.5, the same in either values
                px.append([0.0, 0.0, 0.0, 0.5])
            else:
                px.append([0.0] * 4)
    return eight(px) if display else px


def mosaicked(display):
    """The stripes through Mosaic, blocks 2 by 2: each block the mean of its pixels, in display
    values in 8 bpc and then rounded (D-330), in linear light in Display."""
    drawn = [[srgb_to_linear(v / 255) for v in c[:3]] + [1.0] for row in stripes() for c in row]
    if display:
        drawn = [to_display(p) for p in drawn]
    layer = {"px": drawn, "left": 0, "top": 0, "w": W, "h": H}
    out = MO.mosaic(layer, 2)
    return eight([to_linear(p) for p in out]) if display else out


def feathered():
    """FX-MSK-014's mask, the left three columns feathered 2 pixels, as coverage."""
    return M.coverage([{"points": M.LEFT3, "feather": 2.0}], 0)


SCENES = {
    "screen": ("Four colours under a Screen layer, its second row at 50% opacity.",
               lambda: [(under_px(), "normal"), (over_px(0.5), "screen")]),
    "add": ("The same under an Add layer: where the two pass white it is held to white.",
            lambda: [(under_px(), "normal"), (over_px(0.5), "add")]),
    "normal50": ("A Normal layer at 50% opacity over the same colours.",
                 lambda: [(under_px(), "normal"),
                          ([[v * 0.5 for v in p] for p in over_px(1.0)], "normal")]),
    "shadow": ("A black Drop Shadow at 50%, softness 0, cast one pixel right of a blue block "
               "onto white: column 3 is the shadow alone over white.",
               None),
    "mask": ("A colour under another whose mask, the left three columns, is feathered 2 pixels "
             "(FX-MSK-014's): the soft band at columns 1 to 4.",
             lambda: [(under_px(), "normal"),
                      (solid_px(OVER[0], 0, 0, W, H, coverage=feathered()), "normal")]),
    "mosaic": ("Mosaic in blocks of 2 by 2 over stripes of two colours: each block one colour, "
               "the mean of its four pixels.",
               None),
}


def render(scene, display):
    if scene == "shadow":
        layers = [(solid_px(WHITE, 0, 0, W, H), "normal"), (shadowed(display), "normal")]
    elif scene == "mosaic":
        layers = [(mosaicked(display), "normal")]
    else:
        layers = SCENES[scene][1]()
    return composite(layers, display)


# --- the project files ----------------------------------------------------------------------

def transform(x, y, w, h, opacity=1.0):
    return {"anchor": prop([w / 2, h / 2]), "position": prop([x + w / 2, y + h / 2]),
            "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(opacity)}


def solid(lid, color, x, y, w, h, opacity=1.0, mode="normal", masks=(), effects=()):
    return {"id": lid, "kind": "solid", "name": lid, "enabled": True, "locked": False,
            "in_frame": 0, "out_frame": 1,
            "solid": {"color": [lin(v) for v in color], "width": w, "height": h},
            "transform": transform(x, y, w, h, opacity),
            "masks": [M.mask_json(m, n + 1) for n, m in enumerate(masks)],
            "matte": None, "blend_mode": mode, "effects": list(effects)}


def drawing_layer(lid, effects):
    return {"id": lid, "kind": "raster", "name": lid, "asset_id": "asset-stripes",
            "enabled": True, "locked": False, "in_frame": 0, "out_frame": 1,
            "source_offset_frames": 0, "transform": transform(0, 0, W, H),
            "exposure_spans": [], "masks": [], "matte": None, "blend_mode": "normal",
            "effects": list(effects)}


def layers_json(scene):
    under = [solid("under-0", UNDER[0], 0, 0, 3, H), solid("under-1", UNDER[1], 3, 0, 3, H)]
    if scene in ("screen", "add"):
        return under + [solid("over-0", OVER[0], 0, 0, W, 1, mode=scene),
                        solid("over-1", OVER[1], 0, 1, W, 1, 0.5, mode=scene)]
    if scene == "normal50":
        # one layer at 50% opacity holding both rows, as a precomposed pair would: two solids
        # each at 50%
        return under + [solid("over-0", OVER[0], 0, 0, W, 1, 0.5),
                        solid("over-1", OVER[1], 0, 1, W, 1, 0.5)]
    if scene == "mask":
        return under + [solid("over", OVER[0], 0, 0, W, H,
                              masks=[{"points": M.LEFT3, "feather": 2.0}])]
    if scene == "shadow":
        fx = {"instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
              "parameters": {"color": "#000000", "opacity": 50, "direction": 90,
                             "distance": 1, "softness": 0}}
        return [solid("white", WHITE, 0, 0, W, H), solid("ink", INK, 0, 0, 3, H, effects=[fx])]
    fx = {"instance_id": "fx-0-0", "type_id": "core.mosaic", "enabled": True,
          "parameters": {"size": 2}}
    return [drawing_layer("stripes", [fx])]


def project_json(fx, scene, display):
    layers = layers_json(scene)
    comp = {
        "id": "comp-main", "name": "Main", "width": W, "height": H,
        "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
        "start_frame": 0, "duration_frames": 1,
        "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
        "layer_order": [l["id"] for l in layers], "layers": layers,
    }
    if display:
        comp["eight_bpc"] = True
    assets = []
    if scene == "mosaic":
        assets = [{"id": "asset-stripes", "kind": "still", "name": "stripes",
                   "path": "media/stripes.png",
                   "interpretation": {"color_space": "srgb", "alpha": "straight"}}]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": assets, "compositions": [comp]}


CASES = {}
for n, scene in enumerate(SCENES):
    CASES[f"FX-MIX8-{2 * n + 1:03d}"] = (scene, True)
    CASES[f"FX-MIX8-{2 * n + 2:03d}"] = (scene, False)


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "stripes.png").write_bytes(S.png(stripes()))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (scene, display) in CASES.items():
        says = ("8 bpc: " if display else "Display, unchanged by D-763: ") + SCENES[scene][0]
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project_json(fx, scene, display), indent=2) + "\n",
                                encoding="utf-8", newline="\n")
        expected["cases"][fx] = {"says": says, "scene": scene, "eight_bpc": display,
                                 "project": name, "frames": {"0": render(scene, display)}}
        print(f"{fx}: {says[:70]}")
    (OUT / "expected_display_mix.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8", newline="\n")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    assert min(near) > CLEAR, min(near)
    f = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    shown = lambda p: [round(255 * enc(p[c] / p[3])) for c in range(3)]  # noqa: E731
    # Drop Shadow 50% black over white: mid grey in 8 bpc (After Effects' 127), 188 in Display.
    assert shown(f["FX-MIX8-007"][3]) == [127] * 3, shown(f["FX-MIX8-007"][3])
    assert shown(f["FX-MIX8-008"][3]) == [188] * 3, shown(f["FX-MIX8-008"][3])
    # Screen lightens more on display values; the two depths differ where the layers meet.
    assert all(f["FX-MIX8-001"][i] != f["FX-MIX8-002"][i] for i in range(W * H))
    # Normal at 50%: the encoded value is the plain mean of the two encoded colours in 8 bpc.
    p = f["FX-MIX8-005"][0]
    assert all(abs(enc(p[c]) - (UNDER[0][c] + OVER[0][c]) / 2) < 1e-12 for c in range(3))
    # A mask's soft band: where the mask covers fully or not at all, both depths agree.
    m = feathered()
    for i in range(W * H):
        if m[i] in (0.0, 1.0):
            assert all(abs(a - b) < 1e-12 for a, b in zip(f["FX-MIX8-009"][i], f["FX-MIX8-010"][i]))
    assert any(0 < v < 1 for v in m)
    # Mosaic: each 8 bpc block is the mean of its encoded colours, a whole 255th.
    blk = shown(f["FX-MIX8-011"][0])
    assert blk == [(A[c] + B[c]) // 2 for c in range(3)], blk
    assert shown(f["FX-MIX8-012"][0]) != blk


if __name__ == "__main__":
    main()
