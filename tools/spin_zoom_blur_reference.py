"""Spin & Zoom Blur, worked a second way.

`core.spin_zoom_blur`, modelled on CycoreFX's CC Radial Blur (its manual, and a tutorial that
spins a wheel with it): a camera zooming or turning about a centre during the exposure, in six
kinds, some fading. Nothing is ported; this is this program's own reading. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. With c the centre in layer pixels, (center_x / 100 * width, center_y / 100 * height)
of the drawing's own size, as Radial Blur's (D-95), p the centre of the output pixel, d = p - c
and r = |d|, each pixel is a weighted mean of document 21's bilinear samples of the layer at n
points, transparent outside the layer:

- `type` says where the points lie. With u_k = k / (n - 1), k = 0 to n - 1, k = 0 at the pixel:
  - "straight_zoom": c + (1 - A u_k / 100) d, from the pixel back toward the centre by A per
    cent of r, so everything streaks outward, equally weighted (Radial Blur's zoom, D-110).
  - "fading_zoom": the same points, the k-th weighted n - k, so a streak fades as it runs out.
  - "centered_zoom": c + (1 + A / 200 - A u_k / 100) d, half each way, equally weighted.
  - "rotate": d turned by -A u_k degrees about c, equally weighted: each pixel reads what lies
    behind it, so with A positive the streaks run clockwise on screen, negative anticlockwise.
  - "rotate_fading": the same points, weighted n - k.
  - "scratch": d turned by -A / 2 + A u_k degrees, both ways evenly (Radial Blur's spin).
- `amount` A, -360 to 360: degrees for the turns, per cent of r for the zooms. A zoom past 100
  reads through the centre to the far side.
- `quality` q, 1 to 100: the path is r |A| pi / 180 for the turns and r |A| / 100 for the zooms,
  and n = min(ceil(path q / 50) + 1, 256). At 50 a point per pixel of path, as Radial Blur; less
  shows separate copies, more is smoother. With n = 1 the output is the sample at p.

Premultiplied red, green, blue and alpha are averaged alike. The layer does not grow; an earlier
effect that grew it is sampled over its grown pixels with the centre still in the drawing's
own size. There is no edge choice: past the layer is transparent.

Every case is directional blur's drawing, 16 by 10, as Radial Blur's cases. The projects go into
`Fixtures/spin_zoom_blur`, the expected frames into
`Fixtures/spin_zoom_blur/expected_spin_zoom_blur.json`.

**This file never runs the build's code path.** It works in double precision on lists.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/spin_zoom_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import radial_blur_reference as RB  # noqa: E402

W, H = D.W, D.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "spin_zoom_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
AMOUNT = (-360, 360)
QUALITY = (1, 100)
CENTER = (-1000, 1000)
MOST = 256
TYPES = ("straight_zoom", "fading_zoom", "centered_zoom", "rotate", "scratch", "rotate_fading")


# --- the rule -------------------------------------------------------------------------------

def points(kind, amount, quality, center, x, y):
    """Where the pixel (x, y) of layer space is sampled, and each point's weight."""
    cx, cy = center[0] / 100 * W, center[1] / 100 * H
    px, py = x + 0.5, y + 0.5
    dx, dy = px - cx, py - cy
    r = math.sqrt(dx * dx + dy * dy)
    turn = kind in ("rotate", "scratch", "rotate_fading")
    path = r * abs(amount) * math.pi / 180 if turn else r * abs(amount) / 100
    n = min(math.ceil(path * quality / 50) + 1, MOST)
    if n == 1:
        return [((px, py), 1.0)]
    out = []
    for k in range(n):
        u = k / (n - 1)
        w = float(n - k) if kind in ("fading_zoom", "rotate_fading") else 1.0
        if turn:
            t = math.radians(-amount / 2 + amount * u if kind == "scratch" else -amount * u)
            out.append(((cx + dx * math.cos(t) - dy * math.sin(t),
                         cy + dx * math.sin(t) + dy * math.cos(t)), w))
        else:
            s = 1 + amount / 200 - amount * u / 100 if kind == "centered_zoom" else \
                1 - amount * u / 100
            out.append(((cx + s * dx, cy + s * dy), w))
    return out


