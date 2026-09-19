"""Solid layers, worked a second way.

D-74 proposes a layer whose drawing is a rectangle of one colour: `width` by `height` pixels, every
one opaque and of `color`, a linear working-space RGB. From there on it is a drawing like any
other: its mask, its effects, its transform, its opacity, its matte and its blend mode are
document 21's steps 2 to 7, unchanged. This file is the reference for the numbers document 25 pins
against that.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel from
document 21. Every move is by whole pixels, so no pixel is resampled and the numbers are exact.

Every case is a composition 6 pixels by 2, so each frame is twelve pixels and can be printed.
The one drawing is written into `Fixtures/solid/media`, the projects into `Fixtures/solid`, and
the expected frames and refusals into `Fixtures/solid/expected_solid.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/solid_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, png, decoded, over, exposure, prop, fmt  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "solid"
TOLERANCE = 1e-6
COLOR = [0.2, 0.5, 0.8]  # linear; a sky blue on screen, and no channel 0 or 1
BG_PIXELS = [[(255, 0, 0, 255)] * W, [(128, 128, 128, 255)] * W]  # adjust_reference's "bg"


def multiply(s, d):
    """Document 21's multiply, premultiplied source over premultiplied destination."""
    cs = [s[i] / s[3] if s[3] else 0 for i in range(3)]
    cd = [d[i] / d[3] if d[3] else 0 for i in range(3)]
    return ([(1 - s[3]) * d[i] + (1 - d[3]) * s[i] + s[3] * d[3] * cs[i] * cd[i] for i in range(3)]
            + [s[3] + d[3] - s[3] * d[3]])


def picture(layer):
    """Steps 1 to 6 for one layer, laid on the frame: a list of W*H premultiplied pixels."""
    if layer["kind"] == "raster":
        return decoded("bg")
    w, h = layer.get("size", (W, H))
    color = layer.get("color", COLOR)
    # Step 1: the solid, in its own layer space, w by h.
    src = [[*color, 1.0] for _ in range(w * h)]
    # Step 2: a mask keeping the columns left of `mask_right`, in layer space.
    if "mask_right" in layer:
        src = [p if i % w < layer["mask_right"] else [0.0] * 4 for i, p in enumerate(src)]
    # Step 3: effects, on the solid's own picture.
    for kind, stops in layer.get("effects", []):
        src = exposure(src, stops)
    # Step 4: a whole-pixel move. The anchor is the solid's centre, `at` is where it lands.
    ax, ay = layer.get("at", (W / 2, H / 2))
    ox, oy = int(ax - w / 2), int(ay - h / 2)
    out = []
    for y in range(H):
        for x in range(W):
            sx, sy = x - ox, y - oy
            out.append(list(src[sy * w + sx]) if 0 <= sx < w and 0 <= sy < h else [0.0] * 4)
    # Step 6: opacity.
    op = layer.get("opacity", 1.0)
    return [[v * op for v in p] for p in out]


def active(layer, frame_no):
    return layer.get("in", 0) <= frame_no < layer.get("out", 3)


def render(case, frame_no=0):
    frame = [[0.0] * 4 for _ in range(W * H)]
    layers = case["layers"]
    mattes = {l["id"]: l for l in layers if l.get("matte_only")}
    for layer in layers:
        if layer.get("matte_only") or not active(layer, frame_no):
            continue
        src = picture(layer)
        if "matte" in layer:  # step 5: the matte layer's alpha, where it lands
            m = picture(mattes[layer["matte"]])
            src = [[v * mp[3] for v in p] for p, mp in zip(src, m)]
        mix = multiply if layer.get("blend") == "multiply" else over
        frame = [mix(s, d) for s, d in zip(src, frame)]
    return frame


# --- the project files ----------------------------------------------------------------------

def layer_json(layer):
    at = layer.get("at", (W / 2, H / 2))
    w, h = layer.get("size", (W, H))
    record = {"id": layer["id"], "kind": layer["kind"], "name": layer["id"]}
    if layer["kind"] == "raster":
        record.update({"asset_id": "asset-bg"})
    else:
        record["solid"] = {"color": layer.get("color", COLOR), "width": w, "height": h}
    record.update({
        "enabled": True, "locked": False,
        "in_frame": layer.get("in", 0), "out_frame": layer.get("out", 3),
    })
    if layer["kind"] == "raster":
        record["source_offset_frames"] = 0
    record["transform"] = {
        "anchor": prop([w / 2, h / 2] if layer["kind"] == "solid" else [W / 2, H / 2]),
        "position": prop(list(at)), "scale": prop([100, 100]), "rotation": prop(0),
        "opacity": prop(layer.get("opacity", 1)),
    }
    if layer["kind"] == "raster":
        record["exposure_spans"] = []
    record["mask"] = None
    if "mask_right" in layer:
        m = layer["mask_right"]
        record["mask"] = {"vertices": [[0, 0], [m, 0], [m, h], [0, h]],
                          "enabled": True, "inverted": False}
    record["matte"] = ({"layer_id": layer["matte"], "mode": "alpha", "matte_only": True}
                       if "matte" in layer else None)
    record["blend_mode"] = layer.get("blend", "normal")
    record["effects"] = [{"instance_id": f"fx-{layer['id']}-{n}", "type_id": "core.exposure",
                          "enabled": True, "parameters": {"stops": stops}}
                         for n, (_, stops) in enumerate(layer.get("effects", []))]
    return record


