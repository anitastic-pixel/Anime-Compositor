"""Particle Plane's textured sprites and Composite With Original, worked a second way.

D-710 (B-580) closes two of EFFECTS.md row 356's gaps on `core.particle_plane` (D-612,
`tools/particle_plane_reference.py`, here PL), after what CC Particle Systems II's textured
particle types and Composite With Original are for. Cycore publishes no maths, so every rule below
is ours, and each is D-611's or D-610's rule as Particle Space already draws it
(`tools/particle_texture_reference.py`, here T). No text or code was copied.

The settings added:

- `particle_type` takes two more words, after the six: "textured_disc" and
  "textured_faded_disc". "textured_square" came later, by the owner's decision of 2026-10-13
  (D-714, `tools/particle_plane_square_reference.py`); FX-PPLANE-055 and FX-PPTEX-024 now refuse
  "cube".
- `rotation_speed`, -3600 to 3600 degrees a second, 0 when added; `initial_rotation`, 0 to 360
  degrees, 0 when added. Both keyable, both read at the particle's birth b_k, as Birth Size is.
- `texture_layer`, D-189's layer setting, "" when added; the layer named is read whole, at its
  own size, at the frame drawn (its own timing).
- `composite_on_original`, "off" or "on", "off" when added.

A file written before D-710 has none of the four: each reads as its value when added, so every
FX-PPLANE case draws as before.

The rules:

1. Composite With Original on: the particles are laid over the drawing as it reaches the effect
   (moved with the layer as everything else is), not over a clear layer. Off, as D-612.
2. The two textured kinds are D-611's Textured Disc and Textured Faded Disc at the plane's own
   centre, radius and opacity (D-612's Opacity Map in it), with the turn theta = Initial Rotation
   (b) r(k, 10) + Rotation Speed (b) age / fps, clockwise on screen. The plane has no Depth Cue,
   so D-611's fd is 1: with no texture the sprite is A times the Color Map's colour (so they are
   D-612's Disc and Faded Sphere); with a texture T it is A (Tc.rgb, Tc.a), Tc read by D-611's
   levels and bilinear. A layer whose picture is empty at the frame: nothing drawn by a textured
   kind. A layer not in the composition: no texture, EFFECT_LAYER_MISSING every frame, and the
   effect left out (D-189's rule 4).
3. Drawing order and the transfer modes as D-612's: by number, oldest first.

Line, the four round kinds and the star are as D-612 draws them, the texture and the turn not
used.

**This file never runs the build's code path.** It reuses PL and T in its own process (adding the
settings and words here only; neither file's fixtures are written again). The drawing is
Broadcast Colors' 16 by 10; the textures are T's.

Every case is a project in `Fixtures/particle_plane_texture/`: the layer `art` with the effect
and, under it, the texture layers `dot`, `big` and `late`, every one switched off. The expected
frames are in `Fixtures/particle_plane_texture/expected_particle_plane_texture.json`. Every case
is "not built yet" until B-580 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_plane_texture_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import particle_texture_reference as T  # noqa: E402  (adds D-611's two settings to P)
import particle_plane_reference as PL  # noqa: E402
import particle_space_reference as P  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_plane_texture"
W, H, FPS = P.W, P.H, P.FPS
TEXTURED = ("textured_disc", "textured_faded_disc")

# D-710's settings and words, in this process only.
PL.SETTINGS = PL.SETTINGS + [("rotation_speed", -3600, 3600, 0), ("initial_rotation", 0, 360, 0)]
PL.NAMES = [s[0] for s in PL.SETTINGS]
PL.RANGES = {s[0]: (s[1], s[2]) for s in PL.SETTINGS}
PL.WORDS = [(n, words + TEXTURED, first) if n == "particle_type" else (n, words, first)
            for n, words, first in PL.WORDS] + [("composite_on_original", ("off", "on"), "off")]
plain_engine = PL.engine


def engine(s):
    e = plain_engine(s)
    e.update(rotation_speed=s["rotation_speed"], initial_rotation=s["initial_rotation"])
    return e


PL.engine = engine


def sprites(run, px):
    """D-612's sprites, each textured one with its turn."""
    out = PL.sprites(run, px)
    for k, sp, p in out:
        if sp["kind"] in TEXTURED:
            b = p["b"]
            theta = math.radians(run.at("initial_rotation", b) * run.r(k, 10)
                                 + run.at("rotation_speed", b) * (run.u - b) / FPS)
            sp.update(cos=math.cos(theta), sin=math.sin(theta), fog=[0.0, 0.0, 0.0], fd=1.0)
    return out


