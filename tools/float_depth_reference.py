"""D-319's Float working depth, worked a second way.

The owner's D-308 approval (2026-10-04) adds After Effects' 32-bit float working depth as a
composition setting, `float_depth`: absent or false is Display, today's rule; true is Float. A
composition inside another draws in the outermost one's depth. In Float:

- Add's blend term is `B = cs + cd`, not held to 1 (document 21's Display rule is
  `min(cs + cd, 1)`).
- Screen's is Nuke's rule: `B = cs + cd - cs cd` while either straight colour is at most 1, else
  `max(cs, cd)`, because `1 - (1 - cs)(1 - cd)` turns back down once both pass white.
- Fractal Noise's value is `v = max(0, 0.5 + 0.5 F contrast / 100 + brightness / 100)`, held at
  black only; Display holds it to 0..1. Its own Screen blend takes the same Nuke rule.

The layer equation is document 21's, unchanged: with straight colours cs, cd and alphas as, ad,
`Co = (1 - as) Cd + (1 - ad) Cs + as ad B` and `Ao = as + ad - as ad`. Exposure multiplies the
premultiplied colour by 2^stops (D-90), which is how a layer gets past white.

Every case is a composition 16 pixels by 10 of Fractal Noise's card (`tools/noise_reference.py`),
unmoved. The drawing goes into `Fixtures/float_depth/media`, the projects into
`Fixtures/float_depth`, and the expected frames into
`Fixtures/float_depth/expected_float_depth.json`.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/float_depth_reference.py
"""

import json
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import fractal_noise_reference as F  # noqa: E402
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "float_depth"
W, H = F.W, F.H
FRAMES = 5
TOLERANCE = 2e-5


# --- the rule -------------------------------------------------------------------------------

def screen_float(b, c):
    return b + c - b * c if b <= 1 or c <= 1 else max(b, c)


def blend(mode, src, dst, float_depth):
    a_s, a_d = src[3], dst[3]
    cs = [src[c] / a_s if a_s > 0 else 0.0 for c in range(3)]
    cd = [dst[c] / a_d if a_d > 0 else 0.0 for c in range(3)]
    out = []
    for c in range(3):
        if mode == "add":
            b = cs[c] + cd[c] if float_depth else min(1.0, cs[c] + cd[c])
        else:
            b = screen_float(cs[c], cd[c]) if float_depth else cs[c] + cd[c] - cs[c] * cd[c]
        out.append((1 - a_s) * dst[c] + (1 - a_d) * src[c] + a_s * a_d * b)
    return out + [a_s + a_d - a_s * a_d]


def card():
    return [F.R.working(p) for row in F.DRAWINGS["card"] for p in row]


def exposure(frame, stops):
    return [[p[c] * 2 ** stops for c in range(3)] + [p[3]] for p in frame]


