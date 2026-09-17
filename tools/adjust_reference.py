"""Adjustment layers, worked a second way.

D-65 proposes a layer with no drawing of its own. Its effects run on everything already drawn
beneath it, as one picture the size of the frame, and the result is mixed back in by how much of
the frame the adjustment layer covers:

    out = B + c * (E(B) - B)        c = coverage * opacity, all four channels

where B is the frame below, E is the layer's effect stack run on it, and coverage is the alpha an
opaque rectangle the size of the composition would have at that pixel after the layer's mask,
transform and matte. This file is the reference for the numbers document 25 pins against that.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel from
document 21, and it blurs with the two-dimensional kernel summed directly, where the build runs
two one-dimensional passes. An agreement between the two is two answers and not one answer twice.

Every case is a composition 6 pixels by 2, so each frame is twelve pixels and can be printed.
The drawings are written into `Fixtures/adjust/media`, the projects into `Fixtures/adjust`, and
the expected frames into `Fixtures/adjust/expected_adjust.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/adjust_reference.py
"""

import json
import struct
import zlib
from math import ceil, exp
from pathlib import Path

W, H = 6, 2
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "adjust"
TOLERANCE = 1e-6


# --- drawings -------------------------------------------------------------------------------

def png(pixels):
    """An 8-bit straight RGBA PNG, W by H, rows top to bottom."""
    raw = b"".join(b"\0" + bytes(v for px in row for v in px) for row in pixels)

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", W, H, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


DRAWINGS = {
    # Opaque. The top row is red, the bottom row mid grey, so both a 0-or-1 value and a
    # value the sRGB curve has to work for are in every frame.
    "bg": [[(255, 0, 0, 255)] * W, [(128, 128, 128, 255)] * W],
    # One opaque white pixel at (2, 0), for the blur.
    "dot": [[(255, 255, 255, 255) if x == 2 else (0, 0, 0, 0) for x in range(W)],
            [(0, 0, 0, 0)] * W],
    # Half-covered green everywhere.
    "half": [[(0, 255, 0, 128)] * W] * H,
    # White on the left three columns, nothing on the right three.
    "matte": [[(255, 255, 255, 255 if x < 3 else 0) for x in range(W)]] * H,
}


def srgb_to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def decoded(name):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    frame = []
    for row in DRAWINGS[name]:
        for r, g, b, a in row:
            a = a / 255
            frame.append([srgb_to_linear(r / 255) * a, srgb_to_linear(g / 255) * a,
                          srgb_to_linear(b / 255) * a, a])
    return frame


# --- the pieces of document 21 this file needs --------------------------------------------

def over(s, d):
    return [s[i] + d[i] * (1 - s[3]) for i in range(4)]


def exposure(frame, stops):
    g = 2 ** stops
    return [[p[0] * g, p[1] * g, p[2] * g, p[3]] for p in frame]


def tint(frame, color, amount):
    out = []
    for p in frame:
        a = p[3]
        if a <= 0:
            out.append(list(p))
            continue
        out.append([(p[i] / a + (color[i] - p[i] / a) * amount) * a for i in range(3)] + [a])
    return out


