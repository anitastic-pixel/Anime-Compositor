"""Add Grain's Viewing Mode, worked a second way.

D-937 adds After Effects' Viewing Mode to `core.add_grain` (D-443), and so to the Add Grain that
Match Grain holds (D-454): the word `viewing_mode`, `final_output` (as every file before it reads)
or `preview`. Adobe's page on the Noise & Grain effects says Preview "shows the preview region",
a box "200 x 200 pixels by default" framed in white, with the grain inside it only, and Final
Output renders the grain over the whole frame. Its other two modes are left out: Blending Matte
shows the matte of the Blend With Original group, which this Add Grain has not got, and Noise
Samples belongs to Remove Grain and Match Grain's sampling, which this program does not show.

The rule. The drawing is dw by dh pixels, its own top-left corner (0, 0) however far an effect
above grew the buffer. The box is bw = min(200, dw) across and bh = min(200, dh) down, its corner
at x0 = floor((dw - bw) / 2), y0 = floor((dh - bh) / 2) of the drawing, so a drawing smaller than
200 pixels is boxed round its own edge. In `preview`, a pixel of the buffer in drawing space
(x, y): on the box's one-pixel edge (x is x0 or x0 + bw - 1 with y inside it, or y is y0 or
y0 + bh - 1 with x inside it) is opaque white, whatever it held; inside the edge it is Final
Output's pixel exactly; outside the box it is the pixel as it came, untouched. The box is drawn
at intensity 0 too. `final_output` is D-443 exactly.

**This file never runs the build's code path.** It works in double precision on lists.

    python tools/addgrain_view_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import addgrain_reference as A  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from adjust_reference import prop  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402

OUT = A.OUT
BOX = 200
WHITE = [1.0, 1.0, 1.0, 1.0]
# A strip wider than the box: 220 by 12, every colour in the middle of its channel's range so
# the Film grain shows everywhere.
SW, SH = 220, 12
STRIP = [[(60 + x * 140 // SW, 150 - y * 6, 90 + (x * 7 + y * 13) % 80, 255) for x in range(SW)]
         for y in range(SH)]


def boxed(before, after, dw, dh):
    bw, bh = min(BOX, dw), min(BOX, dh)
    x0, y0 = (dw - bw) // 2, (dh - bh) // 2
    px = []
    for j in range(after["h"]):
        for i in range(after["w"]):
            x, y = after["left"] + i, after["top"] + j
            k = j * after["w"] + i
            inside = x0 <= x < x0 + bw and y0 <= y < y0 + bh
            edge = inside and (x in (x0, x0 + bw - 1) or y in (y0, y0 + bh - 1))
            px.append(WHITE if edge else after["px"][k] if inside else before["px"][k])
    return dict(after, px=px)


def case(viewing_mode="preview", strip=False, **kw):
    c = A.case(**kw)
    c["viewing_mode"], c["strip"] = viewing_mode, strip
    return c


def layer_of(c):
    if not c["strip"]:
        return A.layer_of(c)
    return {"px": [R.working(p) for row in STRIP for p in row], "left": 0, "top": 0,
            "w": SW, "h": SH}


def render(c, frame_no):
    n = {k: A.held(c, k, frame_no) for k in A.RANGES}
    before = layer_of(c)
    after = A.add_grain(before, n, {k: c[k] for k in A.WORDS}, frame_no)
    if c["viewing_mode"] == "preview":
        after = boxed(before, after, *((SW, SH) if c["strip"] else (A.W, A.H)))
    if c["strip"]:
        return after["px"]
    return frame(after, c["shift"])


def plain(c):
    return layer_of(c)["px"] if c["strip"] else A.plain(c)


CASES = {
    "FX-ADDGRAIN-042": ("Preview on a strip 220 by 12, wider than the box: columns 10 to 209 are "
                        "the box, its edge (columns 10 and 209, rows 0 and 11) opaque white and "
                        "the grain inside it FX-ADDGRAIN-043's exactly; columns 0 to 9 and 210 "
                        "to 219 are the strip untouched.", case(strip=True), [0, 2]),
    "FX-ADDGRAIN-043": ("Final Output written on the strip: the grain everywhere, D-443 exactly.",
                        case("final_output", strip=True), [0, 2]),
    "FX-ADDGRAIN-044": ("Preview at intensity 0 on the strip: the box is drawn and nothing else "
                        "changes.", case(strip=True, intensity=0), [0]),
    "FX-ADDGRAIN-045": ("Preview on the 16 by 10 card, smaller than the box: the box is the card's "
                        "own edge, its border pixels white (the empty ones too), the grain "
                        "inside FX-ADDGRAIN-001's frame 0.", case(), [0]),
    "FX-ADDGRAIN-046": ("Preview after a Motion Tile of 300 per cent, the layer moved three "
                        "pixels right: the box is still the drawing's edge, in the drawing's own "
                        "space, so columns 0 to 2 show the tiles round it untouched, column 3 "
                        "and rows 0 and 9 are white.", case(tile=True, shift=3), [0]),
}

INVALID = {
    "FX-ADDGRAIN-047": ("Viewing Mode \"noise_samples\", After Effects' Remove Grain word, which "
                        "this Add Grain has not got.", case("noise_samples")),
}


def project_json(fx, c):
    p = A.project_json(fx, c)
    comp = p["compositions"][0]
    if c["strip"]:
        p["assets"][0]["path"] = "media/strip.png"
        p["assets"][0]["id"] = "asset-strip"
        p["assets"][0]["name"] = "strip"
        comp["layers"][0]["asset_id"] = "asset-strip"
        comp["width"], comp["height"] = SW, SH
        t = comp["layers"][0]["transform"]
        t["anchor"] = prop([SW / 2, SH / 2])
        t["position"] = prop([SW / 2, SH / 2])
    comp["layers"][0]["effects"][-1]["parameters"]["viewing_mode"] = c["viewing_mode"]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media" / "strip.png").write_bytes(S.png(STRIP))
    expected = {"tolerance": A.TOLERANCE, "width": "as each case's composition", "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in frames}}
        print(fx)
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(fx, "invalid")
    (OUT / "expected_addgrain_view.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    strip = layer_of(case(strip=True))["px"]
    p, f = c["FX-ADDGRAIN-042"]["0"], c["FX-ADDGRAIN-043"]["0"]
    at = lambda x, y: y * SW + x  # noqa: E731
    for y in range(SH):
        for x in range(SW):
            k = at(x, y)
            if x < 10 or x > 209:
                assert p[k] == strip[k] and f[k] != strip[k], (x, y)
            elif x in (10, 209) or y in (0, SH - 1):
                assert p[k] == WHITE
            else:
                assert p[k] == f[k]
    # Final Output written is D-443 exactly.
    old = A.add_grain(layer_of(case(strip=True)), {k: A.held(case(), k, 2) for k in A.RANGES},
                      {k: case()[k] for k in A.WORDS}, 2)["px"]
    assert c["FX-ADDGRAIN-043"]["2"] == old
    z = c["FX-ADDGRAIN-044"]["0"]
    assert all(z[at(x, y)] == (WHITE if 10 <= x <= 209 and (x in (10, 209) or y in (0, SH - 1))
                               else strip[at(x, y)]) for x in range(SW) for y in range(SH))
    small, full = c["FX-ADDGRAIN-045"]["0"], A.render(A.case(), 0)
    W, H = A.W, A.H
    for y in range(H):
        for x in range(W):
            edge = x in (0, W - 1) or y in (0, H - 1)
            assert small[y * W + x] == (WHITE if edge else full[y * W + x])
    tiled, tile_plain = c["FX-ADDGRAIN-046"]["0"], A.plain(case(tile=True, shift=3))
    assert all(tiled[y * W + x] == tile_plain[y * W + x] for x in range(3) for y in range(H))
    assert any(tiled[y * W + x][3] > 0 for x in range(3) for y in range(H))
    assert all(tiled[y * W + 3] == WHITE for y in range(H))
    assert all(tiled[y * W + x] == WHITE for x in range(3, W) for y in (0, H - 1))
    print("checked")


if __name__ == "__main__":
    main()