def noise(frame, n, float_depth):
    """Fractal Noise on a premultiplied frame, `n` its settings (size 4 and the case's)."""
    dark = [v / 255 for v in F.R.hex_color("#000000")]
    light = [v / 255 for v in F.R.hex_color("#ffffff")]
    mix = screen_float if (float_depth and n["blend"] == "screen") else F.BLENDS[n["blend"]]
    out = []
    for i, p in enumerate(frame):
        a = p[3]
        if a <= 0:
            out.append(p)
            continue
        raw = 0.5 + 0.5 * F.F(0, 0, *F.point(i % W, i // W, 4, 0, 0, 0), 4) * n["contrast"] / 100 \
            + n["brightness"] / 100
        v = max(0.0, raw) if float_depth else min(1.0, max(0.0, raw))
        g = [srgb_to_linear(d + v * (e - d)) for d, e in zip(dark, light)]
        b = [p[c] / a for c in range(3)]
        out.append([mix(b[c], g[c]) * a for c in range(3)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def fx_exposure(stops):
    return ("exposure", stops)


def fx_noise(contrast, brightness, blend_word):
    return ("noise", {"contrast": contrast, "brightness": brightness, "blend": blend_word})


def layer_frame(effects, float_depth):
    frame = card()
    for kind, arg in effects:
        frame = exposure(frame, arg) if kind == "exposure" else noise(frame, arg, float_depth)
    return frame


def render(c):
    """Bottom to top: the first layer over nothing is itself; each above in its blend mode."""
    frame = None
    for lay in c["layers"]:
        src = layer_frame(lay["effects"], c["float"])
        frame = src if frame is None else [blend(lay["blend"], s, d, c["float"])
                                           for s, d in zip(src, frame)]
    return frame


def case(float_depth, *layers):
    return {"float": float_depth,
            "layers": [{"blend": b, "effects": list(e)} for b, *e in layers]}


def effect_json(index, n, kind, arg):
    if kind == "exposure":
        type_id, params = "core.exposure", {"stops": setting_json(arg)}
    else:
        type_id = "core.fractal_noise"
        params = {"size": 4, "complexity": 4, "contrast": arg["contrast"],
                  "brightness": arg["brightness"], "evolution": 0, "speed": 0, "seed": 0,
                  "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100,
                  "blend": arg["blend"]}
        params = {k: setting_json(v) for k, v in params.items()}
    return {"instance_id": f"fx-{index}-{n}", "type_id": type_id, "enabled": True,
            "parameters": params}


def project_json(fx, c):
    ids = [f"layer{i}" for i in range(len(c["layers"]))]
    comp = {
        "id": "comp-main", "name": "Main", "width": W, "height": H,
        "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
        "start_frame": 0, "duration_frames": FRAMES,
        "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
        "layer_order": ids,
        "layers": [{
            "id": ids[i], "kind": "raster", "name": ids[i], "asset_id": "asset-card",
            "enabled": True, "locked": False, "in_frame": 0, "out_frame": FRAMES,
            "source_offset_frames": 0,
            "transform": {"anchor": prop([W / 2, H / 2]), "position": prop([W / 2, H / 2]),
                          "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1)},
            "exposure_spans": [], "mask": None, "matte": None,
            "blend_mode": "normal" if i == 0 else lay["blend"],
            "effects": [effect_json(i, n, k, a) for n, (k, a) in enumerate(lay["effects"])],
        } for i, lay in enumerate(c["layers"])],
    }
    if c["float"]:
        comp["float_depth"] = True
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-card", "kind": "still", "name": "card",
                    "path": "media/card.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [comp],
    }


PLAIN = ("normal",)
UP2 = (fx_exposure(2),)

CASES = {
    "FX-BLEND-ADDF-001": ("Float. The card, and above it in Add the card two stops up (four "
                          "times): white plus four times white is five, not held to 1.",
                          case(True, PLAIN, ("add", *UP2))),
    "FX-BLEND-ADDF-002": ("The same file in Display (no `float_depth`): Add held to 1, as "
                          "before D-319.",
                          case(False, PLAIN, ("add", *UP2))),
    "FX-BLEND-ADDF-003": ("Float. The card in Add on the card, nothing brightened: white plus "
                          "white is 2.",
                          case(True, PLAIN, ("add",))),
    "FX-BLEND-SCRF-001": ("Float. Both cards two stops up, the top in Screen: where both are "
                          "past white the brighter is kept, Nuke's rule.",
                          case(True, ("normal", *UP2), ("screen", *UP2))),
    "FX-BLEND-SCRF-002": ("Float. The top card two stops up in Screen on the plain card: one of "
                          "the two is at most 1 everywhere, so `cs + cd - cs cd` throughout.",
                          case(True, PLAIN, ("screen", *UP2))),
    "FX-BLEND-SCRF-003": ("FX-BLEND-SCRF-001 in Display: `cs + cd - cs cd` as before D-319, "
                          "which turns down past white.",
                          case(False, ("normal", *UP2), ("screen", *UP2))),
    "FX-FNOISE-HDR-001": ("Float. Fractal Noise, size 4, contrast 300, brightness 50, on the "
                          "card: the brightest clouds go past white, the darkest still stop at "
                          "black.",
                          case(True, ("normal", fx_noise(300, 50, "normal")))),
    "FX-FNOISE-HDR-002": ("FX-FNOISE-HDR-001 in Display: held to 0..1, as before D-319.",
                          case(False, ("normal", fx_noise(300, 50, "normal")))),
    "FX-FNOISE-HDR-003": ("Float. The card two stops up, then FX-FNOISE-HDR-001's noise in "
                          "Screen: where both are past white the brighter is kept.",
                          case(True, ("normal", fx_exposure(2), fx_noise(300, 50, "screen")))),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    shutil.copyfile(F.OUT / "media" / "card.png", OUT / "media" / "card.png")
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n",
                                encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": name, "frames": {"0": render(c)}}
        print(f"{fx}: {says}")
    (OUT / "expected_float_depth.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check({fx: v["frames"]["0"] for fx, v in expected["cases"].items()})


def check(c):
    top = lambda f: max(v for p in f for v in p[:3])  # noqa: E731
    assert top(c["FX-BLEND-ADDF-001"]) > 4.9, "white plus four whites is five"
    # Where the card is opaque; at its soft edge the `(1 - ad) Cs` term may pass 1 in both.
    opaque = lambda f: [p for p in f if p[3] == 1.0]  # noqa: E731
    assert top(opaque(c["FX-BLEND-ADDF-002"])) <= 1.0 + 1e-12, "Display holds Add to 1"
    assert abs(top(c["FX-BLEND-ADDF-003"]) - 2.0) < 1e-9, "white plus white is two"
    assert abs(top(c["FX-BLEND-SCRF-001"]) - 4.0) < 1e-9, "the brighter of two fours"
    assert min(v for p in c["FX-BLEND-SCRF-003"] for v in p[:3]) < 0, "Display turns down"
    assert top(c["FX-FNOISE-HDR-001"]) > 1.5, "the noise past white"
    assert top(c["FX-FNOISE-HDR-002"]) <= 1.0 + 1e-12
    held = F.render(F.case(size=4, contrast=300, brightness=50), 0)
    assert all(abs(a - b) < 1e-12 for p, q in zip(c["FX-FNOISE-HDR-002"], held)
               for a, b in zip(p, q)), "Display is D-128's rule exactly"
    # Where the noise is inside 0..1 the two depths agree.
    same = [(p, q) for p, q in zip(c["FX-FNOISE-HDR-001"], held) if max(q[:3]) < 0.99 * q[3]]
    assert same and all(abs(a - b) < 1e-12 for p, q in same for a, b in zip(p, q))


if __name__ == "__main__":
    main()