def project_json(name, case):
    layers = case["layers"]
    raster = any(l["kind"] == "raster" for l in layers)
    return {
        "schema_version": 0,
        "project_id": "proj-" + name.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": ([{"id": "asset-bg", "kind": "still", "name": "bg", "path": "media/bg.png",
                     "interpretation": {"color_space": "srgb", "alpha": "straight"}}]
                   if raster else []),
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


def solid(**kw):
    return {"id": kw.pop("id", "solid"), "kind": "solid", **kw}


SMALL = {"size": (2, 2), "at": (3, 1)}  # two pixels square, on columns 2 and 3

CASES = {
    "FX-SOL-001": ("A solid the size of the frame, on nothing: every pixel its colour, opaque.",
                   {"layers": [solid()]}),
    "FX-SOL-002": ("The same at half opacity over the red and grey drawing.",
                   {"layers": [BG, solid(opacity=0.5)]}),
    "FX-SOL-003": ("Two pixels square, centred at (3, 1): only columns 2 and 3 are its colour.",
                   {"layers": [BG, solid(**SMALL)]}),
    "FX-SOL-004": ("A mask keeping its left three columns.",
                   {"layers": [BG, solid(mask_right=3)]}),
    "FX-SOL-005": ("One stop of exposure on the solid doubles its colour, past 1 where it goes.",
                   {"layers": [solid(effects=[("exposure", 1)])]}),
    "FX-SOL-006": ("The small solid as a matte-only layer for the drawing: the drawing shows in "
                   "columns 2 and 3 only.",
                   {"layers": [solid(id="matte", matte_only=True, **SMALL),
                               {**BG, "matte": "matte"}]}),
    "FX-SOL-007": ("Multiplied onto the drawing: each colour times the solid's.",
                   {"layers": [BG, solid(blend="multiply")]}),
    "FX-SOL-008": ("Only between its in and out frames (frame 1 only).",
                   {"layers": [BG, solid(**{"in": 1, "out": 2})]}),
}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-SOL-001."""
    def edit(change):
        p = project_json("FX-SOL-001", CASES["FX-SOL-001"][1])
        change(p["compositions"][0]["layers"][0])
        return p

    def put(key, value):
        return lambda l: l.__setitem__(key, value)

    def put_solid(key, value):
        return lambda l: l["solid"].__setitem__(key, value)

    return {
        "FX-SOL-020": ("A solid that names an asset.", edit(put("asset_id", "asset-bg"))),
        "FX-SOL-021": ("A solid with exposures.", edit(put("exposure_spans", []))),
        "FX-SOL-022": ("A solid with a source offset.", edit(put("source_offset_frames", 0))),
        "FX-SOL-023": ("A solid with no solid record.", edit(lambda l: l.pop("solid"))),
        "FX-SOL-024": ("A colour of two numbers.", edit(put_solid("color", [0.2, 0.5]))),
        "FX-SOL-025": ("A colour above 1.", edit(put_solid("color", [0.2, 1.5, 0.8]))),
        "FX-SOL-026": ("A colour below 0.", edit(put_solid("color", [-0.1, 0.5, 0.8]))),
        "FX-SOL-027": ("A width of 0.", edit(put_solid("width", 0))),
        "FX-SOL-028": ("A height that is not a whole number.", edit(put_solid("height", 2.5))),
        "FX-SOL-029": ("A raster layer carrying a solid record.",
                       edit(lambda l: l.update(kind="raster", asset_id="asset-bg"))),
    }


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bg.png").write_bytes(png(BG_PIXELS))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "refused": {}}
    for fx, (says, case) in CASES.items():
        frames = [0, 1, 2] if fx == "FX-SOL-008" else [0]
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

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SOL-001's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_solid.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    bg = render({"layers": [BG]})
    assert c["FX-SOL-001"]["0"] == [[*COLOR, 1.0]] * (W * H)
    three = c["FX-SOL-003"]["0"]
    assert [x for x in range(W) if three[x] != bg[x]] == [2, 3]
    four = c["FX-SOL-004"]["0"]
    assert four[:3] == c["FX-SOL-001"]["0"][:3] and four[3:6] == bg[3:6]
    assert c["FX-SOL-005"]["0"][0] == [0.4, 1.0, 1.6, 1.0]
    six = c["FX-SOL-006"]["0"]
    assert six[2:4] == bg[2:4] and all(p == [0.0] * 4 for p in six[:2] + six[4:6])
    assert c["FX-SOL-007"]["0"][0] == [0.2, 0.0, 0.0, 1.0]
    eight = c["FX-SOL-008"]
    assert eight["0"] == bg == eight["2"] and eight["1"] == c["FX-SOL-001"]["0"]
    # A solid is a drawing of one colour: the same frame as an opaque picture of that colour.
    assert c["FX-SOL-002"]["0"][0] == over([v * 0.5 for v in [*COLOR, 1]], bg[0])


if __name__ == "__main__":
    main()