def blurred(layer, kind, amount, quality, center, x, y):
    if not (0 <= x - layer["left"] < layer["w"] and 0 <= y - layer["top"] < layer["h"]):
        return [0.0] * 4
    at = points(kind, amount, quality, center, x, y)
    total, weight = [0.0] * 4, 0.0
    for (sx, sy), w in at:
        s = RB.bilinear(layer, sx, sy)
        weight += w
        for i in range(4):
            total[i] += w * s[i]
    return [v / weight for v in total]


# --- the cases ------------------------------------------------------------------------------

def case(kind="straight_zoom", amount=30, quality=None, center=(50, 50), shift=0, before=None):
    """`quality` None: the file does not say, so 50."""
    return {"drawing": "bars", "type": kind, "amount": amount, "quality": quality,
            "center": center, "shift": shift, "before": before}


def render(c, frame_no):
    layer = RB.layer_of(c)
    amount = RB.clamp(value_at(c["amount"], frame_no), AMOUNT)
    quality = RB.clamp(value_at(c["quality"] if c["quality"] is not None else 50, frame_no),
                       QUALITY)
    center = [RB.clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    return [blurred(layer, c["type"], amount, quality, center, x - c["shift"], y)
            for y in range(H) for x in range(W)]


def plain(c):
    return render(case(amount=0, shift=c["shift"]), 0)


CASES = {
    "FX-SPINZOOM-001": ("Straight zoom 30 about the middle: every edge streaks outward, evenly; "
                        "Radial Blur's zoom 30, FX-RADIAL-002, exactly.", case(), [0]),
    "FX-SPINZOOM-002": ("Fading zoom 30: the same streaks, fading as they run out, so the "
                        "drawing itself stays stronger.", case("fading_zoom"), [0]),
    "FX-SPINZOOM-003": ("Centered zoom 30: streaks both outward and inward, half as long each "
                        "way.", case("centered_zoom"), [0]),
    "FX-SPINZOOM-004": ("Rotate 30: every edge streaks clockwise only, so the line down the "
                        "left edge streaks upward past its top and not past its foot.",
                        case("rotate"), [0]),
    "FX-SPINZOOM-005": ("Rotate -30: anticlockwise, the line streaking past its foot instead.",
                        case("rotate", -30), [0]),
    "FX-SPINZOOM-006": ("Scratch 30: both ways evenly; Radial Blur's spin 30, FX-RADIAL-001, "
                        "exactly.", case("scratch"), [0]),
    "FX-SPINZOOM-007": ("Rotate fading 30: clockwise, fading as it goes.",
                        case("rotate_fading"), [0]),
    "FX-SPINZOOM-008": ("Straight zoom 60, quality 10: a fifth of the points, so the streaks "
                        "break into separate copies.", case(amount=60, quality=10), [0]),
    "FX-SPINZOOM-009": ("Rotate 30, quality 100: twice the points, smoother than "
                        "FX-SPINZOOM-004.", case("rotate", quality=100), [0]),
    "FX-SPINZOOM-010": ("Amount 0: the drawing, untouched.", case(amount=0), [0]),
    "FX-SPINZOOM-011": ("Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as "
                        "a wheel spinning up.", case("rotate", keyed((0, 0), (4, 40))),
                        [0, 2, 4]),
    "FX-SPINZOOM-012": ("Rotate 30 about the top left corner, moved three pixels right: the "
                        "centre moves with the drawing, and nothing is drawn left of it.",
                        case("rotate", center=(0, 0), shift=3), [0]),
    "FX-SPINZOOM-013": ("Straight zoom -30: the points run outward from the pixel, so the "
                        "streaks run inward, toward the centre.", case(amount=-30), [0]),
    "FX-SPINZOOM-014": ("A directional blur, direction 90 and length 4, then rotate fading 30 "
                        "about 0, 0: the grown layer is read, the centre still the drawing's "
                        "corner.", case("rotate_fading", center=(0, 0), before=(90, 4)), [0]),
    "FX-SPINZOOM-015": ("Rotate 360 about -1000, -1000: every path is longer than 255 pixels, "
                        "so each takes the most points, 256.",
                        case("rotate", 360, center=(-1000, -1000)), [0]),
}

INVALID = {
    "FX-SPINZOOM-016": ("Amount 361, above 360.", case(amount=361)),
    "FX-SPINZOOM-017": ("Amount -361, below -360.", case(amount=-361)),
    "FX-SPINZOOM-018": ("Quality 0, below 1.", case(quality=0)),
    "FX-SPINZOOM-019": ("Quality 101, above 100.", case(quality=101)),
    "FX-SPINZOOM-020": ("Type \"spin\", which is Radial Blur's word, not one of these.",
                        case("spin")),
    "FX-SPINZOOM-021": ("Type \"Rotate\": the word is exact.", case("Rotate")),
    "FX-SPINZOOM-022": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = RB.project_json(fx, RB.case(shift=c["shift"], before=c["before"]))
    effects = p["compositions"][0]["layers"][0]["effects"]
    params = {"type": c["type"], "amount": setting_json(c["amount"]),
              "center": setting_json(c["center"])}
    if c["quality"] is not None:
        params["quality"] = setting_json(c["quality"])
    effects[-1] = {"instance_id": effects[-1]["instance_id"], "type_id": "core.spin_zoom_blur",
                   "enabled": True, "parameters": params}
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in D.DRAWINGS.items():
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
    (OUT / "expected_spin_zoom_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-9 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    radial = json.loads((OUT.parent / "radial_blur" / "expected_radial_blur.json")
                        .read_text(encoding="utf-8"))["cases"]
    assert near(c["FX-SPINZOOM-001"], radial["FX-RADIAL-002"]["frames"]["0"])
    assert near(c["FX-SPINZOOM-006"], radial["FX-RADIAL-001"]["frames"]["0"])
    for f in ("FX-SPINZOOM-002", "FX-SPINZOOM-003", "FX-SPINZOOM-007", "FX-SPINZOOM-008",
              "FX-SPINZOOM-009", "FX-SPINZOOM-013"):
        assert c[f] != c["FX-SPINZOOM-001"] and c[f] != c["FX-SPINZOOM-004"], f
    # Fading keeps more of the drawing: the line's middle is more covered fading than straight.
    assert c["FX-SPINZOOM-002"][at(0, 4)][3] > c["FX-SPINZOOM-001"][at(0, 4)][3]
    # Centered zoom also streaks inward: column 1, between the line and the centre, is reached.
    assert c["FX-SPINZOOM-003"][at(1, 4)][3] > 0 and c["FX-SPINZOOM-001"][at(1, 4)][3] < 1e-9
    # Inward: -30 reaches column 1 as well.
    assert c["FX-SPINZOOM-013"][at(1, 4)][3] > 0
    # Rotate: clockwise past the line's top (0, 1), not its foot (0, 8); -30 the other way.
    four, five = c["FX-SPINZOOM-004"], c["FX-SPINZOOM-005"]
    assert four[at(0, 1)][3] > 0.05 and four[at(0, 8)][3] < 1e-9
    assert five[at(0, 8)][3] > 0.05 and five[at(0, 1)][3] < 1e-9
    assert c["FX-SPINZOOM-010"] == drawn
    eleven = expected["cases"]["FX-SPINZOOM-011"]["frames"]
    assert eleven["0"] == drawn and near(eleven["2"], render(case("rotate", 20), 0))
    assert near(eleven["4"], render(case("rotate", 40), 0))
    twelve = c["FX-SPINZOOM-012"]
    assert all(twelve[at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    assert all(len(points("rotate", 360, 50, (-1000, -1000), x, y)) == MOST
               for x in range(W) for y in range(H))
    assert any(p[3] > 0 for p in c["FX-SPINZOOM-015"])
    for name, px in c.items():
        for p in px:
            assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                      for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
