"""D-321's Gaussian Blur in After Effects' Blurriness, worked a second way.

The owner asked on 2026-10-04 for After Effects' own Gaussian Blur numbers, checked against
After Effects examples on the internet, since they no longer have After Effects. Two players of
After Effects' own files take a Blurriness B as a Gaussian of sigma 0.3 B: lottie-web
(`SVGGaussianBlurEffect.js`: "Empirical value, matching AE's blur appearance. var
kBlurrinessToSigma = 0.3;") and Skia's Skottie (`SkottiePriv.h`: "Close-enough to AE. static
constexpr float kBlurSizeToSigma = 0.3f;").

`core.gaussian_blur` gains `units`: "sigma", what a file without it means, document 21's rule
exactly; or "blurriness", which a Gaussian Blur added from now on writes. In Blurriness:

- sigma = 0.3 B;
- the kernel reaches r = ceil(6.5 sigma) pixels, not ceil(3 sigma), and is normalised after
  that cut as document 21's is. Three sigmas leave the last tap at 1% of the middle one, which
  an Exposure lifted far enough after the blur shows as a straight edge three sigmas out (the
  box round tutorials 2's and 5's glows in P-26). The tail past 6.5 sigmas weighs under 4e-11.

Edges, Blur Dimensions and everything else are as before.

Every case is a composition 40 pixels by 12 holding one drawing the same size, `block`, a 4 by 4
orange square from (0, 4) to (3, 7) on clear, against the left edge, unmoved. The drawing goes into
`Fixtures/blurriness/media`, the projects into `Fixtures/blurriness`, and the expected frames
into `Fixtures/blurriness/expected_blurriness.json`.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, summing the two-dimensional kernel directly at each pixel.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/blurriness_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import edges_reference as E  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402

W, H = 40, 12
FRAMES = 5
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "blurriness"
TOLERANCE = 2e-5  # document 25's default for a filter
ORANGE = (255, 128, 0, 255)


# --- the rule -------------------------------------------------------------------------------

def taps(number, units):
    """The normalised one-dimensional kernel, index 0 being r pixels before the middle."""
    sigma, reach = (0.3 * number, 6.5) if units == "blurriness" else (number, 3.0)
    r = math.ceil(reach * sigma) if sigma > 0 else 0
    if r == 0:
        return [1.0]
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    total = sum(one)
    return [v / total for v in one]


def blur(px, number, units, edges, dimensions):
    """The blurred drawing at the composition's own pixels (the drawing fills it, unmoved)."""
    k = taps(number, units)
    r = len(k) // 2
    across = k if dimensions != "vertical" else [1.0 if i == r else 0.0 for i in range(2 * r + 1)]
    down = k if dimensions != "horizontal" else [1.0 if i == r else 0.0 for i in range(2 * r + 1)]

    def pixel(sx, sy):
        if edges == "repeat":
            sx, sy = min(max(sx, 0), W - 1), min(max(sy, 0), H - 1)
        elif not (0 <= sx < W and 0 <= sy < H):
            return None
        return px[sy * W + sx]

    out = []
    for y in range(H):
        for x in range(W):
            acc = [0.0] * 4
            for j in range(-r, r + 1):
                if down[j + r] == 0:
                    continue
                for i in range(-r, r + 1):
                    if across[i + r] == 0:
                        continue
                    p = pixel(x + i, y + j)
                    if p:
                        for c in range(4):
                            acc[c] += p[c] * across[i + r] * down[j + r]
            out.append(acc)
    return out


def exposure(px, stops):
    return [[p[c] * 2 ** stops for c in range(3)] + [p[3]] for p in px]


# --- the drawing and the cases --------------------------------------------------------------

DRAWING = [[ORANGE if x <= 3 and 4 <= y <= 7 else (0, 0, 0, 0) for x in range(W)]
           for y in range(H)]


def case(number, units="blurriness", edges=None, dimensions=None, stops=None, float_depth=False):
    """`units` None: the file does not say. `stops`: an Exposure after the blur."""
    return {"number": number, "units": units, "edges": edges, "dimensions": dimensions,
            "stops": stops, "float": float_depth}


def render(c):
    px = [D.working(p) for row in DRAWING for p in row]
    px = blur(px, c["number"], c["units"] or "sigma", c["edges"] or "transparent",
              c["dimensions"] or "both")
    return exposure(px, c["stops"]) if c["stops"] is not None else px


def plain():
    return [D.working(p) for row in DRAWING for p in row]


