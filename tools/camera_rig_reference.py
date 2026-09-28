"""A camera that rides a layer, worked a second way.

D-171 proposes that D-58's camera may name a layer of its composition as its parent, which is
After Effects' camera rig: a null carries the camera, and moving the null moves the shot. The
camera's `position` is then a point in the parent's space, carried to the composition by the
parent's whole chain exactly as a layer's anchor point is (D-57), and its `depth` is measured from
the parent's plane, added up the chain exactly as a layer's is (D-58). The zoom is the camera's
own. The view is never turned or scaled: D-56 keeps the camera square to the planes, so a parent
that turns or grows moves the camera's point and nothing else.

**This file never runs the build's code path.** It carries the camera's point through the chain
with `parent_reference.to_comp`, which walks document 21's four steps one at a time, and then
does D-58's two lines by hand, as `camera_reference.py` does.

Two kinds of case. The pixel cases are a composition 6 pixels by 2, as `null_reference.py`'s
are, with the camera drawing the depth-0 plane at true size, so the camera's travel is a whole
number of pixels and the drawing is laid down without being resampled. The point cases say where
the drawing's corners land on the screen, because a turned, grown or deeper parent draws the
picture at a size no whole-pixel frame can show.

The projects go into `Fixtures/camera_rig`, the expected frames, points and refusals into
`Fixtures/camera_rig/expected_camera_rig.json`, and the drawing is `null_reference.py`'s.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/camera_rig_reference.py
"""

import json
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, decoded, over, prop, fmt  # noqa: E402
from parent_reference import chain, to_comp, value  # noqa: E402
from mask_reference import draw_cases  # noqa: E402
import null_reference  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "camera_rig"
PICTURE = ROOT / "verification" / "B-111a proposal" / "rig_cases.png"
TOLERANCE = 1e-9  # the points, worked in 64 bits end to end
# The frames, as null_reference.py's: a picture is decoded and drawn in 32-bit numbers.
PIXEL_TOLERANCE = 1e-6
CENTRE = [W / 2, H / 2]
ZOOM = W * 50 / 36  # D-58's default lens, and the camera the cases name for themselves
NULL_ANCHOR = [50, 50]  # a null is 100 by 100 in its own space, its anchor at the middle
# The drawing's corners, in its own pixels.
CORNERS = [[0, 0], [W, 0], [0, H], [W, H]]


def keyed(base, keys=None):
    """A property in the reference's own shape: a base, and (frame, value) pairs if keyed."""
    return {"base": base, "keys": keys} if keys else {"base": base}


def null(id="rig", at=(CENTRE[0], CENTRE[1]), parent=None, **kw):
    """A null standing with its anchor at `at`: the camera's point is read in its space."""
    return {"id": id, "kind": "null", "position": keyed(list(at), kw.get("keys")),
            "rotation": kw.get("rotation", 0), "scale": kw.get("scale", 100),
            "depth": kw.get("depth", 0), "parent": parent}


# The camera sits over the null's anchor; one pixel right of it is (51, 50) in the null's space.
OVER_RIG = [50, 50]


def chain_layers(case):
    """Every layer in `parent_reference`'s shape: anchor, position, scale, rotation, parent."""
    out = {}
    for l in case["layers"]:
        if l["kind"] == "null":
            out[l["id"]] = {"anchor": {"base": NULL_ANCHOR}, "position": l["position"],
                            "scale": {"base": [l["scale"]] * 2}, "rotation": {"base": l["rotation"]},
                            "parent": l["parent"]}
        else:
            out[l["id"]] = {"anchor": {"base": list(CENTRE)}, "position": {"base": list(CENTRE)},
                            "scale": {"base": [100, 100]}, "rotation": {"base": 0}, "parent": None}
    return out


def world_depth(case, name):
    layers = {l["id"]: l for l in case["layers"]}
    return sum(layers[n].get("depth", 0) for n in chain(chain_layers(case), name))


def camera_at(case, frame):
    """D-171: where the camera is, how far back it is, and its zoom, in the composition."""
    cam = case["camera"]
    local = value(cam["position"], frame)
    depth = cam.get("depth", -ZOOM)
    parent = cam.get("parent")
    layers = chain_layers(case)
    if parent is None or parent not in layers:
        # No parent, or one that is not in the composition: the camera where it would be alone.
        return local, depth
    return to_comp(layers, parent, local, frame), depth + world_depth(case, parent)


def to_screen(case, p, frame):
    """A point of the drawing, in the pixels of the picture: D-58's two lines by hand."""
    eye, cam_depth = camera_at(case, frame)
    s = ZOOM / (0 - cam_depth)  # the drawing is on the depth-0 plane, parented to nothing
    return [c + (q - e) * s for c, q, e in zip(CENTRE, p, eye)]


