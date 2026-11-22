"""Chromatic Aberration at the drawing's edge, worked a second way.

D-1139, accepted on 2026-11-21. D-120's `core.chromatic_aberration` reads a pixel's red at its
place scaled toward the centre and its blue at its place scaled outward. Before D-1139 a sample
past the edge of the effect's input read transparent, so on an adjustment layer over a full frame
the outer band lost its blue and showed a yellow-orange line round the frame (P-28 bug 1).

The edge rule. Each of the two samples is document 21's bilinear sample of the effect's input with
every tap's column held between 0 and the input's width - 1 and its row between 0 and its
height - 1: a point past the edge reads the nearest edge pixel (After Effects' blurs call this
Repeat Edge Pixels; D-109). Everything else is D-120's rule, unchanged: with c the centre in the
drawing's pixels, R = sqrt(W0^2 + H0^2) / 2 and k = amount / R, at the pixel whose centre is P,
d = P - c, red is the red of the sample at c + d (1 - k), blue is the blue of the sample at
c + d (1 + k), green is the pixel's own, and the covering is the largest of the two samples' and
the pixel's own. All premultiplied. The layer does not grow. After Effects has no effect of this
name (it is this program's own, D-120), so no After Effects edge behaviour was there to follow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, and holds each bilinear tap's index rather than the point.

Every case is a composition 16 pixels by 10, five frames long, drawn at frame 0:

- FX-CHROMAEDGE-001: a flat colour filling the frame, under an adjustment layer holding the
  effect at amount 12 about the middle (P-28's setting). Expected: every pixel unchanged.
- FX-CHROMAEDGE-002 and 003: a 6 by 6 opaque square, every pixel a different colour, in the
  middle of the frame with the effect on it at amount 3 and 12. Expected from the rule above.

The drawings go into `<out>/media`, the projects into `<out>`, and the expected frames into
`<out>/expected_chromatic_edge.json`; `<out>` is `Fixtures/chromatic_edge` unless given.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/chromatic_edge_reference.py [out]
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = 16, 10
FRAMES = 5
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "chromatic_edge"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def sample(pic, x, y, hold):
    """Bilinear from pixel centres; `hold` keeps every tap inside the picture (D-1139), without
    it a tap outside reads transparent (D-120 as first built)."""
    w, h, px = pic["w"], pic["h"], pic["px"]
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for j, wy in ((y0, 1 - uy), (y0 + 1, uy)):
        for i, wx in ((x0, 1 - ux), (x0 + 1, ux)):
            if hold:
                i, j2 = min(max(i, 0), w - 1), min(max(j, 0), h - 1)
            else:
                j2 = j
                if not (0 <= i < w and 0 <= j < h):
                    continue
            p = px[j2 * w + i]
            for c in range(4):
                out[c] += p[c] * wx * wy
    return out


def aberration(pic, amount, center, hold=True):
    w, h = pic["w"], pic["h"]
    cx, cy = center[0] / 100 * w, center[1] / 100 * h
    k = amount / (math.sqrt(w * w + h * h) / 2)
    out = []
    for n, p in enumerate(pic["px"]):
        dx, dy = n % w + 0.5 - cx, n // w + 0.5 - cy
        red = sample(pic, cx + dx * (1 - k), cy + dy * (1 - k), hold)
        blue = sample(pic, cx + dx * (1 + k), cy + dy * (1 + k), hold)
        out.append([red[0], p[1], blue[2], max(red[3], p[3], blue[3])])
    return {"w": w, "h": h, "px": out}


# --- the drawings ---------------------------------------------------------------------------

FLAT = (246, 214, 190, 255)            # the skin colour #f6d6be, with plenty of blue
SQ, SQ_LEFT, SQ_TOP = 6, 5, 2          # the square: 6 by 6, its top left at (5, 2)
DRAWINGS = {
    "flat": [[FLAT] * W for _ in range(H)],
    "square": [[(30 + 40 * x, 230 - 35 * y, 60 + 20 * x + 15 * y, 255) for x in range(SQ)]
               for y in range(SQ)],
}


def working(p):
    """Document 21's PNG reading: sRGB to linear, then premultiplied (all opaque here)."""
    return [srgb_to_linear(p[c] / 255) * p[3] / 255 for c in range(3)] + [p[3] / 255]


def decoded(name):
    rows = DRAWINGS[name]
    return {"w": len(rows[0]), "h": len(rows), "px": [working(p) for row in rows for p in row]}


