"""Text animators, worked a second way.

D-350 adds `core.text_animator`, after After Effects' text animators: an effect on a text layer
that moves, scales, turns, fades, recolours and spaces the layer's characters one by one, each by
how much a range selector picks it. It is this program's own method, modelled on After Effects'
Animate menu and its Range Selector; nothing is ported. Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The settings, each in the file under its own name:

- position, two numbers, x then y, pixels, -100000 to 100000, starting 0, 0
- scale, two numbers, per cent, -10000 to 10000, starting 100, 100
- rotation, degrees, -36000 to 36000, starting 0; positive turns clockwise on the picture
- opacity, per cent, 0 to 100, starting 100
- fill, "off" or "on", starting "off"; color, linear RGB 0 to 1, starting 1, 0, 0
- tracking, thousandths of the text's size, -1000 to 1000, starting 0
- start, end, per cent, 0 to 100, starting 0 and 100; offset, per cent, -100 to 100, starting 0
- amount, per cent, -100 to 100, starting 100
- based_on, "characters", "characters_excluding_spaces" or "words", starting "characters"
- shape, "square", "ramp_up", "ramp_down", "triangle", "round" or "smooth", starting "square"
- smoothness, per cent, 0 to 100, starting 100, read by the square only
- ease_high, ease_low, per cent, 0 to 100, starting 0

The rule.

1. Units. The words are taken as the layer lays them out (after All Caps). Line breaks, and a
   carriage return before one, are not characters. "characters": every other character is a unit, numbered from 0. "characters
   excluding spaces": every character that is not white space. "words": each run of characters
   between white space. White space that is not a unit is never picked. n is the number of units.
2. The range, in units: S = (start + offset) / 100 * n and E = (end + offset) / 100 * n, swapped
   when S > E. Unit k spans k to k + 1.
3. How much unit k is picked, v:
   - square: c = min(E, k + 1) - max(S, k), held to 0..1, the part of the unit inside the range.
     With w = smoothness / 100: v = (c - 0.5) / w + 0.5 held to 0..1; with w = 0, v = 1 when
     c >= 0.5, else 0.
   - the others read the unit's middle, m = k + 0.5, as u = (m - S) / (E - S); when E = S, u is
     taken as past the range when m >= S and before it otherwise.
     ramp_up: u held to 0..1 (0 before the range, 1 after it). ramp_down: 1 less ramp_up.
     triangle: 1 - |2u - 1| inside the range (0 <= u <= 1), else 0.
     round: sqrt(1 - (2u - 1)^2) inside, else 0. smooth: (1 - cos(2 pi u)) / 2 inside, else 0.
4. Ease: when ease_high or ease_low is not 0, v becomes y at x = v on the curve from (0, 0) to
   (1, 1) with handles (ease_low / 100, 0) and (1 - ease_high / 100, 1), document 20's ease
   curve; ease_low flattens the picking's low end, ease_high its high end.
5. The character's amount from this animator, a = v * amount / 100.
6. Each character starts at no move, no turn, scale 1, opacity 1 and the text's colour. Each
   animator, top of the stack first, adds a * position to its move, a * rotation to its turn and
   a * tracking / 1000 * size to the room after it; multiplies its scale by 1 + a * (scale / 100
   - 1), each way, and its opacity by 1 + a * (opacity / 100 - 1), the opacity held to 0..1; and,
   with fill on, mixes its colour towards color by a held to 0..1.
7. Layout is D-263's and D-264's with the extra room: each character's pen moves by its advance,
   the text's tracking and its animators' room. A centred or right-aligned line is measured with
   the room. The character's anchor is on its baseline, halfway across its advance.
8. Each outline point p of the character, as the font stores it (not moved to meet the side
   bearing) and placed as D-263 places it (faux italic included),
   goes to anchor + turn(scale * (p - anchor)) + move, the turn clockwise on the picture.
9. A character whose opacity is 0 is not drawn. Fill and stroke are drawn with the character's
   opacity and colour (the stroke keeps its own colour); every stroke is drawn before every fill,
   as D-264 does.

**This file never runs the build's code path.** It reads the bundled font with fontTools, not
ttf-parser, and finds each outline's exact box from its curves, where the build fills straight
pieces about 2 pixels long (D-263). That is why a box is held to 0.25 pixel and every other number
to 1e-6.

The cases go into `Fixtures/text_animator/expected_text_animator.json`; a few as projects,
`Fixtures/text_animator/fx_txa_NNN.json`, for the file, keys and warnings.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/text_animator_reference.py
"""

