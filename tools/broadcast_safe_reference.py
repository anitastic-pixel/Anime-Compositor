"""Broadcast Safe, worked a second way.

`core.broadcast_safe`, modelled on After Effects' Broadcast Colors: colours too strong for an
analogue television signal (above a chosen height of the composite signal, in IRE) are brought
down, either darkened or made greyer, or keyed out, or everything else keyed out to show where
they are. Nothing is ported; Adobe does not publish its method, so the rule below is ours.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. At a pixel with covering a > 0, e = (eR, eG, eB), its straight colour through the sRGB
curve held inside 0 to 1. The space: this program works in linear light, but a video signal is
made from gamma-encoded values, so the signal is worked on e, the encoded colour, as if it were
video's R'G'B' (a deviation recorded in document 21). Then

- Y = 0.299 eR + 0.587 eG + 0.114 eB, the picture's brightness as composite video weighs it;
- U = 0.492 (eB - Y), V = 0.877 (eR - Y), and C = sqrt(U^2 + V^2), the colour signal's swing;
- the signal's top, in IRE: amp = s + (100 - s)(Y + C), where s, the set-up, is 7.5 for
  `locale` "ntsc" and 0 for "pal". So white is 100 IRE, black is s, and pure yellow is about
  131 IRE on NTSC, as test bars measure it.

A pixel is unsafe when amp is above `max_amplitude`, 90 to 120, 110 when added. Let
m = (max_amplitude - s) / (100 - s), the highest Y + C allowed. `method`:

- "reduce_luminance" (the default): an unsafe pixel's three channels multiplied by m / (Y + C),
  so its top lands exactly on the limit, its hue and saturation kept, darker.
- "reduce_saturation": an unsafe pixel moved straight toward its own grey Y,
  Y + (e - Y)(m - Y) / C, so its top lands on the limit at the same brightness; if Y alone is
  past m (a limit below 100 IRE), the pixel becomes the grey (m, m, m).
- "key_out_unsafe": an unsafe pixel cleared, all four numbers 0; the rest left exactly as they
  are.
- "key_out_safe": a safe pixel cleared; the unsafe ones left exactly as they are.

With a reduce method every result, held inside 0 to 1, comes back to linear at the pixel's own
covering (document 21's shared colour rule), a safe pixel too. Only the top of the signal is
checked, not a dip below black. Words are exact.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision
buffers. It asserts that no pixel's signal lies within a thousandth of an IRE of the limit, so
rounding can't put a pixel on the other side.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: columns of strong and everyday colours, darker row by row, drawn below. The drawing
goes into `Fixtures/broadcast_safe/media`, the projects into `Fixtures/broadcast_safe`, and the
expected frames into `Fixtures/broadcast_safe/expected_broadcast_safe.json`. The colour
neutralizer's and colour offset's references draw the same picture.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/broadcast_safe_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "broadcast_safe"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGE = (90, 120)
SETUP = {"ntsc": 7.5, "pal": 0.0}
METHODS = ("reduce_luminance", "reduce_saturation", "key_out_unsafe", "key_out_safe")


# --- the drawing, shared with the colour neutralizer's and colour offset's references ---------

COLOURS = [
    None,               # 0: empty
    (0, 0, 0),          # 1: black
    (255, 255, 255),    # 2: white
    (128, 128, 128),    # 3: grey
    (255, 255, 0),      # 4: yellow
    (0, 255, 255),      # 5: cyan
    (0, 255, 0),        # 6: green
    (255, 0, 255),      # 7: magenta
    (255, 0, 0),        # 8: red
    (0, 0, 255),        # 9: blue
    (246, 214, 190),    # 10: skin
    (255, 140, 0),      # 11: orange
    (60, 45, 30),       # 12: a warm shadow
    (150, 128, 100),    # 13: a warm midtone
    (240, 230, 200),    # 14: a warm highlight
    (255, 255, 0),      # 15: yellow at half covering
]


def pixel(x, y):
    """Column x's colour, each channel times 1 - 0.08 y rounded, so row 0 is the colour itself
    and row 9 under a third of it."""
    c = COLOURS[x]
    if c is None:
        return S.NONE
    f = 1 - 0.08 * y
    return tuple(int(v * f + 0.5) for v in c) + ((128,) if x == 15 else (255,))


DRAWINGS = {"colours": [[pixel(x, y) for x in range(W)] for y in range(H)]}


def pixels():
    return [p for row in DRAWINGS["colours"] for p in row]


def encoded(p):
    """A drawing pixel's encoded straight colour, exactly its 8-bit values over 255."""
    return [v / 255 for v in p[:3]]


