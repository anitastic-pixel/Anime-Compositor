"""Precompositions, worked a second way.

D-67 proposes a layer whose drawing is another composition: at each frame, that composition is
rendered at its own size, and the picture is then taken through document 21's steps as a
drawing would be - mask, effects, transform, matte, opacity, blend. The frame of the inner
composition is the layer's local frame, `comp_frame - in_frame + source_offset_frames`, and
outside the inner composition's own start and duration the picture is transparent. This file
is the reference for the numbers document 25 pins against that.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel,
nesting by plain recursion, and reuses `tools/adjust_reference.py` for the drawings, the PNG
writer, the effects and the adjustment layer. Whole-pixel shifts stand in for the transform,
so no sampling filter is involved.

Every composition is 6 pixels by 2 unless the case says otherwise, so a frame is twelve pixels
and can be printed. The drawings are written into `Fixtures/precomp/media`, the projects into
`Fixtures/precomp`, and the expected frames into `Fixtures/precomp/expected_precomp.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/precomp_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as adj  # noqa: E402
from adjust_reference import W, H, over, prop, run_stack, fmt  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "precomp"
TOLERANCE = 1e-6

# The adjustment drawings, plus one two-by-two opaque blue square for the small composition.
DRAWINGS = {**adj.DRAWINGS, "small": [[(0, 0, 255, 255)] * 2] * 2}


def png(pixels):
    adj.W, adj.H = len(pixels[0]), len(pixels)
    try:
        return adj.png(pixels)
    finally:
        adj.W, adj.H = W, H


def decoded(name):
    rows = DRAWINGS[name]
    return [[adj.srgb_to_linear(r / 255) * (a / 255), adj.srgb_to_linear(g / 255) * (a / 255),
             adj.srgb_to_linear(b / 255) * (a / 255), a / 255]
            for row in rows for r, g, b, a in row], len(rows[0]), len(rows)


# --- a case ---------------------------------------------------------------------------------

def size(comp):
    return comp.get("w", W), comp.get("h", H)


def active(layer, frame_no):
    return layer.get("in", 0) <= frame_no < layer.get("out", 3) and layer.get("enabled", True)


def source(case, layer, frame_no):
    """Step 1: the layer's picture at its own size. A drawing, or the inner composition at the
    layer's local frame; transparent outside the inner composition's own start and duration."""
    if layer["kind"] == "raster":
        return decoded(layer["drawing"])
    inner = case.get(layer["comp"])
    if inner is None:  # COMPOSITION_REFERENCE_MISSING: drawn as nothing
        return [[0.0] * 4 for _ in range(W * H)], W, H
    w, h = size(inner)
    local = frame_no - layer.get("in", 0) + layer.get("offset", 0)
    start, duration = inner.get("start", 0), inner.get("duration", 3)
    if not start <= local < start + duration:
        return [[0.0] * 4 for _ in range(w * h)], w, h
    return render(case, layer["comp"], local), w, h


def render(case, name, frame_no):
    """One composition's frame, bottom layer first, each layer through document 21's steps."""
    comp = case[name]
    cw, ch = size(comp)
    frame = [[0.0] * 4 for _ in range(cw * ch)]
    matte = decoded("matte")[0] if any(l.get("matte") for l in comp["layers"]) else None
    for layer in comp["layers"]:
        if not active(layer, frame_no):
            continue
        if layer["kind"] == "adjustment":
            stack = [e for e in layer["effects"] if e[0] != "off"]
            if not stack:
                continue
            adjusted = run_stack(frame, stack)
            cov = adj.coverage(layer, frame_no)
            op = layer.get("opacity", 1.0)
            frame = [[b[i] + c * op * (e[i] - b[i]) for i in range(4)]
                     for b, e, c in zip(frame, adjusted, cov)]
            continue
        src, sw, sh = source(case, layer, frame_no)
        if "mask_right" in layer:  # step 2, in the source's own space
            src = [p if x < layer["mask_right"] else [0.0] * 4
                   for y in range(sh) for x, p in [(i, src[y * sw + i]) for i in range(sw)]]
        if layer.get("effects"):  # step 3, in the source's own space (6 by 2 only here)
            assert (sw, sh) == (W, H)
            src = run_stack(src, layer["effects"])
        # Step 4: the source's centre lands at the composition's centre, then the shift.
        ox, oy = (cw - sw) // 2 + layer.get("shift", 0), (ch - sh) // 2
        op = layer.get("opacity", 1.0)
        for y in range(ch):
            for x in range(cw):
                sx, sy = x - ox, y - oy
                if not (0 <= sx < sw and 0 <= sy < sh):
                    continue
                p = src[sy * sw + sx]
                c = op * (matte[y * cw + x][3] if layer.get("matte") else 1.0)  # steps 5 and 6
                frame[y * cw + x] = over([v * c for v in p], frame[y * cw + x])  # step 7
    return frame


# --- the project files ----------------------------------------------------------------------

def layer_json(layer, index, cw, ch, case):
    record = adj.layer_json({**layer, "kind": "raster", "drawing": layer.get("drawing", "bg")}
                            if layer["kind"] == "composition" else layer, index)
    record["kind"] = layer["kind"]
    if layer["kind"] == "composition":
        inner = case.get(layer["comp"], {})
        iw, ih = size(inner)
        del record["asset_id"], record["exposure_spans"]
        record["source_offset_frames"] = layer.get("offset", 0)
        record["transform"]["anchor"] = prop([iw / 2, ih / 2])
        record["transform"]["position"] = prop([cw / 2 + layer.get("shift", 0), ch / 2])
        # The build's key order: composition_id where asset_id would be.
        record = {**{k: record[k] for k in ("id", "kind", "name")},
                  "composition_id": "comp-" + layer["comp"],
                  **{k: v for k, v in record.items() if k not in ("id", "kind", "name")}}
    if "mask_right" in layer:
        m = layer["mask_right"]
        record["mask"]["vertices"] = [[0, 0], [m, 0], [m, ch], [0, ch]]
    return record


def comp_json(name, comp, case):
    layers = list(comp["layers"])
    if any(l.get("matte") for l in layers):
        layers = [{"id": "matte", "kind": "raster", "drawing": "matte"}] + layers
    cw, ch = size(comp)
    start, duration = comp.get("start", 0), comp.get("duration", 3)
    return {
        "id": "comp-" + name, "name": name.capitalize(), "width": cw, "height": ch,
        "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
        "start_frame": start, "duration_frames": duration,
        "work_area": {"start_frame": start, "end_frame_exclusive": start + duration},
        "layer_order": [l["id"] for l in layers],
        "layers": [layer_json(l, i, cw, ch, case) for i, l in enumerate(layers)],
    }


def project_json(fx, case):
    comps = {name: comp_json(name, comp, case) for name, comp in case.items()}
    used = sorted({l["asset_id"][6:] for c in comps.values() for l in c["layers"] if "asset_id" in l})
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}
                   for d in used],
        # Main first: it is the composition the window opens and the fixture renders.
        "compositions": [comps["main"]] + [c for n, c in comps.items() if n != "main"],
    }


# --- the cases ------------------------------------------------------------------------------

BG = {"id": "bg", "kind": "raster", "drawing": "bg"}
HALF = {"id": "half", "kind": "raster", "drawing": "half"}
DOT = {"id": "dot", "kind": "raster", "drawing": "dot"}
LEFT = {"id": "left", "kind": "raster", "drawing": "matte"}
SMALL = {"id": "small", "kind": "raster", "drawing": "small"}


def pre(comp="inner", **kw):
    return {"id": kw.pop("id", "pre"), "kind": "composition", "comp": comp, **kw}


def adjustment(**kw):
    return {"id": "adj", "kind": "adjustment", **kw}


CASES = {
    "FX-PRE-001": ("A composition layer shows the inner composition's frame.",
                   {"main": {"layers": [pre()]}, "inner": {"layers": [BG]}}),
    "FX-PRE-002": ("A blur on the composition layer blurs the inner picture as one, not each "
                   "drawing on its own.",
                   {"main": {"layers": [pre(effects=[("blur", 1.0)])]},
                    "inner": {"layers": [BG, DOT]}}),
    "FX-PRE-003": ("Half opacity on the composition layer: the inner picture at half.",
                   {"main": {"layers": [pre(opacity=0.5)]}, "inner": {"layers": [BG]}}),
    "FX-PRE-004": ("Moved two pixels right: the inner picture moves as one drawing.",
                   {"main": {"layers": [pre(shift=2)]}, "inner": {"layers": [LEFT]}}),
    "FX-PRE-005": ("A two-by-two inner composition lands centred, at its own size.",
                   {"main": {"layers": [BG, pre()]},
                    "inner": {"w": 2, "h": 2, "layers": [SMALL]}}),
    "FX-PRE-006": ("The inner composition's own time: a source offset of one frame, and "
                   "nothing past its end.",
                   {"main": {"layers": [pre(offset=1)]},
                    "inner": {"layers": [{**BG, "in": 1, "out": 2}]}}),
    "FX-PRE-007": ("Only between the composition layer's own in and out frames (frame 1 only).",
                   {"main": {"layers": [pre(**{"in": 1, "out": 2})]},
                    "inner": {"layers": [BG]}}),
    "FX-PRE-008": ("Two levels deep: each level's own opacity and effects.",
                   {"main": {"layers": [pre("mid", effects=[("exposure", 1)])]},
                    "mid": {"layers": [pre("inner", opacity=0.5)]},
                    "inner": {"layers": [BG]}}),
    "FX-PRE-009": ("An adjustment layer inside stays inside: the layer above the composition "
                   "layer is not adjusted.",
                   {"main": {"layers": [pre(), HALF]},
                    "inner": {"layers": [BG, adjustment(effects=[("exposure", 1)])]}}),
    "FX-PRE-010": ("An adjustment layer above a composition layer adjusts it like any layer.",
                   {"main": {"layers": [pre(), adjustment(effects=[("exposure", 1)])]},
                    "inner": {"layers": [BG]}}),
    "FX-PRE-011": ("A mask on the composition layer: only the left three columns.",
                   {"main": {"layers": [pre(mask_right=3)]}, "inner": {"layers": [BG]}}),
    "FX-PRE-012": ("A matte-only layer shapes the composition layer: only the left three "
                   "columns.",
                   {"main": {"layers": [pre(matte=True)]}, "inner": {"layers": [BG]}}),
    "FX-PRE-013": ("The same composition twice, the second moved three right: two layers.",
                   {"main": {"layers": [pre(id="pre1"), pre(id="pre2", shift=3)]},
                    "inner": {"layers": [HALF]}}),
    "FX-PRE-014": ("A composition that does not exist: drawn as nothing, with "
                   "COMPOSITION_REFERENCE_MISSING.",
                   {"main": {"layers": [pre("gone")]}}),
}

# FX-PRE-015 is a file, not a frame: Main holds a layer of Inner, and Inner a layer of Main.
CYCLE = {"main": {"layers": [pre("inner")]}, "inner": {"layers": [BG, pre("main")]}}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, case) in CASES.items():
        frames = [0, 1, 2] if fx in ("FX-PRE-006", "FX-PRE-007") else [0]
        rendered = {str(f): render(case, "main", f) for f in frames}
        project = f"{fx.lower().replace('-', '_')}.json"
        (OUT / project).write_text(json.dumps(project_json(fx, case), indent=2) + "\n",
                                   encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": project, "frames": rendered}
        if fx == "FX-PRE-014":
            expected["cases"][fx]["warning"] = "COMPOSITION_REFERENCE_MISSING"

        print(f"{fx}: {says}\n")
        print("| frame, row | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
        print("| --- " * (W + 1) + "|")
        for f, px in rendered.items():
            for y in range(H):
                cells = (" ".join(fmt(v) for v in px[y * W + x]) for x in range(W))
                print(f"| {f}, {y} | " + " | ".join(cells) + " |")
        print()

    says = "Main holds a layer of Inner and Inner a layer of Main: the file is refused."
    (OUT / "fx_pre_015.json").write_text(json.dumps(project_json("FX-PRE-015", CYCLE), indent=2)
                                         + "\n", encoding="utf-8")
    expected["cases"]["FX-PRE-015"] = {"says": says, "project": "fx_pre_015.json",
                                       "rejected": "COMPOSITION_CYCLE"}
    print(f"FX-PRE-015: {says}\n")

    (OUT / "expected_precomp.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v.get("frames") for fx, v in expected["cases"].items()}
    bg = render({"main": {"layers": [BG]}}, "main", 0)
    nothing = [[0.0] * 4] * (W * H)
    assert c["FX-PRE-001"]["0"] == bg
    each = render({"main": {"layers": [{**BG, "effects": [("blur", 1.0)]},
                                       {**DOT, "effects": [("blur", 1.0)]}]}}, "main", 0)
    assert c["FX-PRE-002"]["0"] != each, "blurring each drawing on its own gave the same"
    assert c["FX-PRE-003"]["0"] == [[v * 0.5 for v in p] for p in bg]
    four = c["FX-PRE-004"]["0"]
    assert four[:2] == nothing[:2] and all(p[3] == 1 for p in four[2:5]) and four[5][3] == 0
    five = c["FX-PRE-005"]["0"]
    assert [p[2] > 0 for p in five[:W]] == [False, False, True, True, False, False]
    assert c["FX-PRE-006"] == {"0": bg, "1": nothing, "2": nothing}
    assert c["FX-PRE-007"] == {"0": nothing, "1": bg, "2": nothing}
    assert c["FX-PRE-008"]["0"] == [[p[0], p[1], p[2], 0.5] for p in bg]
    half, bright = decoded("half")[0], adj.exposure(bg, 1)
    assert c["FX-PRE-009"]["0"] == [over(s, d) for s, d in zip(half, bright)]
    assert c["FX-PRE-010"]["0"] == bright
    assert c["FX-PRE-011"]["0"] == bg[:3] + nothing[3:6] + bg[6:9] + nothing[9:]
    assert c["FX-PRE-012"]["0"] == c["FX-PRE-011"]["0"]
    assert [round(p[3], 6) for p in c["FX-PRE-013"]["0"][:W]] == [round(128 / 255, 6)] * 3 \
        + [round(1 - (1 - 128 / 255) ** 2, 6)] * 3
    assert c["FX-PRE-014"]["0"] == nothing


if __name__ == "__main__":
    main()
