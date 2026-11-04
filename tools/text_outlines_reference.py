"""Create Shapes from Text and Create Masks from Text, worked a second way.

D-1086 adds After Effects' Layer > Create > Create Shapes from Text and Create Masks from Text.
It is this program's own method, modelled on After Effects' menu items; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins.

The rule.

1. The words are laid out as they are drawn on the frame (D-263, D-350's animators as they are
   on that frame; D-1085's Path Options too). Each character that has an outline gives its
   outline's contours, each a closed Bezier path in the layer's pixels: the font's points as they
   are, a quadratic piece raised to the cubic it is (each handle two thirds of the way from its
   end to the control point), a contour that comes back to its first point keeping one point
   there. Spaces and other characters with no outline give nothing.
2. A contour is a hole when it goes round the other way from the character's largest contour
   (the sign of its area, the curves sampled finely), as the inside of an O or an 8 does.
3. Create Shapes from Text: a new shape layer named "<layer> Outlines", the text layer's
   transform and times, above the text layer. Every character is one group named after it, in
   reading order, holding one path per contour named "Path 1", "Path 2" and so on, each filled
   with the character's colour (and stroked as the text is, if it is), and a Merge Paths in
   Merge mode at the foot of the group, so the group is filled once by the even-odd rule and a
   hole stays open. The text layer is switched off.
4. Create Masks from Text: a new solid named "<layer> Outlines" in the text's colour, the
   composition's size, the text layer's transform and times, above the text layer. Every
   contour is one mask named after its character, character by character in reading order;
   within a character its outer contours first, mode Add, then its holes, mode Subtract. The
   text layer is switched off.
5. A layer that is not a text layer, or whose words have no outline at all, is refused with a
   sentence that says so; nothing is added.

Contours are compared as closed loops: the same points in the same order, from whichever
point the font reader starts on.
"""

import json
import math
from pathlib import Path

from fontTools.pens.basePen import BasePen

from text_animator_reference import FONT, UPEM, GLYPHS, glyph, ASCENT, DESCENT, GAP, DEFAULTS as ANIMATOR

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "text_outlines"
TOLERANCE = 1e-6


class Nodes(BasePen):
    """A glyph's contours as Bezier nodes [x, y, in_x, in_y, out_x, out_y], handles relative."""

    def __init__(self, place):
        super().__init__(None)
        self.place, self.contours, self.now = place, [], None

    def _moveTo(self, p):
        self.now = [[*self.place(p), 0.0, 0.0, 0.0, 0.0]]

    def _node(self, out, into, to):
        last = self.now[-1]
        last[4], last[5] = out[0] - last[0], out[1] - last[1]
        self.now.append([to[0], to[1], into[0] - to[0], into[1] - to[1], 0.0, 0.0])

    def _lineTo(self, p):
        to = self.place(p)
        last = self.now[-1]
        self._node((last[0], last[1]), to, to)

    def _qCurveToOne(self, c, p):
        c, to = self.place(c), self.place(p)
        last = self.now[-1]
        third = lambda a: (a[0] + (c[0] - a[0]) * 2 / 3, a[1] + (c[1] - a[1]) * 2 / 3)
        self._node(third((last[0], last[1])), third(to), to)

    def _curveToOne(self, c1, c2, p):
        self._node(self.place(c1), self.place(c2), self.place(p))

    def _closePath(self):
        n = self.now
        if len(n) > 1 and math.hypot(n[0][0] - n[-1][0], n[0][1] - n[-1][1]) < 1e-9:
            last = n.pop()
            n[0][2], n[0][3] = last[2], last[3]
        if len(n) >= 2:
            self.contours.append(n)
        self.now = None

    _endPath = _closePath


def area(nodes):
    """The contour's signed area, its curves sampled finely."""
    pts = []
    for i, a in enumerate(nodes):
        b = nodes[(i + 1) % len(nodes)]
        p0, p3 = (a[0], a[1]), (b[0], b[1])
        p1, p2 = (a[0] + a[4], a[1] + a[5]), (b[0] + b[2], b[1] + b[3])
        for k in range(32):
            t = k / 32
            v = 1 - t
            pts.append(tuple(v ** 3 * p0[j] + 3 * v * v * t * p1[j] + 3 * v * t * t * p2[j] + t ** 3 * p3[j]
                             for j in (0, 1)))
    return sum(pts[i][0] * pts[(i + 1) % len(pts)][1] - pts[(i + 1) % len(pts)][0] * pts[i][1]
               for i in range(len(pts))) / 2


