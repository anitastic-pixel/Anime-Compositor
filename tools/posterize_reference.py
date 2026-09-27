"""Posterize, worked a second way.

D-137 adds `core.posterize`. It cuts each of a drawing's colour channels down to a few flat
steps, as a cel is painted in a handful of flat tones or a print is made in a few inks: a smooth
shading turns into bands. `levels` is 2 to 256, starting at 6, of which only the whole part
counts: how many steps each of red, green and blue keeps, spread evenly from none to full on the
encoded scale a drawing program shows. At 2 every channel is either off or full; at 256 a
drawing's own 8-bit colours are kept exactly. The covering is kept, so a soft edge stays soft and
takes the same steps as its colour, and a pixel that does not show stays as it is. It is this
program's own method, modelled on After Effects' Posterize; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. At a pixel with covering a > 0, with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its
encoded straight colour and n the levels, held inside 2..256 and then floored: per channel
e'_c = min(floor(e_c n + 1e-4), n - 1) / (n - 1), and the output is (srgb_to_linear(e'_c) * a,
a). The rule is a cliff: a channel just below a step and one just above it land a whole step
apart. The 1e-4 decides an 8-bit value that sits exactly on a step (85 at levels 3, say) upward,
the same way in the build's single precision as here; `check` asserts that no fixture channel
lies within 1e-5 of a step after it. The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: four bands of ramps and swatches, drawn below. The drawing goes into
`Fixtures/posterize/media`, the projects into `Fixtures/posterize`, and the expected frames into
`Fixtures/posterize/expected_posterize.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/posterize_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "posterize"
TOLERANCE = 2e-5  # document 25's default for a filter
LEVELS = (2, 256)
NUDGE = 1e-4      # the rule's own lift before the floor
CLIFF = 1e-5      # how near a step no fixture channel may lie


# --- the rule -------------------------------------------------------------------------------

def step(e, n):
    """One encoded channel cut to n steps."""
    return min(math.floor(e * n + NUDGE), n - 1) / (n - 1)


def posterize(pixels, n):
    """`pixels` are 8-bit straight RGBA; n is already held and floored."""
    out = []
    for p in pixels:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        a = p[3] / 255
        out.append([srgb_to_linear(step(p[c] / 255, n)) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                    # #1e1a24
SOFT = R.SOFT                    # the line at half covering, its antialiased edge
SKIN = R.SKIN                    # #f6d6be
SOFT_SKIN = SKIN[:3] + (128,)    # the skin at half covering
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
TRACE = R.TRACE                  # a red, #c82828
BLUE = (64, 96, 255, 255)        # #4060ff
WHITE = (255, 255, 255, 255)
BLACK = (0, 0, 0, 255)
NONE = S.NONE
# Greys either side of levels 6's steps (at 42.5, 85, 127.5, 170 and 212.5) and on them.
GREYS = (0, 20, 42, 43, 64, 85, 100, 127, 128, 150, 170, 212, 213, 255)
SOFT_GREY = (128, 128, 128, 128)
WARM = [(round(255 * i / 13), 96, 255 - round(255 * i / 13), 255) for i in range(14)]
SOFT_WARM = WARM[-1][:3] + (128,)
SWATCH = (LINE, LINE, SKIN, SKIN, SKIN, SHADE, SHADE, SHADE, TRACE, TRACE, BLUE, BLUE, WHITE,
          BLACK)
SHADING = [tuple(round(u + (v - u) * i / 13) for u, v in zip(SHADE[:3], SKIN[:3])) + (255,)
           for i in range(14)]


def bands(x, y):
    """Column 0 and rows 0 and 9 are empty. In columns 1 to 14: rows 1 and 2 a grey ramp,
    GREYS; rows 3 and 4 a warm-to-cool ramp, red rising from 0 to 255 as blue falls, green 96;
    rows 5 and 6 swatches, line 1-2, skin 3-5, shadow 6-8, red 9-10, blue 11-12, white 13 and
    black 14; rows 7 and 8 a skin shading, shadow #dca08c at column 1 to skin at column 14.
    Column 15 holds each band's soft edge at half covering: the grey 128, the ramp's last
    colour, the skin and the line."""
    if x == 0 or y in (0, 9):
        return NONE
    band = (y - 1) // 2
    if x == 15:
        return (SOFT_GREY, SOFT_WARM, SOFT_SKIN, SOFT)[band]
    g = GREYS[x - 1]
    return ((g, g, g, 255), WARM[x - 1], SWATCH[x - 1], SHADING[x - 1])[band]


DRAWINGS = {"ramps": [[bands(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(levels=6, shift=0):
    return {"drawing": "ramps", "levels": levels, "shift": shift}


def levels_at(c, frame_no):
    """Held inside 2..256, then floored."""
    return math.floor(min(LEVELS[1], max(LEVELS[0], value_at(c["levels"], frame_no))))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(posterize(pixels, levels_at(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(levels=256, shift=c["shift"]), 0)


CASES = {
    "FX-POSTER-001": ("Levels 6, as it starts: every channel of every pixel that shows lands on "
                      "one of six steps, 0, 51, 102, 153, 204 or 255 (encoded, of 255); the grey "
                      "ramp turns to six flat greys, the greys 42 and 43 either side of a step "
                      "landing on 0 and 51, 127 and 128 on 102 and 153, 212 and 213 on 204 and "
                      "255; black and white stay; a pixel at half covering takes the same steps "
                      "as its colour at full covering, at its own covering; the empty pixels "
                      "stay empty.",
                      case(), [0]),
    "FX-POSTER-002": ("Levels 2, the fewest: every channel is either 0 or full, so every pixel "
                      "is one of eight colours; the grey 127 turns black and 128 white, the "
                      "skin and its shadow white, the line black, the red #ff0000 and the blue "
                      "#0000ff.",
                      case(levels=2), [0]),
    "FX-POSTER-003": ("Levels 3: steps 0, 127.5 and 255; the greys 85 and 170 sit exactly on "
                      "the steps and are taken up to 127.5 and 255, as the rule's 1e-4 "
                      "decides, while 64 falls to 0, 128 lands on 127.5 and 212 on 255.",
                      case(levels=3), [0]),
    "FX-POSTER-004": ("Levels 16: sixteen steps 17 apart; every channel lands on a step within "
                      "one step of its own value, and the ramps show their bands close "
                      "together.",
                      case(levels=16), [0]),
    "FX-POSTER-005": ("Levels 256, the most: every 8-bit channel is already on a step, so the "
                      "drawing is untouched, exactly.",
                      case(levels=256), [0]),
    "FX-POSTER-006": ("Levels 255: 255 steps, one fewer than an 8-bit value has, so every "
                      "channel between 0 and 255 is lifted a little, by at most one 8-bit step "
                      "(v becomes 255 v / 254); black and white stay.",
                      case(levels=255), [0]),
    "FX-POSTER-007": ("Levels 6.9: only the whole part counts, so it is FX-POSTER-001.",
                      case(levels=6.9), [0]),
    "FX-POSTER-008": ("Levels keyed from 2 at frame 0 to 10 at frame 4, linear: frame 0 is "
                      "FX-POSTER-002, frame 2, at 6, is FX-POSTER-001, and frame 4 has ten "
                      "steps.",
                      case(levels=keyed((0, 2), (4, 10))), [0, 2, 4]),
    "FX-POSTER-009": ("Levels keyed from 4 at frame 0 to 7 at frame 4, linear, every frame: "
                      "4.75 at frame 1 counts as 4, the same as frame 0; 5.5 at frame 2 counts "
                      "as 5; 6.25 at frame 3 counts as 6, FX-POSTER-001; frame 4 has seven "
                      "steps.",
                      case(levels=keyed((0, 4), (4, 7))), [0, 1, 2, 3, 4]),
    "FX-POSTER-010": ("Levels eased from 256 at frame 0 to 2 at frame 4 on a curve that "
                      "overshoots: frame 0 is the drawing; at frame 2 the levels have gone past "
                      "2 (to -80.55) and are held at 2, so frames 2 and 4 are both "
                      "FX-POSTER-002.",
                      case(levels=keyed((0, 256, OVERSHOOT), (4, 2))), [0, 2, 4]),
    "FX-POSTER-011": ("Levels 3 held (a hold key) at frame 0, then 16 at frame 4: frames 0 and "
                      "2 are FX-POSTER-003 and frame 4 is FX-POSTER-004.",
                      case(levels=keyed((0, 3, "hold"), (4, 16))), [0, 2, 4]),
    "FX-POSTER-012": ("FX-POSTER-001 moved three pixels right: the same, moved.",
                      case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-POSTER-013": ("Levels 1, below 2.", case(levels=1)),
    "FX-POSTER-014": ("Levels 257, above 256.", case(levels=257)),
    "FX-POSTER-015": ("Levels 256.5, above 256: the range is on the number as written, before "
                      "its whole part is taken.", case(levels=256.5)),
    "FX-POSTER-016": ("Levels -6, below 2.", case(levels=-6)),
    "FX-POSTER-017": ("Levels keyed to 300 at frame 4.", case(levels=keyed((0, 6), (4, 300)))),
    "FX-POSTER-018": ("Levels keyed from 0 at frame 0.", case(levels=keyed((0, 0), (4, 6)))),
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
        "instance_id": "fx-0-0", "type_id": "core.posterize", "enabled": True,
        "parameters": {"levels": setting_json(c["levels"])}}]
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
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_posterize.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda u, v, e=1e-12: all(abs(p - q) < e for p, q in zip(u, v))  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    lin = lambda *e8: [srgb_to_linear(v / 255) for v in e8] + [1.0]  # noqa: E731
    flat = [p for row in DRAWINGS["ramps"] for p in row]
    shows = [i for i in range(W * H) if flat[i][3] > 0]
    grey = {g: at(x + 1, 1) for x, g in enumerate(GREYS)}
    soft_pairs = [(at(15, 1), grey[128]), (at(15, 3), at(14, 3)), (at(15, 5), at(3, 5)),
                  (at(15, 7), at(1, 5))]

    # The rule's own pieces: the ends, a step's two sides, the 1e-4 on a step, and levels 256
    # keeping every 8-bit value.
    assert step(0.0, 2) == 0 and step(1.0, 2) == 1 and step(1.0, 256) == 1
    assert step(127 / 255, 2) == 0 and step(128 / 255, 2) == 1
    assert step(85 / 255, 3) == 0.5 and step(170 / 255, 3) == 1
    assert all(step(v / 255, 256) == v / 255 for v in range(256))
    assert all(step(v / 255, 255) == (v if v < 255 else 254) / 254 for v in range(256))

    # The cliff: no channel any fixture decides lies within 1e-5 of a step, 1e-4 included.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            n = levels_at(cs, f)
            for i in shows:
                for v in flat[i][:3]:
                    x = v / 255 * n + NUDGE
                    assert abs(x - round(x)) >= CLIFF, (fx, f, i, v)

    # A pixel that does not show, and the covering, are kept by every case; a soft pixel takes
    # its full colour's steps.
    for fx, frames in c.items():
        moved = fx == "FX-POSTER-012"
        base = plain(case(shift=3 if moved else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
            if not moved:
                for s, full in soft_pairs:
                    assert near(enc(px[s]), enc(px[full]), 1e-9), (fx, s)

    def on_steps(f, n):
        return all(abs(e * (n - 1) - round(e * (n - 1))) < 1e-9 for i in shows for e in enc(f[i]))

    one = c["FX-POSTER-001"]["0"]
    assert on_steps(one, 6) and not on_steps(drawn, 6)
    for g, want in ((0, 0), (42, 0), (43, 51), (85, 102), (127, 102), (128, 153), (170, 204),
                    (212, 204), (213, 255), (255, 255)):
        assert near(one[grey[g]], lin(want, want, want)), g
    assert one[grey[0]] == drawn[grey[0]] and one[grey[255]] == drawn[grey[255]]
    assert len({tuple(one[at(x, 1)]) for x in range(1, 15)}) == 6

    two = c["FX-POSTER-002"]["0"]
    assert on_steps(two, 2) and len({tuple(two[i]) for i in shows if flat[i][3] == 255}) <= 8
    assert near(two[grey[127]], lin(0, 0, 0)) and near(two[grey[128]], lin(255, 255, 255))
    for x, want in ((1, (0, 0, 0)), (3, (255, 255, 255)), (6, (255, 255, 255)),
                    (9, (255, 0, 0)), (11, (0, 0, 255))):
        assert near(two[at(x, 5)], lin(*want)), x

    three = c["FX-POSTER-003"]["0"]
    assert on_steps(three, 3)
    for g, want in ((64, 0), (85, 127.5), (128, 127.5), (170, 255), (212, 255)):
        assert near(three[grey[g]], lin(want, want, want)), g

    sixteen = c["FX-POSTER-004"]["0"]
    assert on_steps(sixteen, 16)
    for i in shows:
        assert all(abs(u - v) <= 1 / 15 + 1e-12 for u, v in zip(enc(sixteen[i]), enc(drawn[i])))
    assert len({tuple(sixteen[at(x, 1)]) for x in range(1, 15)}) > 6

    assert c["FX-POSTER-005"]["0"] == drawn
    lifted = c["FX-POSTER-006"]["0"]
    for i in shows:
        for u, v in zip(enc(lifted[i]), enc(drawn[i])):
            assert v - 1e-12 <= u <= v + 1 / 255 + 1e-12, i
            assert (abs(u - v) < 1e-9) == (abs(v) < 1e-9 or abs(v - 1) < 1e-9), i
    assert near(lifted[grey[127]], lin(*[255 * 127 / 254] * 3))

    assert c["FX-POSTER-007"]["0"] == one
    eight = c["FX-POSTER-008"]
    assert eight["0"] == two and eight["2"] == one and eight["4"] == render(case(levels=10), 0)
    assert on_steps(eight["4"], 10)
    nine = c["FX-POSTER-009"]
    assert nine["0"] == nine["1"] == render(case(levels=4), 0)
    assert nine["2"] == render(case(levels=5), 0) and nine["3"] == one
    assert nine["4"] == render(case(levels=7), 0) and nine["2"] != nine["1"]
    ten = c["FX-POSTER-010"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(CASES["FX-POSTER-010"][1]["levels"], 2) < 2
    assert ten["0"] == drawn and ten["2"] == two and ten["4"] == two
    eleven = c["FX-POSTER-011"]
    assert eleven["0"] == three and eleven["2"] == three and eleven["4"] == sixteen
    moved = c["FX-POSTER-012"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