import json
import math
from pathlib import Path

from fontTools.pens.basePen import BasePen
from fontTools.ttLib import TTFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "text_animator"
FONT = TTFont(str(ROOT / "assets" / "fonts" / "MPLUSRounded1c-Regular.ttf"))
GLYPHS = FONT.getGlyphSet()
CMAP = FONT.getBestCmap()
UPEM = FONT["head"].unitsPerEm
OS2 = FONT["OS/2"]
TYPO = bool(OS2.fsSelection & 0x80)
ASCENT = OS2.sTypoAscender if TYPO else FONT["hhea"].ascent
DESCENT = OS2.sTypoDescender if TYPO else FONT["hhea"].descent
GAP = OS2.sTypoLineGap if TYPO else FONT["hhea"].lineGap
BOX_TOLERANCE = 0.25
TOLERANCE = 1e-6

DEFAULTS = {
    "position": [0, 0], "scale": [100, 100], "rotation": 0, "opacity": 100, "fill": "off",
    "color": [1, 0, 0], "tracking": 0, "start": 0, "end": 100, "offset": 0, "amount": 100,
    "based_on": "characters", "shape": "square", "smoothness": 100, "ease_high": 0, "ease_low": 0,
}


def animator(**settings):
    return {**DEFAULTS, **settings}


def text(words, **settings):
    return {"text": words, "font": "MPLUSRounded1c-Regular.ttf", "size": 100, "color": [1, 1, 1],
            "at": [80, 200], "align": "left", **settings}


# --- the selector ---------------------------------------------------------------------------

def units(chars, based_on):
    """Each character's unit, or None, and how many units there are."""
    out, n, inside = [], 0, False
    for c in chars:
        space = c.isspace()
        if based_on == "characters":
            out.append(n)
            n += 1
        elif based_on == "characters_excluding_spaces":
            out.append(None if space else n)
            n += 0 if space else 1
        else:  # words
            if space:
                out.append(None)
                if inside:
                    n += 1
                inside = False
            else:
                out.append(n)
                inside = True
    if based_on == "words" and inside:
        n += 1
    return out, n


def bezier(a, b, t):
    v = 1 - t
    return 3 * v * v * t * a + 3 * v * t * t * b + t ** 3


def ease(v, high, low):
    if high == 0 and low == 0:
        return v
    x1, x2 = low / 100, 1 - high / 100
    if v <= 0 or v >= 1:
        return min(1.0, max(0.0, v))
    lo, hi = 0.0, 1.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if bezier(x1, x2, mid) < v:
            lo = mid
        else:
            hi = mid
    return bezier(0.0, 1.0, (lo + hi) / 2)


def picked(k, n, a):
    s = (a["start"] + a["offset"]) / 100 * n
    e = (a["end"] + a["offset"]) / 100 * n
    if s > e:
        s, e = e, s
    shape = a["shape"]
    if shape == "square":
        c = min(1.0, max(0.0, min(e, k + 1) - max(s, k)))
        w = a["smoothness"] / 100
        v = (1.0 if c >= 0.5 else 0.0) if w == 0 else min(1.0, max(0.0, (c - 0.5) / w + 0.5))
    else:
        m = k + 0.5
        u = (m - s) / (e - s) if e > s else (math.inf if m >= s else -math.inf)
        inside = 0 <= u <= 1
        ramp = min(1.0, max(0.0, u))
        v = {
            "ramp_up": ramp,
            "ramp_down": 1 - ramp,
            "triangle": 1 - abs(2 * u - 1) if inside else 0.0,
            "round": math.sqrt(max(0.0, 1 - (2 * u - 1) ** 2)) if inside else 0.0,
            "smooth": (1 - math.cos(2 * math.pi * u)) / 2 if inside else 0.0,
        }[shape]
    return ease(v, a["ease_high"], a["ease_low"]) * a["amount"] / 100


def amounts(chars, a):
    unit, n = units(chars, a["based_on"])
    return [0.0 if u is None or n == 0 else picked(u, n, a) for u in unit]


# --- the outlines ---------------------------------------------------------------------------

