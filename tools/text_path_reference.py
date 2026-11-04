"""Text on a path, worked a second way.

D-1085 adds `core.text_path`, shown as "Path Options", after After Effects' Path Options on a
text layer: the layer's characters follow one of its masks. It is this program's own method,
modelled on After Effects' panel; nothing is ported. Document 21 is the rule in words; this file
is the reference for the numbers document 25 pins against it.

The settings, each in the file under its own name:

- path, the number of the layer's mask the words follow, 1 for the first, 0 for none; starting 0
- reverse, "off" or "on", starting "off"
- perpendicular, "off" or "on", starting "on"
- force_alignment, "off" or "on", starting "off"
- first_margin, last_margin, pixels along the path, -100000 to 100000, starting 0; both keyable

The rule.

1. The path is the mask as drawn on the frame, cut into straight pieces as a mask is. Its length
   L is the pieces' lengths added. Reverse: an open path is walked from its other end; a closed
   path keeps its first point and is walked the other way round.
2. Each line of the words is laid out as without a path (its advances, tracking and the
   animators' tracking); c is a character's centre measured from the line's left edge and w the
   line's width. The character's distance along the path:
   - Left: First Margin + c.
   - Right: L - Last Margin - w + c.
   - Center: (L - w) / 2 + First Margin - Last Margin + c.
   - Force Alignment (any alignment): the line's first character starts at First Margin and its
     last ends at L - Last Margin, the room between them shared equally between every two
     characters: First Margin + c + k * (L - Last Margin - First Margin - w) / (n - 1), k the
     character's place in the line from 0 and n how many there are (n = 1: as Left).
   An animator's Position x is added to the distance.
3. A closed path is walked round again past its end (the distance taken modulo L); an open path
   goes on straight along its first piece before the start and its last piece after the end.
4. At that distance the path has a point P and a direction, the angle a degrees of its piece
   (clockwise on the picture, from the right). With Perpendicular To Path on, the character is
   turned by a, so its baseline lies along the path; off, it stays upright.
5. Each character is drawn as without a path about its anchor (its turn, scale), then lifted by
   its animators' Position y and by the line's distance below the first line, and that is placed
   with its anchor at P, turned by a when Perpendicular is on. So "at", where its anchor lands,
   is P + R(a)(0, lift), and its turn is the animators' turn plus a.
6. Path 0, or a layer without that mask: the words are drawn where they are without a path. A
   mask number the layer does not have is said on every frame (EFFECT_PATH_MISSING).

A curved mask is measured here by sampling its Bezier curves finely, not by the build's pieces,
so those cases carry a looser tolerance.
"""

import json
import math
from pathlib import Path

from text_animator_reference import FONT, UPEM, glyph, ASCENT, DESCENT, GAP

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "text_path"
TOLERANCE = 0.02
CURVE_TOLERANCE = 0.1
CURVE_TURN_TOLERANCE = 0.3
KAPPA = 0.5522847498307936

DEFAULTS = {"path": 1, "reverse": "off", "perpendicular": "on", "force_alignment": "off",
            "first_margin": 0, "last_margin": 0}


def words(w, **settings):
    return {"text": w, "font": "MPLUSRounded1c-Regular.ttf", "size": 60, "color": [1, 1, 1],
            "at": [80, 120], "align": "left", **settings}


def corners(points, closed):
    return {"closed": closed, "points": [{"point": p, "in": [0, 0], "out": [0, 0]} for p in points]}


def circle(cx, cy, r):
    """After Effects' ellipse mask: from the top, anticlockwise on the picture."""
    k = KAPPA * r
    pts = [((cx, cy - r), (k, 0), (-k, 0)), ((cx - r, cy), (0, -k), (0, k)),
           ((cx, cy + r), (-k, 0), (k, 0)), ((cx + r, cy), (0, k), (0, -k))]
    return {"closed": True,
            "points": [{"point": list(p), "in": list(i), "out": list(o)} for p, i, o in pts]}


def polyline(mask):
    """The path as many short straight pieces: corners as they are, curves sampled finely."""
    pts, closed = mask["points"], mask["closed"]
    n = len(pts)
    out = []
    for i in range(n if closed else n - 1):
        a, b = pts[i], pts[(i + 1) % n]
        p0 = a["point"]
        p3 = b["point"]
        p1 = [p0[0] + a["out"][0], p0[1] + a["out"][1]]
        p2 = [p3[0] + b["in"][0], p3[1] + b["in"][1]]
        out.append(tuple(p0))
        if a["out"] == [0, 0] and b["in"] == [0, 0]:
            continue
        for k in range(1, 4000):
            t = k / 4000
            u = 1 - t
            out.append(tuple(u ** 3 * p0[j] + 3 * u * u * t * p1[j] + 3 * u * t * t * p2[j]
                             + t ** 3 * p3[j] for j in (0, 1)))
    if not closed:
        out.append(tuple(pts[-1]["point"]))
    return out


