"""D-335's Exposure Offset, Gamma Correction and Bypass Linear Light Conversion, worked a second way.

From P-26's tutorial 2, under the owner's request of 2026-10-07 to build what the tutorial uses
and the app lacks. The tutorial's ground texture lifts its street plate with After Effects'
Exposure at 2.47 stops and Gamma Correction 1.69; this program's Exposure had stops alone.

After Effects' Exposure has Exposure, Offset and Gamma Correction, applied in that order (its
help: "Exposure ... Offset: shifts the shadows and highlights ... Gamma Correction: adjusts the
image's gamma curve"), and Bypass Linear Light Conversion, which applies it "to the original
pixel values" instead of linear light. `core.exposure` gains:

- `offset`, -0.5 to 0.5, absent 0; `gamma`, 0.01 to 9.99, absent 1; both may be keyed;
- `bypass`, "off" or "on" (Bypass Linear Light Conversion), absent "off";
- each written only when it is not its default or the file had it, so files from before D-335
  read, draw and save as before.

On a pixel with alpha, its straight value `v` (the linear colour in Float and at 8 bpc, the
2.2-curve value at 32 bpc (After Effects), D-333) becomes `v * 2^stops + offset`, then
`sign(u) * |u|^(1 / gamma)`, and is taken back the same way (at 32 bpc, `sign * |v|^(1/2.2)`
to the display value, then the sRGB curve to linear), times the alpha. With `bypass` on at
8 bpc or 32 bpc (After Effects), `v` is the display value itself, the straight colour through
the sRGB curve, and back. In Float bypass changes nothing: it is linear light already. A pixel
with no alpha is multiplied by 2^stops alone, as before. With offset 0 and gamma 1 every pixel
is D-333's or Float's exactly. Nothing is held at 0 or at white, but a straight value below 0
is held at 0 before it starts, as D-333.

Which curve After Effects uses at 32 bpc is D-333's reading (2.2). The order, and that bypass
works on display values, are After Effects' help; how a negative value takes a gamma is this
program's own rule.

Every case is glow_reference's 16 by 10 composition and drawings. The projects go into
`Fixtures/exposure_ae`, the expected frames into `Fixtures/exposure_ae/expected_exposure_ae.json`.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawing's 8-bit values.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/exposure_ae_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import eight_bpc_reference as E  # noqa: E402
import glow_reference as G  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "exposure_ae"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def power(u, g):
    return math.copysign(abs(u) ** (1 / g), u)


def exposure(px, stops, offset, gamma, bypass, depth):
    gain = 2 ** stops
    out = []
    for p in px:
        a = p[3]
        if a <= 0:
            out.append([v * gain for v in p[:3]] + [a])
            continue
        q = []
        for c in range(3):
            s = max(p[c] / a, 0.0)
            if depth == "float" or (depth == "eight" and not bypass):
                v = s
                back = lambda x: x  # noqa: E731
            elif bypass:
                v = S.linear_to_srgb(s)
                back = srgb_to_linear
            else:
                v = S.linear_to_srgb(s) ** 2.2
                back = lambda x: srgb_to_linear(power(x, 2.2))  # noqa: E731
            u = power(v * gain + offset, gamma)
            q.append(back(u) * a)
        out.append(q + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(depth="float", drawing="patches", stops=0.0, offset=0.0, gamma=1.0, bypass=None,
         frames=(0,)):
    """`bypass` None: the file does not say; otherwise its word."""
    return {"depth": depth, "drawing": drawing, "stops": stops, "offset": offset,
            "gamma": gamma, "bypass": bypass, "frames": list(frames)}


def render(c, frame_no):
    px = [G.working(p) for row in G.DRAWINGS[c["drawing"]] for p in row]
    px = exposure(px, value_at(c["stops"], frame_no), value_at(c["offset"], frame_no),
                  value_at(c["gamma"], frame_no), c["bypass"] == "on", c["depth"])
    return E.eight(px) if c["depth"] == "eight" else px


TEXTURE = dict(stops=2.47, gamma=1.69)

CASES = {
    "FX-EXPAE-001": ("Float, gamma 1.69 alone: each straight colour to the power 1 / 1.69, "
                     "brighter in the middle, black and white held.", case(gamma=1.69)),
    "FX-EXPAE-002": ("Float, offset 0.1 alone: 0.1 added to each straight colour, so the "
                     "patches are lifted and nothing is black, though the empty space stays "
                     "empty.", case(offset=0.1)),
    "FX-EXPAE-003": ("Float, Exposure +1, offset -0.2, gamma 2: in that order, doubled, lowered "
                     "and then the square root; where the offset takes a colour below 0 it stays "
                     "below 0.", case(stops=1, offset=-0.2, gamma=2)),
    "FX-EXPAE-004": ("Float, the tutorial's ground texture, Exposure 2.47, gamma 1.69, on the "
                     "half-covering patches.", case(drawing="faint", **TEXTURE)),
    "FX-EXPAE-005": ("32 bpc (After Effects), the tutorial's ground texture, Exposure 2.47, "
                     "gamma 1.69: through D-333's 2.2 curve.", case("ae32", **TEXTURE)),
    "FX-EXPAE-006": ("32 bpc (After Effects) with bypass on: Exposure +1, offset 0.05, gamma "
                     "1.2 on the display values themselves.",
                     case("ae32", stops=1, offset=0.05, gamma=1.2, bypass="on")),
    "FX-EXPAE-007": ("Float with bypass on: Float is linear light already, so this is "
                     "FX-EXPAE-003 exactly.", case(stops=1, offset=-0.2, gamma=2, bypass="on")),
    "FX-EXPAE-008": ("8 bpc, Exposure +1, gamma 1.69: linear light as D-330's Exposure, then "
                     "held to 8 bits.", case("eight", stops=1, gamma=1.69)),
    "FX-EXPAE-009": ("8 bpc with bypass on, Exposure +1, gamma 1.69: on the display values, "
                     "then held to 8 bits.", case("eight", stops=1, gamma=1.69, bypass="on")),
    "FX-EXPAE-010": ("Float, gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is "
                     "the drawing, frame 2 is gamma 2.",
                     case(gamma=keyed((0, 1), (4, 3)), frames=(0, 2, 4))),
    "FX-EXPAE-011": ("32 bpc (After Effects), bypass written \"off\" and offset 0, gamma 1 "
                     "written: Exposure +1 as D-333's.",
                     case("ae32", stops=1, bypass="off")),
}

INVALID = {
    "FX-EXPAE-012": ("Gamma 0, below 0.01.", case(gamma=0)),
    "FX-EXPAE-013": ("Gamma 10, above 9.99.", case(gamma=10)),
    "FX-EXPAE-014": ("Offset 0.6, above 0.5.", case(offset=0.6)),
    "FX-EXPAE-015": ("Offset keyed to -1 at frame 4.", case(offset=keyed((0, 0), (4, -1)))),
    "FX-EXPAE-016": ("Bypass \"yes\", not \"off\" or \"on\".", case(bypass="yes")),
}

# --- the project files ----------------------------------------------------------------------

def project_json(fx, c, written_defaults=False):
    p = G.project_json(fx, G.case(c["drawing"]))
    params = {"stops": setting_json(c["stops"])}
    if c["offset"] != 0 or written_defaults:
        params["offset"] = setting_json(c["offset"])
    if c["gamma"] != 1 or written_defaults:
        params["gamma"] = setting_json(c["gamma"])
    if c["bypass"] is not None:
        params["bypass"] = c["bypass"]
    comp = p["compositions"][0]
    comp["layers"][0]["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.exposure",
                                     "enabled": True, "parameters": params}]
    if c["depth"] == "eight":
        comp["eight_bpc"] = True
    elif c["depth"] == "ae32":
        comp["float_depth"] = True
        comp["ae_32bpc"] = True
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    p = project_json(fx, c, written_defaults=fx == "FX-EXPAE-011")
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in G.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in c["frames"]}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        plain = [G.working(p) for row in G.DRAWINGS[c["drawing"]] for p in row]
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_exposure_ae.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = [G.working(p) for row in G.DRAWINGS["patches"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    yellow = drawn[at(3, 4)]

    one = c["FX-EXPAE-001"]["0"]
    assert all(abs(one[at(3, 4)][ch] - yellow[ch] ** (1 / 1.69)) < 1e-12 for ch in range(3))
    assert one[at(3, 4)][1] > yellow[1] and one[at(0, 0)] == [0.0] * 4
    two = c["FX-EXPAE-002"]["0"]
    assert abs(two[at(3, 4)][2] - (yellow[2] + 0.1)) < 1e-12 and two[at(0, 0)] == [0.0] * 4
    three = c["FX-EXPAE-003"]["0"]
    assert any(p[ch] < 0 for p in three for ch in range(3)), "the offset takes something below 0"
    assert c["FX-EXPAE-007"] == c["FX-EXPAE-003"]
    # 32 bpc (After Effects) differs from Float, and bypass differs from both.
    five = c["FX-EXPAE-005"]["0"]
    flt = render(case(**TEXTURE), 0)
    assert abs(five[at(12, 4)][0] - flt[at(12, 4)][0]) > 1e-3
    # 8 bpc: held to 8 bits; bypass changes the result.
    assert c["FX-EXPAE-008"]["0"] != c["FX-EXPAE-009"]["0"]
    # The keyed gamma: frame 0 the drawing, frame 2 gamma 2.
    ten = c["FX-EXPAE-010"]
    assert all(abs(a - b) < 1e-12 for p, q in zip(ten["0"], drawn) for a, b in zip(p, q))
    assert ten["2"] == render(case(gamma=2), 0)
    # Defaults written are D-333's Exposure +1 exactly.
    k = 2 ** (1 / 2.2)
    p = c["FX-EXPAE-011"]["0"][at(3, 4)]
    want = srgb_to_linear(S.linear_to_srgb(yellow[0]) * k)
    assert abs(p[0] - want) < 1e-12, (p[0], want)


if __name__ == "__main__":
    main()