def blur(frame, sigma):
    """The two-dimensional kernel, summed directly. Outside the frame is transparent black, and
    the result is the frame's own size: an adjustment layer's picture is the frame."""
    r = ceil(3 * sigma)
    if r == 0:
        return [list(p) for p in frame]
    one = [exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    total = sum(one)
    one = [v / total for v in one]
    out = []
    for y in range(H):
        for x in range(W):
            acc = [0.0] * 4
            for j in range(-r, r + 1):
                for i in range(-r, r + 1):
                    sx, sy = x + i, y + j
                    if 0 <= sx < W and 0 <= sy < H:
                        k = one[i + r] * one[j + r]
                        for c in range(4):
                            acc[c] += frame[sy * W + sx][c] * k
            out.append(acc)
    return out


EFFECTS = {"exposure": exposure, "tint": tint, "blur": blur}


def run_stack(frame, stack):
    for kind, *args in stack:
        frame = EFFECTS[kind](frame, *args)
    return frame


# --- a case ---------------------------------------------------------------------------------

def coverage(layer, frame_no):
    """How much of each pixel the adjustment layer covers: its opaque rectangle, cut by its mask
    in its own space, moved by a whole-pixel shift, and cut again by its matte."""
    cov = []
    matte = decoded("matte") if layer.get("matte") else None
    dx = layer.get("shift", 0)
    for y in range(H):
        for x in range(W):
            sx = x - dx  # the pixel of the rectangle this frame pixel shows
            c = 1.0 if 0 <= sx < W else 0.0
            if "mask_right" in layer and not sx < layer["mask_right"]:
                c = 0.0
            if matte:
                c *= matte[y * W + x][3]
            cov.append(c)
    return cov


def active(layer, frame_no):
    return layer.get("in", 0) <= frame_no < layer.get("out", 3) and layer.get("enabled", True)


def render(case, frame_no=0):
    """Far to near, then the stack, exactly as document 21 orders a frame."""
    order = sorted(enumerate(case["layers"]), key=lambda il: -il[1].get("depth", 0))
    frame = [[0.0] * 4 for _ in range(W * H)]
    for _, layer in order:
        if not active(layer, frame_no):
            continue
        if layer["kind"] == "raster":
            src = decoded(layer["drawing"])
            frame = [over(s, d) for s, d in zip(src, frame)]
            continue
        stack = [e for e in layer["effects"] if e[0] != "off"]
        if not stack:
            continue
        adjusted = run_stack(frame, stack)
        cov = coverage(layer, frame_no)
        op = layer.get("opacity", 1.0)
        frame = [[b[i] + c * op * (e[i] - b[i]) for i in range(4)]
                 for b, e, c in zip(frame, adjusted, cov)]
    return frame


# --- the project files ----------------------------------------------------------------------

def prop(base):
    return {"base": base, "keyframes": []}


def layer_json(layer, index):
    shift = layer.get("shift", 0)
    record = {
        "id": layer["id"], "kind": layer["kind"], "name": layer["id"],
        "enabled": layer.get("enabled", True), "locked": False,
        "in_frame": layer.get("in", 0), "out_frame": layer.get("out", 3),
        "transform": {
            "anchor": prop([W / 2, H / 2]), "position": prop([W / 2 + shift, H / 2]),
            "scale": prop([100, 100]), "rotation": prop(0),
            "opacity": prop(layer.get("opacity", 1)),
        },
        "mask": None, "matte": None, "blend_mode": "normal", "effects": [],
    }
    if layer["kind"] == "raster":
        record["asset_id"] = "asset-" + layer["drawing"]
        record["source_offset_frames"] = 0
        record["exposure_spans"] = []
        # A still needs no exposures; keep the order of keys the build writes.
        record = {k: record[k] for k in ("id", "kind", "name", "asset_id", "enabled", "locked",
                                         "in_frame", "out_frame", "source_offset_frames",
                                         "transform", "exposure_spans", "mask", "matte",
                                         "blend_mode", "effects")}
    if "mask_right" in layer:
        m = layer["mask_right"]
        record["mask"] = {"vertices": [[0, 0], [m, 0], [m, H], [0, H]],
                          "enabled": True, "inverted": False}
    if layer.get("matte"):
        record["matte"] = {"layer_id": "matte", "mode": "alpha", "matte_only": True}
    if "depth" in layer:
        record["depth"] = prop(layer["depth"])
    for n, (kind, *args) in enumerate(layer.get("effects", [])):
        enabled = kind != "off"
        if not enabled:
            kind, *args = args
        type_id, params = {
            "exposure": lambda s: ("core.exposure", {"stops": s}),
            "tint": lambda c, a: ("core.tint", {"color": list(c), "amount": a}),
            "blur": lambda s: ("core.gaussian_blur", {"sigma_px": s}),
        }[kind](*args)
        record["effects"].append({"instance_id": f"fx-{index}-{n}", "type_id": type_id,
                                  "enabled": enabled, "parameters": params})
    return record


def project_json(name, case):
    layers = list(case["layers"])
    if any(l.get("matte") for l in layers):
        layers = [{"id": "matte", "kind": "raster", "drawing": "matte"}] + layers
    used = sorted({l["drawing"] for l in layers if l["kind"] == "raster"})
    return {
        "schema_version": 0,
        "project_id": "proj-" + name.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + d, "kind": "still", "name": d, "path": f"media/{d}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}
                   for d in used],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 3,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 3},
            # layer_order is bottom first here, as the cases are written.
            "layer_order": [l["id"] for l in layers],
            "layers": [layer_json(l, i) for i, l in enumerate(layers)],
        }],
    }