def walk(points, closed, reverse):
    if reverse:
        points = [points[0]] + points[1:][::-1] if closed else points[::-1]
    segs = list(zip(points, points[1:] + ([points[0]] if closed else [])))
    lengths = [math.dist(a, b) for a, b in segs]
    return segs, lengths, sum(lengths)


def at(segs, lengths, total, closed, s):
    """The point at distance s along the path and its direction in degrees."""
    if closed:
        s %= total
    if s < 0:
        i = 0
    else:
        i, run = 0, 0.0
        while i < len(segs) - 1 and run + lengths[i] <= s:
            run += lengths[i]
            i += 1
        s -= run
    (ax, ay), (bx, by) = segs[i]
    d = lengths[i]
    ux, uy = (bx - ax) / d, (by - ay) / d
    return (ax + ux * s, ay + uy * s), math.degrees(math.atan2(uy, ux))


def lay_out(t, mask, options, animators=()):
    """Every character: its index, where its anchor lands and how far it is turned."""
    size = t["size"]
    scale = size / UPEM
    line = (ASCENT - DESCENT + GAP) * scale
    segs, lengths, total = walk(polyline(mask), mask["closed"], options["reverse"] == "on")
    first, last = options["first_margin"], options["last_margin"]
    out, k = [], 0
    for n, paragraph in enumerate(t["text"].split("\n")):
        chars = list(paragraph)
        raw = [FONT["hmtx"][glyph(c)][0] * scale for c in chars]
        step = raw
        width = sum(step[:-1]) + raw[-1] if chars else 0.0
        x = 0.0
        for i, c in enumerate(chars):
            centre = x + raw[i] / 2
            if options["force_alignment"] == "on" and len(chars) > 1:
                s = first + centre + i * (total - last - first - width) / (len(chars) - 1)
            else:
                s = {"left": first + centre, "right": total - last - width + centre,
                     "center": (total - width) / 2 + first - last + centre}[t["align"]]
            move = [sum(a["position"][j] for a in animators) for j in (0, 1)]
            turn = sum(a["rotation"] for a in animators)
            (px, py), angle = at(segs, lengths, total, mask["closed"], s + move[0])
            if options["perpendicular"] != "on":
                angle = 0.0
            lift = move[1] + n * line
            cos, sin = math.cos(math.radians(angle)), math.sin(math.radians(angle))
            out.append({"index": k + i, "char": c, "at": [px - sin * lift, py + cos * lift],
                        "turn": turn + angle})
            x += step[i]
        k += len(chars)
    return out


BENT = corners([[100, 300], [500, 300], [700, 100]], False)
SQUARE = corners([[200, 100], [600, 100], [600, 500], [200, 500]], True)
RING = circle(640, 360, 300)

CASES = {
    "FX-TXP-001": ("Left, First Margin 20, on an open path with a corner: the letters past the "
                   "corner climb its second piece.", words("ALONG A PATH"), BENT,
                   {"first_margin": 20}, []),
    "FX-TXP-002": ("Center on the open path.", words("CENTER", align="center"), BENT, {}, []),
    "FX-TXP-003": ("Right, Last Margin 30, on the open path.", words("RIGHT", align="right"), BENT,
                   {"last_margin": 30}, []),
    "FX-TXP-004": ("Force Alignment, margins 10 and 10: the word spreads over the whole path.",
                   words("FORCE"), BENT, {"force_alignment": "on", "first_margin": 10,
                                          "last_margin": 10}, []),
    "FX-TXP-005": ("Perpendicular To Path off: the letters follow the path but stay upright.",
                   words("UPRIGHT"), BENT, {"perpendicular": "off", "first_margin": 300}, []),
    "FX-TXP-006": ("An open path's ends go on straight: First Margin -80 starts before it, and "
                   "the last letters run past its end.", words("BEYOND THE END"), BENT,
                   {"first_margin": -80}, []),
    "FX-TXP-007": ("A closed square, First Margin 1500 of its 1600: the word wraps round its "
                   "first corner.", words("ROUND"), SQUARE, {"first_margin": 1500}, []),
    "FX-TXP-008": ("Reverse Path on the closed square: same first corner, walked the other way.",
                   words("REVERSE"), SQUARE, {"reverse": "on", "first_margin": 40}, []),
    "FX-TXP-009": ("The tutorial's circle (tQ5wqg10x_I at 2:30): an ellipse mask, centred words, "
                   "First Margin 73.6: inside the ring, just right of the bottom.",
                   words("CIRCULAR TEXT", align="center"), RING, {"first_margin": 73.6}, []),
    "FX-TXP-010": ("The tutorial's circle at 2:40: Reverse Path on, First Margin 1146: outside "
                   "the ring, just right of the top.", words("CIRCULAR TEXT", align="center"),
                   RING, {"reverse": "on", "first_margin": 1146}, []),
    "FX-TXP-011": ("A text animator on a path: Position 15, -20 and Rotation 30 on every letter; "
                   "x moves along the path, y lifts off it.", words("LIFTED"), BENT,
                   {"first_margin": 40}, [{"position": [15, -20], "rotation": 30}]),
    "FX-TXP-012": ("Two lines: each starts at the path's start, the second a line lower.",
                   words("TOP\nLOW"), BENT, {"first_margin": 60}, []),
}

