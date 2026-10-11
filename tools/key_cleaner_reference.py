"""Key Cleaner, worked a second way.

D-503 (EFFECTS.md row 311, P0-20) adds Key Cleaner (`core.key_cleaner`), after After Effects'
Key Cleaner (Keying). Adobe says it "recovers alpha-channel detail from a scene keyed by a
typical keying effect", that its edge radius defaults to 10, that Alpha Contrast is contrast in
the alpha channel's semitransparent regions along the edges, and that Strength lets you back off
the final effect. It publishes no formula, so the rule below is ours, built on D-352's matte
refinement kit (the guided filter of He, Sun and Tang), and Reduce Chatter, a smoothing over
time, is not built. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

**The rule.** Three settings: Additional Edge Radius R (0 to 100 pixels, 10 when added), Alpha
Contrast k (0 to 100 per cent, 0 when added) and Strength s (0 to 100 per cent, 100 when added).
The refined layer is D-352's Refine Soft Matte with edge radius R, feather 0, contrast k, shift
edge 0 and decontaminate off (document 21's rule for it, worked here by
`tools/matte_refine_reference.py`'s `refine`): r_e = floor(R); the edge region is the pixels
whose r_e square holds a covered pixel and a pixel not fully covered, and there the covering
becomes the guided filter of the covering by the pixels' straight sRGB colours at r_e, held
inside 0 to 1, so it follows the colours; then ramp(a, 0.5, 1 - k / 100) steepens the part-
covered pixels; an empty pixel that gains covering takes the covering-weighted average colour
of its 2 r_e square. The output, in premultiplied linear light, is the original layer plus s /
100 of the way to the refined one. R = 0 with k = 0, or s = 0, leaves the layer as it was.

**This file never runs the build's code path.** It works in double precision with numpy on the
drawing's exact 8-bit values, and asserts that every case's numbers move by less than a tenth of
the tolerance when the inputs are rounded to single precision, and that the one hard step
(contrast 100) has every covering clear of its threshold by a hundred-thousandth.

Every case is a composition 24 by 16, five frames, holding `tools/matte_refine_reference.py`'s
drawing, the same size, unmoved unless the case says. The drawing goes into
`Fixtures/key_cleaner/media`, the projects into `Fixtures/key_cleaner`, and the expected frames
into `Fixtures/key_cleaner/expected_key_cleaner.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/key_cleaner_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import matte_refine_reference as MR  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "key_cleaner"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = MR.W, MR.H, MR.FRAMES
KEYS = ("additional_edge_radius", "alpha_contrast", "strength")


def drawn():
    return MR.drawn()


def key_cleaner(P, c, n):
    R, k, s = (value_at(c[key], n) for key in KEYS)
    a = P[..., 3]
    if k == 100:  # the one hard step: every covering clear of 0.5 before the ramp
        guided = MR.guided(MR.to_srgb(MR.straight(P)), a, math.floor(R)) if R >= 1 else a
        edge = MR.edge_region(a, math.floor(R)) if R >= 1 else np.zeros(a.shape, bool)
        before = np.where(edge, np.clip(guided, 0, 1), a)
        assert np.abs(before - 0.5).min() > 1e-5, before
    refined = MR.refine(P, MR.soft(edge_radius=R, contrast=k, decontaminate="off"), n)
    return P + (s / 100) * (refined - P)


def cleaner(additional_edge_radius=10, alpha_contrast=0, strength=100, shift=0):
    return {"additional_edge_radius": additional_edge_radius, "alpha_contrast": alpha_contrast,
            "strength": strength, "shift": shift}


def render(c, n, P=None):
    P = drawn() if P is None else P
    return MR.moved(key_cleaner(P, c, n), c["shift"])


CASES = {
    "FX-KCLEAN-001": ("Key Cleaner as added: Additional Edge Radius 10, Alpha Contrast 0, "
                      "Strength 100. Within 10 pixels of the matte's edge, on so small a drawing "
                      "nearly all of it, the covering is fitted to the colours: the strands' "
                      "covering follows their orange and teal rows, the rectangle's teal ring "
                      "and right side lose some, and empty pixels near the matte gain a little, "
                      "in the colours about them.", cleaner(), (0,)),
    "FX-KCLEAN-002": ("Additional Edge Radius 4: the fit is worked over smaller squares, so it "
                      "follows the colours more closely.", cleaner(4), (0,)),
    "FX-KCLEAN-003": ("Additional Edge Radius 0, Alpha Contrast 0: nothing to do, the output is "
                      "the drawing.", cleaner(0), (0,)),
    "FX-KCLEAN-004": ("Additional Edge Radius 4, Alpha Contrast 50: FX-KCLEAN-002's covering "
                      "steepened, below a quarter gone and above three quarters full.",
                      cleaner(4, 50), (0,)),
    "FX-KCLEAN-005": ("Additional Edge Radius 4, Alpha Contrast 100: every pixel full or empty, "
                      "full where FX-KCLEAN-002's covering is above a half.",
                      cleaner(4, 100), (0,)),
    "FX-KCLEAN-006": ("Additional Edge Radius 0, Alpha Contrast 50: no fit, only the part-"
                      "covered pixels steepened: the strands and the half-covered right edge.",
                      cleaner(0, 50), (0,)),
    "FX-KCLEAN-007": ("FX-KCLEAN-002 at Strength 50: halfway between the drawing and "
                      "FX-KCLEAN-002, premultiplied.", cleaner(4, strength=50), (0,)),
    "FX-KCLEAN-008": ("Strength 0: the output is the drawing.", cleaner(strength=0), (0,)),
    "FX-KCLEAN-009": ("Additional Edge Radius keyed from 0 at frame 0 to 6 at frame 4: whole "
                      "pixels, so frame 1 (1.5) works at 1, frame 2 at 3 and frame 3 (4.5) at "
                      "4.", cleaner(keyed((0, 0), (4, 6))), (0, 1, 2, 3)),
    "FX-KCLEAN-010": ("Strength keyed from 0 at frame 0 to 100 at frame 4: the drawing, then "
                      "halfway at frame 2, then FX-KCLEAN-001.",
                      cleaner(strength=keyed((0, 0), (4, 100))), (0, 2, 4)),
    "FX-KCLEAN-011": ("FX-KCLEAN-001 moved three pixels right: worked in the drawing's own "
                      "space, so the same, moved.", cleaner(shift=3), (0,)),
}

INVALID = {
    "FX-KCLEAN-012": ("Additional Edge Radius 101, above 100.", cleaner(101)),
    "FX-KCLEAN-013": ("Additional Edge Radius -1, below 0.", cleaner(-1)),
    "FX-KCLEAN-014": ("Alpha Contrast 101, above 100.", cleaner(alpha_contrast=101)),
    "FX-KCLEAN-015": ("Alpha Contrast -1, below 0.", cleaner(alpha_contrast=-1)),
    "FX-KCLEAN-016": ("Strength 101, above 100.", cleaner(strength=101)),
    "FX-KCLEAN-017": ("Strength -1, below 0.", cleaner(strength=-1)),
    "FX-KCLEAN-018": ("Strength keyed to 101 at frame 4, above 100.",
                      cleaner(strength=keyed((0, 100), (4, 101)))),
}


def project_json(fx, c):
    return {
        "schema_version": 0,
        "project_id": "proj-" + fx.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-subject", "kind": "still", "name": "subject",
                    "path": "media/subject.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "raster", "name": "art", "asset_id": "asset-subject",
                "enabled": True, "locked": False, "in_frame": 0, "out_frame": FRAMES,
                "source_offset_frames": 0,
                "transform": {
                    "anchor": prop([W / 2, H / 2]),
                    "position": prop([W / 2 + c["shift"], H / 2]),
                    "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
                },
                "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": "core.key_cleaner",
                             "enabled": True,
                             "parameters": {k: setting_json(c[k]) for k in KEYS}}],
            }],
        }],
    }


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def plain(c):
    return MR.as_list(MR.moved(drawn(), c["shift"]))


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in MR.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    rounded = drawn().astype(np.float32).astype(np.float64)
    for fx, (says, c, frames) in sorted(CASES.items()):
        rendered = {}
        for f in frames:
            got = render(c, f)
            moved_by = float(np.abs(render(c, f, rounded) - got).max())
            assert moved_by < TOLERANCE / 10, (fx, f, moved_by)
            rendered[str(f)] = MR.as_list(got)
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in sorted(INVALID.items()):
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    expected["cases"] = dict(sorted(expected["cases"].items()))

    (OUT / "expected_key_cleaner.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: {f: np.array(px) for f, px in v["frames"].items()}
         for fx, v in expected["cases"].items()}
    P = drawn()
    flat = np.array(MR.as_list(P))
    p = P[..., 3]
    at = lambda x, y: y * W + x  # noqa: E731
    alpha = lambda f: f[:, 3].reshape(H, W)  # noqa: E731

    # As added: Refine Soft Matte's fit at radius 10 with nothing else, so FX-MREF-023's
    # covering (decontamination moves only colour).
    one = alpha(c["FX-KCLEAN-001"]["0"])
    mref = MR.refine(P, MR.soft(), 0)[..., 3]
    assert np.allclose(one, mref, atol=1e-12)
    assert abs(one[4, 2] - one[5, 2]) > abs(p[4, 2] - p[5, 2])  # follows the rows' colours
    assert one[3, 6] < 0.9 and one[8, 16] < 0.9 and one[8, 12] > 0.9
    assert ((one > 0) & (p == 0)).any()  # empty pixels gain covering
    two = alpha(c["FX-KCLEAN-002"]["0"])
    assert np.abs(two - one).max() > 1e-3
    assert np.array_equal(c["FX-KCLEAN-003"]["0"], flat)
    four = alpha(c["FX-KCLEAN-004"]["0"])
    assert np.allclose(four, np.clip((two - 0.25) / 0.5, 0, 1), atol=1e-12)
    five = alpha(c["FX-KCLEAN-005"]["0"])
    assert set(np.unique(five)) <= {0.0, 1.0} and np.array_equal(five > 0, two > 0.5)
    six = alpha(c["FX-KCLEAN-006"]["0"])
    hard = (p == 0) | (p == 1)
    assert np.array_equal(six[hard], p[hard])
    assert six[5, 1] < p[5, 1] and six[5, 3] > p[5, 3]  # 64 down, 224 up
    seven = c["FX-KCLEAN-007"]["0"]
    assert np.allclose(seven, (flat + c["FX-KCLEAN-002"]["0"]) / 2, atol=1e-12)
    assert np.array_equal(c["FX-KCLEAN-008"]["0"], flat)
    k9 = c["FX-KCLEAN-009"]
    for f, r in (("0", 0), ("1", 1), ("2", 3), ("3", 4)):
        assert np.array_equal(k9[f], np.array(MR.as_list(render(cleaner(r), int(f)))))
    assert np.array_equal(k9["3"], c["FX-KCLEAN-002"]["0"])
    k10 = c["FX-KCLEAN-010"]
    assert np.array_equal(k10["0"], flat)
    assert np.allclose(k10["2"], (flat + c["FX-KCLEAN-001"]["0"]) / 2, atol=1e-12)
    assert np.allclose(k10["4"], c["FX-KCLEAN-001"]["0"], atol=1e-12)
    eleven = c["FX-KCLEAN-011"]["0"]
    for y in range(H):
        assert np.array_equal(eleven[at(3, y):at(W, y)], c["FX-KCLEAN-001"]["0"][at(0, y):at(W - 3, y)])

    for name, frames in c.items():
        for px in frames.values():
            assert (px[:, 3] >= -1e-12).all() and (px[:, 3] <= 1 + 1e-12).all(), name
            assert (px[:, :3] >= -1e-12).all() and (px[:, :3] <= px[:, 3:] + 1e-12).all(), name
    print("checked")


if __name__ == "__main__":
    main()
