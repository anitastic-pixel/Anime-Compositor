"""Selective colour blur, worked a second way.

D-87 adds a fifth effect, `core.selective_color_blur`. It softens the edge where two chosen cel
colours meet, such as skin and its shadow, and leaves every other colour, and every line, as
sharp as it was drawn: F's Plugins SelectiveColorBlur, as its own source does it, with D-87's
changes. Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers and
takes each pixel's colour back out of them; the two agree to far inside the tolerance. The first
case is also checked below by its mirror symmetry, which holds for any correct blur whatever its
arithmetic.

The blur below is ported from F's Plugins, `SelectiveColorBlur/SelectiveColorBlurSub.cpp` in
bryful/F-s-PluginsProjects, commit db6dad3959522cfcd593dfe12a9fa4ad022d31e3 (2026-05-04), under
its licence, kept in `docs/third_party/F-s-PluginsProjects-LICENSE.txt`:

    MIT License. Copyright (c) 2019 bryful. The full text, its permission notice and its
    disclaimer are in the file named above.

Every case is a composition 12 pixels by 8 holding one drawing the same size, unmoved unless the
case says. The drawings go into `Fixtures/selblur/media`, the projects into `Fixtures/selblur`,
and the expected frames into `Fixtures/selblur/expected_selblur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/selblur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = S.W, S.H
FRAMES = 5
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "selblur"
TOLERANCE = 2e-5  # document 25's default for a filter; see D-87
MAX_BLUR, MAX_COLORS = 200, 8


# --- the rule -------------------------------------------------------------------------------

def hex_color(s):
    return tuple(int(s[i:i + 2], 16) for i in (1, 3, 5))


def selective_blur(pixels, colors, blur):
    """Document 21's selective colour blur on a drawing's 8-bit straight pixels. Handed back is
    each pixel's straight encoded colour, 0 to 1, and its alpha, which never changes."""
    n = len(pixels)
    enc = [[v / 255 for v in p[:3]] for p in pixels]
    targets = {hex_color(c) for c in colors}
    # A pixel is chosen when it shows at all and its colour is one of the chosen ones exactly.
    chosen = [p[3] > 0 and tuple(p[:3]) in targets for p in pixels]
    r = blur
    if r > 0 and any(chosen):
        zone = r / 3
        tbl = [math.exp(-k * k / (2 * zone * zone)) for k in range(r + 1)]
        # The original's passes: across, down, then across, down and across again at a quarter
        # of the reach each time, leaving out a pass whose reach comes to nothing. Every pass
        # reads what the one before it wrote, and the weights are always the full blur's.
        for reach, across in ((r, True), (r, False), (r // 4, True), (r // 16, False),
                              (r // 64, True)):
            if reach > 0:
                enc = _pass(enc, chosen, tbl, reach, across)
    return [(enc[i] if chosen[i] else [v / 255 for v in pixels[i][:3]]) + [pixels[i][3] / 255]
            for i in range(n)]


def _pass(enc, chosen, tbl, reach, across):
    """One pass of the edge-stopping blur. Each chosen pixel becomes the weighted average of
    itself and the chosen pixels on either side of it, out to `reach`; each side stops at the
    first pixel that is not chosen, or at the edge of the picture, and the weights of what is
    left are made to sum to one."""
    out = [list(c) for c in enc]
    lines = ([[y * W + x for x in range(W)] for y in range(H)] if across
             else [[y * W + x for y in range(H)] for x in range(W)])
    for line in lines:
        for j, i in enumerate(line):
            if not chosen[i]:
                continue
            got, total = [0.0, 0.0, 0.0], tbl[0]
            for step in (-1, 1):
                for k in range(1, reach + 1):
                    m = j + step * k
                    if m < 0 or m >= len(line) or not chosen[line[m]]:
                        break
                    # Written as a difference from the pixel's own colour, which is the same
                    # average, so that a flat patch comes out exactly as it went in.
                    for c in range(3):
                        got[c] += tbl[k] * (enc[line[m]][c] - enc[i][c])
                    total += tbl[k]
            out[i] = [enc[i][c] + got[c] / total for c in range(3)]
    return out


def working(straight):
    """Back to the working buffer: the straight colour through the sRGB curve, premultiplied."""
    a = straight[3]
    if a <= 0:
        return [0.0, 0.0, 0.0, 0.0]
    return [srgb_to_linear(straight[c]) * a for c in range(3)] + [a]


def round_half_up(v):
    return math.floor(v + 0.5)


# --- drawings -------------------------------------------------------------------------------

SKIN, SHADOW = (246, 214, 190, 255), (219, 160, 142, 255)
LIGHT, LINE = (255, 243, 232, 255), (30, 26, 36, 255)
SKIN_SOFT, SHADOW_SOFT = (246, 214, 190, 128), (219, 160, 142, 128)
NONE = S.NONE
SKIN_HEX, SHADOW_HEX, LIGHT_HEX = "#f6d6be", "#dba08e", "#fff3e8"


def cols(*spans):
    """A drawing of upright stripes: spans of (first column, colour), left to right."""
    def colour(x):
        return [c for first, c in spans if x >= first][-1]
    return [[colour(x) for x in range(W)] for _ in range(H)]


DRAWINGS = {
    # Skin in columns 0 to 5, shadow in 6 to 11, with no line between.
    "halves": cols((0, SKIN), (6, SHADOW)),
    # The same with a one-pixel line in column 6: skin 0 to 5, line 6, shadow 7 to 11.
    "lined": cols((0, SKIN), (6, LINE), (7, SHADOW)),
    # Skin, shadow and highlight, four columns each.
    "stripes": cols((0, SKIN), (4, SHADOW), (8, LIGHT)),
    # The halves in columns 2 to 9 and rows 2 to 5, on nothing.
    "block": [[(SKIN if x < 6 else SHADOW) if 2 <= x <= 9 and 2 <= y <= 5 else NONE
               for x in range(W)] for y in range(H)],
    # The halves, with rows 0 to 3 half covering.
    "faint": [[(SKIN_SOFT if x < 6 else SHADOW_SOFT) if y < 4 else (SKIN if x < 6 else SHADOW)
               for x in range(W)] for y in range(H)],
    # Skin above a line falling one row every one and a half columns, shadow below.
    "diagonal": [[SHADOW if 2 * x > 3 * y else SKIN for x in range(W)] for y in range(H)],
}


# --- the cases ------------------------------------------------------------------------------

def case(name, blur=6, colors=(SKIN_HEX, SHADOW_HEX), shift=0):
    return {"drawing": name, "blur": blur, "colors": list(colors), "shift": shift}


def render(c, frame_no):
    blur = round_half_up(min(MAX_BLUR, max(0.0, value_at(c["blur"], frame_no))))
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    out = [working(p) for p in selective_blur(pixels, [s.lower() for s in c["colors"]], blur)]
    s = c["shift"]
    return [out[y * W + x - s] if x >= s else [0.0] * 4 for y in range(H) for x in range(W)]


CASES = {
    "FX-SELBLUR-001": ("Skin beside shadow with no line between, both chosen, blur 6: the edge "
                       "turns into a soft ramp across the whole row, the same on every row.",
                       case("halves"), [0]),
    "FX-SELBLUR-002": ("The same at blur 0: the drawing, untouched.",
                       case("halves", blur=0), [0]),
    "FX-SELBLUR-003": ("The same with only the shadow chosen: it has no chosen colour beside "
                       "it to mix with, so nothing changes.",
                       case("halves", colors=[SHADOW_HEX]), [0]),
    "FX-SELBLUR-004": ("Skin and shadow with a black line between, both chosen: the line is not "
                       "chosen, so neither colour reaches the other and nothing changes.",
                       case("lined"), [0]),
    "FX-SELBLUR-005": ("Skin, shadow and highlight, with skin and shadow chosen: the skin and "
                       "shadow edge goes soft; the highlight stays exactly as drawn, and none of "
                       "it mixes into the shadow.",
                       case("stripes"), [0]),
    "FX-SELBLUR-006": ("The halves as a block on nothing, with black chosen as well: nothing "
                       "stays nothing, even though its colour is black, and the soft edge "
                       "reaches no further than the block.",
                       case("block", colors=[SKIN_HEX, SHADOW_HEX, "#000000"]), [0]),
    "FX-SELBLUR-007": ("The halves with the top four rows half covering: every pixel keeps its "
                       "own covering, and its colour is FX-SELBLUR-001's.",
                       case("faint"), [0]),
    "FX-SELBLUR-008": ("The halves with the skin chosen one step off in blue (#f6d6bf) beside "
                       "the shadow: the skin is not chosen, so nothing changes.",
                       case("halves", colors=["#f6d6bf", SHADOW_HEX]), [0]),
    "FX-SELBLUR-009": ("Blur keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is "
                       "FX-SELBLUR-002, frame 2 is FX-SELBLUR-001, frame 4 is the blur at 12.",
                       case("halves", blur=keyed((0, 0), (4, 12))), [0, 2, 4]),
    "FX-SELBLUR-010": ("FX-SELBLUR-001 moved two pixels right: the same softened drawing, moved; "
                       "the blur is done on the drawing's own pixels before it is moved.",
                       case("halves", shift=2), [0, 3]),
    "FX-SELBLUR-011": ("Blur 5.5: a blur is a whole number of pixels, and a half rounds up, so "
                       "this is FX-SELBLUR-001.",
                       case("halves", blur=5.5), [0]),
    "FX-SELBLUR-012": ("Skin above a sloping edge, shadow below, blur 12: the edge goes soft all "
                       "along its length.",
                       case("diagonal", blur=12), [0]),
    "FX-SELBLUR-013": ("The same at blur 64, where all five passes act.",
                       case("diagonal", blur=64), [0]),
    "FX-SELBLUR-014": ("No colours chosen: nothing changes. An effect just added has none.",
                       case("halves", colors=[]), [0]),
    "FX-SELBLUR-015": ("FX-SELBLUR-001 with its colours written in capitals: the same.",
                       case("halves", colors=[SKIN_HEX.upper(), SHADOW_HEX.upper()]), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SELBLUR-020": ("Blur 201, above 200.", case("halves", blur=201)),
    "FX-SELBLUR-021": ("Blur -1, below 0.", case("halves", blur=-1)),
    "FX-SELBLUR-022": ("Blur keyed to 250 at frame 4.",
                       case("halves", blur=keyed((0, 0), (4, 250)))),
    "FX-SELBLUR-023": ("Nine colours, one more than eight.",
                       case("halves", colors=[SKIN_HEX, SHADOW_HEX] + [f"#0000{i:02x}"
                                                                        for i in range(7)])),
    "FX-SELBLUR-024": ("A colour written \"#12345\", one digit short.",
                       case("halves", colors=[SKIN_HEX, "#12345"])),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.selective_color_blur", "enabled": True,
        "parameters": {"blur": setting_json(c["blur"]), "colors": c["colors"]}}]
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
        plain = render(case(c["drawing"], blur=0, shift=c["shift"]), 0)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        plain = render(case(c["drawing"], blur=0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_selblur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    plain = {name: render(case(name, blur=0), 0) for name in DRAWINGS}
    straight = lambda p: [p[i] / p[3] for i in range(3)] if p[3] > 0 else None  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(v) for v in straight(p)]  # noqa: E731

    # 001: every row the same; every pixel changes; and the ramp is its own mirror image, so
    # that a pixel and the one opposite it across the edge sum to skin plus shadow. Skin gets
    # darker towards the edge and shadow lighter, and neither passes the other's colour.
    one = c["FX-SELBLUR-001"]["0"]
    for y in range(1, H):
        assert one[y * W:(y + 1) * W] == one[:W]
    assert all(one[i] != plain["halves"][i] for i in range(W * H))
    row = [enc(p) for p in one[:W]]
    for x in range(6):
        for ch in range(3):
            assert abs(row[x][ch] + row[11 - x][ch] - (SKIN[ch] + SHADOW[ch]) / 255) < 1e-12
    reds = [p[0] * 255 for p in row]
    assert SKIN[0] > reds[0] and all(a > b for a, b in zip(reds, reds[1:])) and \
        reds[-1] > SHADOW[0], reds
    for p in one:
        assert p[3] == 1

    assert c["FX-SELBLUR-002"]["0"] == plain["halves"]
    assert c["FX-SELBLUR-003"]["0"] == plain["halves"]
    assert c["FX-SELBLUR-004"]["0"] == plain["lined"]

    # 005: the skin and shadow change, and the highlight is exactly as drawn.
    five = c["FX-SELBLUR-005"]["0"]
    for i in range(W * H):
        assert (five[i] != plain["stripes"][i]) == (i % W < 8), i % W

    # 006: nothing stays exactly nothing, and inside the block the rows are 001's ramp over a
    # shorter run.
    six = c["FX-SELBLUR-006"]["0"]
    for i in range(W * H):
        if plain["block"][i][3] == 0:
            assert six[i] == [0.0] * 4
    assert six[3 * W + 2:3 * W + 10] != plain["block"][3 * W + 2:3 * W + 10]

    # 007: the covering is the drawing's, and the colour is 001's.
    seven = c["FX-SELBLUR-007"]["0"]
    for i in range(W * H):
        assert seven[i][3] == plain["faint"][i][3]
        assert all(abs(a - b) < 1e-12 for a, b in zip(straight(seven[i]), straight(one[i])))

    assert c["FX-SELBLUR-008"]["0"] == plain["halves"]
    twelve_blur = render(case("halves", blur=12), 0)
    assert c["FX-SELBLUR-009"] == {"0": plain["halves"], "2": one, "4": twelve_blur}
    assert twelve_blur != one
    moved = c["FX-SELBLUR-010"]["0"]
    assert moved == c["FX-SELBLUR-010"]["3"]
    for y in range(H):
        assert moved[y * W:y * W + 2] == [[0.0] * 4] * 2
        assert moved[y * W + 2:(y + 1) * W] == one[y * W:(y + 1) * W - 2]
    assert c["FX-SELBLUR-011"]["0"] == one
    assert c["FX-SELBLUR-012"]["0"] != plain["diagonal"]
    assert c["FX-SELBLUR-013"]["0"] not in (plain["diagonal"], c["FX-SELBLUR-012"]["0"])
    assert c["FX-SELBLUR-014"]["0"] == plain["halves"]
    assert c["FX-SELBLUR-015"]["0"] == one

    # Every value stays a colour: none below nothing or above full.
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert all(0 <= v <= 1 for v in p), (name, p)


if __name__ == "__main__":
    main()