# --- the cases ------------------------------------------------------------------------------

BG = {"id": "bg", "kind": "raster", "drawing": "bg"}
HALF = {"id": "half", "kind": "raster", "drawing": "half"}
DOT = {"id": "dot", "kind": "raster", "drawing": "dot"}


def adj(**kw):
    return {"id": kw.pop("id", "adj"), "kind": "adjustment", **kw}


CASES = {
    "FX-ADJ-001": ("One stop brighter on everything below.",
                   {"layers": [BG, adj(effects=[("exposure", 1)])]}),
    "FX-ADJ-002": ("The same at half opacity: half way between.",
                   {"layers": [BG, adj(effects=[("exposure", 1)], opacity=0.5)]}),
    "FX-ADJ-003": ("A layer above the adjustment layer is not adjusted.",
                   {"layers": [BG, adj(effects=[("exposure", 1)]), HALF]}),
    "FX-ADJ-004": ("A mask on the adjustment layer: only the left three columns.",
                   {"layers": [BG, adj(effects=[("exposure", 1)], mask_right=3)]}),
    "FX-ADJ-005": ("Moved four pixels right: only the right two columns.",
                   {"layers": [BG, adj(effects=[("exposure", 1)], shift=4)]}),
    "FX-ADJ-006": ("A matte-only layer shapes the adjustment: only the left three columns.",
                   {"layers": [BG, adj(effects=[("exposure", 1)], matte=True)]}),
    "FX-ADJ-007": ("A blur spreads one pixel over the frame and is cut off at its edge.",
                   {"layers": [DOT, adj(effects=[("blur", 1.0)])]}),
    "FX-ADJ-008": ("A full tint on a half-covered picture keeps its coverage.",
                   {"layers": [HALF, adj(effects=[("tint", (0, 0, 1), 1.0)])]}),
    "FX-ADJ-009": ("Two adjustment layers, lower one first.",
                   {"layers": [BG, adj(id="adj1", effects=[("exposure", 1)]),
                               adj(id="adj2", effects=[("tint", (0, 0, 1), 0.5)])]}),
    "FX-ADJ-010": ("The same two, the other way up.",
                   {"layers": [BG, adj(id="adj2", effects=[("tint", (0, 0, 1), 0.5)]),
                               adj(id="adj1", effects=[("exposure", 1)])]}),
    "FX-ADJ-011": ("No effects switched on: the frame is the frame without the layer.",
                   {"layers": [BG, adj(effects=[("off", "exposure", 1)])]}),
    "FX-ADJ-012": ("Behind the picture in depth: drawn first, onto nothing, so nothing changes.",
                   {"layers": [BG, adj(effects=[("exposure", 1)], depth=500)]}),
    "FX-ADJ-013": ("Only between its in and out frames (frame 1 only).",
                   {"layers": [BG, adj(effects=[("exposure", 1)], **{"in": 1, "out": 2})]}),
}


def fmt(v):
    return f"{v:.9g}"


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, case) in CASES.items():
        frames = [0, 1, 2] if fx == "FX-ADJ-013" else [0]
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

    (OUT / "expected_adjust.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = expected["cases"]
    bg = render({"layers": [BG]})
    assert c["FX-ADJ-011"]["frames"]["0"] == bg
    assert c["FX-ADJ-012"]["frames"]["0"] == bg
    assert c["FX-ADJ-013"]["frames"]["0"] == bg == c["FX-ADJ-013"]["frames"]["2"]
    assert c["FX-ADJ-013"]["frames"]["1"] == c["FX-ADJ-001"]["frames"]["0"]
    assert c["FX-ADJ-009"]["frames"]["0"] != c["FX-ADJ-010"]["frames"]["0"]
    four = c["FX-ADJ-004"]["frames"]["0"]
    assert four[:3] == c["FX-ADJ-001"]["frames"]["0"][:3] and four[3:6] == bg[3:6]
    assert c["FX-ADJ-006"]["frames"]["0"] == four
    assert all(p[3] == 128 / 255 for p in c["FX-ADJ-008"]["frames"]["0"])
    assert sum(p[3] for p in c["FX-ADJ-007"]["frames"]["0"]) < 1, "the edge kept everything"


if __name__ == "__main__":
    main()