def cover(sp, x, y, lt, tex):
    if sp["kind"] in TEXTURED:
        return T.cover(sp, x, y, 0.0, lt, tex)
    return PL.cover(sp, x, y, lt)


def particle_plane(px, c, n, tex):
    """D-612's drawing, over the drawing with Composite With Original on, the textured kinds by
    D-611's covering."""
    run = PL.Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    out = [list(p) if c["composite_on_original"] == "on" else [0.0] * 4 for p in px]
    lt = P.light(run.now)
    mode = c["transfer_mode"]
    for _, sp, _ in sprites(run, px):
        if sp["kind"] == "line":
            r = sp["w"] / 2 + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        else:
            r = sp["R"] + 1
            xs, ys = (sp["X"] - r, sp["X"] + r), (sp["Y"] - r, sp["Y"] + r)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = cover(sp, gx + 0.5, gy + 0.5, lt, tex)
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


def render(c, n):
    tex = T.texture_at(c, n)
    if tex == "gone":
        return PL.plain(c)
    return R.frame(particle_plane(P.working(), c, n, tex), c["shift"])


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, in_frame=0, texture_layer="", **given):
    c = PL.case(shift, in_frame, **given)
    c["texture_layer"] = texture_layer
    return c


def file_json(fx, c):
    p = PL.file_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["texture_layer"] = \
        c["texture_layer"]
    p["assets"] += [L.still(n, f"media/{n}.png") for n in T.DRAWINGS]
    comp = p["compositions"][0]
    frames = comp["duration_frames"]
    under = [L.raster(n, "asset-" + d, enabled=False, in_frame=start, out_frame=frames)
             for n, (d, start) in T.LAYERS.items() if start < frames]
    comp["layers"] += under
    comp["layer_order"] += [l["id"] for l in under]
    return p


ONE, TWO, FLOW = PL.ONE, PL.TWO, PL.FLOW
DISC = dict(ONE, particle_type="textured_disc", texture_layer="dot")
FADED = dict(ONE, particle_type="textured_faded_disc", texture_layer="dot")
TWO_T = dict(TWO, particle_type="textured_disc", texture_layer="dot")
ON = dict(composite_on_original="on")