def placed(pic, left, top):
    """The picture in the W by H frame at (left, top), transparent round it."""
    return [pic["px"][(y - top) * pic["w"] + x - left]
            if 0 <= x - left < pic["w"] and 0 <= y - top < pic["h"] else [0.0] * 4
            for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

CASES = {
    "FX-CHROMAEDGE-001": ("A flat #f6d6be filling the frame, under an adjustment layer holding "
                          "Chromatic Aberration amount 12 about the middle: every sample past the "
                          "frame's edge reads the edge's own colour, so every pixel is unchanged "
                          "and no band round the frame loses its blue.",
                          {"kind": "adjustment", "amount": 12}),
    "FX-CHROMAEDGE-002": ("A 6 by 6 opaque square of different colours in the middle of the "
                          "frame, the effect on it at amount 3: blue sampled past the square's "
                          "edge reads the nearest edge pixel, so the square keeps blue to its "
                          "edge, keeps its full covering, and nothing is drawn outside it.",
                          {"kind": "square", "amount": 3}),
    "FX-CHROMAEDGE-003": ("FX-CHROMAEDGE-002 at amount 12, past the square's half diagonal 4.24: "
                          "the red picture is turned over through the centre and both samples "
                          "fall mostly past the edge, each reading the nearest edge pixel.",
                          {"kind": "square", "amount": 12}),
}


def near(a, b):
    return all(abs(p - q) < 1e-12 for u, v in zip(a, b) for p, q in zip(u, v))


def render(c, hold=True):
    if c["kind"] == "adjustment":
        flat = decoded("flat")
        px = aberration(flat, c["amount"], (50, 50), hold)["px"]
        # Under the hold rule every sample of a flat picture is the flat colour; the weights'
        # rounding (1e-16) is not written, the claim is.
        return flat["px"] if hold and near(px, flat["px"]) else px
    return placed(aberration(decoded("square"), c["amount"], (50, 50), hold), SQ_LEFT, SQ_TOP)


# --- the project files ----------------------------------------------------------------------

def effect(c):
    return [{"instance_id": "fx-0-0", "type_id": "core.chromatic_aberration", "enabled": True,
             "parameters": {"amount": c["amount"], "center": [50, 50]}}]


def layer(id, kind, anchor, effects=(), **extra):
    return {"id": id, "kind": kind, "name": id, **extra, "enabled": True, "locked": False,
            "in_frame": 0, "out_frame": FRAMES,
            "transform": {"anchor": prop(list(anchor)), "position": prop(list(anchor)),
                          "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1)},
            "mask": None, "matte": None, "blend_mode": "normal", "effects": list(effects)}


def raster(name, anchor, effects=()):
    return layer("art", "raster", anchor, effects, asset_id="asset-" + name,
                 source_offset_frames=0, exposure_spans=[])


def project_json(fx, c):
    if c["kind"] == "adjustment":
        name = "flat"
        layers = [raster(name, (W / 2, H / 2)), layer("adj", "adjustment", (W / 2, H / 2),
                                                      effect(c))]
    else:
        name = "square"
        # Anchor at the square's middle, placed at (SQ_LEFT, SQ_TOP) + that middle.
        art = raster(name, (SQ / 2, SQ / 2), effect(c))
        art["transform"]["position"] = prop([SQ_LEFT + SQ / 2, SQ_TOP + SQ / 2])
        layers = [art]
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-" + name, "kind": "still", "name": name,
                    "path": f"media/{name}.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": [l["id"] for l in layers],  # bottom first
            "layers": layers,
        }],
    }


def main(out):
    (out / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (out / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        project = f"{fx.lower().replace('-', '_')}.json"
        (out / project).write_text(json.dumps(project_json(fx, c), indent=2) + "\n",
                                   encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": project, "frames": {"0": render(c)}}
    (out / "expected_chromatic_edge.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)
    print_changes(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    got = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    flat = decoded("flat")["px"]
    # 001: unchanged, exactly; and the rule as first built leaves the blueless band P-28 measured.
    assert got["FX-CHROMAEDGE-001"] == flat
    old = render(CASES["FX-CHROMAEDGE-001"][1], hold=False)
    band = [i for i in range(W * H) if old[i][2] == 0 and old[i][3] == 1]
    assert band and at(0, 0) in band and at(W // 2, H // 2) not in band
    print(f"FX-CHROMAEDGE-001: unchanged; under the rule as first built, {len(band)} of "
          f"{W * H} pixels lose all their blue")
    square = placed(decoded("square"), SQ_LEFT, SQ_TOP)
    blues = sorted(p[2] for p in decoded("square")["px"])
    for fx in ("FX-CHROMAEDGE-002", "FX-CHROMAEDGE-003"):
        px = got[fx]
        for i in range(W * H):
            inside = square[i][3] == 1
            # Nothing outside the square; inside, its own green and full covering, and a blue
            # that is one of the square's (or between them), never zero.
            assert px[i] == ([0.0] * 4 if not inside else px[i]), (fx, i)
            if inside:
                assert px[i][1] == square[i][1] and px[i][3] == 1, (fx, i)
                assert blues[0] - 1e-12 <= px[i][2] <= blues[-1] + 1e-12, (fx, i)
        old = render(CASES[fx][1], hold=False)
        lost = sum(1 for i in range(W * H) if square[i][3] == 1 and old[i][2] == 0)
        print(f"{fx}: blue kept at every pixel of the square; as first built, {lost} lose it")
    # The two ways of holding agree: holding the taps is holding the point inside the centres.
    pic = decoded("square")
    for x, y in ((-3.2, 2.7), (8.9, -1.4), (0.2, 5.9), (6.6, 6.6), (3.1, 2.4)):
        held = (min(max(x, 0.5), SQ - 0.5), min(max(y, 0.5), SQ - 0.5))
        a, b = sample(pic, x, y, True), sample(pic, *held, False)
        assert all(abs(p - q) < 1e-12 for p, q in zip(a, b)), (x, y)
    print("checked")


def print_changes(expected):
    """Document 25's tables: each case's pixels that differ from the frame without the effect."""
    before = {"FX-CHROMAEDGE-001": decoded("flat")["px"]}
    sq = placed(decoded("square"), SQ_LEFT, SQ_TOP)
    f = lambda p: " ".join(f"{v:.7g}" for v in p)  # noqa: E731
    for fx, v in expected["cases"].items():
        px, base = v["frames"]["0"], before.get(fx, sq)
        changed = [i for i in range(W * H) if px[i] != base[i]]
        print(f"\n{fx} frame 0, {len(changed)} pixels change:\n")
        if changed:
            print("| x | y | without the effect | with it |\n| --- | --- | --- | --- |")
            for i in changed:
                print(f"| {i % W} | {i // W} | {f(base[i])} | {f(px[i])} |")


if __name__ == "__main__":
    main(Path(sys.argv[1]) if len(sys.argv) > 1 else OUT)