def back(e, p):
    """An encoded colour held inside 0 to 1, back to linear at pixel p's covering."""
    a = p[3] / 255
    return [srgb_to_linear(min(1.0, max(0.0, v))) * a for v in e] + [a]


def project_json(fx, type_id, params, shift):
    p = S.project_json(fx, {"drawing": "colours", "shift": shift, "softness": 0,
                            "threshold": 0})
    p["assets"][0]["path"] = "media/colours.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + shift, H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": type_id, "enabled": True, "parameters": params}]
    return p


def write_cases(out, stem, cases, invalid, render, plain, file_json):
    """Writes the drawing, every case's project and the expected frames; returns them."""
    (out / "media").mkdir(parents=True, exist_ok=True)
    (out / "media" / "colours.png").write_bytes(S.png(DRAWINGS["colours"]))

    def write(fx, c):
        name = f"{fx.lower().replace('-', '_')}.json"
        (out / name).write_text(json.dumps(file_json(fx, c), indent=2) + "\n", encoding="utf-8")
        return name

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in cases.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in invalid.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (out / f"expected_{stem}.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    return expected


# --- the rule -------------------------------------------------------------------------------

def signal(e, locale):
    """Y, C and the signal's top in IRE for an encoded colour."""
    y = 0.299 * e[0] + 0.587 * e[1] + 0.114 * e[2]
    u, v = 0.492 * (e[2] - y), 0.877 * (e[0] - y)
    c = math.hypot(u, v)
    s = SETUP[locale]
    return y, c, s + (100 - s) * (y + c)


def broadcast(px, locale, method, top):
    out = []
    s = SETUP[locale]
    m = (top - s) / (100 - s)
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        e = encoded(p)
        y, c, amp = signal(e, locale)
        assert abs(amp - top) > 1e-3, "a pixel's signal lies on the limit"
        unsafe = amp > top
        if method == "key_out_unsafe" or method == "key_out_safe":
            cleared = unsafe == (method == "key_out_unsafe")
            out.append([0.0] * 4 if cleared else R.working(p))
            continue
        if unsafe and method == "reduce_luminance":
            e = [v * m / (y + c) for v in e]
        elif unsafe:
            e = [y + (v - y) * (m - y) / c for v in e] if y < m and c > 0 else [m] * 3
        out.append(back(e, p))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(locale="ntsc", method="reduce_luminance", top=110, shift=0):
    return {"locale": locale, "method": method, "top": top, "shift": shift}


def render(c, frame_no):
    top = min(RANGE[1], max(RANGE[0], value_at(c["top"], frame_no)))
    return R.frame(broadcast(pixels(), c["locale"], c["method"], top), c["shift"])


def plain(c):
    return R.frame([R.working(p) for p in pixels()], c["shift"])


def file_json(fx, c):
    return project_json(fx, "core.broadcast_safe", {
        "locale": c["locale"], "method": c["method"],
        "max_amplitude": setting_json(c["top"])}, c["shift"])


CASES = {
    "FX-BCAST-001": ("NTSC, reduce luminance, 110 IRE, the settings as they start: the three "
                     "brightest rows of yellow (pure yellow is about 131 IRE) and cyan and the "
                     "top row of green are darkened until their signal is 110 IRE, keeping "
                     "their colour; everything else is safe and stays as it is.", case(), [0]),
    "FX-BCAST-002": ("Reduce saturation: the same unsafe pixels keep their brightness and are "
                     "made greyer until their signal is 110 IRE.",
                     case(method="reduce_saturation"), [0]),
    "FX-BCAST-003": ("Key out unsafe: the unsafe pixels are cleared, showing what is behind; "
                     "everything else stays exactly as it is.",
                     case(method="key_out_unsafe"), [0]),
    "FX-BCAST-004": ("Key out safe: the other way round, only the unsafe pixels are left, to "
                     "show where they are.", case(method="key_out_safe"), [0]),
    "FX-BCAST-005": ("PAL: no set-up, so black is 0 IRE rather than 7.5 and the limit allows a "
                     "little less colour signal; the same pixels are unsafe as on NTSC, each "
                     "brought down a little further.",
                     case(locale="pal"), [0]),
    "FX-BCAST-006": ("NTSC at 90 IRE: even the white and the light greys are above the limit, "
                     "so they are darkened too.", case(top=90), [0]),
    "FX-BCAST-007": ("NTSC at 120 IRE, the loosest limit: only the two brightest rows of "
                     "yellow and cyan are brought down.", case(top=120), [0]),
    "FX-BCAST-008": ("PAL at 95 IRE, reduce saturation: the white's brightness alone is past "
                     "the limit and it has no colour to take away, so it becomes the grey at "
                     "the limit; the bright colours lose colour, keeping their brightness.",
                     case(locale="pal", method="reduce_saturation", top=95), [0]),
    "FX-BCAST-009": ("The limit keyed from 90 IRE at frame 0 to 120 at frame 4, linear: frame "
                     "0 is FX-BCAST-006, frame 2 is 105 IRE, frame 4 is FX-BCAST-007.",
                     case(top=keyed((0, 90), (4, 120))), [0, 2, 4]),
    "FX-BCAST-010": ("FX-BCAST-002 moved three pixels right: the same, moved.",
                     case(method="reduce_saturation", shift=3), [0, 3]),
}

INVALID = {
    "FX-BCAST-011": ("Maximum signal amplitude 121, above 120.", case(top=121)),
    "FX-BCAST-012": ("Maximum signal amplitude 89, below 90.", case(top=89)),
    "FX-BCAST-013": ("Locale \"secam\", which is not one.", case(locale="secam")),
    "FX-BCAST-014": ("Locale \"NTSC\": the word is exact, so capitals are not it.",
                     case(locale="NTSC")),
    "FX-BCAST-015": ("Method \"reduce\", which is not one.", case(method="reduce")),
}


def main():
    expected = write_cases(OUT, "broadcast_safe", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = pixels()
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    amp = lambda i, loc="ntsc": signal(encoded(px[i]), loc)[2]  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3] > 0]
    unsafe = lambda top, loc="ntsc": {i for i in shows if amp(i, loc) > top}  # noqa: E731

    # The signal's own scale: black at the set-up, white at 100, pure yellow about 131 on NTSC.
    assert abs(amp(at(1, 0)) - 7.5) < 1e-12 and abs(amp(at(1, 0), "pal")) < 1e-12
    assert abs(amp(at(2, 0)) - 100) < 1e-12 and 130 < amp(at(4, 0)) < 132
    # Which pixels are unsafe at 110 NTSC: bright yellow, cyan, green, magenta, orange.
    bad = unsafe(110)
    assert bad == {at(x, y) for x in (4, 5, 15) for y in range(3)} | {at(6, 0)}
    assert unsafe(110, "pal") == bad and unsafe(120) < bad < unsafe(90)
    assert at(2, 0) in unsafe(90) and at(2, 0) not in unsafe(110)

    one = c["FX-BCAST-001"]["0"]
    for i in shows:
        e = encoded(px[i])
        y, cc, a = signal(e, "ntsc")
        if i in bad:
            out = [S.linear_to_srgb(v / one[i][3]) for v in one[i][:3]]
            assert abs(signal(out, "ntsc")[2] - 110) < 1e-6  # lands on the limit
            assert near([v * (y + cc) for v in out], [v * 102.5 / 92.5 for v in e], 1e-9)
        else:
            assert near(one[i], drawn[i])
    two = c["FX-BCAST-002"]["0"]
    for i in bad:
        out = [S.linear_to_srgb(v / two[i][3]) for v in two[i][:3]]
        y0 = signal(encoded(px[i]), "ntsc")[0]
        assert abs(signal(out, "ntsc")[0] - y0) < 1e-9  # the brightness kept
        assert abs(signal(out, "ntsc")[2] - 110) < 1e-6
    three, four = c["FX-BCAST-003"]["0"], c["FX-BCAST-004"]["0"]
    for i in range(W * H):
        assert (three[i] == [0.0] * 4) == (i in bad or px[i][3] == 0)
        assert (four[i] == [0.0] * 4) == (i not in bad)
        assert three[i] == drawn[i] or four[i] == drawn[i]
    eight = c["FX-BCAST-008"]["0"]
    m = 0.95
    for y in range(H):
        i = at(2, y)
        if signal(encoded(px[i]), "pal")[0] > m:
            assert near(eight[i], back([m] * 3, px[i]))
    assert near(eight[at(2, 0)], back([m] * 3, px[at(2, 0)]))
    nine = c["FX-BCAST-009"]
    assert like(nine["0"], c["FX-BCAST-006"]["0"]) and like(nine["4"], c["FX-BCAST-007"]["0"])
    assert like(nine["2"], render(case(top=105), 0))
    moved = c["FX-BCAST-010"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    # The soft yellow is graded as the yellow, at half covering.
    assert near([v * 255 / 128 for v in one[at(15, 0)][:3]], one[at(4, 0)][:3])
    for fx, frames in c.items():
        for f in frames.values():
            for p in f:
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