CASES = {
    "FX-BLURRY-001": ("Units blurriness, Blurriness 10: a Gaussian of sigma 3 reaching 20 "
                      "pixels, so the square's orange spreads softly well past nine pixels "
                      "out, where document 21's kernel of sigma 3 stops.", case(10)),
    "FX-BLURRY-002": ("The same number in a file without units: document 21's sigma 10, "
                      "exactly as before D-321, far softer than FX-BLURRY-001.",
                      case(10, units=None)),
    "FX-BLURRY-003": ("Units written \"sigma\": FX-BLURRY-002 exactly.", case(10, units="sigma")),
    "FX-BLURRY-004": ("Units blurriness, Blurriness 10, edges repeat: the layer does not grow, "
                      "and past its left edge the square's own orange is read, so the left "
                      "edge stays strong.", case(10, edges="repeat")),
    "FX-BLURRY-005": ("Units blurriness, Blurriness 10, Blur Dimensions horizontal: spread "
                      "across only, the rows above and below the square still clear.",
                      case(10, dimensions="horizontal")),
    "FX-BLURRY-006": ("Units blurriness, Blurriness 0: the drawing untouched.", case(0)),
    "FX-BLURRY-007": ("A Float composition: units blurriness, Blurriness 4 (sigma 1.2, "
                      "reaching 8 pixels), then Exposure +6. The light falls away smoothly "
                      "for eight pixels right of the square; document 21's kernel of sigma "
                      "1.2 would stop dead four pixels out, the straight edge P-26 saw.",
                      case(4, stops=6, float_depth=True)),
}

INVALID = {
    "FX-BLURRY-008": ("Units \"Blurriness\": the word is exact, so a capital is not it.",
                      case(10, units="Blurriness")),
    "FX-BLURRY-009": ("Units \"pixels\", which is not one.", case(10, units="pixels")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    params = {"sigma_px": setting_json(c["number"])}
    for word in ("units", "edges", "dimensions"):
        if c[word] is not None:
            params[word] = c[word]
    effects = [{"instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": True,
                "parameters": params}]
    if c["stops"] is not None:
        effects.append({"instance_id": "fx-0-1", "type_id": "core.exposure", "enabled": True,
                        "parameters": {"stops": setting_json(c["stops"])}})
    comp = {
        "id": "comp-main", "name": "Main", "width": W, "height": H,
        "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
        "start_frame": 0, "duration_frames": FRAMES,
        "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
        "layer_order": ["art"],
        "layers": [{
            "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-block",
            "enabled": True, "locked": False, "in_frame": 0, "out_frame": FRAMES,
            "source_offset_frames": 0,
            "transform": {"anchor": prop([W / 2, H / 2]), "position": prop([W / 2, H / 2]),
                          "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1)},
            "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
            "effects": effects,
        }],
    }
    if c["float"]:
        comp["float_depth"] = True
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-block", "kind": "still", "name": "block",
                    "path": "media/block.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [comp],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "block.png").write_bytes(S.png(DRAWING))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": render(c)}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(), "4": plain()},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_blurriness.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731

    # Sigma units are document 21's rule: edges_reference's Gaussian, summed its own way.
    layer = {"px": plain(), "left": 0, "top": 0, "w": W, "h": H}
    old = E.gaussian(layer, 10, "transparent")
    g = -old["left"]
    for y in range(H):
        for x in range(W):
            a, b = c["FX-BLURRY-002"][at(x, y)], old["px"][(y + g) * old["w"] + x + g]
            assert all(abs(p - q) < 1e-12 for p, q in zip(a, b)), (x, y)
    assert c["FX-BLURRY-003"] == c["FX-BLURRY-002"]
    assert c["FX-BLURRY-001"] != c["FX-BLURRY-002"]
    # Sigma 3: the square's middle keeps far more of its orange than at sigma 10.
    assert c["FX-BLURRY-001"][at(2, 6)][3] > c["FX-BLURRY-002"][at(2, 6)][3] + 0.1
    # The tail: ten pixels right of the square (column 13) is past 3 sigmas (9) and inside
    # 6.5 sigmas (20), still lit above the tolerance; three sigmas would leave it clear.
    assert c["FX-BLURRY-001"][at(13, 6)][3] > 10 * TOLERANCE
    assert len(taps(10, "blurriness")) == 41 and len(taps(3, "sigma")) == 19
    # Repeat reads the held orange past the left edge, so column 0 is stronger than 001's.
    assert c["FX-BLURRY-004"][at(0, 6)][3] > c["FX-BLURRY-001"][at(0, 6)][3] + 0.1
    # Horizontal: rows 0 to 3 and 8 to 11 stay clear; row 6 spreads.
    five = c["FX-BLURRY-005"]
    assert all(five[at(x, y)] == [0.0] * 4 for x in range(W) for y in (0, 1, 2, 3, 8, 9, 10, 11))
    assert five[at(6, 6)][3] > 0.01
    assert c["FX-BLURRY-006"] == plain()
    # Float: past white on the square, and falling smoothly with no step for 8 pixels right
    # of it; 5 out still shows (document 21's sigma 1.2 kernel ends 4 out).
    seven = c["FX-BLURRY-007"]
    assert seven[at(1, 6)][0] > 1
    row = [seven[at(x, 6)][0] for x in range(3, 12)]
    assert all(a > b > 0 for a, b in zip(row, row[1:])), row
    assert seven[at(8, 6)][0] > 1e-3, seven[at(8, 6)]
    for name, px in c.items():
        for p in px:
            assert -1e-12 <= p[3] <= 1 + 1e-12, (name, p)


if __name__ == "__main__":
    main()
