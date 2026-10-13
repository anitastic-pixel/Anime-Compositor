"""Particle Plane's Textured Square, worked a second way.

D-714 (B-584) closes the last textured gap on `core.particle_plane` (EFFECTS.md row 356), by the
owner's decision of 2026-10-13: FX-PPLANE-055 and FX-PPTEX-024 now refuse "cube", and the plane
takes "textured_square". The rule is D-611's Textured Square as Particle Space already draws it
(`tools/particle_texture_reference.py`, here T), at the plane's own centre, radius, opacity and
turn as D-710 gives its textured discs (`tools/particle_plane_texture_reference.py`, here PTX).
Cycore publishes no maths, so the rule is ours. No text or code was copied.

The setting changed:

- `particle_type` takes one more word, after D-710's eight: "textured_square".

The rule: a square of half-side R (the plane's radius), turned by theta as D-710's textured discs
are, covering clamp(R - |a| + 0.5) clamp(R - |c| + 0.5), (a, c) the pixel centre's offset turned
into the square's frame; no texture: A times the Color Map's colour; a texture: A (Tc.rgb, Tc.a),
Tc read by D-611's levels and bilinear across the square. Everything else (Composite With
Original, the transfer modes, drawing order, a texture layer empty or missing) is D-710's.

**This file never runs the build's code path.** It reuses PTX, PL and T in its own process
(adding the word here only; no other file's fixtures are written again).

Every case is a project in `Fixtures/particle_plane_square/`, laid out as PTX's. The expected
frames are in `Fixtures/particle_plane_square/expected_particle_plane_square.json`. Every case is
"not built yet" until B-584 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_plane_square_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import particle_plane_texture_reference as PTX  # noqa: E402  (adds D-710's settings to PL)
import particle_texture_reference as T  # noqa: E402
import particle_plane_reference as PL  # noqa: E402
import particle_space_reference as P  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_plane_square"
W, H = P.W, P.H

# D-714's word, in this process only.
PTX.TEXTURED = PTX.TEXTURED + ("textured_square",)
PL.WORDS = [(n, words + ("textured_square",), first) if n == "particle_type" else (n, words, first)
            for n, words, first in PL.WORDS]


def particle_plane(px, c, n, tex):
    """D-710's drawing, the square's corners reached (R sqrt 2)."""
    run = PL.Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    out = [list(p) if c["composite_on_original"] == "on" else [0.0] * 4 for p in px]
    lt = P.light(run.now)
    mode = c["transfer_mode"]
    for _, sp, _ in PTX.sprites(run, px):
        if sp["kind"] == "line":
            r = sp["w"] / 2 + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        else:
            r = sp["R"] * (math.sqrt(2) if sp["kind"] == "textured_square" else 1) + 1
            xs, ys = (sp["X"] - r, sp["X"] + r), (sp["Y"] - r, sp["Y"] + r)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = PTX.cover(sp, gx + 0.5, gy + 0.5, lt, tex)
                if s is None:
                    continue
                d = out[gy * W + gx]
                if mode == "composite":
                    d = [s[i] + (1 - s[3]) * d[i] for i in range(4)]
                elif mode == "screen":
                    d = [s[i] + d[i] - s[i] * d[i] for i in range(4)]
                else:
                    d = [s[0] + d[0], s[1] + d[1], s[2] + d[2], min(1.0, s[3] + d[3])]
                out[gy * W + gx] = d
    return out


PTX.particle_plane = particle_plane
render, case, file_json = PTX.render, PTX.case, PTX.file_json

ONE, TWO, FLOW = PL.ONE, PL.TWO, PL.FLOW
SQ = dict(ONE, particle_type="textured_square", texture_layer="dot")
TWO_S = dict(TWO, particle_type="textured_square", texture_layer="dot")
ON = dict(composite_on_original="on")

CASES = {
    "FX-PPSQ-001": ("Textured Square with no texture: a square in the Color Map's colour, where "
                    "FX-PPLANE-002 draws its disc.",
                    case(**dict(ONE, particle_type="textured_square")), [20]),
    "FX-PPSQ-002": ("Textured Square with the dot texture: its red, green, blue and white "
                    "quarters, upright, the red at the top left.", case(**SQ), [20]),
    "FX-PPSQ-003": ("Initial Rotation 360: the square turned by its own random amount.",
                    case(**dict(SQ, initial_rotation=360)), [20]),
    "FX-PPSQ-004": ("Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees "
                    "clockwise.", case(**dict(SQ, rotation_speed=90)), [20]),
    "FX-PPSQ-005": ("Rotation Speed keyed from -180 at frame 0 to 180 at frame 24: read at the "
                    "particle's birth, so it turns anticlockwise for life.",
                    case(**dict(SQ, rotation_speed=keyed((0, -180), (24, 180)))), [20]),
    "FX-PPSQ-006": ("The 20 by 12 texture on the square: taken down a level, its blue checker "
                    "greying.", case(**dict(SQ, texture_layer="big")), [20]),
    "FX-PPSQ-007": ("Two textured squares, Composite: the newer over the older.",
                    case(**TWO_S), [20]),
    "FX-PPSQ-008": ("The two, Add.", case(**dict(TWO_S, transfer_mode="add")), [20]),
    "FX-PPSQ-009": ("The two, Screen, over the drawing (Composite With Original on), Max "
                    "Opacity 70.",
                    case(**dict(TWO_S, transfer_mode="screen", max_opacity=70, **ON)), [20]),
    "FX-PPSQ-010": ("The textured square faded out over its life (Opacity Map Fade Out).",
                    case(**dict(SQ, opacity_map="fade_out")), [20]),
    "FX-PPSQ-011": ("The texture layer starting at frame 15: at frame 10 nothing is drawn; at "
                    "frame 20 it is FX-PPSQ-002.", case(**dict(SQ, texture_layer="late")),
                    [10, 20]),
    "FX-PPSQ-012": ("A texture layer not in the composition, `gone`: the effect is left out, "
                    "the drawing shows, and the warning every frame.",
                    case(**dict(SQ, texture_layer="gone")), [20]),
    "FX-PPSQ-013": ("FX-PPSQ-002 with the layer moved 3 pixels right.",
                    case(shift=3, **SQ), [20]),
    "FX-PPSQ-014": ("A burst of turning textured squares, Rotation Speed 180 and Initial "
                    "Rotation 360, at frames 12 and 24.",
                    case(**dict(FLOW, particle_type="textured_square", texture_layer="dot",
                                rotation_speed=180, initial_rotation=360)), [12, 24]),
    "FX-PPSQ-015": ("A tiny textured square, size 3: 1.4 pixels across and fainter, the texture "
                    "taken down to its last level.",
                    case(**dict(SQ, birth_size=3, death_size=3)), [20]),
    "FX-PPSQ-016": ("A large square, size 60, turned by its own random amount (Initial Rotation "
                    "360): its corners reach past the disc's radius.",
                    case(**dict(ONE, particle_type="textured_square", birth_size=60,
                                death_size=60, initial_rotation=360)), [20]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
PL.FRAMES_OF = FRAMES_OF
assert all(PL.in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PPSQ-017": ("Particle Type \"Textured Square\": the word is exact.",
                    case(particle_type="Textured Square")),
    "FX-PPSQ-018": ("Particle Type \"textured_polygon\": not one of the plane's kinds.",
                    case(particle_type="textured_polygon")),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(file_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "colours.png").write_bytes(B.S.png(B.DRAWINGS["colours"]))
    for name, rows in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    expected = {"tolerance": B.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["texture_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = PL.plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = PL.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_particle_plane_square.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    blank = [[0.0] * 4 for _ in range(W * H)]
    drawing = PL.plain(case())
    disc = render(case(**dict(ONE, particle_type="textured_disc")), 20)
    sq = c["FX-PPSQ-001"]["20"]
    # The square covers the disc and more.
    assert total(sq) > total(disc) > 0
    assert all(sq[i][3] >= disc[i][3] - 1e-12 for i in range(W * H))
    tx = c["FX-PPSQ-002"]["20"]
    tl, br = tx[3 * W + 6], tx[7 * W + 10]
    assert tl[0] > 3 * tl[1] and min(br[:3]) > 0.8, (tl, br)
    for n in (3, 4, 5, 6):
        assert c["FX-PPSQ-%03d" % n]["20"] != tx, n
    assert c["FX-PPSQ-003"]["20"] != c["FX-PPSQ-004"]["20"] != c["FX-PPSQ-005"]["20"]
    assert c["FX-PPSQ-007"]["20"] != c["FX-PPSQ-008"]["20"]
    assert total(c["FX-PPSQ-010"]["20"]) < total(tx)
    assert c["FX-PPSQ-011"]["10"] == blank
    assert c["FX-PPSQ-011"]["20"] == tx
    assert c["FX-PPSQ-012"]["20"] == drawing
    mv = c["FX-PPSQ-013"]["20"]
    for y in range(H):
        assert mv[y * W + 3:(y + 1) * W] == tx[y * W:y * W + W - 3]
    assert c["FX-PPSQ-014"]["12"] != c["FX-PPSQ-014"]["24"]
    assert 0 < total(c["FX-PPSQ-015"]["20"]) < total(tx)
    print("checked")


if __name__ == "__main__":
    main()