CASES = {
    "FX-PPTEX-001": ("Composite With Original on, one disc: FX-PPLANE-002's dot over the drawing "
                     "instead of over a clear layer.", case(**dict(ONE, **ON)), [20]),
    "FX-PPTEX-002": ("Composite With Original on, the falling burst: FX-PPLANE-025 over the "
                     "drawing, at frames 12 and 24.", case(**dict(FLOW, **ON)), [12, 24]),
    "FX-PPTEX-003": ("Composite With Original on, the two discs Screened over the drawing.",
                     case(**dict(TWO, transfer_mode="screen", **ON)), [20]),
    "FX-PPTEX-004": ("Composite With Original on, the layer starting at frame 10: at its first "
                     "frame no particle yet, so the drawing as it is; frame 22 is FX-PPTEX-002's "
                     "frame 12.", case(in_frame=10, **dict(FLOW, **ON)), [10, 22]),
    "FX-PPTEX-005": ("Textured Disc with no texture: FX-PPLANE-002's disc.",
                     case(**dict(ONE, particle_type="textured_disc")), [20]),
    "FX-PPTEX-006": ("Textured Faded Disc with no texture: FX-PPLANE-003's Faded Sphere.",
                     case(**dict(ONE, particle_type="textured_faded_disc")), [20]),
    "FX-PPTEX-007": ("Textured Disc with the dot texture: its red, green, blue and white "
                     "quarters cut round, upright, the red at the top left.", case(**DISC), [20]),
    "FX-PPTEX-008": ("Textured Faded Disc with the dot: the quarters fading to nothing at the "
                     "edge.", case(**FADED), [20]),
    "FX-PPTEX-009": ("Initial Rotation 360: the disc turned by its own random amount.",
                     case(**dict(DISC, initial_rotation=360)), [20]),
    "FX-PPTEX-010": ("Rotation Speed 90 a second: at 10.8 frames old, turned about 40 degrees "
                     "clockwise.", case(**dict(DISC, rotation_speed=90)), [20]),
    "FX-PPTEX-011": ("Rotation Speed keyed from -180 at frame 0 to 180 at frame 24: read at the "
                     "particle's birth (frame 9.2), so it turns anticlockwise for life.",
                     case(**dict(DISC, rotation_speed=keyed((0, -180), (24, 180)))), [20]),
    "FX-PPTEX-012": ("The 20 by 12 texture on the disc: taken down a level, its blue checker "
                     "greying.", case(**dict(DISC, texture_layer="big")), [20]),
    "FX-PPTEX-013": ("Two textured discs, Composite: the newer over the older.",
                     case(**TWO_T), [20]),
    "FX-PPTEX-014": ("The two, Add.", case(**dict(TWO_T, transfer_mode="add")), [20]),
    "FX-PPTEX-015": ("The two over the drawing (Composite With Original on), Max Opacity 70.",
                     case(**dict(TWO_T, max_opacity=70, **ON)), [20]),
    "FX-PPTEX-016": ("The textured disc faded out over its life (Opacity Map Fade Out): four "
                     "fifths covering.", case(**dict(DISC, opacity_map="fade_out")), [20]),
    "FX-PPTEX-017": ("The texture layer starting at frame 15: at frame 10 it has no picture, so "
                     "nothing is drawn; at frame 20 it is FX-PPTEX-007.",
                     case(**dict(DISC, texture_layer="late")), [10, 20]),
    "FX-PPTEX-018": ("A texture layer not in the composition, `gone`: the effect is left out, "
                     "the drawing shows, and the warning every frame.",
                     case(**dict(DISC, texture_layer="gone")), [20]),
    "FX-PPTEX-019": ("FX-PPTEX-007 with the layer moved 3 pixels right.",
                     case(shift=3, **DISC), [20]),
    "FX-PPTEX-020": ("A burst of turning textured faded discs, Rotation Speed 180 and Initial "
                     "Rotation 360, at frames 12 and 24.",
                     case(**dict(FLOW, particle_type="textured_faded_disc", texture_layer="dot",
                                 rotation_speed=180, initial_rotation=360)), [12, 24]),
    "FX-PPTEX-021": ("A tiny textured disc, size 3: drawn 1.4 pixels across and fainter, the "
                     "texture taken down to its last level.",
                     case(**dict(DISC, birth_size=3, death_size=3)), [20]),
    "FX-PPTEX-022": ("A Disc (not textured) with the texture and rotation set: FX-PPLANE-002, "
                     "the texture not used.",
                     case(**dict(ONE, texture_layer="dot", rotation_speed=90)), [20]),
    "FX-PPTEX-023": ("A star over the drawing (Composite With Original on), the texture set and "
                     "not used.", case(**dict(ONE, particle_type="star", texture_layer="dot",
                                              **ON)), [20]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
PL.FRAMES_OF = FRAMES_OF
assert all(PL.in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PPTEX-024": ("Particle Type \"cube\": not one of the plane's kinds (D-714, the owner, "
                     "2026-10-13: was \"textured_square\", which the plane now draws).",
                     case(particle_type="cube")),
    "FX-PPTEX-025": ("Rotation Speed 3601, above 3600.", case(rotation_speed=3601)),
    "FX-PPTEX-026": ("Initial Rotation -1, below 0.", case(initial_rotation=-1)),
    "FX-PPTEX-027": ("Initial Rotation 361, above 360.", case(initial_rotation=361)),
    "FX-PPTEX-028": ("Composite With Original \"yes\": the word is \"off\" or \"on\".",
                     case(composite_on_original="yes")),
    "FX-PPTEX-029": ("A texture layer written as the number 3, not a word.",
                     case(texture_layer=3)),
}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(file_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def clear():
    """As D-612's: no birth at the frame drawn, no age at its life."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            run = PL.Run(c, f)
            if run.u < 0:
                continue
            for k, b in P.births(run):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
                assert abs((run.u - b) - run.at("longevity", b) * FPS) > 1e-6, (fx, f, k)
    print("clear")


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "colours.png").write_bytes(B.S.png(B.DRAWINGS["colours"]))
    for name, rows in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    clear()
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
    (OUT / "expected_particle_plane_texture.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = lambda fx, f=20: PL.render(dict(case(), **PL.CASES[fx][1]), f)  # noqa: E731
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    blank = [[0.0] * 4 for _ in range(W * H)]
    drawing = PL.plain(case())
    # Composite With Original: the plane's drawing laid over the input by its own transfer mode.
    for fx, base, frames in (("FX-PPTEX-001", "FX-PPLANE-002", [20]),
                             ("FX-PPTEX-002", "FX-PPLANE-025", [12, 24])):
        for f in frames:
            top = old(base, f)
            want = [[top[i][j] + (1 - top[i][3]) * drawing[i][j] for j in range(4)]
                    for i in range(W * H)]
            assert all(abs(a - b) < 1e-12 for p, q in zip(c[fx][str(f)], want)
                       for a, b in zip(p, q)), (fx, f)
    assert c["FX-PPTEX-003"]["20"] != old("FX-PPLANE-022")
    assert c["FX-PPTEX-004"]["10"] == drawing
    assert c["FX-PPTEX-004"]["22"] == c["FX-PPTEX-002"]["12"]
    # No texture: D-612's Disc and Faded Sphere; a Disc ignores the texture and turn.
    assert c["FX-PPTEX-005"]["20"] == old("FX-PPLANE-002")
    assert c["FX-PPTEX-006"]["20"] == old("FX-PPLANE-003")
    assert c["FX-PPTEX-022"]["20"] == old("FX-PPLANE-002")
    dc, fa = c["FX-PPTEX-007"]["20"], c["FX-PPTEX-008"]["20"]
    assert total(dc) > total(fa) > 0
    # The quarters upright: the top left red, the bottom right white.
    tl, br = dc[3 * W + 6], dc[7 * W + 10]
    assert tl[0] > 3 * tl[1] and min(br[:3]) > 0.8, (tl, br)
    for n in (9, 10, 11, 12):
        assert c["FX-PPTEX-%03d" % n]["20"] != dc, n
    assert c["FX-PPTEX-009"]["20"] != c["FX-PPTEX-010"]["20"] != c["FX-PPTEX-011"]["20"]
    assert c["FX-PPTEX-013"]["20"] != c["FX-PPTEX-014"]["20"]
    assert abs(total(c["FX-PPTEX-016"]["20"]) - (1 - 10.77 / 48) * total(dc)) < 0.02 * total(dc)
    assert c["FX-PPTEX-017"]["10"] == blank
    assert c["FX-PPTEX-017"]["20"] == dc
    assert c["FX-PPTEX-018"]["20"] == drawing
    mv = c["FX-PPTEX-019"]["20"]
    for y in range(H):
        assert mv[y * W + 3:(y + 1) * W] == dc[y * W:y * W + W - 3]
    assert c["FX-PPTEX-020"]["12"] != c["FX-PPTEX-020"]["24"]
    assert 0 < total(c["FX-PPTEX-021"]["20"]) < total(dc)
    # The star over the drawing: where the star is clear, the drawing.
    st, plain_star = c["FX-PPTEX-023"]["20"], old("FX-PPLANE-006")
    assert all(st[i] == drawing[i] for i in range(W * H) if plain_star[i][3] == 0)
    print("checked")


if __name__ == "__main__":
    main()