def render(case, frame=0):
    """The drawing laid down where the camera puts it, which must be a whole-pixel shift."""
    (ox, oy), (x1, y1) = to_screen(case, [0, 0], frame), to_screen(case, [1, 1], frame)
    assert (x1 - ox, y1 - oy) == (1, 1), "drawn at true size only"
    assert ox == int(ox) and oy == int(oy), "whole pixels only"
    src = decoded("bg")
    out = []
    for y in range(H):
        for x in range(W):
            sx, sy = x - int(ox), y - int(oy)
            out.append(list(src[sy * W + sx]) if 0 <= sx < W and 0 <= sy < H else [0.0] * 4)
    return [over(s, d) for s, d in zip(out, [[0.0] * 4] * (W * H))]


# --- the project files ----------------------------------------------------------------------

def file_property(p):
    """A reference property in document 19's shape."""
    keys = [{"frame": f, "value": v, "interp": "linear"} for f, v in p.get("keys") or []]
    return {"base": p["base"], "keyframes": keys}


def layer_json(layer):
    if layer["kind"] == "raster":
        return null_reference.layer_json({"id": "bg", "kind": "raster"})
    if layer["kind"] == "audio":
        return {"id": layer["id"], "kind": "audio", "name": layer["id"],
                "asset_id": "asset-sound", "enabled": True, "locked": False, "in_frame": 0,
                "out_frame": 3, "source_offset_frames": 0, "gain_db": 0}
    record = null_reference.layer_json({"id": layer["id"], "kind": "null"})
    t = record["transform"]
    t["position"] = file_property(layer["position"])
    t["rotation"] = prop(layer["rotation"])
    t["scale"] = prop([layer["scale"], layer["scale"]])
    if layer["depth"]:
        record["depth"] = prop(layer["depth"])
    if layer["parent"]:
        record["parent"] = layer["parent"]
    return record


def project_json(name, case):
    p = null_reference.project_json(name, {"layers": [{"id": "bg", "kind": "raster"}]})
    comp = p["compositions"][0]
    comp["layer_order"] = [l["id"] for l in case["layers"]]
    comp["layers"] = [layer_json(l) for l in case["layers"]]
    cam = case["camera"]
    comp["camera"] = {"position": file_property(cam["position"]), "depth": prop(cam.get("depth", -ZOOM)),
                      "zoom": prop(ZOOM)}
    if "parent" in cam:
        comp["camera"]["parent"] = cam["parent"]
    if any(l["kind"] == "audio" for l in case["layers"]):
        p["assets"].append({"id": "asset-sound", "kind": "audio", "name": "sound",
                            "path": "media/pcm16_mono_48k.wav"})
    return p


# --- the cases ------------------------------------------------------------------------------

BG = {"id": "bg", "kind": "raster"}


def rig(camera_at=OVER_RIG, **kw):
    """The drawing, a null, and the camera riding the null at `camera_at` in the null's space."""
    layers = kw.pop("layers", None) or [BG, null(**kw)]
    return {"layers": layers, "camera": {"position": keyed(list(camera_at)), "parent": "rig"}}


PIXELS = {
    "FX-RIG-001": ("The camera rides a null standing at the middle of the frame, right over the "
                   "null's own anchor: the frame is exactly the frame with no camera at all.",
                   rig(), [0]),
    "FX-RIG-002": ("The null two pixels right: the camera goes with it, so the drawing moves two "
                   "pixels left, and columns 4 and 5 are empty.",
                   rig(at=(CENTRE[0] + 2, CENTRE[1])), [0]),
    "FX-RIG-003": ("The null keyed from the middle at frame 0 to two pixels right at frame 2: the "
                   "drawing slides left one pixel a frame.",
                   rig(keys=[(0, list(CENTRE)), (2, [CENTRE[0] + 2, CENTRE[1]])]), [0, 1, 2]),
    "FX-RIG-004": ("The null on a second null, each a pixel right: the camera rides the whole "
                   "chain, and the drawing moves two pixels left, as in FX-RIG-002.",
                   rig(layers=[BG, null(id="outer", at=(CENTRE[0] + 1, CENTRE[1])),
                               null(at=(51, 50), parent="outer")]), [0]),
    "FX-RIG-005": ("The camera one pixel right of the null, in the null's space, with the null at "
                   "the middle: the drawing moves one pixel left.",
                   rig(camera_at=[51, 50]), [0]),
}

POINTS = {
    "FX-RIG-010": ("The null turned a quarter, clockwise, with the camera a pixel right of it in "
                   "its space: the camera's point swings down to (3, 2), and the picture moves up "
                   "one pixel without turning, because the view never turns.",
                   rig(camera_at=[51, 50], rotation=90), [0]),
    "FX-RIG-011": ("The null one lens length back, at depth -8.33: the camera rides back with it "
                   "to twice as far from the drawing, which is drawn at half size about the "
                   "middle.",
                   rig(depth=-ZOOM), [0]),
    "FX-RIG-012": ("The null at 200%, with the camera a pixel right of it in its space: the "
                   "camera's point is two pixels right, and the picture is not made bigger, "
                   "because a camera's zoom is its own.",
                   rig(camera_at=[51, 50], scale=200), [0]),
}

