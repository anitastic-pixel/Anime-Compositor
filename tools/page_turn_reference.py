"""Page Turn, worked a second way.

D-388 adds `core.page_turn`, after CycoreFX's CC Page Turn: the layer peeled back like the page
of a book, rolled round a cylinder along a fold line, its back side shown on the part turned
over, in a paper colour or another layer, shaded where it curls. CycoreFX's manual says what each
control does in a sentence and publishes no formula; the rule below is this program's own reading
of it, and nothing is ported.

The rule. W and H are the drawing's own size; every point below is in its space, y down.

1. The fold line and n, the unit step across it towards the part that lifts:
   - `controls` `classic`: F is `fold_position`, per cent of W and H; the page travels the way
     `fold_direction` points (degrees clockwise from up, Drop Shadow's reading), so n is the
     opposite way, and a point X lies d = (X - F) . n past the line.
   - the four corners (`top_left`, `top_right`, `bottom_left`, `bottom_right`): the chosen corner
     K of the drawing is turned over so that it lies at P, `fold_position`. With D = |K - P|
     (0 leaves the layer as it was), n = (K - P) / D and the line is a = (D - pi R) / 2 past P:
     d = (X - P) . n - a, the line that lays K flat on P after half a turn of the cylinder.
2. The page rolls round a cylinder of radius R, `fold_radius` in pixels, lying on the line. A
   point of the flat page s past the line (s along n) is carried, for s from 0 to pi R, round the
   cylinder to d = R sin(s / R), its front facing up while s < pi R / 2 and its back after; past
   pi R it lies flat again, back up, at d = pi R - s. So at an output point X, d past the line:
   - d > R: nothing;
   - 0 <= d <= R: the back at s = R (pi - asin(d / R)) over the front at s = R asin(d / R);
   - d < 0: the back at s = pi R - d over the page itself, flat, at X.
   At R = 0 every d >= 0 is nothing and the back is the page mirrored in the line.
3. The page at s is document 21's bilinear sample f of the input at X + (s - d) n, transparent
   outside it. Its back side: m, the back page at that same point of the page (`back_page`,
   D-189's layer setting fitted to W by H as `stretch` fits it, clear outside), or with "" (or a
   layer not in the composition, with the warning) the opaque `paper_color`; with o =
   `back_opacity` / 100, the back is rgb f.a o m.rgb + (1 - o m.a) f.rgb, covering f.a: the back
   page laid over the page's own picture and cut to its shape. So the back page sits where the
   front does on the paper, and a turned page shows it mirrored, as paper seen through.
4. Where the back curls round the cylinder (0 <= d <= R) it is shaded: its rgb times
   clamp(l (d / R) + sqrt(1 - (d / R)^2), 0, 1), l = L . n with L the step `light_direction`
   points; the flat back is unshaded.
5. `render` `full` lays the back over the front (over, premultiplied); `front` keeps only the
   front, the page with no back, and `back` only the back, so two copies of the effect can put
   another layer between them. The layer does not grow.

`controls` `classic`, `top_left`, `top_right`, `bottom_left` or `bottom_right`, `bottom_right`;
`fold_position` -1000 to 1000 per cent each way, 75, 75; `fold_direction` -3600 to 3600, -60;
`fold_radius` 0 to 1000 pixels, 30; `light_direction` -3600 to 3600, -45; `render` `full`,
`front` or `back`, `full`; `back_page` D-189's word, ""; `back_opacity` 0 to 100, 100;
`paper_color` a colour, #f0f0f0. The numbers are keyable. The values when added are chosen here,
after CycoreFX's defaults where its manual gives them (Fold Direction -60). For a draft the radius
is halved.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says; the layer is `art`, so `back_page` `art` is the layer itself and
`gone` a layer not there. The drawing goes into `Fixtures/page_turn/media`, the projects into
`Fixtures/page_turn`, and the expected frames into `Fixtures/page_turn/expected_page_turn.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/page_turn_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import bulge_reference as B  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import unit  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "page_turn"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"fold_position": (-1000, 1000), "fold_direction": (-3600, 3600),
          "fold_radius": (0, 1000), "light_direction": (-3600, 3600), "back_opacity": (0, 100)}
WORDS = ("controls", "render", "back_page", "paper_color")
NAMES = ("controls", "fold_position", "fold_direction", "fold_radius", "light_direction",
         "render", "back_page", "back_opacity", "paper_color")
CORNERS = {"top_left": (0, 0), "top_right": (W, 0), "bottom_left": (0, H),
           "bottom_right": (W, H)}
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def hex_linear(h):
    return [srgb_to_linear(v / 255) for v in R.hex_color(h.lower())]


def fold(s_):
    """(n, offset) with d = X . n - offset, or None when the page lies as it was."""
    fx, fy = s_["fold_position"][0] / 100 * W, s_["fold_position"][1] / 100 * H
    if s_["controls"] == "classic":
        mx, my = unit(s_["fold_direction"])
        n = (-mx, -my)
        return n, fx * n[0] + fy * n[1]
    kx, ky = CORNERS[s_["controls"]]
    D = math.hypot(kx - fx, ky - fy)
    if D == 0:
        return None
    n = ((kx - fx) / D, (ky - fy) / D)
    a = (D - math.pi * s_["fold_radius"]) / 2
    return n, fx * n[0] + fy * n[1] + a


def over(top, under):
    return [t + (1 - top[3]) * u for t, u in zip(top, under)]


def turned_at(layer, back_map, s_, X):
    """(top, under): the back and the front at the output point X."""
    n, off = fold(s_)
    r = s_["fold_radius"]
    d = X[0] * n[0] + X[1] * n[1] - off
    page = lambda s: bilinear(layer, X[0] + (s - d) * n[0], X[1] + (s - d) * n[1])  # noqa: E731

    def back(s, shade):
        f = page(s)
        q = (X[0] + (s - d) * n[0], X[1] + (s - d) * n[1])
        m = bilinear(back_map, *q) if back_map else hex_linear(s_["paper_color"]) + [1.0]
        o = s_["back_opacity"] / 100
        return [(f[3] * o * m[i] + (1 - o * m[3]) * f[i]) * shade for i in range(3)] + [f[3]]

    if d > r or (r == 0 and d >= 0):
        return EMPTY, EMPTY
    if d >= 0:
        t = min(1.0, d / r)
        lx, ly = unit(s_["light_direction"])
        shade = min(1.0, max(0.0, (lx * n[0] + ly * n[1]) * t + math.sqrt(1 - t * t)))
        return back(r * (math.pi - math.asin(t)), shade), page(r * math.asin(t))
    return back(math.pi * r - d, 1.0), page(d)


def page_turn(layer, back_map, s_):
    if fold(s_) is None:
        return layer
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            top, under = turned_at(layer, back_map, s_,
                                   (layer["left"] + i + 0.5, layer["top"] + j + 0.5))
            px.append({"full": over(top, under), "front": under, "back": top}[s_["render"]])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **more):
    c = {"drawing": "stripes", "shift": shift, "controls": "bottom_right",
         "fold_position": (75, 75), "fold_direction": -60, "fold_radius": 30,
         "light_direction": -45, "render": "full", "back_page": "", "back_opacity": 100,
         "paper_color": "#f0f0f0"}
    c.update(more)
    return c


def settings(c, frame_no):
    held = {k: c[k] for k in WORDS}
    for k, (lo, hi) in RANGES.items():
        v = value_at(c[k], frame_no)
        held[k] = (tuple(min(hi, max(lo, u)) for u in v) if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    back_map = B.drawn_layer(c["drawing"]) if c["back_page"] == "art" else None
    out = page_turn(layer, back_map, settings(c, frame_no))
    return [out["px"][y * W + x - c["shift"]] if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


def plain(c):
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


# The page folded on the upright line down the middle, travelling right: the left half lifts.
HALF = dict(controls="classic", fold_position=(50, 50), fold_direction=90)

CASES = {
    "FX-PAGETURN-001": ("The settings as they start: the bottom right corner turned up to three "
                        "quarters of the way across and down, Fold Radius 30. On a drawing 16 "
                        "pixels wide a cylinder that size has rolled the whole page up and out "
                        "of the frame: clear.", case(), [0]),
    "FX-PAGETURN-002": ("Classic, the fold line upright down the middle, the page travelling "
                        "right, Fold Radius 0: the left half turned over flat onto the right, "
                        "showing its back, the paper colour #f0f0f0, cut to the page's shape; "
                        "the left half is clear.", case(fold_radius=0, **HALF), [0]),
    "FX-PAGETURN-003": ("The same with Back Opacity 0: the back shows the page's own picture "
                        "seen through, so the right half is the left half mirrored.",
                        case(fold_radius=0, back_opacity=0, **HALF), [0]),
    "FX-PAGETURN-004": ("The same with Back Opacity 100 and the layer itself as the Back Page: "
                        "the back is the page's picture again, so the frame is "
                        "FX-PAGETURN-003's.", case(fold_radius=0, back_page="art", **HALF), [0]),
    "FX-PAGETURN-005": ("Classic down the middle, Fold Radius 2: the fold rolls round a "
                        "cylinder two pixels across, its back shaded where it curls, the turned "
                        "part laid down past it.", case(fold_radius=2, **HALF), [0]),
    "FX-PAGETURN-006": ("FX-PAGETURN-005 with Render Front: only the page's front, no back.",
                        case(fold_radius=2, render="front", **HALF), [0]),
    "FX-PAGETURN-007": ("FX-PAGETURN-005 with Render Back: only the back; laid over "
                        "FX-PAGETURN-006 it gives FX-PAGETURN-005.",
                        case(fold_radius=2, render="back", **HALF), [0]),
    "FX-PAGETURN-008": ("FX-PAGETURN-005 lit from the right, Light Direction 90: the curl's "
                        "crest faces left, away from the light, and darkens, where lit from the "
                        "upper left, as it starts, it shows the paper.",
                        case(fold_radius=2, light_direction=90, **HALF), [0]),
    "FX-PAGETURN-009": ("The bottom right corner turned to the middle, Fold Radius 1: the "
                        "corner laid flat on the middle, its back the paper colour, the "
                        "lower right of the frame clear.",
                        case(fold_position=(50, 50), fold_radius=1), [0]),
    "FX-PAGETURN-010": ("The top left corner turned to the middle, Fold Radius 1.",
                        case(controls="top_left", fold_position=(50, 50), fold_radius=1), [0]),
    "FX-PAGETURN-011": ("The bottom right corner's Fold Position keyed from (100, 100) at frame "
                        "0 to (0, 0) at frame 4, linear, Fold Radius 1: frame 0 the page as it "
                        "was, then turned ever further, frame 2 FX-PAGETURN-009.",
                        case(fold_radius=1, fold_position=keyed((0, (100, 100)), (4, (0, 0)))),
                        [0, 2, 4]),
    "FX-PAGETURN-012": ("Classic as it starts, Fold Direction -60, the line through the middle, "
                        "Fold Radius 2: the lower right part turned up and to the left.",
                        case(controls="classic", fold_position=(50, 50), fold_radius=2), [0]),
    "FX-PAGETURN-013": ("FX-PAGETURN-002 with Paper Color #2040a0: the back is blue.",
                        case(fold_radius=0, paper_color="#2040a0", **HALF), [0]),
    "FX-PAGETURN-014": ("FX-PAGETURN-002 with Back Page a layer that is not in the "
                        "composition, `gone`: the paper colour, as FX-PAGETURN-002, and the "
                        "warning every frame.",
                        case(fold_radius=0, back_page="gone", **HALF), [0]),
    "FX-PAGETURN-015": ("FX-PAGETURN-009 with the layer moved three pixels right: the same, "
                        "moved; nothing grows.",
                        case(3, fold_position=(50, 50), fold_radius=1), [0]),
    "FX-PAGETURN-016": ("FX-PAGETURN-002 with Back Opacity keyed from 0 at frame 0 to 100 at "
                        "frame 4: frame 0 FX-PAGETURN-003, frame 4 FX-PAGETURN-002, frame 2 "
                        "half way.", case(fold_radius=0, back_opacity=keyed((0, 0), (4, 100)),
                                          **HALF), [0, 2, 4]),
    "FX-PAGETURN-017": ("Classic, Fold Radius 1, Fold Direction keyed from 90 at frame 0 to 450 "
                        "at frame 4: the fold line turns a whole turn about the middle, frames 0 "
                        "and 4 the same.", case(fold_radius=1, **dict(
                            HALF, fold_direction=keyed((0, 90), (4, 450)))), [0, 2, 4]),
    "FX-PAGETURN-018": ("Classic down the middle, Fold Radius eased from 0 at frame 0 to 1000 at "
                        "frame 4 on a curve that overshoots: frame 2 would pass 1000 and is "
                        "held there, the same as frame 4.",
                        case(**dict(HALF, fold_radius=keyed((0, 0, OVERSHOOT), (4, 1000)))),
                        [0, 2, 4]),
}

INVALID = {
    "FX-PAGETURN-019": ("Controls \"middle\", not a word it takes.", case(controls="middle")),
    "FX-PAGETURN-020": ("Render \"sides\", not a word it takes.", case(render="sides")),
    "FX-PAGETURN-021": ("Fold Radius 1001, above 1000.", case(fold_radius=1001)),
    "FX-PAGETURN-022": ("Fold Radius -1, below 0.", case(fold_radius=-1)),
    "FX-PAGETURN-023": ("Back Opacity 101, above 100.", case(back_opacity=101)),
    "FX-PAGETURN-024": ("Paper Color \"white\", not a colour.", case(paper_color="white")),
    "FX-PAGETURN-025": ("Back Page written as the number 3, not a word.", case(back_page=3)),
    "FX-PAGETURN-026": ("Fold Position at (50, 1001), past ten heights.",
                        case(fold_position=(50, 1001))),
    "FX-PAGETURN-027": ("Fold Direction keyed to 3601 at frame 4.",
                        case(fold_direction=keyed((0, 0), (4, 3601)))),
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
        "instance_id": "fx-0-0", "type_id": "core.page_turn", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["back_page"] == "gone":
            expected["cases"][fx]["warning"] = "EFFECT_LAYER_MISSING"
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

    (OUT / "expected_page_turn.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    clear = lambda p: p == EMPTY  # noqa: E731
    paper = hex_linear("#f0f0f0")

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if expected["cases"][fx].get("warning") == "EFFECT_PARAMETER_INVALID":
            assert all(px == drawn for px in frames.values())

    # The cylinder: the front and back meet the flat page where they should.
    s_ = settings(case(fold_radius=2, **HALF), 0)
    layer = B.drawn_layer("stripes")
    for X in ((8.0, 3.5), (8.0 - 1e-9, 3.5)):
        top, under = turned_at(layer, None, s_, X)
        assert near(under, bilinear(layer, *X), 1e-6)

    one = c["FX-PAGETURN-001"]["0"]
    assert one != drawn
    two = c["FX-PAGETURN-002"]["0"]
    assert all(clear(two[at(x, y)]) for x in range(8) for y in range(H))
    assert all(near(two[at(x, y)], paper + [1.0]) for x in range(8, W) for y in range(1, 9))
    three = c["FX-PAGETURN-003"]["0"]
    assert all(near(three[at(x, y)], over(drawn[at(15 - x, y)], drawn[at(x, y)]))
               for x in range(8, W) for y in range(H))
    assert same(c["FX-PAGETURN-004"]["0"], three)
    five, six, seven = (c[f"FX-PAGETURN-00{i}"]["0"] for i in (5, 6, 7))
    assert all(near(f, over(b, u)) for f, b, u in zip(five, seven, six))
    assert five != c["FX-PAGETURN-008"]["0"]
    nine = c["FX-PAGETURN-009"]["0"]
    # The corner's back laid flat at (9, 6), the paper cut to the corner's soft edge, over the
    # page itself: under the paper's covering, the paper colour.
    flat = turned_at(layer, None, settings(case(fold_position=(50, 50), fold_radius=1), 0),
                     (9.5, 6.5))[0]
    assert clear(nine[at(15, 8)]) and 0 < flat[3] < 0.5
    assert near([v / flat[3] for v in flat[:3]], paper, 1e-9)
    assert c["FX-PAGETURN-010"]["0"] != nine
    eleven = c["FX-PAGETURN-011"]
    assert eleven["0"] == drawn and eleven["2"] == nine and eleven["4"] != nine
    assert c["FX-PAGETURN-012"]["0"] != drawn
    thirteen = c["FX-PAGETURN-013"]["0"]
    assert near(thirteen[at(10, 4)], hex_linear("#2040a0") + [1.0])
    assert c["FX-PAGETURN-014"]["0"] == two
    fifteen = c["FX-PAGETURN-015"]["0"]
    assert all(fifteen[at(x + 3, y)] == nine[at(x, y)] for x in range(W - 3) for y in range(H))
    sixteen = c["FX-PAGETURN-016"]
    assert same(sixteen["0"], three) and same(sixteen["4"], two)
    assert not same(sixteen["2"], two) and not same(sixteen["2"], three)
    seventeen = c["FX-PAGETURN-017"]
    assert same(seventeen["0"], seventeen["4"]) and seventeen["2"] != seventeen["0"]
    eighteen = c["FX-PAGETURN-018"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 1000)), 2) > 1000
    assert eighteen["2"] == eighteen["4"] != eighteen["0"]
    print("checked")


def show(px):
    """. clear, p paper, b band, s skin, l line, + part covered, o other."""
    paper = hex_linear("#f0f0f0")
    rows = []
    for y in range(H):
        row = ""
        for x in range(W):
            p = px[y * W + x]
            row += ("." if p[3] < 1e-9 else "+" if p[3] < 0.99 else
                    "p" if all(abs(p[i] - paper[i]) < 0.02 for i in range(3)) else
                    "b" if p[2] > p[0] + 0.1 else "s" if p[0] > 0.3 else
                    "l" if p[0] < 0.05 else "o")
        rows.append(row)
    return "\n".join(rows)


if __name__ == "__main__":
    if sys.argv[1:] == ["show"]:
        for fx, (_, c, frames) in CASES.items():
            for f in frames:
                print(fx, f)
                print(show(render(c, f)))
    else:
        main()
