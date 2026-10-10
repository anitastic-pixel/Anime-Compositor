"""Arbitrary Map, worked a second way.

D-395 adds `core.arbitrary_map`, after After Effects' PS Arbitrary Map: the layer's colours
through a Photoshop arbitrary map file (.amp), the curve file Photoshop's Curves dialog saves
when a curve is drawn with the pencil. The file is a lookup file of the project, as Color
Lookup's .cube is (D-182): an asset of kind `lut`, kept where it is, copied by Collect Files, and
a missing or unreadable one reported and kept, never dropped. It is this program's own method,
written from Adobe's published file format; nothing is ported.

Reading a .amp file (Adobe's Photoshop File Formats Specification, "Arbitrary Map"). It has no
header and no version: it is one or more tables of 256 bytes, table entry x the value byte x
becomes. One table is the master, for every colour channel at once. Exactly three tables are red,
green and blue, the master left straight. Any other count is the master and then the channels in
turn: two tables master and red; four master, red, green and blue; five those and alpha (a file
Photoshop writes of five tables carries a straight fifth table, which leaves alpha as it is). A
channel the file has no table for is straight (table entry x is x). The file is refused, with
the reason `read_amp` gives word for word, when it is empty, when its length is not a whole
number of 256-byte tables, or when it holds more than five tables, which a layer of red, green,
blue and alpha has no channel for.

The rule. `phase`, -255 to 255 levels, keyable, cycles each table the file holds to the right:
a held table T becomes T'(u) = T(v) mixed linearly between entries floor(v) and floor(v) + 1, the
entry past 255 being entry 0, at v = (u - phase) mod 256. A straight table the file did not hold
is not cycled. The colour tables are first made into one table per colour, 256 entries,
F_c[x] = M'(C_c'(x)) for x = 0 to 255, the channel's table then the master's, as Curves applies
its master after each channel (D-111). At a pixel with covering a > 0,
e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) is its encoded straight colour, u = 255 e, and the
channel becomes F_c at u mixed linearly between the entries around it, divided by 255, back to
linear and times a; this is Color Lookup's 1D rule (D-182) over the table F. A pixel with a = 0
is left as it is. With `apply_to_alpha` "on" and a file with an alpha table, the covering then
goes through it, cycled, as Curves' alpha curve does (D-302): a becomes A'(255 a) / 255, the
straight colour kept, so p.rgb becomes p.rgb / a times the new covering, and a pixel that did not
show and now does is black. Without an alpha table, alpha is left as it is. The layer does not
grow and a draft changes nothing.

`map` is the id of a `lut` asset of the project, or "" for none; "" leaves the layer as it is.
`phase` 0 when added; `apply_to_alpha` "off" or "on", "off" when added. A `map` naming no `lut`
asset, a missing file and a file that cannot be read each leave the layer as it is, with
EFFECT_PARAMETER_INVALID, MEDIA_MISSING and MEDIA_DECODE_FAILED.

**This file never runs the build's code path.** It works in double precision on lists, from the
file's bytes, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, the bands of
`tools/invert_reference.py` (with a half and a quarter covered column), unmoved unless the case
says. The drawing goes into `Fixtures/arbitrary_map/media`, the map files into
`Fixtures/arbitrary_map/maps` (the refused ones into `maps/refused`), the projects into
`Fixtures/arbitrary_map`, and the expected frames into
`Fixtures/arbitrary_map/expected_arbitrary_map.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/arbitrary_map_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from invert_reference import DRAWINGS  # noqa: E402
import cube_lut_reference as LUT  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "arbitrary_map"
TOLERANCE = 2e-5  # document 25's default for a filter
PHASE = (-255, 255)
STRAIGHT = list(range(256))


# --- reading a .amp file --------------------------------------------------------------------

class Refused(Exception):
    pass


def read_amp(data):
    """{channel: table} for the channels the file holds, of "master", "red", "green", "blue" and
    "alpha", or Refused with the reason."""
    n = len(data)
    if n == 0:
        raise Refused("the file is empty")
    if n % 256:
        raise Refused(f"the file is {n} bytes long, not a whole number of 256-byte tables")
    k = n // 256
    if k > 5:
        raise Refused(f"the file holds {k} tables, and this program reads at most five: master, "
                      "red, green, blue and alpha")
    tables = [list(data[i * 256:(i + 1) * 256]) for i in range(k)]
    names = ("red", "green", "blue") if k == 3 else ("master", "red", "green", "blue", "alpha")
    return dict(zip(names, tables))


# --- the rule -------------------------------------------------------------------------------

def cycled(table, phase, u):
    """The held `table` cycled right by `phase`, at u."""
    v = (u - phase) % 256
    i = math.floor(v)
    f = v - i
    return table[i] * (1 - f) + table[(i + 1) % 256] * f


def curve(m, name, phase):
    """The channel's table as a function of u: cycled when the file holds it, else straight."""
    if name in m:
        return lambda u: cycled(m[name], phase, u)
    return lambda u: u


