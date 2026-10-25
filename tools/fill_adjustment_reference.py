"""Fill on an adjustment layer, worked a second way.

D-786: After Effects' Fill put on an adjustment layer, with Fill Mask None (0) and All Masks off,
fills everything beneath it with its colour: Ben Marriott's "2D Smoke & Explosions" tutorial turns
its noisy grey cloud strokes pure white that way (9:27 to 9:45), and D-418 already says Fill Mask
0 fills the whole layer. Before D-786 the build left the frame beneath as it was and said nothing.

The rule is the two already pinned, one after the other, and nothing new:

1. D-418's Fill with no mask (m = 1 everywhere): each premultiplied pixel P of the frame beneath
   becomes P (1 - k) + C P.a k, k = Opacity / 100, its alpha kept, C the colour in linear light.
   Invert makes m = 0, so nothing is filled.
2. D-65's adjustment mix: out = B + c (E(B) - B), c = the adjustment's coverage x its opacity.

D-830: a Fill on an adjustment layer that names a mask (Fill Mask 1 or more, or All Masks on)
fills inside that mask of the adjustment layer, moved onto the frame beneath by the layer's
transform, as D-418's Fill fills inside a mask of its own layer. A mask whose mode is None draws
nothing of the adjustment's coverage and is there only to be named. Before D-830 such a Fill
filled nothing and said EFFECT_PATH_MISSING every frame (D-786); those values, FX-FILLADJ-007 and
008's first, are kept in the repository's history at commit e047023d. A Fill Mask past the last
mask still fills nothing and says EFFECT_PATH_MISSING, as on any layer.

**This file never runs the build's code path.** It works each pixel from the drawings' 8-bit
values in double precision, through `tools/adjust_reference.py`'s document 21 pieces.

Every case is a composition 6 by 2, `adjust_reference.py`'s drawings. The expected frames are in
`Fixtures/fill_adjustment/expected_fill_adjustment.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fill_adjustment_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as A  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fill_adjustment"
TOLERANCE = 2e-5  # document 25's default for a filter
DEFAULTS = {"mask": 0, "all_masks": "off", "color": "#ff0000", "invert": "off",
            "horizontal_feather": 0, "vertical_feather": 0, "opacity": 100}


def colour(hexed):
    return [A.srgb_to_linear(int(hexed[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def fill(frame, p):
    """D-418: m is 1 inside the named mask (each mask here is whole columns `_cols` of the frame,
    lo to hi, after the layer's shift), every mask with All Masks on, everywhere with neither;
    a Fill Mask past the last mask fills nothing."""
    masks = p.get("_cols", [])
    if p["all_masks"] == "on":
        named = masks
    elif p["mask"] >= 1:
        if p["mask"] > len(masks):
            return [list(px) for px in frame]
        named = [masks[p["mask"] - 1]]
    else:
        named = None
    c = colour(p["color"])
    out = []
    for n, px in enumerate(frame):
        x = n % A.W
        m = 1.0 if named is None else float(any(lo <= x < hi for lo, hi in named))
        if p["invert"] == "on":
            m = 1.0 - m
        k = m * p["opacity"] / 100
        out.append([px[i] * (1 - k) + c[i] * px[3] * k for i in range(3)] + [px[3]])
    return out


A.EFFECTS["fill"] = fill
BG, HALF = A.BG, A.HALF


def adj(**kw):
    """`path_right` (D-830) is a mask of mode None over the layer's columns 0 to path_right: a
    path to name, cutting nothing. `_cols` hands each mask's frame columns to `fill`."""
    fills = kw.pop("fills", [{}])
    shift = kw.get("shift", 0)
    right = kw.get("mask_right", kw.get("path_right"))
    cols = [] if right is None else [(shift, shift + right)]
    return A.adj(effects=[("fill", {**DEFAULTS, **f, "_cols": cols}) for f in fills], **kw)


def mask_json(right):
    """The `masks` record (document 07) of a mode None mask over columns 0 to `right`."""
    corner = lambda x, y: {"point": [x, y], "in": [0, 0], "out": [0, 0]}
    return {"name": "Mask 1", "enabled": True, "inverted": False, "mode": "none", "opacity": 1.0,
            "feather_px": 0.0, "expansion_px": 0.0,
            "path": {"base": {"points": [corner(0, 0), corner(right, 0), corner(right, A.H),
                                         corner(0, A.H)]}, "keyframes": []}}


def project_json(name, case):
    """`adjust_reference.py`'s file, each adjustment's effects written as Fills."""
    bare = {"layers": [{**l, "effects": []} if l["kind"] == "adjustment" else l
                       for l in case["layers"]]}
    project = A.project_json(name, bare)
    project["project_id"] = "proj-" + name.lower()
    layers = project["compositions"][0]["layers"]
    for i, (record, layer) in enumerate(zip(layers[-len(case["layers"]):], case["layers"])):
        for n, (_, params) in enumerate(layer.get("effects", [])):
            record["effects"].append({"instance_id": f"fx-{i}-{n}", "type_id": "core.fill",
                                      "enabled": True,
                                      "parameters": {k: v for k, v in params.items()
                                                     if not k.startswith("_")}})
        if "path_right" in layer:
            record["masks"] = [mask_json(layer["path_right"])]
    return project


CASES = {
    "FX-FILLADJ-001": ("Fill as it starts (Fill Mask 0, red, opacity 100) on an adjustment layer: "
                       "everything beneath turns red, both rows.",
                       {"layers": [BG, adj()]}, None),
    "FX-FILLADJ-002": ("The same at Fill opacity 50: half way to red.",
                       {"layers": [BG, adj(fills=[{"opacity": 50}])]}, None),
    "FX-FILLADJ-003": ("White on half-covered green: white where the green was, its half "
                       "covering kept, nothing laid on the empty.",
                       {"layers": [HALF, adj(fills=[{"color": "#ffffff"}])]}, None),
    "FX-FILLADJ-004": ("A mask on the adjustment layer: only the left three columns turn red.",
                       {"layers": [BG, adj(mask_right=3)]}, None),
    "FX-FILLADJ-005": ("The adjustment layer at opacity 50: half way to red.",
                       {"layers": [BG, adj(opacity=0.5)]}, None),
    "FX-FILLADJ-006": ("Invert on with no mask: nothing is left to fill, the frame as it was.",
                       {"layers": [BG, adj(fills=[{"invert": "on"}])]}, None),
    "FX-FILLADJ-007": ("Fill Mask 1 on an adjustment layer with a mask over the left three "
                       "columns: those three turn red, as 004 (D-830).",
                       {"layers": [BG, adj(mask_right=3, fills=[{"mask": 1}])]}, None),
    "FX-FILLADJ-008": ("All Masks on, the same: the left three columns red (D-830).",
                       {"layers": [BG, adj(mask_right=3, fills=[{"all_masks": "on"}])]}, None),
    "FX-FILLADJ-009": ("Blue at opacity 50, then red, the tutorial's chain in small: the last Fill "
                       "wins, red.",
                       {"layers": [BG, adj(fills=[{"color": "#3080ff", "opacity": 50}, {}])]},
                       None),
    "FX-FILLADJ-010": ("A mask of mode None over the left three columns: the adjustment covers "
                       "the whole frame, and Fill Mask 1 fills only inside the mask (D-830).",
                       {"layers": [BG, adj(path_right=3, fills=[{"mask": 1}])]}, None),
    "FX-FILLADJ-011": ("The same adjustment layer moved 2 pixels right: its mask goes with it, "
                       "columns 2 to 4 red, the two it no longer covers as they were (D-830).",
                       {"layers": [BG, adj(path_right=3, shift=2, fills=[{"mask": 1}])]}, None),
    "FX-FILLADJ-012": ("Fill Mask 2 with one mask: nothing to fill, the frame as it was, and "
                       "EFFECT_PATH_MISSING said every frame, as on any layer.",
                       {"layers": [BG, adj(path_right=3, fills=[{"mask": 2}])]},
                       "EFFECT_PATH_MISSING"),
}


def fmt(v):
    return f"{v:.9g}"


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name in ("bg", "half"):
        (OUT / "media" / f"{name}.png").write_bytes(A.png(A.DRAWINGS[name]))
    expected = {"tolerance": TOLERANCE, "width": A.W, "height": A.H, "cases": {}}
    for fx, (says, case, warning) in CASES.items():
        project = f"{fx.lower().replace('-', '_')}.json"
        (OUT / project).write_text(json.dumps(project_json(fx, case), indent=2) + "\n",
                                   encoding="utf-8", newline="\n")
        frame = A.render(case, 0)
        entry = {"says": says, "project": project, "frames": {"0": frame}}
        if warning:
            entry["frame_warning"] = warning
        expected["cases"][fx] = entry
        print(f"{fx}: {says}\n")
        print("| row | " + " | ".join(f"x = {x}" for x in range(A.W)) + " |")
        print("| --- " * (A.W + 1) + "|")
        for y in range(A.H):
            cells = (" ".join(fmt(v) for v in frame[y * A.W + x]) for x in range(A.W))
            print(f"| {y} | " + " | ".join(cells) + " |")
        print()
    (OUT / "expected_fill_adjustment.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8", newline="\n")

    # The claims the cases make, checked on the numbers just worked.
    c = {k: v["frames"]["0"] for k, v in expected["cases"].items()}
    bg = A.render({"layers": [BG]})
    assert all(p == [1.0, 0.0, 0.0, 1.0] for p in c["FX-FILLADJ-001"])
    assert c["FX-FILLADJ-002"] == c["FX-FILLADJ-005"]
    assert c["FX-FILLADJ-004"][:3] == c["FX-FILLADJ-001"][:3] and c["FX-FILLADJ-004"][3:6] == bg[3:6]
    assert c["FX-FILLADJ-006"] == bg == c["FX-FILLADJ-012"]
    assert c["FX-FILLADJ-004"] == c["FX-FILLADJ-007"] == c["FX-FILLADJ-008"] == c["FX-FILLADJ-010"]
    # Row 1 is grey beneath (row 0 is red already).
    red, row = c["FX-FILLADJ-001"][0], c["FX-FILLADJ-011"][A.W:]
    assert [px == red for px in row] == [False, False, True, True, True, False]
    assert row[0] == bg[A.W] and row[5] == bg[A.W + 5]
    assert c["FX-FILLADJ-009"] == c["FX-FILLADJ-001"]
    assert all(abs(v - 128 / 255) < 1e-12 for p in c["FX-FILLADJ-003"] for v in p)


if __name__ == "__main__":
    main()
