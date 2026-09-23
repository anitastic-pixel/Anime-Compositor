"""Null layers, worked a second way.

D-82 proposes a layer that is never drawn: it has a transform and nothing else, and its only use
is as a parent (D-57). So a null must leave every frame exactly as it would be without it, and a
layer parented to it must move by the null's transform and by nothing else of the null's. This
file is the reference for the numbers document 25 pins against that.

**This file never runs the build's code path.** It carries the drawing's corner through the
parent chain with `parent_reference.to_comp`, which walks document 21's four steps one at a time,
and then lays the drawing down by that whole-pixel offset, so no pixel is resampled.

Every case is a composition 6 pixels by 2, so each frame is twelve pixels and can be printed.
The one drawing is written into `Fixtures/null/media`, the projects into `Fixtures/null`, and
the expected frames and refusals into `Fixtures/null/expected_null.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/null_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, png, decoded, over, prop, fmt  # noqa: E402
from parent_reference import to_comp  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "null"
TOLERANCE = 1e-6
BG_PIXELS = [[(255, 0, 0, 255)] * W, [(128, 128, 128, 255)] * W]  # adjust_reference's "bg"
CENTRE = [50, 50]  # a null is 100 by 100 in its own space, its anchor at the middle


def active(layer, frame_no):
    return layer.get("in", 0) <= frame_no < layer.get("out", 3)


def render(case, frame_no=0):
    """A null draws nothing, whatever its switch, opacity or in and out frames say. The drawing
    is laid down where the chain carries its top-left corner, which must be a whole pixel."""
    frame = [[0.0] * 4 for _ in range(W * H)]
    chain = {l["id"]: {"anchor": {"base": CENTRE if l["kind"] == "null" else [W / 2, H / 2]},
                       "position": {"base": l.get("at", [W / 2, H / 2])},
                       "scale": {"base": [100, 100]}, "rotation": {"base": 0},
                       "parent": l.get("parent")}
             for l in case["layers"]}
    for layer in case["layers"]:
        if layer["kind"] == "null" or not active(layer, frame_no):
            continue
        ox, oy = to_comp(chain, layer["id"], [0, 0], frame_no)
        assert ox == int(ox) and oy == int(oy), "whole pixels only"
        src = decoded("bg")
        out = []
        for y in range(H):
            for x in range(W):
                sx, sy = x - int(ox), y - int(oy)
                out.append(list(src[sy * W + sx]) if 0 <= sx < W and 0 <= sy < H else [0.0] * 4)
        frame = [over(s, d) for s, d in zip(out, frame)]
    return frame


# --- the project files ----------------------------------------------------------------------

def layer_json(layer):
    null = layer["kind"] == "null"
    record = {"id": layer["id"], "kind": layer["kind"], "name": layer["id"]}
    if not null:
        record["asset_id"] = "asset-bg"
    record.update({
        "enabled": layer.get("enabled", True), "locked": False,
        "in_frame": layer.get("in", 0), "out_frame": layer.get("out", 3),
    })
    if not null:
        record["source_offset_frames"] = 0
    record["transform"] = {
        "anchor": prop(list(CENTRE if null else [W / 2, H / 2])),
        "position": prop(list(layer.get("at", [W / 2, H / 2]))),
        "scale": prop([100, 100]), "rotation": prop(0),
        "opacity": prop(layer.get("opacity", 1)),
    }
    if not null:
        record["exposure_spans"] = []
    record["mask"] = None
    record["matte"] = None
    record["blend_mode"] = "normal"
    record["effects"] = []
    if "parent" in layer:
        record["parent"] = layer["parent"]
    return record


def project_json(name, case):
    layers = case["layers"]
    return {
        "schema_version": 0,
        "project_id": "proj-" + name.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": ([{"id": "asset-bg", "kind": "still", "name": "bg", "path": "media/bg.png",
                     "interpretation": {"color_space": "srgb", "alpha": "straight"}}]
                   if any(l["kind"] == "raster" for l in layers) else []),
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 3,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 3},
            # layer_order is bottom first here, as the cases are written.
            "layer_order": [l["id"] for l in layers],
            "layers": [layer_json(l) for l in layers],
        }],
    }


# --- the cases ------------------------------------------------------------------------------

BG = {"id": "bg", "kind": "raster"}
# The drawing keeps its own position, (3, 1), now read in the null's space. The null's anchor is
# (50, 50), so a null standing at (50, 50) leaves the drawing where it was, and one at (52, 50)
# carries it two pixels right.
ON_NULL = {**BG, "parent": "null"}


def null(**kw):
    return {"id": kw.pop("id", "null"), "kind": "null", "at": [52, 50], **kw}


CASES = {
    "FX-NULL-001": ("A null on its own, switched on: every pixel transparent.",
                    {"layers": [null()]}),
    "FX-NULL-002": ("A null switched on above the red and grey drawing: the drawing, untouched.",
                    {"layers": [BG, null()]}),
    "FX-NULL-003": ("The drawing parented to a null that stands two pixels right: the drawing "
                    "moves two pixels right, and columns 0 and 1 are empty.",
                    {"layers": [null(), ON_NULL]}),
    "FX-NULL-004": ("The same with the null switched off: the same frame.",
                    {"layers": [null(enabled=False), ON_NULL]}),
    "FX-NULL-005": ("The same with the null alive on frame 1 only: every frame is moved.",
                    {"layers": [null(**{"in": 1, "out": 2}), ON_NULL]}),
    "FX-NULL-006": ("The same with the null at a tenth opacity: the drawing stays opaque, as "
                    "opacity does not pass to a child.",
                    {"layers": [null(opacity=0.1), ON_NULL]}),
    "FX-NULL-007": ("A null parented to a null, each a pixel right: the drawing moves two, as in "
                    "FX-NULL-003.",
                    {"layers": [null(id="outer", at=[51, 50]),
                                null(at=[51, 50], parent="outer"), ON_NULL]}),
}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-NULL-002,
    whose layers are the drawing and then the null."""
    def edit(change):
        p = project_json("FX-NULL-002", CASES["FX-NULL-002"][1])
        bg, n = p["compositions"][0]["layers"]
        change(n, bg)
        return p

    def put(key, value):
        return lambda n, bg: n.__setitem__(key, value)

    return {
        "FX-NULL-020": ("A null that names an asset.", edit(put("asset_id", "asset-bg"))),
        "FX-NULL-021": ("A null with exposures.", edit(put("exposure_spans", []))),
        "FX-NULL-022": ("A null with a source offset.", edit(put("source_offset_frames", 0))),
        "FX-NULL-023": ("A null carrying a solid record.",
                        edit(put("solid", {"color": [0.2, 0.5, 0.8], "width": 2, "height": 2}))),
        "FX-NULL-024": ("A null carrying shapes.", edit(put("shapes", []))),
        "FX-NULL-025": ("A null with a mask.",
                        edit(put("mask", {"vertices": [[0, 0], [3, 0], [3, 2], [0, 2]],
                                          "enabled": True, "inverted": False}))),
        "FX-NULL-026": ("A null with an effect.",
                        edit(put("effects", [{"instance_id": "fx-null-0",
                                              "type_id": "core.exposure", "enabled": True,
                                              "parameters": {"stops": 1}}]))),
        "FX-NULL-027": ("A null with a matte of its own.",
                        edit(put("matte", {"layer_id": "bg", "mode": "alpha",
                                           "matte_only": False}))),
        "FX-NULL-028": ("A null with a blend mode other than normal.",
                        edit(put("blend_mode", "multiply"))),
        "FX-NULL-029": ("A drawing whose matte is a null.",
                        edit(lambda n, bg: bg.__setitem__(
                            "matte", {"layer_id": "null", "mode": "alpha",
                                      "matte_only": False}))),
    }


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bg.png").write_bytes(png(BG_PIXELS))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "refused": {}}
    for fx, (says, case) in CASES.items():
        frames = [0, 1, 2] if fx == "FX-NULL-005" else [0]
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

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-NULL-002's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_null.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    bg = render({"layers": [BG]})
    empty = [[0.0] * 4] * 2
    assert c["FX-NULL-001"]["0"] == [[0.0] * 4] * (W * H)
    assert c["FX-NULL-002"]["0"] == bg
    moved = c["FX-NULL-003"]["0"]
    assert moved[:2] == empty and moved[W:W + 2] == empty
    assert moved[2:W] == bg[:W - 2] and moved[W + 2:] == bg[W:2 * W - 2]
    assert c["FX-NULL-004"]["0"] == moved and c["FX-NULL-006"]["0"] == moved
    assert all(f == moved for f in c["FX-NULL-005"].values())
    assert c["FX-NULL-007"]["0"] == moved
    # A null at its own anchor leaves the drawing where it was.
    assert render({"layers": [null(at=[50, 50]), ON_NULL]}) == bg


if __name__ == "__main__":
    main()