class Box(BasePen):
    """The exact box of an outline after `place`, curves included."""

    def __init__(self, place):
        super().__init__(GLYPHS)
        self.place, self.box, self.now = place, None, None

    def add(self, p):
        b = self.box
        self.box = [p[0], p[1], p[0], p[1]] if b is None else [min(b[0], p[0]), min(b[1], p[1]),
                                                                max(b[2], p[0]), max(b[3], p[1])]

    def _moveTo(self, p):
        self.now = self.place(p)
        self.add(self.now)

    def _lineTo(self, p):
        self.now = self.place(p)
        self.add(self.now)

    def curve(self, pts):
        """A Bezier of any degree: its ends and every turning point, x and y apart."""
        pts = [self.now] + [self.place(p) for p in pts]

        def at(t):
            q = pts
            while len(q) > 1:
                q = [(q[i][0] + (q[i + 1][0] - q[i][0]) * t, q[i][1] + (q[i + 1][1] - q[i][1]) * t)
                     for i in range(len(q) - 1)]
            return q[0]

        for axis in (0, 1):
            c = [p[axis] for p in pts]
            if len(c) == 3:
                d = c[0] - 2 * c[1] + c[2]
                roots = [(c[0] - c[1]) / d] if d != 0 else []
            else:
                # The derivative of a cubic, a t^2 + b t + c.
                qa = -c[0] + 3 * c[1] - 3 * c[2] + c[3]
                qb = 2 * (c[0] - 2 * c[1] + c[2])
                qc = c[1] - c[0]
                if abs(qa) < 1e-12:
                    roots = [-qc / qb] if qb != 0 else []
                else:
                    disc = qb * qb - 4 * qa * qc
                    roots = [] if disc < 0 else [(-qb + s * math.sqrt(disc)) / (2 * qa) for s in (1, -1)]
            for t in roots:
                if 0 < t < 1:
                    self.add(at(t))
        self.now = pts[-1]
        self.add(self.now)

    def _qCurveToOne(self, p1, p2):
        self.curve([p1, p2])

    def _curveToOne(self, p1, p2, p3):
        self.curve([p1, p2, p3])

    def _closePath(self):
        pass


def glyph(c):
    return CMAP.get(ord(c), ".notdef")


def lay_out(t, animators):
    """Every character: its index, anchor, move, scale, turn, opacity, colour, amounts and box."""
    size = t["size"]
    scale = size / UPEM
    words = t["text"].upper() if t.get("all_caps") else t["text"]
    flat = [c for paragraph in words.split("\n") for c in paragraph.rstrip("\r")]
    per = [amounts(flat, a) for a in animators]
    slant = 0.2126 if t.get("faux_italic") else 0.0
    track = t.get("tracking", 0) / 1000 * size
    line = (ASCENT - DESCENT + GAP) * scale
    out, k = [], 0
    for n, paragraph in enumerate(words.split("\n")):
        chars = list(paragraph.rstrip("\r"))
        raw = [FONT["hmtx"][glyph(c)][0] * scale for c in chars]
        state = []
        for i in range(len(chars)):
            s = {"move": [0.0, 0.0], "turn": 0.0, "scale": [1.0, 1.0], "opacity": 1.0,
                 "color": list(map(float, t["color"])), "room": 0.0, "amounts": []}
            for a, row in zip(animators, per):
                v = row[k + i]
                s["amounts"].append(v)
                s["move"] = [s["move"][j] + v * a["position"][j] for j in (0, 1)]
                s["turn"] += v * a["rotation"]
                s["room"] += v * a["tracking"] / 1000 * size
                s["scale"] = [s["scale"][j] * (1 + v * (a["scale"][j] / 100 - 1)) for j in (0, 1)]
                s["opacity"] = min(1.0, max(0.0, s["opacity"] * (1 + v * (a["opacity"] / 100 - 1))))
                if a["fill"] == "on":
                    f = min(1.0, max(0.0, v))
                    s["color"] = [s["color"][j] + f * (a["color"][j] - s["color"][j]) for j in range(3)]
            state.append(s)
        step = [raw[i] + track + state[i]["room"] for i in range(len(chars))]
        width = sum(step[:-1]) + raw[-1] if chars else 0.0
        left = t["at"][0] - {"left": 0.0, "justify": 0.0, "center": width / 2, "right": width}[t["align"]]
        y = t["at"][1] + n * line
        x = left
        for i, c in enumerate(chars):
            s = state[i]
            ax, ay = x + raw[i] / 2, y
            cos, sin = math.cos(math.radians(s["turn"])), math.sin(math.radians(s["turn"]))

            def place(p, x=x, ax=ax, ay=ay, s=s, cos=cos, sin=sin):
                px, py = x + (p[0] + p[1] * slant) * scale, y - p[1] * scale
                dx, dy = (px - ax) * s["scale"][0], (py - ay) * s["scale"][1]
                return (ax + cos * dx - sin * dy + s["move"][0], ay + sin * dx + cos * dy + s["move"][1])

            pen = Box(place)
            # The points as the font stores them. fontTools' glyph set would first slide each
            # outline so its left edge meets the side bearing in hmtx, as FreeType does; D-263
            # places the stored points, so a few letters of this font (x, y, w) differ by 0.4 to
            # 0.6 pixel at size 100 if drawn that way.
            FONT["glyf"][glyph(c)].draw(pen, FONT["glyf"])
            out.append({"index": k + i, "char": c, "anchor": [ax, ay], "move": s["move"],
                        "scale": s["scale"], "turn": s["turn"], "opacity": s["opacity"],
                        "color": s["color"], "amounts": s["amounts"], "box": pen.box})
            x += step[i]
        k += len(chars)
    return out


