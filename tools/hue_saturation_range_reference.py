"""Hue/Saturation's Channel Range, worked a second way (D-1006).

D-307 gave Hue/Saturation After Effects' six colour ranges, each fixed: all of it within 15
degrees of its centre (0, 60 ... 300), fading to nothing at 45. D-1006 lets each range be moved
and widened as After Effects' Channel Range bar does, by its four handles. `reds_range` to
`magentas_range` are each four hue angles in degrees, 0 to 360, read going round the wheel the
way the hue grows: where the fade in starts, where the full range starts, where it ends, and
where the fade out ends. A hue `h` takes the range at

    a = (h - fade_start) mod 360, and each handle likewise measured from fade_start going round:
    a below range_start:            a / range_start (the fade in)
    a from range_start to range_end: 1
    a past range_end, before fade_end: (fade_end - a) / (fade_end - range_end) (the fade out)
    otherwise 0.

The four must come in that order going round less than one turn: the measured fade end below
360. A file without a range's four reads it at After Effects' own, the centre less 45, less 15,
plus 15 and plus 45, which is D-307's fixed rule, so FX-HUESAT-001 to 036 are as they were.

Everything else is `tools/hue_saturation_reference.py`, imported here. **This file never runs the
build's code path.**

    python tools/hue_saturation_range_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import hue_saturation_reference as HS  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = HS.W, HS.H
OUT = HS.OUT
NAMES = {k: k.replace("_hsl", "_range") for k in HS.COLOURS}


def default(centre):
    return [(centre + d) % 360 for d in (-45, -15, 15, 45)]


def spans(b):
    """The handles measured from the fade start going round: range start, range end, fade end."""
    fs, rs, re, fe = b
    r1 = (rs - fs) % 360
    r2 = r1 + (re - rs) % 360
    r3 = r2 + (fe - re) % 360
    return r1, r2, r3


def valid(b):
    return len(b) == 4 and all(0 <= v <= 360 for v in b) and spans(b)[2] < 360


def weight(h, b):
    r1, r2, r3 = spans(b)
    a = (h - b[0]) % 360
    if a < r1:
        return a / r1
    if a <= r2:
        return 1.0
    if a < r3:
        return (r3 - a) / (r3 - r2)
    return 0.0


def with_ranges(e, three, ranges, bounds):
    hue, saturation, lightness = three
    h, s, _ = HS.to_hsl(*e)
    if s == 0:
        return hue, saturation, lightness
    for name in HS.COLOURS:
        w = weight(h, bounds[name])
        hue += w * ranges[name][0]
        saturation += w * max(-100, min(100, ranges[name][1]))
        lightness += w * max(-100, min(100, ranges[name][2]))
    return hue, saturation, min(100.0, max(-100.0, lightness))


def render(c):
    pixels = [p for row in HS.DRAWINGS["wheel"] for p in row]
    ranges = {k: c["ranges"].get(k, (0, 0, 0)) for k in HS.COLOURS}
    bounds = {k: c["bounds"].get(NAMES[k], default(v)) for k, v in HS.COLOURS.items()}
    out = [R.working(p) for p in pixels]
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in w[:3]]
            three = with_ranges(e, (0, 0, 0), ranges, bounds)
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a for v in HS.grade(e, *three)]
    return R.frame(out, 0)


def case(bounds, **ranges):
    return {"bounds": bounds, "ranges": ranges}


EVERY_DEFAULT = {NAMES[k]: default(v) for k, v in HS.COLOURS.items()}
CASES = {
    "FX-HUESAT-037": ("All six Channel Ranges written at After Effects' own (reds 315, 345, 15, "
                      "45 and so on round the wheel) with the greens hue +90, saturation -40, "
                      "lightness +10: the same as FX-HUESAT-024, which leaves them out.",
                      case(EVERY_DEFAULT, greens_hsl=(90, -40, 10))),
    "FX-HUESAT-038": ("Reds widened to 275, 320, 35, 85, saturation -100: red (0), orange (30) "
                      "and rose (330) turn fully grey; magenta (300) and yellow (60) lose half "
                      "their colour; yellow-green (90) to violet (270) stay.",
                      case({"reds_range": [275, 320, 35, 85]}, reds_hsl=(0, -100, 0))),
    "FX-HUESAT-039": ("Yellows narrowed to 50, 55, 65, 70, saturation -100: only yellow (60) "
                      "turns grey; orange (30) and yellow-green (90), half-taken at the default, "
                      "now stay.",
                      case({"yellows_range": [50, 55, 65, 70]}, yellows_hsl=(0, -100, 0))),
    "FX-HUESAT-040": ("Blues 180, 240, 240, 300, a full range of one hue, lightness -50: blue "
                      "(240) darkens fully; azure (210) and violet (270) half; cyan (180) and "
                      "magenta (300) stay.",
                      case({"blues_range": [180, 240, 240, 300]}, blues_hsl=(0, 0, -50))),
    "FX-HUESAT-041": ("Greens 85, 85, 155, 155, no fade, hue +60: yellow-green (90), green "
                      "(120) and spring green (150) all turn 60 degrees; yellow (60) and cyan "
                      "(180) stay.",
                      case({"greens_range": [85, 85, 155, 155]}, greens_hsl=(60, 0, 0))),
    "FX-HUESAT-042": ("Reds widened to 240, 300, 60, 120 over the yellows at their own, both "
                      "saturation -50: yellow (60) is fully in both, so it takes -100 and turns "
                      "grey; overlapping ranges add.",
                      case({"reds_range": [240, 300, 60, 120]}, reds_hsl=(0, -50, 0),
                           yellows_hsl=(0, -50, 0))),
    "FX-HUESAT-043": ("Magentas dragged round to 330, 0, 35, 55, saturation -100: red (0) and "
                      "orange (30) turn grey; magenta (300), its own colour, stays.",
                      case({"magentas_range": [330, 0, 35, 55]}, magentas_hsl=(0, -100, 0))),
}
INVALID = {
    "FX-HUESAT-044": ("Reds 45, 15, 345, 315, the handles backwards, more than one turn.",
                      case({"reds_range": [45, 15, 345, 315]}, reds_hsl=(0, -100, 0))),
    "FX-HUESAT-045": ("A greens handle at 361, past 360.",
                      case({"greens_range": [75, 105, 135, 361]}, greens_hsl=(0, -100, 0))),
    "FX-HUESAT-046": ("A cyans handle at -1, below 0.",
                      case({"cyans_range": [-1, 165, 195, 225]}, cyans_hsl=(0, -100, 0))),
    "FX-HUESAT-047": ("Blues with three handles, not four.",
                      case({"blues_range": [195, 225, 285]}, blues_hsl=(0, -100, 0))),
}


def project_json(c):
    p = HS.project_json("", HS.case(drawing="wheel"))
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    params |= {k: list(v) for k, v in c["ranges"].items()} | c["bounds"]
    return p


def main():
    expected = {"tolerance": HS.TOLERANCE, "width": W, "height": H, "cases": {}}
    plain = render(case({}))
    for fx, (says, c) in CASES.items():
        f = render(c)
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project_json(c), indent=2) + "\n", encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": name, "frames": {"0": f}}
        print(f"{fx}: {sum(p != q for p, q in zip(f, plain))} changed")
    for fx, (says, c) in INVALID.items():
        assert not all(valid(b) for b in c["bounds"].values())
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project_json(c), indent=2) + "\n", encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": name, "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_hue_saturation_range.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x: 4 * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    plain = render(case({}))
    px = [p for row in HS.DRAWINGS["wheel"] for p in row]
    master = lambda x, *three: HS.hue_saturation([px[at(x)]], *three)[0]  # noqa: E731
    # The default four are D-307's rule, hue by hue.
    for centre in HS.COLOURS.values():
        for h in range(0, 360, 5):
            assert abs(weight(h, default(centre)) - HS.weight(h, centre)) < 1e-12, (centre, h)
    assert all(near(p, q) for p, q in zip(c["FX-HUESAT-037"],
                                          HS.render(HS.case(drawing="wheel",
                                                            greens_hsl=(90, -40, 10)), 0)))
    # Columns 1 to 12 are 0, 30 ... 330 degrees (an 8-bit step off at the halves).
    for fx, setting, full, half, kept in (
            ("FX-HUESAT-038", (0, -100, 0), [1, 2, 12], [3, 11], [4, 5, 6, 7, 8, 9, 10, 13, 14]),
            ("FX-HUESAT-039", (0, -100, 0), [3], [], [1, 2, 4, 5, 12, 13]),
            ("FX-HUESAT-040", (0, 0, -50), [9], [8, 10], [1, 3, 5, 7, 11, 13]),
            ("FX-HUESAT-041", (60, 0, 0), [4, 5, 6], [], [1, 2, 3, 7, 8, 13]),
            ("FX-HUESAT-042", (0, -100, 0), [3], [], [7, 8, 9, 13]),
            ("FX-HUESAT-043", (0, -100, 0), [1, 2], [], [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])):
        f = c[fx]
        for x in full:
            assert near(f[at(x)], master(x, *setting)), (fx, x)
        for x in half:
            assert not near(f[at(x)], master(x, *setting)) and not near(f[at(x)], plain[at(x)]), \
                (fx, x)
        for x in kept:
            assert near(f[at(x)], plain[at(x)], 1e-12), (fx, x)
    print("checked")


if __name__ == "__main__":
    main()