KEYED = {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"},
                                  {"frame": 10, "value": 500, "interp": "linear"}]}


def mask_json(mask, i):
    return {"id": f"mask-{i}", "name": f"Mask {i}", "enabled": True, "inverted": False,
            "mode": "none", "opacity": 1, "feather_px": 0, "expansion_px": 0,
            "closed": mask["closed"],
            "path": {"base": {"points": [{"point": p["point"], "in": p["in"], "out": p["out"]}
                                         for p in mask["points"]]}, "keyframes": []}}


def project(case, t, masks, options):
    fx = [{"instance_id": "fx-0", "type_id": "core.text_path", "enabled": True,
           "parameters": {**DEFAULTS, **options}}]
    prop = lambda v: {"base": v, "keyframes": []}
    layer = {"id": "art", "kind": "text", "name": "art", "enabled": True, "locked": False,
             "in_frame": 0, "out_frame": 11,
             "transform": {"anchor": prop([640, 360]), "position": prop([640, 360]),
                           "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1)},
             "matte": None, "blend_mode": "normal", "effects": fx,
             "masks": [mask_json(m, i + 1) for i, m in enumerate(masks)], "source_text": t}
    return {
        "schema_version": 0, "project_id": "proj-" + case.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": 1280, "height": 720,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 11,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 11},
            "layer_order": ["art"], "layers": [layer]}],
    }


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": TOLERANCE, "curve_tolerance": CURVE_TOLERANCE,
                "curve_turn_tolerance": CURVE_TURN_TOLERANCE, "defaults": DEFAULTS, "cases": {},
                "projects": {}}
    for case, (says, t, mask, options, animators) in CASES.items():
        o = {**DEFAULTS, **options}
        a = [{"position": [0, 0], "rotation": 0, **x} for x in animators]
        expected["cases"][case] = {"says": says, "text": t, "mask": mask_json(mask, 1),
                                   "options": o, "animators": animators,
                                   "curved": mask is RING, "chars": lay_out(t, mask, o, a)}
    t = words("KEYED MARGIN")
    (OUT / "fx_txp_020.json").write_text(
        json.dumps(project("FX-TXP-020", t, [BENT], {"first_margin": KEYED}), indent=2) + "\n",
        encoding="utf-8")
    expected["projects"]["FX-TXP-020"] = {
        "says": "First Margin keyed 0 at frame 0 to 500 at frame 10, linear: frame 5 is First "
                "Margin 250, its letters as FX-TXP-001's rule gives them.",
        "project": "fx_txp_020.json",
        "frames": {str(f): lay_out(t, BENT, {**DEFAULTS, "first_margin": 50 * f})
                   for f in (0, 5, 10)}}
    (OUT / "fx_txp_021.json").write_text(
        json.dumps(project("FX-TXP-021", t, [BENT], {"path": 3}), indent=2) + "\n",
        encoding="utf-8")
    expected["projects"]["FX-TXP-021"] = {
        "says": "Path is mask 3 on a layer with one mask: said on every frame "
                "(EFFECT_PATH_MISSING) and the words are drawn where they are without a path.",
        "project": "fx_txp_021.json", "frame_warning": "EFFECT_PATH_MISSING"}
    (OUT / "expected_text_path.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    for case, c in expected["cases"].items():
        shown = " ".join(f"{x['char']}({x['at'][0]:.0f},{x['at'][1]:.0f},{x['turn']:.0f})"
                         for x in c["chars"][:4])
        print(case, len(c["chars"]), shown)


if __name__ == "__main__":
    main()