# --- the cases ------------------------------------------------------------------------------

THIRTY = "Thirty characters on one line!"
assert len(THIRTY) == 30

CASES = {
    "FX-TXA-001": ("Typewriter, hard: Opacity 0, Start 40 per cent of 15 characters, Smoothness 0. "
                   "The first six show; the rest are not drawn.",
                   text("Typewriter text"), [animator(opacity=0, start=40, smoothness=0)]),
    "FX-TXA-002": ("Typewriter, soft: Start 43.3, Smoothness 100. The seventh character is "
                   "half inside the range and half faded.",
                   text("Typewriter text"), [animator(opacity=0, start=43.3)]),
    "FX-TXA-003": ("Fade in by character (tutorial 1): Opacity 0, Ramp Up, Ease High and Low 50, "
                   "Offset -30. Left letters show, a soft edge, right letters hidden.",
                   text("Fade in by letter"),
                   [animator(opacity=0, shape="ramp_up", ease_high=50, ease_low=50, offset=-30)]),
    "FX-TXA-004": ("Fade out (tutorial 1's second animator): Ramp Down, Offset 30.",
                   text("Fade in by letter"),
                   [animator(opacity=0, shape="ramp_down", ease_high=50, ease_low=50, offset=30)]),
    "FX-TXA-005": ("Word by word (tutorial 2): Position 0, 100, Opacity 0, Based On Words, Ramp Up, "
                   "Ease High 25, Ease Low 100, Offset -20. Spaces are never picked.",
                   text("Smooth word by word"),
                   [animator(position=[0, 100], opacity=0, based_on="words", shape="ramp_up",
                             ease_high=25, ease_low=100, offset=-20)]),
    "FX-TXA-006": ("A wave (tutorial 3): Position 0, -60, Start 20, End 50, Offset 10, Triangle. "
                   "The letters in the range rise, most in its middle.",
                   text("Wave of letters"),
                   [animator(position=[0, -60], start=20, end=50, offset=10, shape="triangle")]),
    "FX-TXA-007": ("Smooth and Round: a smooth hump of Position 0, -40 over Start 0 to End 60, Offset "
                   "20, and a round one of Scale 150 over the whole line.",
                   text("Humps and bumps"),
                   [animator(position=[0, -40], end=60, offset=20, shape="smooth"),
                    animator(scale=[150, 150], shape="round")]),
    "FX-TXA-008": ("Turn, stretch, room and colour: Rotation 30, Scale 150 by 50 on all; then "
                   "Rotation -10, Tracking 50, Fill on in green on the second half, faux italic, "
                   "centred at 640.",
                   text("Spin & Scale", faux_italic=True, align="center", at=[640, 220]),
                   [animator(rotation=30, scale=[150, 50]),
                    animator(rotation=-10, tracking=50, fill="on", color=[0, 1, 0.25], start=50)]),
    "FX-TXA-009": ("Characters excluding spaces, Start 80 above End 20 (taken the other way), "
                   "Amount -50, Position 0, 40: picked letters move up by 20.",
                   text("a b c d e"),
                   [animator(position=[0, 40], based_on="characters_excluding_spaces", start=80,
                             end=20, amount=-50)]),
    "FX-TXA-010": ("Two lines: the line break is not a character. Square, Start 25, End 75 of the "
                   "four letters: B and C picked, scaled 200.",
                   text("AB\nCD"), [animator(scale=[200, 200], start=25, end=75)]),
    "FX-TXA-011": ("The timing line: 30 characters with two animators, a wave and a fade by "
                   "character.",
                   text(THIRTY, size=60, at=[40, 200]),
                   [animator(position=[0, -50], start=10, end=40, offset=25, shape="smooth"),
                    animator(opacity=0, shape="ramp_up", offset=-40, ease_high=50, ease_low=50)]),
    "FX-TXA-012": ("A range of no width (Start and End 50): Ramp Up picks all after it, the "
                   "square picks none.",
                   text("Empty range"),
                   [animator(opacity=0, start=50, end=50, shape="ramp_up"),
                    animator(scale=[300, 300], start=50, end=50)]),
    "FX-TXA-013": ("Room by the animator on a right-aligned line with the text's own tracking 100: "
                   "Tracking 200 at Amount 50 on the first half moves the start of the line left.",
                   text("Right side", align="right", tracking=100, at=[1200, 200]),
                   [animator(tracking=200, amount=50, end=50)]),
}