def colour_tables(m, phase):
    """F_c[x] for the three colours, x = 0 to 255."""
    master = curve(m, "master", phase)
    return [[master(curve(m, c, phase)(x)) for x in range(256)] for c in ("red", "green", "blue")]


def mixed(table, u):
    """`table` at u in 0..255, mixed between the entries around it."""
    i = min(math.floor(u), 254)
    f = u - i
    return table[i] * (1 - f) + table[i + 1] * f


def arbitrary_map(pixels, m, phase, alpha):
    out = [R.working(p) for p in pixels]
    if m is None:
        return out
    tables = colour_tables(m, phase)
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, w[c] / a))) for c in range(3)]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, mixed(tables[c], 255 * e[c]) / 255))) * a
                     for c in range(3)]
    if alpha == "on" and "alpha" in m:
        f = curve(m, "alpha", phase)
        for w in out:
            a = w[3]
            to = min(255.0, max(0.0, f(255 * a))) / 255
            w[:3] = [v / a * to if a > 0 else 0.0 for v in w[:3]]
            w[3] = to
    return out


# --- the map files --------------------------------------------------------------------------

def table(f):
    return [min(255, max(0, round(f(x / 255) * 255))) for x in range(256)]


GAMMA = table(lambda t: t ** 0.6)                              # lighter
INVERT = table(lambda t: 1 - t)
HALF = table(lambda t: 0.25 + t / 2)                           # flatter, lifted
SINE = table(lambda t: math.sin(math.pi / 2 * t))
S_CURVE = table(lambda t: t * t * (3 - 2 * t))                 # more contrast
POWER = table(lambda t: t ** 1.5)
LIFT = table(lambda t: 30 / 255 + 0.8 * t)
ROOT = table(lambda t: math.sqrt(t))                           # alpha lifted
SQUARE = table(lambda t: t * t)

MAPS = {
    "straight_256.amp": STRAIGHT,
    "master_256.amp": GAMMA,
    "rgb_768.amp": INVERT + HALF + SINE,
    "two_512.amp": INVERT + SQUARE,
    "master_rgb_1024.amp": S_CURVE + POWER + INVERT + LIFT,
    "alpha_1280.amp": S_CURVE + POWER + INVERT + LIFT + ROOT,
}
REFUSED = {
    "empty.amp": (b"", "the file is empty"),
    "odd_300.amp": (bytes(GAMMA) + bytes(44), "the file is 300 bytes long, not a whole number "
                                              "of 256-byte tables"),
    "six_1536.amp": (bytes(STRAIGHT * 6), "the file holds 6 tables, and this program reads at "
                                          "most five: master, red, green, blue and alpha"),
    "cube.amp": (LUT.LUTS["tint_1d.cube"].encode("ascii"), "the file is 135 bytes long, not a "
                                                           "whole number of 256-byte tables"),
}


def map_of(name):
    return read_amp((OUT / "maps" / name).read_bytes())


