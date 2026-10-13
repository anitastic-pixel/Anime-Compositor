"""Smear, worked a second way.

D-607 adds `core.mask_smear`, after After Effects' Smear (Distort): the drawing inside one of the
layer's masks is carried along as that mask is moved, turned and scaled, the rest of the drawing
following, held still at a second mask or at the drawing's own edge. (`core.smear` is D-392's
Smear, a different effect.) After Effects names its controls and publishes no formula; the rule
is this program's own and D-606's Reshape's (`tools/reshape_reference.py`), the destination
worked from the source. Nothing is ported.

The settings. `source_mask` and `boundary_mask`, 0 to 1000, the mask numbered so (its floor
taken), 0 for none; 0 and 0 when added. `mask_offset`, x then y in pixels, -10000 to 10000,
(0, 0) when added; `mask_rotation`, degrees clockwise, -3600 to 3600, 0 when added; `mask_scale`,
per cent, 0 to 1000, 100 when added; `percent`, 0 to 100, 100 when added; `elasticity`, one of
Reshape's seven words, `normal` when added. Every number is keyable.

The masks are chosen as Reshape's are (D-606): a source of 0 draws the drawing as it is and says
nothing; a boundary of 0 is the drawing's own rectangle; a mask named and not found draws the
drawing as it is and says EFFECT_PATH_MISSING every frame.

The rule. The source outline is resampled to Reshape's 64 points S_k; c is their mean (summed in
order, divided by 64). The destination point D_k is the source point moved about c: with
s = `mask_scale` / 100, a = `mask_rotation` in radians and (dx, dy) = s (S_k - c),
D_k = c + `mask_offset` + (dx cos a - dy sin a, dx sin a + dy cos a), clockwise on the screen,
y down. From there the drawing is drawn as Reshape draws it from S_k and D_k: pulled
`percent` / 100 of the way, the boundary held, at the elasticity's power, through D-601's
backward field. With `percent` 0 the drawing is untouched.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says, with masks of mode None. The drawing goes into
`Fixtures/mask_smear/media`, the projects into `Fixtures/mask_smear`, and the expected frames
into `Fixtures/mask_smear/expected_mask_smear.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mask_smear_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, OVERSHOOT  # noqa: E402
import bulge_reference as B  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402
from mask_reference import flatten, points_at  # noqa: E402
from mask_reference import keyed as moving_path  # noqa: E402
import reshape_reference as RS  # noqa: E402
from reshape_reference import none, SQUARE, FENCE, LOW, MISSING_PATH, resample  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = RS.W, RS.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mask_smear"
TOLERANCE = RS.TOLERANCE
NAMES = ("source_mask", "boundary_mask", "mask_offset", "mask_rotation", "mask_scale", "percent",
         "elasticity")
RANGES = {"source_mask": (0, 1000), "boundary_mask": (0, 1000), "mask_offset": (-10000, 10000),
          "mask_rotation": (-3600, 3600), "mask_scale": (0, 1000), "percent": (0, 100)}
MASKS = [none(SQUARE), none(FENCE)]


def moved(src, offset, rotation, scale):
    """The destination points: `src` moved about its mean."""
    cx = cy = 0.0
    for x, y in src:
        cx += x
        cy += y
    cx, cy = cx / len(src), cy / len(src)
    s, a = scale / 100, math.radians(rotation)
    out = []
    for x, y in src:
        dx, dy = s * (x - cx), s * (y - cy)
        out.append((cx + offset[0] + (dx * math.cos(a) - dy * math.sin(a)),
                    cy + offset[1] + (dx * math.sin(a) + dy * math.cos(a))))
    return out


def case(masks=None, source_mask=1, boundary_mask=0, mask_offset=(0, 0), mask_rotation=0,
         mask_scale=100, percent=100, elasticity="normal", shift=0):
    return {"drawing": "stripes", "shift": shift, "masks": MASKS if masks is None else masks,
            "source_mask": source_mask, "boundary_mask": boundary_mask, "mask_offset": mask_offset,
            "mask_rotation": mask_rotation, "mask_scale": mask_scale, "percent": percent,
            "elasticity": elasticity}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (tuple, list)):
        return [min(hi, max(lo, u)) for u in v]
    return min(hi, max(lo, v))


def chosen(c, k, frame_no):
    i = math.floor(held(c, k, frame_no)) - 1
    if i < 0:
        return None
    if i >= len(c["masks"]):
        return MISSING_PATH
    m = c["masks"][i]
    pts = points_at(m, frame_no)
    if not m.get("enabled", True) or len(pts) < 2:
        return MISSING_PATH
    return flatten(pts)


def smear(layer, c, frame_no):
    src, bound = chosen(c, "source_mask", frame_no), chosen(c, "boundary_mask", frame_no)
    if MISSING_PATH in (src, bound):
        return layer, True
    t = held(c, "percent", frame_no) / 100
    if src is None or t == 0:
        return layer, False
    a = resample(src)
    b = moved(a, held(c, "mask_offset", frame_no), held(c, "mask_rotation", frame_no),
              held(c, "mask_scale", frame_no))
    steps, gx, gy = RS.field(a, b, bound, t, RS.ELASTICITY[c["elasticity"]], W, H)
    return M.draw_field(layer, steps, gx, gy, W, H), False


def render(c, frame_no):
    return frame(smear(B.drawn_layer(c["drawing"]), c, frame_no)[0], c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-SMEAR-001": ("The settings as they start: no source mask: the drawing, untouched.",
                     case(source_mask=0), [0]),
    "FX-SMEAR-002": ("Source mask 1, a square from (4, 3) to (8, 7), offset three pixels right: "
                     "the stripes in the square carried right, the drawing's edges held; "
                     "FX-RESHAPE-002's frame, whose destination is that square moved so.",
                     case(mask_offset=(3, 0)), [0]),
    "FX-SMEAR-003": ("The square turned 45 degrees clockwise about its middle: the stripes in it "
                     "twisted.", case(mask_rotation=45), [0]),
    "FX-SMEAR-004": ("The square scaled to 150 per cent: the middle swollen.",
                     case(mask_scale=150), [0]),
    "FX-SMEAR-005": ("The square scaled to 50 per cent, turned 90 degrees and offset by (2, 1).",
                     case(mask_scale=50, mask_rotation=90, mask_offset=(2, 1)), [0]),
    "FX-SMEAR-006": ("FX-SMEAR-002 with boundary mask 2, (3, 1) to (13, 9): nothing outside it "
                     "moves, so columns 0 to 1 and 14 to 15 are the drawing's own.",
                     case(mask_offset=(3, 0), boundary_mask=2), [0]),
    "FX-SMEAR-007": ("FX-SMEAR-002 at Percent 50: half way.",
                     case(mask_offset=(3, 0), percent=50), [0]),
    "FX-SMEAR-008": ("FX-SMEAR-003 at Elasticity Liquid: the twist kept near the square's edges.",
                     case(mask_rotation=45, elasticity="liquid"), [0]),
    "FX-SMEAR-009": ("Mask Offset keyed from (0, 0) at frame 0 to (3, 0) at frame 4, linear: "
                     "frame 0 the drawing, frame 4 FX-SMEAR-002's.",
                     case(mask_offset=keyed((0, (0, 0)), (4, (3, 0)))), [0, 2, 4]),
    "FX-SMEAR-010": ("FX-SMEAR-002 with the layer moved three pixels right: the masks are the "
                     "layer's, so all of it moves with the layer.",
                     case(mask_offset=(3, 0), shift=3), [0]),
    "FX-SMEAR-011": ("Percent keyed from 20 at frame 0 to 100 at frame 4 by an ease that passes "
                     "its end: held at 100 at frame 2, the same as frame 4.",
                     case(mask_offset=(3, 0), percent=keyed((0, 20, OVERSHOOT), (4, 100))),
                     [0, 2, 4]),
    "FX-SMEAR-012": ("The source's path keyed from the square at frame 0 to two rows lower at "
                     "frame 4, offset three right: frame 0 FX-SMEAR-002's.",
                     case(masks=[{**moving_path((0, SQUARE), (4, LOW)), "mode": "none"},
                                 none(FENCE)], mask_offset=(3, 0)), [0, 2, 4]),
    "FX-SMEAR-013": ("Percent 0, the square offset: the drawing, untouched.",
                     case(mask_offset=(3, 0), percent=0), [0]),
}

MISSING = {
    "FX-SMEAR-014": ("Source mask 3, of two.", case(source_mask=3)),
    "FX-SMEAR-015": ("Source mask 1, switched off.",
                     case(masks=[none(SQUARE, enabled=False), none(FENCE)])),
    "FX-SMEAR-016": ("Boundary mask 3 with two masks.", case(boundary_mask=3)),
}

INVALID = {
    "FX-SMEAR-017": ("Elasticity \"rubber\", not one of the seven.", case(elasticity="rubber")),
    "FX-SMEAR-018": ("Mask Scale 1001, above 1000.", case(mask_scale=1001)),
    "FX-SMEAR-019": ("Mask Rotation 3601, above 3600.", case(mask_rotation=3601)),
    "FX-SMEAR-020": ("Mask Offset (10001, 0), its x above 10000.", case(mask_offset=(10001, 0))),
    "FX-SMEAR-021": ("Percent keyed to 101 at frame 4, above 100.",
                     case(percent=keyed((0, 100), (4, 101)))),
    "FX-SMEAR-022": ("Source mask -1, below 0.", case(source_mask=-1)),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    p = RS.project_json(fx, c, type_id="core.mask_smear", names=NAMES)
    (OUT / name).write_bytes((json.dumps(p, indent=2) + "\n").encode("utf-8"))
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in MISSING.items():
        says += (" Nothing to smear: the layer is drawn without the effect, which is kept as "
                 "written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert smear(B.drawn_layer(c["drawing"]), c, 0)[1] and render(c, 0) == before
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_mask_smear.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    at = lambda px, x, y: px[y * W + x]  # noqa: E731

    # Moving about the mean: a quarter turn of a square about its middle lands on its corners.
    sq = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]
    assert all(near(p, q) for p, q in zip(moved(sq, (0, 0), 90, 100), [sq[1], sq[2], sq[3], sq[0]]))
    assert all(near(p, q) for p, q in zip(moved(sq, (1, 2), 0, 50), [(1.5, 2.5), (2.5, 2.5), (2.5, 3.5), (1.5, 3.5)]))

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx] or "frame_warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    carried = c["FX-SMEAR-002"]["0"]
    assert c["FX-SMEAR-001"]["0"] == drawn and c["FX-SMEAR-013"]["0"] == drawn
    reshaped = RS.render(RS.case(), 0)
    assert same(carried, reshaped, 1e-9)
    looks = [c[f"FX-SMEAR-{n:03}"]["0"] for n in (2, 3, 4, 5, 6, 7, 8)]
    for i, a in enumerate(looks):
        assert not same(a, drawn), i
        for b in looks[i + 1:]:
            assert not same(a, b), i
    fenced = c["FX-SMEAR-006"]["0"]
    for y in range(H):
        for x in (0, 1, 14, 15):
            assert at(fenced, x, y) == at(drawn, x, y), (x, y)
    keyed_on = c["FX-SMEAR-009"]
    assert same(keyed_on["0"], drawn, 1e-9) and same(keyed_on["4"], carried)
    assert not same(keyed_on["2"], drawn) and not same(keyed_on["2"], carried)
    assert same(c["FX-SMEAR-010"]["0"],
                frame(smear(B.drawn_layer("stripes"), case(mask_offset=(3, 0)), 0)[0], 3))
    eased = c["FX-SMEAR-011"]
    assert value_at(keyed((0, 20, OVERSHOOT), (4, 100)), 2) > 100 and eased["2"] == eased["4"]
    assert same(eased["4"], carried)
    moving = c["FX-SMEAR-012"]
    assert same(moving["0"], carried) and not same(moving["4"], carried)
    print("checked")


if __name__ == "__main__":
    main()
