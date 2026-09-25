"""Glow, worked a second way.

D-89 adds a sixth effect, `core.glow`. It makes the bright parts of a drawing, or the colours
chosen for it, shine: a blurred copy of them is added on top, and spreads past the drawing's own
edge into the space around it. It is modelled on After Effects' Glow, with its setting names and
defaults, and done in this program's own terms: document 21's Gaussian blur, in linear light.
Nothing is ported; OpenToonz's Glow and Bloom and F's Plugins were read and not used. Document
21 is the rule in words; this file is the reference for the numbers document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, and sums the blur in two dimensions at once, where the build
works in single precision on its buffers, blurs across and then down, and takes each pixel's
colour back out of them; the two agree to far inside the tolerance.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says. The drawings leave empty space round what they show, so the glow can be seen
spreading into it. The drawings go into `Fixtures/glow/media`, the projects into `Fixtures/glow`,
and the expected frames into `Fixtures/glow/expected_glow.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glow_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = 16, 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glow"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_THRESHOLD, MAX_RADIUS, MAX_INTENSITY, MAX_TOLERANCE, MAX_COLORS = 100, 500, 10, 255, 8


# --- the rule -------------------------------------------------------------------------------

def hex_color(s):
    return tuple(int(s[i:i + 2], 16) for i in (1, 3, 5))


def working(p):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    a = p[3] / 255
    return [srgb_to_linear(p[c] / 255) * a for c in range(3)] + [a]


def glows(p, c, threshold, tolerance):
    """Whether a pixel of the drawing lights the glow. It must show at all. Bright parts: its
    brightest channel, in 8-bit steps, is at least the threshold's share of 255. Chosen colours:
    each of its red, green and blue is within the tolerance of one chosen colour's (D-88's rule)."""
    if p[3] == 0:
        return False
    if c["based_on"] == "bright":
        return max(p[:3]) * 100 >= threshold * 255
    return any(all(abs(p[i] - t[i]) <= tolerance for i in range(3))
               for t in (hex_color(s.lower()) for s in c["colors"]))


def glow(pixels, c, frame_no, shift):
    """The glowing drawing, as the composition's W by H frame. `pixels` is the drawing's 8-bit
    straight pixels, row by row; the composition's column x shows the drawing's column x - shift,
    and a drawing column outside 0 to W - 1 is the space the glow has grown into."""
    threshold = min(MAX_THRESHOLD, max(0.0, value_at(c["threshold"], frame_no)))
    radius = min(MAX_RADIUS, max(0.0, value_at(c["radius"], frame_no)))
    intensity = min(MAX_INTENSITY, max(0.0, value_at(c["intensity"], frame_no)))
    tolerance = min(MAX_TOLERANCE, max(0.0, value_at(c["tolerance"], frame_no)))

    # 1. What glows: the pixel as it is, or, with a tint, the tint at the pixel's own covering.
    source = []
    for p in pixels:
        if not glows(p, c, threshold, tolerance):
            source.append([0.0] * 4)
        elif c["tint"]:
            a = p[3] / 255
            source.append([srgb_to_linear(v / 255) * a for v in hex_color(c["tint"].lower())]
                          + [a])
        else:
            source.append(working(p))

    # 2. Document 21's Gaussian blur at sigma radius / 3, so it reaches ceil(radius) pixels.
    sigma = radius / 3
    reach = math.ceil(3 * sigma) if sigma > 0 else 0
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-reach, reach + 1)] \
        if reach else [1.0]
    total = sum(one)
    one = [v / total for v in one]

    out = []
    for y in range(H):
        for x in range(W):
            dx = x - shift
            g = [0.0] * 4
            for j in range(-reach, reach + 1):
                for i in range(-reach, reach + 1):
                    sx, sy = dx + i, y + j
                    if 0 <= sx < W and 0 <= sy < H:
                        k = one[i + reach] * one[j + reach]
                        for ch in range(4):
                            g[ch] += source[sy * W + sx][ch] * k
            # 3. Intensity scales the glow, its covering too.
            g = [v * intensity for v in g]
            o = working(pixels[y * W + dx]) if 0 <= dx < W else [0.0] * 4
            # 4. On top of the drawing. Add: light adds up and is not cut off at white, and the
            # covering stops at full. Screen: each held inside 0 to 1, then screened.
            if c["operation"] == "add":
                out.append([o[ch] + g[ch] for ch in range(3)] + [min(1.0, o[3] + g[3])])
            else:
                g = [min(1.0, max(0.0, v)) for v in g]
                out.append([o[ch] + g[ch] - o[ch] * g[ch] for ch in range(4)])
    return out


# --- drawings -------------------------------------------------------------------------------

BRIGHT = (250, 220, 120, 255)  # warm yellow: brightest channel 250, 98 %
EDGE = (153, 102, 51, 255)     # brown: brightest channel 153, exactly 60 %
DARK = (60, 40, 110, 255)      # purple: brightest channel 110, 43 %
NONE = S.NONE
DARK_HEX, DARK_OFF_HEX = "#3c286e", "#3c286f"  # the purple, and one step off in blue


def patches(cover=255):
    """Three patches in rows 3 to 6 on nothing: bright in columns 2 to 4, edge in 7 and 8, dark
    in 11 to 13."""
    def colour(x, y):
        if not 3 <= y <= 6:
            return NONE
        for first, last, p in ((2, 4, BRIGHT), (7, 8, EDGE), (11, 13, DARK)):
            if first <= x <= last:
                return p[:3] + (cover,)
        return NONE
    return [[colour(x, y) for x in range(W)] for y in range(H)]


DRAWINGS = {
    "patches": patches(),
    # The same patches half covering.
    "faint": patches(128),
}


# --- the cases ------------------------------------------------------------------------------

def case(name="patches", based_on="bright", threshold=60, colors=(), tolerance=0, radius=4,
         intensity=1, operation="add", tint="", shift=0):
    return {"drawing": name, "based_on": based_on, "threshold": threshold,
            "colors": list(colors), "tolerance": tolerance, "radius": radius,
            "intensity": intensity, "operation": operation, "tint": tint, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return glow(pixels, c, frame_no, c["shift"])


CASES = {
    "FX-GLOW-001": ("Bright parts, threshold 60, radius 4: the yellow patch and the brown one, "
                    "whose brightest channel is exactly 60 %, glow; the purple does not, though "
                    "the light of the others reaches it. The glow spreads into the empty space "
                    "round the patches, which shows it.",
                    case(), [0]),
    "FX-GLOW-002": ("Intensity 0: the drawing, untouched.",
                    case(intensity=0), [0]),
    "FX-GLOW-003": ("Radius 0: nothing spreads, and each glowing pixel is added onto itself, "
                    "twice as bright.",
                    case(radius=0), [0]),
    "FX-GLOW-004": ("Threshold 100: nothing is that bright, so the drawing is untouched.",
                    case(threshold=100), [0]),
    "FX-GLOW-005": ("Threshold 0: every pixel that shows glows, the purple too.",
                    case(threshold=0), [0]),
    "FX-GLOW-006": ("Threshold 61: the brown, at exactly 60 %, no longer glows.",
                    case(threshold=61), [0]),
    "FX-GLOW-007": ("Chosen colours, the purple chosen: only the purple glows.",
                    case(based_on="colors", colors=[DARK_HEX]), [0]),
    "FX-GLOW-008": ("The purple chosen one step off in blue, tolerance 0: nothing is chosen, and "
                    "the drawing is untouched.",
                    case(based_on="colors", colors=[DARK_OFF_HEX]), [0]),
    "FX-GLOW-009": ("The same at tolerance 1: the purple is chosen again, and this is "
                    "FX-GLOW-007.",
                    case(based_on="colors", colors=[DARK_OFF_HEX], tolerance=1), [0]),
    "FX-GLOW-010": ("Chosen colours with none chosen: the drawing, untouched.",
                    case(based_on="colors"), [0]),
    "FX-GLOW-011": ("Intensity 2.5: the glow is two and a half times as strong, and where it "
                    "adds past white it is not cut off.",
                    case(intensity=2.5), [0]),
    "FX-GLOW-012": ("Operation Screen: the glow is screened on, which never passes white.",
                    case(operation="screen"), [0]),
    "FX-GLOW-013": ("Tint #ff4000: the glow is orange, whatever the colour that lit it.",
                    case(tint="#ff4000"), [0]),
    "FX-GLOW-014": ("After Effects' own defaults, threshold 60, radius 10, intensity 1, Add.",
                    case(radius=10), [0]),
    "FX-GLOW-015": ("The patches half covering: they glow as in FX-GLOW-001, at half the "
                    "strength.",
                    case("faint"), [0]),
    "FX-GLOW-016": ("Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is "
                    "FX-GLOW-003, frame 2 is FX-GLOW-001, frame 4 is radius 8.",
                    case(radius=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-GLOW-017": ("Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is "
                    "the drawing, frame 2 is FX-GLOW-001, frame 4 has the purple glowing too.",
                    case(threshold=keyed((0, 100), (4, 20))), [0, 2, 4]),
    "FX-GLOW-018": ("Intensity keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the "
                    "drawing, frame 2 is FX-GLOW-001.",
                    case(intensity=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-GLOW-019": ("FX-GLOW-001 moved three pixels right: the glow is done on the drawing "
                    "before it is moved, and the part of it that spread past the drawing's left "
                    "edge now shows in columns 1 and 2.",
                    case(shift=3), [0, 3]),
    "FX-GLOW-020": ("Radius 2.5: a radius is not rounded, so this is neither radius 2 nor 3.",
                    case(radius=2.5), [0]),
    "FX-GLOW-021": ("FX-GLOW-007 and FX-GLOW-013 with the colours written in capitals: the "
                    "same.",
                    case(based_on="colors", colors=[DARK_HEX.upper()], tint="#FF4000"), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GLOW-022": ("Radius 501, above 500.", case(radius=501)),
    "FX-GLOW-023": ("Radius -1, below 0.", case(radius=-1)),
    "FX-GLOW-024": ("Radius keyed to 600 at frame 4.", case(radius=keyed((0, 0), (4, 600)))),
    "FX-GLOW-025": ("Threshold 101, above 100.", case(threshold=101)),
    "FX-GLOW-026": ("Intensity 11, above 10.", case(intensity=11)),
    "FX-GLOW-027": ("Intensity -1, below 0.", case(intensity=-1)),
    "FX-GLOW-028": ("Tolerance 256, above 255.", case(tolerance=256)),
    "FX-GLOW-029": ("Nine colours, one more than eight.",
                    case(colors=[f"#0000{i:02x}" for i in range(9)])),
    "FX-GLOW-030": ("A colour written \"#12345\", one digit short.",
                    case(based_on="colors", colors=[DARK_HEX, "#12345"])),
    "FX-GLOW-031": ("A tint written \"orange\".", case(tint="orange")),
    "FX-GLOW-032": ("Operation \"multiply\", which is not Add or Screen.",
                    case(operation="multiply")),
    "FX-GLOW-033": ("Based on \"dark\", which is not bright or colors.", case(based_on="dark")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.glow", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in (
            "based_on", "threshold", "colors", "tolerance", "radius", "intensity", "operation",
            "tint")}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        plain = render(case(c["drawing"], intensity=0, shift=c["shift"]), 0)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        plain = render(case(c["drawing"], intensity=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_glow.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    plain = {name: [working(p) for row in DRAWINGS[name] for p in row] for name in DRAWINGS}
    drawn = plain["patches"]
    at = lambda x, y: y * W + x  # noqa: E731
    one = c["FX-GLOW-001"]["0"]

    # 001: every pixel the patches light gets brighter, including the purple and the empty
    # space round the patches; the space far from them stays empty; covering never passes 1.
    for x in (2, 3, 4, 7, 8, 11):
        assert all(one[at(x, 4)][ch] > drawn[at(x, 4)][ch] for ch in range(3)), x
    assert one[at(0, 4)][3] > 0 and one[at(5, 1)][3] > 0  # glow in the empty space
    assert one[at(15, 0)] == [0.0] * 4  # more than 4 pixels from anything that glows
    # The purple glows in 005 and not in 001: at its far edge, only its own light counts.
    assert c["FX-GLOW-005"]["0"][at(13, 4)] != one[at(13, 4)]

    assert c["FX-GLOW-002"]["0"] == drawn
    three = c["FX-GLOW-003"]["0"]
    for i in range(W * H):
        g = drawn[i] if drawn[i] in (working(BRIGHT), working(EDGE)) else [0.0] * 4
        assert three[i] == [drawn[i][ch] + g[ch] for ch in range(3)] + [drawn[i][3]]
    assert c["FX-GLOW-004"]["0"] == drawn
    six = c["FX-GLOW-006"]["0"]
    assert six != one and six[at(8, 4)] != one[at(8, 4)]
    # 007: only the purple is lit, so nothing moves near the yellow.
    seven = c["FX-GLOW-007"]["0"]
    assert seven[at(2, 4)] == drawn[at(2, 4)] and seven[at(13, 4)] != drawn[at(13, 4)]
    assert c["FX-GLOW-008"]["0"] == drawn
    assert c["FX-GLOW-009"]["0"] == seven
    assert c["FX-GLOW-010"]["0"] == drawn
    eleven = c["FX-GLOW-011"]["0"]
    assert any(v > 1 for p in eleven for v in p[:3])  # past white, not cut off
    for p in c["FX-GLOW-012"]["0"]:
        assert all(0 <= v <= 1 for v in p)
    # 013: in the empty space the glow is the tint's colour, straight.
    tint = [srgb_to_linear(v / 255) for v in (0xff, 0x40, 0x00)]
    p = c["FX-GLOW-013"]["0"][at(0, 4)]
    assert all(abs(p[ch] / p[3] - tint[ch]) < 1e-12 for ch in range(3))
    assert c["FX-GLOW-014"]["0"][at(15, 0)][3] > 0  # radius 10 reaches the corner
    # 015: half covering; the patch pixels keep a covering of at least their own.
    fifteen = c["FX-GLOW-015"]["0"]
    assert fifteen != one and fifteen[at(3, 4)][3] > plain["faint"][at(3, 4)][3]
    assert c["FX-GLOW-016"]["0"] == three and c["FX-GLOW-016"]["2"] == one
    assert c["FX-GLOW-016"]["4"] == render(case(radius=8), 0)
    assert c["FX-GLOW-017"]["0"] == drawn and c["FX-GLOW-017"]["2"] == one
    assert c["FX-GLOW-017"]["4"] == render(case(threshold=20), 0)
    assert c["FX-GLOW-018"]["0"] == drawn and c["FX-GLOW-018"]["2"] == one
    moved = c["FX-GLOW-019"]["0"]
    assert moved == c["FX-GLOW-019"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    assert moved[at(0, 4)] == [0.0] * 4  # 5 pixels from the yellow, too far for radius 4
    assert all(moved[at(x, 4)][3] > 0 for x in (1, 2))  # grown past the drawing's edge
    twenty = c["FX-GLOW-020"]["0"]
    assert twenty not in (render(case(radius=2), 0), render(case(radius=3), 0))
    assert c["FX-GLOW-021"]["0"] == render(case(based_on="colors", colors=[DARK_HEX],
                                                tint="#ff4000"), 0)

    # Everywhere: covering inside 0 to 1, and no colour below nothing.
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