# --- the cases ------------------------------------------------------------------------------

def case(file="master_256.amp", map_="asset-map", phase=0, alpha="off", shift=0):
    """`file` is the map asset's file under maps/ (None: the project has no map asset), `map_`
    the effect's setting."""
    return {"file": file, "map": map_, "phase": phase, "apply_to_alpha": alpha, "shift": shift}


def render(c, frame_no):
    pixels = [p for row in DRAWINGS["bands"] for p in row]
    m = None
    if c["map"] == "asset-map" and c["file"] and (OUT / "maps" / c["file"]).exists():
        m = map_of(c["file"])
    phase = min(PHASE[1], max(PHASE[0], value_at(c["phase"], frame_no)))
    return R.frame(arbitrary_map(pixels, m, phase, c["apply_to_alpha"]), c["shift"])


def plain(c):
    return render(case(file=None, map_="", shift=c["shift"]), 0)


CASES = {
    "FX-AMAP-001": ("straight_256.amp, one straight table: the drawing, unchanged but for "
                    "rounding.", case(file="straight_256.amp"), [0]),
    "FX-AMAP-002": ("master_256.amp, one table, the master, a lightening curve (entry x is "
                    "255 (x / 255)^0.6): every colour channel through it, black kept, the grey "
                    "#808080 to #a9a9a9, the blue #3a6fd8 to #699be7.", case(), [0]),
    "FX-AMAP-003": ("rgb_768.amp, three tables, red, green and blue, the master left straight: "
                    "red inverted, green flattened to between 64 and 191, blue along a quarter "
                    "sine; white to #00bfff, black to #ff4000.", case(file="rgb_768.amp"), [0]),
    "FX-AMAP-004": ("master_rgb_1024.amp, four tables, the master (an S-curve) then red (a "
                    "power of 1.5), green (inverted) and blue (lifted to 30 and narrowed): each "
                    "channel through its own table, then the master.",
                    case(file="master_rgb_1024.amp"), [0]),
    "FX-AMAP-005": ("alpha_1280.amp, five tables, FX-AMAP-004's four and an alpha table (the "
                    "square root), Apply Phase Map To Alpha off: FX-AMAP-004's picture; alpha "
                    "untouched.", case(file="alpha_1280.amp"), [0]),
    "FX-AMAP-006": ("FX-AMAP-005 with Apply Phase Map To Alpha on: the colours as FX-AMAP-005, "
                    "and the covering through the fifth table, the half-covered column to 181 "
                    "of 255 and the quarter one to 128, the straight colour kept.",
                    case(file="alpha_1280.amp", alpha="on"), [0]),
    "FX-AMAP-007": ("two_512.amp, two tables: the master (inverted) and red (squared); green "
                    "and blue straight, so every channel is inverted and red squared first.",
                    case(file="two_512.amp"), [0]),
    "FX-AMAP-008": ("FX-AMAP-002 with phase 64: the master cycled 64 levels right, so entry x "
                    "is what entry x - 64 was, wrapping: black (entry 0) takes entry 192's "
                    "value, and the grey takes entry 64's.", case(phase=64), [0]),
    "FX-AMAP-009": ("FX-AMAP-003 with phase -100.25: each of the three tables cycled left, "
                    "between entries mixed; the master, which the file does not hold, stays "
                    "straight.", case(file="rgb_768.amp", phase=-100.25), [0]),
    "FX-AMAP-010": ("FX-AMAP-002 with the phase keyed from 0 at frame 0 to 96 at frame 4, "
                    "linear: frame 0 FX-AMAP-002, frame 2 cycled 48, frame 4 cycled 96.",
                    case(phase=keyed((0, 0), (4, 96))), [0, 2, 4]),
    "FX-AMAP-011": ("FX-AMAP-002 with Apply Phase Map To Alpha on: master_256.amp has no "
                    "alpha table, so alpha is left as it is: FX-AMAP-002's picture.",
                    case(alpha="on"), [0]),
    "FX-AMAP-012": ("FX-AMAP-006 with phase 100: the alpha table cycled with the others, so a "
                    "pixel that did not show (entry 0 takes entry 156's value, 199) now shows, "
                    "black, at 199 of 255.", case(file="alpha_1280.amp", alpha="on", phase=100),
                    [0]),
    "FX-AMAP-013": ("No map file chosen, the effect as it is added: the drawing, untouched, "
                    "with no warning.", case(map_=""), [0]),
    "FX-AMAP-014": ("FX-AMAP-002 moved three pixels right: the same, moved.", case(shift=3),
                    [0, 3]),
    "FX-AMAP-015": ("The map asset's file maps/gone.amp is not there: the drawing, untouched, "
                    "with MEDIA_MISSING when the file is opened and at every frame; the asset "
                    "and the setting are kept, to relink.", case(file="gone.amp"), [0]),
    "FX-AMAP-016": ("The setting names asset-nothing, which the project does not have: the "
                    "drawing, untouched, with EFFECT_PARAMETER_INVALID; the setting is kept as "
                    "written.", case(map_="asset-nothing"), [0]),
    "FX-AMAP-017": ("The setting names asset-bands, the drawing, which is not a lookup file: "
                    "the drawing, untouched, with EFFECT_PARAMETER_INVALID.",
                    case(map_="asset-bands"), [0]),
}
WARNINGS = {"FX-AMAP-015": "MEDIA_MISSING", "FX-AMAP-016": "EFFECT_PARAMETER_INVALID",
            "FX-AMAP-017": "EFFECT_PARAMETER_INVALID"}