def outlines(t, animators):
    """Every character with an outline: its index, character, colour and contours with holes."""
    size = t["size"]
    scale = size / UPEM
    words = t["text"]
    from text_animator_reference import amounts
    flat = [c for paragraph in words.split("\n") for c in paragraph.rstrip("\r")]
    per = [amounts(flat, a) for a in animators]
    track = t.get("tracking", 0) / 1000 * size
    line = (ASCENT - DESCENT + GAP) * scale
    out, k = [], 0
    for n, paragraph in enumerate(words.split("\n")):
        chars = list(paragraph.rstrip("\r"))
        raw = [FONT["hmtx"][glyph(c)][0] * scale for c in chars]
        state = []
        for i in range(len(chars)):
            s = {"move": [0.0, 0.0], "turn": 0.0, "scale": [1.0, 1.0], "room": 0.0,
                 "color": list(map(float, t["color"]))}
            for a, row in zip(animators, per):
                v = row[k + i]
                s["move"] = [s["move"][j] + v * a["position"][j] for j in (0, 1)]
                s["turn"] += v * a["rotation"]
                s["room"] += v * a["tracking"] / 1000 * size
                s["scale"] = [s["scale"][j] * (1 + v * (a["scale"][j] / 100 - 1)) for j in (0, 1)]
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
                px, py = x + p[0] * scale, y - p[1] * scale
                dx, dy = (px - ax) * s["scale"][0], (py - ay) * s["scale"][1]
                return (ax + cos * dx - sin * dy + s["move"][0], ay + sin * dx + cos * dy + s["move"][1])

            pen = Nodes(place)
            GLYPHS[glyph(c)].draw(pen)
            if pen.contours:
                areas = [area(cn) for cn in pen.contours]
                big = max(areas, key=abs)
                out.append({"index": k + i, "char": c, "color": s["color"],
                            "contours": [{"hole": (a > 0) != (big > 0), "points": cn}
                                         for a, cn in zip(areas, pen.contours)]})
            x += step[i]
        k += len(chars)
    return out


def masks(chars):
    """Create Masks from Text's masks: name, mode and points, in order."""
    out = []
    for ch in chars:
        for hole in (False, True):
            for cn in ch["contours"]:
                if cn["hole"] == hole:
                    out.append({"name": ch["char"], "mode": "subtract" if hole else "add",
                                "points": cn["points"]})
    return out


def words(w, **settings):
    return {"text": w, "font": "MPLUSRounded1c-Regular.ttf", "size": 100, "color": [1, 1, 1],
            "at": [80, 200], "align": "left", **settings}


def animator(**settings):
    return {**ANIMATOR, **settings}


CASES = {
    "FX-TXO-001": ("Plain words: one group per letter, no group for the space; T, x and t have "
                   "one contour each, e two (its eye a hole).", words("Text ext"), []),
    "FX-TXO-002": ("Holes: O and 8 and B, each outer contour Add and each inside Subtract.",
                   words("O8B", color=[1, 0.5, 0]), []),
    "FX-TXO-003": ("Centred, two lines: the second line a line lower, each centred on the place.",
                   words("AB\nCD", align="center", at=[640, 200]), []),
    "FX-TXO-004": ("A Text Animator as it is on the frame: Rotation 30, Position 10, -20 and a "
                   "red fill on every letter; the outlines are where the letters are drawn.",
                   words("Spin"), [animator(rotation=30, position=[10, -20], fill="on",
                                             color=[1, 0, 0])]),
}


def prop(base):
    return {"base": base, "keyframes": []}


def project(name, t, position, rotation):
    layer = {"id": "art", "kind": "text", "name": "Title", "enabled": True, "locked": False,
             "in_frame": 2, "out_frame": 20,
             "transform": {"anchor": prop([640, 360]), "position": prop(position),
                           "scale": prop([100, 100]), "rotation": prop(rotation), "opacity": prop(1)},
             "matte": None, "blend_mode": "normal", "effects": [], "masks": [], "source_text": t}
    return {"schema_version": 0, "project_id": "proj-" + name,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [],
            "compositions": [{"id": "comp-main", "name": "comp-main", "width": 1280, "height": 720,
                              "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
                              "start_frame": 0, "duration_frames": 24,
                              "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
                              "layer_order": ["art"], "layers": [layer]}]}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": TOLERANCE, "cases": {}, "projects": {}}
    for case, (says, t, animators) in CASES.items():
        chars = outlines(t, animators)
        expected["cases"][case] = {"says": says, "text": t, "animators": animators,
                                   "chars": chars, "masks": masks(chars)}
    t = words("Hi O", color=[0.2, 0.6, 1.0], at=[400, 400])
    chars = outlines(t, [])
    expected["projects"]["FX-TXO-020"] = {
        "file": "fx_txo_020.json",
        "says": "A text layer named Title, moved and turned 10 degrees, frames 2 to 20: both "
                "commands add a layer named \"Title Outlines\" just above it with its transform "
                "and times, and switch Title off.",
        "name": "Title Outlines", "in_frame": 2, "out_frame": 20, "position": [700, 380],
        "rotation": 10, "groups": [c["char"] for c in chars],
        "paths": [len(c["contours"]) for c in chars], "color": t["color"],
        "masks": [{"name": m["name"], "mode": m["mode"]} for m in masks(chars)]}
    (OUT / "fx_txo_020.json").write_text(json.dumps(project("fx-txo-020", t, [700, 380], 10), indent=2),
                                         encoding="utf-8", newline="\n")
    spaces = words("   ")
    (OUT / "fx_txo_021.json").write_text(json.dumps(project("fx-txo-021", spaces, [640, 360], 0), indent=2),
                                         encoding="utf-8", newline="\n")
    expected["projects"]["FX-TXO-021"] = {
        "file": "fx_txo_021.json",
        "says": "Words that are only spaces have no outline: both commands are refused, saying so, "
                "and nothing is added."}
    (OUT / "expected_text_outlines.json").write_text(json.dumps(expected, indent=1), encoding="utf-8",
                                                     newline="\n")
    print("wrote", OUT)


if __name__ == "__main__":
    main()