# Projects: (says, text, effects as file records, frames to check, warning on open, warning on a frame)
KEYED = {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"},
                                  {"frame": 15, "value": 100, "interp": "linear"}]}
PROJECTS = {
    "FX-TXA-020": ("Typewriter keyed: Start 0 at frame 0 to 100 at frame 15, linear, Smoothness 0, "
                   "on \"Typewriter text\"; frame 6 is Start 40, FX-TXA-001.",
                   "text", [{"start": KEYED, "opacity": 0, "smoothness": 0}], {"0": 0, "6": 40, "15": 100},
                   None, None),
    "FX-TXA-021": ("A setting this build does not know (\"selector_mode\") is kept as written and "
                   "the animator still works.",
                   "text", [{"opacity": 0, "start": 40, "smoothness": 0, "selector_mode": "add"}],
                   {"0": 40}, None, None),
    "FX-TXA-022": ("A shape this build does not have (\"wiggly\") is reported on opening and the "
                   "words are drawn as if the animator were off.",
                   "text", [{"opacity": 0, "shape": "wiggly"}], {}, "EFFECT_PARAMETER_INVALID",
                   "EFFECT_PARAMETER_INVALID"),
    "FX-TXA-023": ("An animator on a solid has no characters to move: said on every frame.",
                   "solid", [{"opacity": 0}], {}, None, "TEXT_ANIMATOR_NO_TEXT"),
}


def prop(base):
    return {"base": base, "keyframes": []}


def project(case, kind, effects):
    fx = [{"instance_id": f"fx-{i}", "type_id": "core.text_animator", "enabled": True,
           "parameters": {**DEFAULTS, **e}} for i, e in enumerate(effects)]
    layer = {"id": "art", "kind": kind, "name": "art", "enabled": True, "locked": False,
             "in_frame": 0, "out_frame": 16,
             "transform": {"anchor": prop([640, 180]), "position": prop([640, 180]),
                           "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1)},
             "matte": None, "blend_mode": "normal", "effects": fx, "masks": []}
    if kind == "text":
        layer["source_text"] = text("Typewriter text")
    else:
        layer["solid"] = {"color": [0.2, 0.4, 0.8], "width": 1280, "height": 360}
    return {
        "schema_version": 0, "project_id": "proj-" + case.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": 1280, "height": 360,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 16,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 16},
            "layer_order": ["art"], "layers": [layer]}],
    }


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": TOLERANCE, "box_tolerance": BOX_TOLERANCE, "cases": {}, "projects": {}}
    for case, (says, t, animators) in CASES.items():
        expected["cases"][case] = {"says": says, "text": t, "animators": animators,
                                   "chars": lay_out(t, animators)}
    for case, (says, kind, effects, frames, on_open, on_frame) in PROJECTS.items():
        file = f"{case.lower().replace('-', '_')}.json"
        (OUT / file).write_text(json.dumps(project(case, kind, effects), indent=2) + "\n", encoding="utf-8")
        visible = {}
        for frame, start in frames.items():
            a = animator(**{**{k: v for k, v in effects[0].items() if k in DEFAULTS}, "start": start})
            visible[frame] = [c["index"] for c in lay_out(text("Typewriter text"), [a]) if c["opacity"] > 0]
        expected["projects"][case] = {"says": says, "project": file, "visible": visible,
                                      "warning": on_open, "frame_warning": on_frame}
    (OUT / "expected_text_animator.json").write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    for case, c in expected["cases"].items():
        shown = " ".join(f"{x['char']}:{x['opacity']:.2f}" for x in c["chars"][:8])
        print(case, len(c["chars"]), shown)


if __name__ == "__main__":
    main()