WARNED = {
    "FX-RIG-020": ("The camera names a parent that is not in the composition: the file opens, "
                   "`PARENT_REFERENCE_MISSING` says so, the reference is kept, and the camera is "
                   "where it would be with no parent, which here is over the middle.",
                   {"layers": [BG], "camera": {"position": keyed(list(CENTRE)), "parent": "gone"}},
                   [0]),
}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-RIG-001."""
    def edit(change):
        p = project_json("FX-RIG-001", PIXELS["FX-RIG-001"][1])
        change(p["compositions"][0])
        return p

    def audio_parent(comp):
        comp["layer_order"].append("sound")
        comp["layers"].append(layer_json({"id": "sound", "kind": "audio"}))
        comp["camera"]["parent"] = "sound"

    audio = edit(audio_parent)
    audio["assets"].append({"id": "asset-sound", "kind": "audio", "name": "sound",
                            "path": "media/pcm16_mono_48k.wav"})
    return {
        "FX-RIG-030": ("The camera's parent is a number, not a layer's identifier.",
                       edit(lambda c: c["camera"].__setitem__("parent", 7))),
        "FX-RIG-031": ("The camera's parent is an audio layer, which has no place (D-71).", audio),
    }


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "media").mkdir(exist_ok=True)
    shutil.copyfile(ROOT / "Fixtures" / "null" / "media" / "bg.png", OUT / "media" / "bg.png")
    shutil.copyfile(ROOT / "Fixtures" / "audio" / "media" / "pcm16_mono_48k.wav",
                    OUT / "media" / "pcm16_mono_48k.wav")

    expected = {"tolerance": TOLERANCE, "pixel_tolerance": PIXEL_TOLERANCE, "width": W, "height": H,
                "cases": {}, "points": {}, "warned": {}, "refused": {}}

    def write(fx, case):
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project_json(fx, case), indent=2) + "\n",
                                encoding="utf-8")
        return name

    for fx, (says, case, frames) in PIXELS.items():
        rendered = {str(f): render(case, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, case), "frames": rendered}
        print(f"{fx}: {says}\n")
        print("| frame, row | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
        print("| --- " * (W + 1) + "|")
        for f, px in rendered.items():
            for y in range(H):
                cells = (" ".join(fmt(v) for v in px[y * W + x]) for x in range(W))
                print(f"| {f}, {y} | " + " | ".join(cells) + " |")
        print()

    for fx, (says, case, frames) in POINTS.items():
        points = {str(f): [to_screen(case, c, f) for c in CORNERS] for f in frames}
        eye, depth = camera_at(case, frames[0])
        expected["points"][fx] = {"says": says, "project": write(fx, case), "layer": "bg",
                                  "corners": CORNERS, "screen": points,
                                  "camera": {"position": eye, "depth": depth}}
        print(f"{fx}: {says}\n")
        print(f"The camera is at ({fmt(eye[0])}, {fmt(eye[1])}), depth {fmt(depth)}.\n")
        print("| frame | drawing corner | on the screen |")
        print("| --- | --- | --- |")
        for f, where in points.items():
            for c, s in zip(CORNERS, where):
                print(f"| {f} | ({c[0]}, {c[1]}) | ({fmt(s[0])}, {fmt(s[1])}) |")
        print()

    for fx, (says, case, frames) in WARNED.items():
        rendered = {str(f): render(case, f) for f in frames}
        expected["warned"][fx] = {"says": says, "project": write(fx, case),
                                  "code": "PARENT_REFERENCE_MISSING", "frames": rendered}
        print(f"- {fx}: {says}")
    print()

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-RIG-001's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    # The pixel cases as a picture, each row one case and its frames left to right.
    draw_cases({**{fx: list(v["frames"].values()) for fx, v in expected["cases"].items()},
                **{fx: list(v["frames"].values()) for fx, v in expected["warned"].items()}},
               PICTURE)
    print(f"Drawn: {PICTURE}")

    (OUT / "expected_camera_rig.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    bg = render({"layers": [BG], "camera": {"position": keyed(list(CENTRE))}})
    empty = [[0.0] * 4] * 2
    assert c["FX-RIG-001"]["0"] == bg
    moved = c["FX-RIG-002"]["0"]
    assert moved[W - 2:W] == empty and moved[2 * W - 2:] == empty
    assert moved[:W - 2] == bg[2:W] and moved[W:2 * W - 2] == bg[W + 2:]
    assert c["FX-RIG-003"]["0"] == bg and c["FX-RIG-003"]["2"] == moved
    assert c["FX-RIG-003"]["1"] == c["FX-RIG-005"]["0"] != bg
    assert c["FX-RIG-004"]["0"] == moved
    assert expected["warned"]["FX-RIG-020"]["frames"]["0"] == bg
    p = expected["points"]
    assert p["FX-RIG-010"]["camera"]["position"] == [3, 2]
    assert p["FX-RIG-010"]["screen"]["0"][0] == [0, -1]  # moved up one, and not turned
    assert p["FX-RIG-011"]["screen"]["0"][0] == [1.5, 0.5]  # half size about the middle
    assert p["FX-RIG-012"]["screen"]["0"][0] == [-2, 0]  # two left, and not grown


if __name__ == "__main__":
    main()
