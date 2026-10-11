"""Linear Color Key, worked a second way.

`core.linear_color_key`, modelled on After Effects' Linear Color Key (Keying): it compares each
pixel with a key colour and makes the pixels that match transparent, those that match less less
transparent, and those that do not match opaque. Adobe describes the effect ("Linear keys create
a range of transparency across an image. A linear key compares each pixel in the image to the
key color you specify ... uses RGB, hue, or chroma information"; "Matching Tolerance ... A value
of 0 makes the entire image opaque; a value of 100 makes the entire image transparent"; "Matching
Softness: Softens the tolerance by tapering the tolerance value ... values under 20% produce the
best results") and names its views (Final Output, Source Only, Matte Only), but publishes no
formula and no defaults that we found. The rule and the settings when added are ours (document
21, D-504); where they differ from Adobe's own maths is a logged gap. D-541 (the owner's word of
2026-10-11) adds Key Operation's Keep Colors, a later instance that brings back colours the
keyers above it keyed out (Adobe: "use Keep Colors to restore a color"; the Creative COW
workflow with a second instance), and makes tolerance 0 key nothing, as Adobe says. The
eyedroppers and thumbnails are After Effects' window, not settings.

The settings. `view`, "final" (when added), "source" or "matte"; `key_color`, a "#rrggbb"
colour, "#0000ff" when added; `match_colors`, "rgb" (when added, Adobe's advice), "hue" or
"chroma"; `key_operation`, "key" (when added, and when a file written before D-541 has none) or
"keep"; every word exact. `matching_tolerance` and `matching_softness`, 0 to 100, 10 and 10
when added, both keyable.

The rule. At a pixel with covering a > 0, e = (eR, eG, eB) its straight colour through the sRGB
curve held inside 0 to 1, and k the key colour's 8-bit values over 255. The distance d, 0 to 100:

- "rgb": 100 times the largest of |e_i - k_i|.
- "hue": 100 times the angle between their HSV hues over 180 (the shorter way round), and 100
  when either is a grey (no hue).
- "chroma": the chromaticities c = e / (eR + eG + eB) (a black's is (1/3, 1/3, 1/3)), 100 times
  the largest of |c_i - ck_i|; a colour and its darker or brighter self are the same here.

The share kept is m = (d - tolerance) / softness held inside 0 to 1; with softness 0 it is 0
where d <= tolerance and 1 elsewhere. Tolerance 0 keys nothing: m = 1 everywhere, whatever the
softness (D-541, Adobe: "A value of 0 makes the entire image opaque"); tolerance 100 keys
everything. The views: "final", every channel of the pixel, colour and covering, times m, so the
colour is kept and only the covering falls; "source", the layer exactly as it is; "matte", m as
an encoded grey, back to linear at the pixel's covering, which is kept. A pixel with a = 0 is
left as it is.

Keep Colors (D-541). The run above a "keep" instance is the Linear Color Key instances directly
above it in the stack, one after another (switched-off and refused instances, which draw
nothing, are passed over; any other effect ends the run), and the run's input P is the pixels
before its first instance (the keep instance's own input when nothing above it is a Linear Color
Key). At a pixel where P has covering a0 > 0, the keep share is kk = 1 - m, m worked as above
on P's straight colour; Q is the keep instance's input, covering aq. "final": where a0 kk > aq
the pixel becomes P times kk, colour and covering (P's colour brought back at covering a0 kk),
else Q stays; "matte": the encoded grey max(aq, a0 kk) / a0, back to linear at covering a0;
"source": Q as it is. Where a0 = 0, Q stays. So tolerance 0 brings nothing back.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its single-precision buffers. The rule
is continuous in the colour except where softness is 0 and at a grey in "hue"; the drawing's
greys are exact, and every case with softness 0 is checked that no pixel lies near its step.
Every case with more than one instance starts its run at the drawing and stacks only "final"
instances, which keep the straight colour wherever covering is left, so each instance reads the
drawing's own colour there (asserted in `render`).

The drawing is Color Difference Key's (tools/color_difference_key_reference.py): black, white,
grey, a green screen, a blue screen, skin, pure blue, cyan, pure green, yellow, red, magenta,
a violet, a pale blue, and the blue screen at half covering, each row darker by 8%. The
projects go into `Fixtures/linear_color_key`, the expected frames into
`Fixtures/linear_color_key/expected_linear_color_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/linear_color_key_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_difference_key_reference as D  # noqa: E402  (draws the shared picture)

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "linear_color_key"
NUMBERS = ("matching_tolerance", "matching_softness")
BLUE_SCREEN = "#1e46c8"   # (30, 70, 200), the drawing's blue screen
GREEN_SCREEN = "#28b43c"  # (40, 180, 60), the drawing's green screen
PURE_GREEN = "#00ff00"


# --- the rule -------------------------------------------------------------------------------

def chromaticity(e):
    s = sum(e)
    return [1 / 3] * 3 if s == 0 else [v / s for v in e]


def distance(e, k, match):
    if match == "rgb":
        return 100 * max(abs(u - v) for u, v in zip(e, k))
    if match == "hue":
        h, hk = D.hue(e), D.hue(k)
        if h is None or hk is None:
            return 100.0
        a = abs(h - hk) % 360
        return 100 * min(a, 360 - a) / 180
    return 100 * max(abs(u - v) for u, v in zip(chromaticity(e), chromaticity(k)))


def kept(d, k):
    if k["tolerance"] == 0:
        return 1.0
    if k["softness"] == 0:
        return 0.0 if d <= k["tolerance"] else 1.0
    return min(1.0, max(0.0, (d - k["tolerance"]) / k["softness"]))


def key(px, k, cur=None):
    """A "key" instance on cur (the drawing's own pixels when None), reading the straight colour
    from the drawing (see the note above)."""
    cur = cur or [R.working(p) for p in px]
    if k["view"] == "source":
        return [list(q) for q in cur]
    out = []
    for p, w in zip(px, cur):
        if w[3] == 0:
            out.append(w)
            continue
        m = kept(distance(B.encoded(p), k["key"], k["match"]), k)
        if k["view"] == "final":
            out.append([v * m for v in w])
        else:
            a = w[3]
            g = S.srgb_to_linear(m) * a
            out.append([g, g, g, a])
    return out


def keep(px, cur, k):
    """A "keep" instance on cur, its run's input the drawing's own pixels."""
    out = []
    for p, q in zip(px, cur):
        w = R.working(p)
        a0 = w[3]
        if k["view"] == "source" or a0 == 0:
            out.append(list(q))
            continue
        kk = 1 - kept(distance(B.encoded(p), k["key"], k["match"]), k)
        if k["view"] == "final":
            out.append([v * kk for v in w] if a0 * kk > q[3] else list(q))
        else:
            g = S.srgb_to_linear(max(q[3], a0 * kk) / a0) * a0
            out.append([g, g, g, a0])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(view="final", key_color="#0000ff", match_colors="rgb", shift=0, key_operation="key",
         above=(), **numbers):
    """One instance; `above`, the instances before it on the layer, first first."""
    c = {"view": view, "key_color": key_color, "match_colors": match_colors, "shift": shift,
         "key_operation": key_operation, "above": list(above),
         "matching_tolerance": 10, "matching_softness": 10}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    v = {n: min(100, max(0, value_at(c[n], frame_no))) for n in NUMBERS}
    return {"view": c["view"], "key": D.hex_colour(c["key_color"]), "match": c["match_colors"],
            "tolerance": v["matching_tolerance"], "softness": v["matching_softness"],
            "op": c["key_operation"]}


def render(c, frame_no):
    px, cur = B.pixels(), None
    for i, inst in enumerate([*c["above"], c]):
        assert i == 0 or c["above"][i - 1]["view"] == "final", "a run of final instances only"
        k = settings(inst, frame_no)
        if k["op"] == "keep":
            cur = keep(px, cur or [R.working(p) for p in px], k)
        else:
            cur = key(px, k, cur)
    return R.frame(cur, c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    def params(i):
        p = {"view": i["view"], "key_color": i["key_color"], "match_colors": i["match_colors"],
             "key_operation": i["key_operation"]}
        p.update({n: setting_json(i[n]) for n in NUMBERS})
        return p
    project = B.project_json(fx, "core.linear_color_key", params(c), c["shift"])
    layer = project["compositions"][0]["layers"][0]
    layer["effects"] = [{"instance_id": f"fx-0-{n}", "type_id": "core.linear_color_key",
                         "enabled": True, "parameters": params(i)}
                        for n, i in enumerate([*c["above"], c])]
    return project


FOUR = case(key_color=GREEN_SCREEN, match_colors="hue", matching_tolerance=20)  # FX-LCKEY-004
CASES = {
    "FX-LCKEY-001": ("The settings as they start: final, key colour pure blue, RGB, tolerance "
                     "10, softness 10. Pure blue's top two rows (within 10% of it) lose all their "
                     "covering, its third row keeps a share, its darker rows and every other "
                     "colour keep all of theirs.", case(), [0]),
    "FX-LCKEY-002": ("RGB, key colour the blue screen (30, 70, 200), tolerance 10, softness 10: "
                     "the blue screen's top two rows and the half-covered one's fall, its third "
                     "row keeps a quarter, its darker rows all of it.", case(key_color=BLUE_SCREEN),
                     [0]),
    "FX-LCKEY-003": ("The same with softness 0: inside 10% gone, outside kept, nothing between.",
                     case(key_color=BLUE_SCREEN, matching_softness=0), [0]),
    "FX-LCKEY-004": ("Hue, key colour the green screen (hue about 129 degrees), tolerance 20, "
                     "softness 10: every row of the green screen and of pure green (hue 120) "
                     "falls, lightness not counted; cyan (hue 180) keeps a share; the greys, "
                     "which have no hue, and the far hues keep all their covering.",
                     case(key_color=GREEN_SCREEN, match_colors="hue", matching_tolerance=20),
                     [0]),
    "FX-LCKEY-005": ("Chroma, key colour the green screen, tolerance 10, softness 10: every row "
                     "of the green screen falls, its darker self has the same chromaticity; pure "
                     "green, far in chromaticity, keeps all its covering.",
                     case(key_color=GREEN_SCREEN, match_colors="chroma"), [0]),
    "FX-LCKEY-006": ("Tolerance 100: every pixel is within it, so the whole layer is keyed out.",
                     case(matching_tolerance=100), [0]),
    "FX-LCKEY-007": ("Tolerance 0, softness 20: tolerance 0 makes the whole image opaque, as "
                     "Adobe documents (D-541), so nothing is keyed, not even pure blue itself, "
                     "and the layer is as it was.",
                     case(matching_tolerance=0, matching_softness=20), [0]),
    "FX-LCKEY-008": ("Hue with a grey key colour (#808080), tolerance 50: a grey has no hue, so "
                     "nothing matches and the layer is as it was.",
                     case(key_color="#808080", match_colors="hue", matching_tolerance=50), [0]),
    "FX-LCKEY-009": ("View matte, FX-LCKEY-004's settings: the share kept as grey, black where "
                     "the greens were keyed, white where nothing was, covering kept.",
                     case("matte", GREEN_SCREEN, "hue", matching_tolerance=20), [0]),
    "FX-LCKEY-010": ("View source, tolerance 100: the layer exactly as it is.",
                     case("source", matching_tolerance=100), [0]),
    "FX-LCKEY-011": ("Tolerance keyed from 0 at frame 0 to 40 at frame 4, linear, RGB on the "
                     "blue screen: at frame 0, tolerance 0, nothing is keyed (D-541); frame 2 is "
                     "tolerance 20's frame.", case(key_color=BLUE_SCREEN,
                                          matching_tolerance=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-LCKEY-012": ("Softness keyed from 0 at frame 0 to 60 at frame 4, linear, hue on the "
                     "green screen, tolerance 20.",
                     case(key_color=GREEN_SCREEN, match_colors="hue", matching_tolerance=20,
                          matching_softness=keyed((0, 0), (4, 60))), [2, 4]),
    "FX-LCKEY-013": ("FX-LCKEY-002 moved three pixels right: the same, moved.",
                     case(key_color=BLUE_SCREEN, shift=3), [0, 3]),
    "FX-LCKEY-019": ("Keep Colors (D-541): FX-LCKEY-004's hue key on the green screen, then a "
                     "second instance, keep, RGB, key pure green, tolerance 10, softness 10: "
                     "pure green's top two rows come back whole and its third row keeps a share; "
                     "the green screen and every other pixel are as FX-LCKEY-004 left them.",
                     case(key_color=PURE_GREEN, key_operation="keep", above=[FOUR]), [0]),
    "FX-LCKEY-020": ("Keep Colors with nothing keyed above it: nothing to bring back, the layer "
                     "as it was.", case(key_color=PURE_GREEN, key_operation="keep"), [0]),
    "FX-LCKEY-021": ("Keep Colors after a run of two keys, FX-LCKEY-002's RGB key on the blue "
                     "screen and FX-LCKEY-004's hue key on the green screen: the colours come "
                     "back from before the first, so pure green's top rows return as in "
                     "FX-LCKEY-019 and the blue screen stays keyed as in FX-LCKEY-002.",
                     case(key_color=PURE_GREEN, key_operation="keep",
                          above=[case(key_color=BLUE_SCREEN), FOUR]), [0]),
    "FX-LCKEY-022": ("FX-LCKEY-019 with the keep instance's view matte: the covering after it "
                     "as grey, at the drawing's covering.",
                     case("matte", PURE_GREEN, key_operation="keep", above=[FOUR]), [0]),
    "FX-LCKEY-023": ("FX-LCKEY-019 with the keep instance's tolerance 0: nothing comes back, "
                     "FX-LCKEY-004's frame.",
                     case(key_color=PURE_GREEN, key_operation="keep", above=[FOUR],
                          matching_tolerance=0), [0]),
}

INVALID = {
    "FX-LCKEY-014": ("Matching tolerance 101, above 100.", case(matching_tolerance=101)),
    "FX-LCKEY-015": ("Matching softness -1, below 0.", case(matching_softness=-1)),
    "FX-LCKEY-016": ("View \"Final\": the word is exact, so a capital is not it.", case("Final")),
    "FX-LCKEY-017": ("Match colors \"lab\": not one of RGB, hue and chroma.",
                     case(match_colors="lab")),
    "FX-LCKEY-018": ("Key colour \"blue\", not a #rrggbb colour.", case(key_color="blue")),
    "FX-LCKEY-024": ("Key operation \"Keep\": the word is exact, so a capital is not it.",
                     case(key_operation="Keep")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "linear_color_key", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """With softness 0 a pixel is in or out; none may lie within a hair of the tolerance."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["softness"] != 0:
                continue
            for p in B.pixels():
                if p[3]:
                    d = distance(B.encoded(p), k["key"], k["match"])
                    assert abs(d - k["tolerance"]) > 0.05, (fx, p, d)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    cover = lambda f, x, y: f[at(x, y)][3]  # noqa: E731
    full = lambda f, x, y: abs(cover(f, x, y) - px[at(x, y)][3] / 255) < 1e-9  # noqa: E731
    gone = lambda f, x, y: near(f[at(x, y)], [0, 0, 0, 0])  # noqa: E731
    others = lambda f, skip: all(full(f, x, y) for x in range(1, 16) if x not in skip  # noqa
                                 for y in range(H))

    one = c["FX-LCKEY-001"]["0"]
    assert gone(one, 7, 0) and gone(one, 7, 1) and 0.01 < cover(one, 7, 2) < 0.99
    assert all(full(one, 7, y) for y in range(4, H)) and others(one, {7})
    two = c["FX-LCKEY-002"]["0"]
    assert gone(two, 5, 0) and gone(two, 5, 1) and gone(two, 15, 0) and gone(two, 15, 1)
    assert abs(cover(two, 5, 2) - 0.25) < 0.05 and full(two, 5, 4)
    three = c["FX-LCKEY-003"]["0"]
    for x in range(1, 16):
        for y in range(H):
            assert gone(three, x, y) or full(three, x, y), (x, y)
    assert gone(three, 5, 1) and full(three, 5, 2)
    four = c["FX-LCKEY-004"]["0"]
    assert all(gone(four, 4, y) and gone(four, 9, y) for y in range(H))
    assert 0.01 < cover(four, 8, 0) < 0.99
    assert all(full(four, x, y) for x in (1, 2, 3, 7, 11, 12) for y in range(H))
    five = c["FX-LCKEY-005"]["0"]
    assert all(gone(five, 4, y) for y in range(H)) and full(five, 9, 0)
    assert all(near(p, [0, 0, 0, 0]) for p in c["FX-LCKEY-006"]["0"])
    assert like(c["FX-LCKEY-007"]["0"], drawn)
    assert like(c["FX-LCKEY-008"]["0"], drawn)
    nine = c["FX-LCKEY-009"]["0"]
    for i, (p, q) in enumerate(zip(nine, four)):
        a = px[i][3] / 255
        assert abs(p[3] - a) < 1e-12 and p[0] == p[1] == p[2], i
        if a:
            assert abs(p[0] - S.srgb_to_linear(q[3] / a) * a) < 1e-12, i
    assert like(c["FX-LCKEY-010"]["0"], drawn)
    eleven = c["FX-LCKEY-011"]
    assert like(eleven["2"], render(case(key_color=BLUE_SCREEN, matching_tolerance=20), 0))
    assert gone(eleven["4"], 5, 4) and like(eleven["0"], drawn)
    # Keep Colors.
    nineteen = c["FX-LCKEY-019"]["0"]
    assert full(nineteen, 9, 0) and full(nineteen, 9, 1) and 0.3 < cover(nineteen, 9, 2) < 0.5
    assert all(gone(nineteen, 9, y) for y in range(3, H))
    assert all(nineteen[at(x, y)] == four[at(x, y)] for x in range(W) if x != 9 for y in range(H))
    assert like(c["FX-LCKEY-020"]["0"], drawn)
    twentyone = c["FX-LCKEY-021"]["0"]
    assert all(near(twentyone[at(9, y)], nineteen[at(9, y)]) for y in range(H))
    assert all(near(twentyone[at(x, y)], two[at(x, y)]) for x in (5, 15) for y in range(H))
    for i, (p, q) in enumerate(zip(c["FX-LCKEY-022"]["0"], nineteen)):
        a = px[i][3] / 255
        assert abs(p[3] - a) < 1e-12 and p[0] == p[1] == p[2], i
        if a:
            assert abs(p[0] - S.srgb_to_linear(q[3] / a) * a) < 1e-12, i
    assert like(c["FX-LCKEY-023"]["0"], four)
    twelve = c["FX-LCKEY-012"]
    assert like(twelve["2"], render(case(key_color=GREEN_SCREEN, match_colors="hue",
                                         matching_tolerance=20, matching_softness=30), 0))
    # The colour is kept wherever any covering is left.
    for fx, frames in c.items():
        for f in frames.values():
            for p in f:
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    for i, p in enumerate(two):
        q = drawn[i]
        if p[3] > 1e-6:
            assert near([u / p[3] for u in p[:3]], [u / q[3] for u in q[:3]], 1e-7)
    moved = c["FX-LCKEY-013"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