INVALID = {
    "FX-AMAP-018": ("Phase 256, above 255.", case(phase=256)),
    "FX-AMAP-019": ("Phase -256, below -255.", case(phase=-256)),
    "FX-AMAP-020": ("Phase keyed to 300 at frame 4.", case(phase=keyed((0, 0), (4, 300)))),
    "FX-AMAP-021": ("Apply Phase Map To Alpha \"yes\", not \"off\" or \"on\".", case(alpha="yes")),
}

# Checked by the test on its own, as opening the file cannot know: a map file that is there but
# cannot be read is found when a frame reads it.
UNREADABLE = ("FX-AMAP-022", "The map asset's file is maps/refused/odd_300.amp, 300 bytes: the "
              "drawing, untouched, with MEDIA_DECODE_FAILED at every frame and nothing on "
              "opening; the asset and the setting are kept.",
              case(file="refused/odd_300.amp"))


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = LUT.project_json(fx, LUT.case(file=None, lut="", shift=c["shift"]))
    if c["file"]:
        p["assets"].append({"id": "asset-map", "kind": "lut", "name": Path(c["file"]).stem,
                            "path": f"maps/{c['file']}"})
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.arbitrary_map", "enabled": True,
        "parameters": {"map": c["map"], "phase": setting_json(c["phase"]),
                       "apply_to_alpha": c["apply_to_alpha"]}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "maps" / "refused").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bands.png").write_bytes(S.png(DRAWINGS["bands"]))
    for name, t in MAPS.items():
        (OUT / "maps" / name).write_bytes(bytes(t))
    for name, (data, _) in REFUSED.items():
        (OUT / "maps" / "refused" / name).write_bytes(data)

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if fx in WARNINGS:
            expected["cases"][fx]["warning"] = WARNINGS[fx]
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
    fx, says, c = UNREADABLE
    expected["unreadable"] = {"case": fx, "says": says, "project": write(fx, c),
                              "frames": {"0": plain(c), "4": plain(c)},
                              "on_opening": [], "at_each_frame": "MEDIA_DECODE_FAILED"}
    expected["refused"] = {f"refused/{name}": reason for name, (_, reason) in REFUSED.items()}
    (OUT / "expected_arbitrary_map.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    enc = lambda px: [round(S.linear_to_srgb(v / px[3]) * 255) for v in px[:3]]  # noqa: E731

    # The files read back as their tables, by count; every refused file for its reason.
    assert map_of("master_256.amp") == {"master": GAMMA}
    assert map_of("rgb_768.amp") == {"red": INVERT, "green": HALF, "blue": SINE}
    assert map_of("two_512.amp") == {"master": INVERT, "red": SQUARE}
    assert set(map_of("alpha_1280.amp")) == {"master", "red", "green", "blue", "alpha"}
    for name, (data, reason) in REFUSED.items():
        try:
            read_amp(data)
            raise AssertionError(name + " was read")
        except Refused as r:
            assert str(r) == reason, (name, str(r))
    # Phase cycles whole tables; a whole cycle is no cycle.
    assert [cycled(GAMMA, 64, x) for x in range(256)] == GAMMA[192:] + GAMMA[:192]
    assert [cycled(GAMMA, 256 - 1, x) for x in range(256)] == [cycled(GAMMA, -1, x)
                                                               for x in range(256)]

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if fx not in ("FX-AMAP-006", "FX-AMAP-012", "FX-AMAP-014"):
            for px in frames.values():
                assert all(abs(px[i][3] - drawn[i][3]) < 1e-15 for i in range(W * H)), fx
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values()), fx

    assert like(c["FX-AMAP-001"]["0"], drawn)
    two = c["FX-AMAP-002"]["0"]
    assert enc(two[at(1, 4)]) == [0, 0, 0]
    assert enc(two[at(7, 4)]) == [GAMMA[128]] * 3 == [0xa9] * 3
    assert enc(two[at(11, 4)]) == [GAMMA[58], GAMMA[111], GAMMA[216]] == [0x69, 0x9b, 0xe7]
    three = c["FX-AMAP-003"]["0"]
    assert enc(three[at(13, 4)]) == [0x00, 0xbf, 0xff] and enc(three[at(1, 4)]) == [0xff, 0x40, 0]
    four = c["FX-AMAP-004"]["0"]
    for x in (1, 3, 5, 7, 9, 11, 13):
        src = DRAWINGS["bands"][4][x]
        assert enc(four[at(x, 4)]) == [S_CURVE[POWER[src[0]]], S_CURVE[INVERT[src[1]]],
                                       S_CURVE[LIFT[src[2]]]], x
    assert c["FX-AMAP-005"]["0"] == four
    six = c["FX-AMAP-006"]["0"]
    assert round(six[at(15, 2)][3] * 255) == ROOT[128] == 181
    assert round(six[at(15, 6)][3] * 255) == ROOT[64] == 128
    assert enc(six[at(15, 2)]) == enc(four[at(15, 2)])
    assert all(near(six[i], four[i]) for i in range(W * H) if four[i][3] in (0, 1))
    seven = c["FX-AMAP-007"]["0"]
    src = DRAWINGS["bands"][4][5]
    assert enc(seven[at(5, 4)]) == [INVERT[SQUARE[src[0]]], INVERT[src[1]], INVERT[src[2]]]
    eight = c["FX-AMAP-008"]["0"]
    assert enc(eight[at(1, 4)]) == [GAMMA[192]] * 3 and enc(eight[at(7, 4)]) == [GAMMA[64]] * 3
    nine = c["FX-AMAP-009"]["0"]
    assert not like(nine, three, 1e-3)
    ten = c["FX-AMAP-010"]
    assert ten["0"] == two and ten["4"] == render(case(phase=96), 0)
    assert ten["2"] == render(case(phase=48), 0)
    assert c["FX-AMAP-011"]["0"] == two
    twelve = c["FX-AMAP-012"]["0"]
    assert twelve[at(0, 0)] == [0.0, 0.0, 0.0, ROOT[156] / 255] and ROOT[156] == 199
    assert c["FX-AMAP-013"]["0"] == drawn
    fourteen = c["FX-AMAP-014"]
    assert fourteen["0"] == fourteen["3"]
    for y in range(H):
        assert fourteen["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    assert expected["unreadable"]["frames"]["0"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
