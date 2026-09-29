"""Color Lookup, worked a second way.

D-182 adds `core.color_lookup` and a new kind of asset, `lut`: a colour lookup file in the .cube
format that colourists and grading programs trade, a table of what each colour becomes. The effect
has one setting, `lut`, the id of a `lut` asset of the project, or "" for none; the project keeps
the file where it is, as it keeps a drawing, so Collect Files copies it and a missing one is
reported and kept, never dropped. It is this program's own method, written from the published
.cube format; nothing is ported. Document 21 is the colour rule in words and document 19 the
reading rule; this file is the reference for the numbers document 25 pins against them.

Reading a .cube file. It is text. A leading byte-order mark is skipped. Lines end in LF or CRLF;
each is trimmed, and blank lines and lines starting with # are skipped. A line starting with a
letter is a keyword line, any other a table line. The keywords are TITLE (the rest of the line is
ignored), LUT_3D_SIZE N (a whole number 2 to 256), LUT_1D_SIZE N (2 to 65536), DOMAIN_MIN r g b
and DOMAIN_MAX r g b (three numbers each, 0 0 0 and 1 1 1 when not given), and
LUT_1D_INPUT_RANGE lo hi or LUT_3D_INPUT_RANGE lo hi (the domain of every channel at once). Each
keyword comes at most once and before the table. A table line is three finite numbers. A 3D
table has N x N x N lines with red changing fastest, then green, then blue; a 1D table has N lines.
Anything else refuses the whole file with the reason `read_cube` gives, which the build must give
word for word: an unknown keyword, a keyword after the table has begun or given twice, a size or
domain that is not the numbers it must be, both sizes (a 1D table before a 3D one, as some grading
programs write; a known limit), no size, the domain given more than one way, a domain whose top
is not above its bottom, a table line that is not three numbers, and the wrong number of lines.

The colour rule. At a pixel with covering a > 0, e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) is
its encoded straight colour. For each channel, u = (clamp(e, lo, hi) - lo) / (hi - lo) * (N - 1),
i = min(floor(u), N - 2) and f = u - i. A 3D table gives the trilinear mix of the eight lines
around (i_r, i_g, i_b), each weighted by the product of f or 1 - f in each channel; a 1D table
gives, for each channel alone, its own column at i and i + 1 mixed by f. The output channel is
srgb_to_linear(clamp(out, 0, 1)) * a, and the covering is kept; a pixel with a = 0 is left as it
is. The lookup works on the encoded sRGB colour, which is what a creative look made for sRGB
pictures expects; a camera's log lookup does not suit it (a known limit). The layer does not grow.
A `lut` naming no `lut` asset of the project, a missing file and a file that cannot be read each
leave the layer as it is, with EFFECT_PARAMETER_INVALID, MEDIA_MISSING and MEDIA_DECODE_FAILED.

**This file never runs the build's code path.** It works in double precision on lists, from the
file's numbers as written, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, the bands of
`tools/invert_reference.py`, unmoved unless the case says. The drawing goes into
`Fixtures/cube_lut/media`, the lookup files into `Fixtures/cube_lut/luts` (the refused ones into
`luts/refused`), the projects into `Fixtures/cube_lut`, and the expected frames into
`Fixtures/cube_lut/expected_color_lookup.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cube_lut_reference.py
"""

import json
import math
import string
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from invert_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cube_lut"
TOLERANCE = 2e-5  # document 25's default for a filter
SIZES = {"LUT_3D_SIZE": 256, "LUT_1D_SIZE": 65536}
KEYWORDS = ("TITLE", "LUT_3D_SIZE", "LUT_1D_SIZE", "DOMAIN_MIN", "DOMAIN_MAX",
            "LUT_1D_INPUT_RANGE", "LUT_3D_INPUT_RANGE")


# --- reading a .cube file -------------------------------------------------------------------

class Refused(Exception):
    pass


def finite(tokens, k):
    """`k` finite numbers, or None."""
    if len(tokens) != k:
        return None
    try:
        v = [float(t) for t in tokens]
    except ValueError:
        return None
    return v if all(math.isfinite(x) for x in v) else None


def read_cube(data):
    """(dimensions, N, lo, hi, table) from a file's bytes, or Refused with the reason."""
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        raise Refused("the file is not text")
    if text.startswith("\ufeff"):
        text = text[1:]
    seen, table = set(), []
    size = {}
    lo, hi = [0.0] * 3, [1.0] * 3
    for n, line in enumerate(text.split("\n"), 1):
        t = line.strip()
        if not t or t.startswith("#"):
            continue
        tokens = t.split()
        if t[0] not in string.ascii_letters:
            v = finite(tokens, 3)
            if v is None:
                raise Refused(f"line {n}: a table line must be three numbers")
            table.append(v)
            continue
        word, rest = tokens[0], tokens[1:]
        if table:
            raise Refused(f"line {n}: {word} comes after the table has begun")
        if word not in KEYWORDS:
            raise Refused(f"line {n}: {word} is not a keyword of a .cube file this program reads")
        if word in seen:
            raise Refused(f"line {n}: {word} is given twice")
        seen.add(word)
        if word in SIZES:
            top = SIZES[word]
            ok = (len(rest) == 1 and rest[0].isascii() and rest[0].isdigit()
                  and 2 <= int(rest[0]) <= top)
            if not ok:
                raise Refused(f"line {n}: {word} must be one whole number from 2 to {top}")
            size[word] = int(rest[0])
        elif word in ("DOMAIN_MIN", "DOMAIN_MAX"):
            v = finite(rest, 3)
            if v is None:
                raise Refused(f"line {n}: {word} must be three numbers")
            if word == "DOMAIN_MIN":
                lo = v
            else:
                hi = v
        elif word != "TITLE":
            v = finite(rest, 2)
            if v is None:
                raise Refused(f"line {n}: {word} must be two numbers")
            lo, hi = [v[0]] * 3, [v[1]] * 3
    if len(size) == 2:
        raise Refused("the file gives both LUT_1D_SIZE and LUT_3D_SIZE, a 1D table before a 3D "
                      "one, which this program does not read")
    if not size:
        raise Refused("the file gives no LUT_3D_SIZE or LUT_1D_SIZE")
    ways = (("DOMAIN_MIN" in seen or "DOMAIN_MAX" in seen) + ("LUT_1D_INPUT_RANGE" in seen)
            + ("LUT_3D_INPUT_RANGE" in seen))
    if ways > 1:
        raise Refused("the file gives its domain more than one way")
    if any(h <= l for l, h in zip(lo, hi)):
        raise Refused("the domain's top must be above its bottom in each colour")
    (word, n), = size.items()
    dims = 3 if word == "LUT_3D_SIZE" else 1
    need = n ** dims
    if len(table) != need:
        raise Refused(f"the table has {len(table)} lines where {word} {n} needs {need}")
    return dims, n, lo, hi, table


# --- the rule -------------------------------------------------------------------------------

def lookup(cube, e):
    """What the table makes of the encoded straight colour `e`."""
    dims, n, lo, hi, table = cube
    at = []
    for c in range(3):
        u = (min(hi[c], max(lo[c], e[c])) - lo[c]) / (hi[c] - lo[c]) * (n - 1)
        i = min(math.floor(u), n - 2)
        at.append((i, u - i))
    if dims == 1:
        return [table[i][c] * (1 - f) + table[i + 1][c] * f for c, (i, f) in enumerate(at)]
    (ri, rf), (gi, gf), (bi, bf) = at
    out = [0.0] * 3
    for db in (0, 1):
        for dg in (0, 1):
            for dr in (0, 1):
                w = ((rf if dr else 1 - rf) * (gf if dg else 1 - gf) * (bf if db else 1 - bf))
                v = table[(ri + dr) + (gi + dg) * n + (bi + db) * n * n]
                for c in range(3):
                    out[c] += w * v[c]
    return out


def color_lookup(pixels, cube):
    """The drawing's 8-bit pixels through the table, as working values; `cube` None leaves
    them as they are."""
    out = [R.working(p) for p in pixels]
    if cube is None:
        return out
    for w in out:
        a = w[3]
        if a > 0:
            e = [S.linear_to_srgb(min(1.0, max(0.0, w[c] / a))) for c in range(3)]
            w[:3] = [srgb_to_linear(min(1.0, max(0.0, v))) * a for v in lookup(cube, e)]
    return out


# --- the lookup files -----------------------------------------------------------------------

def table_text(dims, n, f, fmt="{:.6f}"):
    """The table lines of `f` over an N-point grid, red fastest."""
    grid = [i / (n - 1) for i in range(n)]
    if dims == 1:
        points = [(x, x, x) for x in grid]
    else:
        points = [(r, g, b) for b in grid for g in grid for r in grid]
    return ["{} {} {}".format(*(fmt.format(v) for v in f(*p))) for p in points]


def warm(r, g, b):
    """A warm look: a gentle S-curve for contrast, red lifted and blue held down, and a touch
    less saturated."""
    s = lambda x: 0.65 * x + 0.35 * x * x * (3 - 2 * x)  # noqa: E731
    r, g, b = 0.06 + 0.94 * s(r), 0.035 + 0.92 * s(g), 0.015 + 0.82 * s(b)
    lum = 0.2126 * r + 0.7152 * g + 0.0722 * b
    return [lum + 0.9 * (c - lum) for c in (r, g, b)]


def cool(r, g, b):
    """A cool look whose channels mix, so only a 3D table holds it, with a value below 0 and
    one above 1 for the clamp."""
    return [0.85 * r * r + 0.05 * g - 0.05 * (1 - g) * (1 - b), 0.9 * g + 0.05 * b,
            0.1 + 0.8 * b + 0.2 * r * g]


def identity(r, g, b):
    return [r, g, b]


def cool_text():
    """3 points a side, written as a grading program might: CRLF, a title, comments, blank lines,
    a tab, and numbers in short and exponent forms."""
    lines = table_text(3, 3, cool, "{:.6g}")
    lines[0] = "-5e-2 0 1e-1"
    lines[4] = lines[4].replace(" ", "\t", 1)
    body = (["# Created for the Color Lookup fixtures", "TITLE \"Cool\"", "", "LUT_3D_SIZE 3", ""]
            + lines[:13] + ["# comments may come between table lines", ""] + lines[13:])
    return "\r\n".join(body) + "\r\n"


LUTS = {
    "identity_2.cube": "LUT_3D_SIZE 2\n" + "\n".join(table_text(3, 2, identity)) + "\n",
    "warm_17.cube": ("TITLE \"Warm\"\nLUT_3D_SIZE 17\n"
                     + "\n".join(table_text(3, 17, warm)) + "\n"),
    "cool_3.cube": cool_text(),
    "tint_1d.cube": ("TITLE \"Tint\"\nLUT_1D_SIZE 4\n0.080000 0.050000 0.030000\n"
                     "0.420000 0.330000 0.240000\n0.780000 0.660000 0.520000\n"
                     "1.000000 0.940000 0.800000\n"),
    "domain.cube": ("LUT_3D_SIZE 2\nDOMAIN_MIN 0.25 0.2 0.1\nDOMAIN_MAX 0.75 0.9 1.0\n"
                    + "\n".join(table_text(3, 2, identity)) + "\n"),
    "range_1d.cube": "LUT_1D_SIZE 2\nLUT_1D_INPUT_RANGE 0 0.5\n0 0 0\n1 1 1\n",
}

EIGHT = "\n".join(table_text(3, 2, identity)) + "\n"
REFUSED = {
    "count.cube": ("LUT_3D_SIZE 2\n" + "\n".join(table_text(3, 2, identity)[:7]) + "\n",
                   "the table has 7 lines where LUT_3D_SIZE 2 needs 8"),
    "both.cube": ("LUT_1D_SIZE 2\nLUT_3D_SIZE 2\n0 0 0\n1 1 1\n" + EIGHT,
                  "the file gives both LUT_1D_SIZE and LUT_3D_SIZE, a 1D table before a 3D one, "
                  "which this program does not read"),
    "no_size.cube": (EIGHT, "the file gives no LUT_3D_SIZE or LUT_1D_SIZE"),
    "empty.cube": ("", "the file gives no LUT_3D_SIZE or LUT_1D_SIZE"),
    "word.cube": ("LUT_3D_SIZE 2\n0 0 0\n1 0 0\n0.5 red 0.25\n",
                  "line 4: a table line must be three numbers"),
    "four.cube": ("LUT_3D_SIZE 2\n0 0 0 0\n", "line 2: a table line must be three numbers"),
    "nan.cube": ("LUT_3D_SIZE 2\n0 nan 0\n", "line 2: a table line must be three numbers"),
    "late.cube": ("LUT_3D_SIZE 2\n0 0 0\nDOMAIN_MIN 0 0 0\n",
                  "line 3: DOMAIN_MIN comes after the table has begun"),
    "unknown.cube": ("LUT_3D_SIZE 2\nLUT_4D_SIZE 2\n" + EIGHT,
                     "line 2: LUT_4D_SIZE is not a keyword of a .cube file this program reads"),
    "twice.cube": ("LUT_3D_SIZE 2\nLUT_3D_SIZE 2\n" + EIGHT, "line 2: LUT_3D_SIZE is given twice"),
    "big.cube": ("LUT_3D_SIZE 300\n", "line 1: LUT_3D_SIZE must be one whole number from 2 to 256"),
    "half.cube": ("LUT_3D_SIZE 2.5\n", "line 1: LUT_3D_SIZE must be one whole number from 2 to 256"),
    "domain_order.cube": ("LUT_3D_SIZE 2\nDOMAIN_MIN 0.5 0.5 0.5\nDOMAIN_MAX 0.5 0.8 0.8\n" + EIGHT,
                          "the domain's top must be above its bottom in each colour"),
    "domain_short.cube": ("LUT_3D_SIZE 2\nDOMAIN_MAX 1 1\n" + EIGHT,
                          "line 2: DOMAIN_MAX must be three numbers"),
    "domain_twice.cube": ("LUT_3D_SIZE 2\nDOMAIN_MIN 0 0 0\nLUT_3D_INPUT_RANGE 0 1\n" + EIGHT,
                          "the file gives its domain more than one way"),
    "range_short.cube": ("LUT_1D_SIZE 2\nLUT_1D_INPUT_RANGE 0\n0 0 0\n1 1 1\n",
                         "line 2: LUT_1D_INPUT_RANGE must be two numbers"),
}
NOT_TEXT = ("not_text.cube", b"LUT_3D_SIZE 2\n\xff\xfe\n", "the file is not text")


def cube_of(name):
    return read_cube((OUT / "luts" / name).read_bytes())


# --- the cases ------------------------------------------------------------------------------

def case(file="warm_17.cube", lut="asset-lut", shift=0, adjust=False):
    """`file` is the lookup asset's file under luts/ (None: the project has no lookup asset),
    `lut` the effect's setting, and `adjust` puts the effect on an adjustment layer above the
    drawing instead of on it."""
    return {"file": file, "lut": lut, "shift": shift, "adjust": adjust}


def render(c):
    pixels = [p for row in DRAWINGS["bands"] for p in row]
    cube = None
    if c["lut"] == "asset-lut" and c["file"] and (OUT / "luts" / c["file"]).exists():
        cube = cube_of(c["file"])
    return R.frame(color_lookup(pixels, cube), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(file=None, lut="", shift=c["shift"]))


CASES = {
    "FX-LUT-001": ("identity_2.cube, a 3D table of 2 points a side that gives each colour back: "
                   "the drawing, unchanged but for rounding.",
                   case(file="identity_2.cube"), [0]),
    "FX-LUT-002": ("warm_17.cube, a warm look in a 3D table of 17 points a side, the size grading "
                   "programs write: more contrast, white turned cream #fef4d8, black lifted to a "
                   "dark brown #0f0904, the blue #3a6fd8 a duller #426cb3; each colour is the mix "
                   "of the eight table lines around it.",
                   case(), [0]),
    "FX-LUT-003": ("cool_3.cube, a 3D table of 3 points a side written with Windows line ends, a "
                   "title, comments between its lines, a tab and numbers such as -5e-2: read the "
                   "same as any other; its red channel mixes in green, so only a 3D table holds "
                   "it, and a value below 0 or above 1 is clamped to 0 or 1 after the mix: white "
                   "turns a pale blue #e5f2ff and black a navy #00001a.",
                   case(file="cool_3.cube"), [0]),
    "FX-LUT-004": ("tint_1d.cube, a 1D table of 4 lines: each channel through its own curve, "
                   "black to a dark brown #140d08 (the first line, 0.08 0.05 0.03) and white to a "
                   "cream #fff0cc (the last), between them mixed from the two nearest lines.",
                   case(file="tint_1d.cube"), [0]),
    "FX-LUT-005": ("domain.cube, a 3D identity table over the domain 0.25 to 0.75 for red, 0.2 "
                   "to 0.9 for green and 0.1 to 1 for blue: each channel below its domain turns "
                   "0 and above it 1, and between is stretched to fill 0 to 1, so the picture "
                   "gains contrast, most in red.",
                   case(file="domain.cube"), [0]),
    "FX-LUT-006": ("range_1d.cube, a 1D table of 2 lines over the input range 0 to 0.5: each "
                   "channel doubles, and a channel above one half turns 1.",
                   case(file="range_1d.cube"), [0]),
    "FX-LUT-007": ("No lookup file chosen, the effect as it is added: the drawing, untouched, "
                   "with no warning.",
                   case(lut=""), [0]),
    "FX-LUT-008": ("FX-LUT-002 moved three pixels right: the same, moved.",
                   case(shift=3), [0, 3]),
    "FX-LUT-009": ("The lookup asset's file luts/gone.cube is not there: the drawing, "
                   "untouched, with MEDIA_MISSING when the file is opened and at every frame; "
                   "the asset and the setting are kept, to relink.",
                   case(file="gone.cube"), [0]),
    "FX-LUT-010": ("The setting names asset-nothing, which the project does not have: the "
                   "drawing, untouched, with EFFECT_PARAMETER_INVALID; the setting is kept as "
                   "written.",
                   case(lut="asset-nothing"), [0]),
    "FX-LUT-011": ("The setting names asset-bands, the drawing, which is not a lookup file: the "
                   "drawing, untouched, with EFFECT_PARAMETER_INVALID.",
                   case(lut="asset-bands"), [0]),
    "FX-LUT-012": ("FX-LUT-002's effect on an adjustment layer above the drawing instead of on "
                   "it: the same picture, as the drawing over nothing is the drawing.",
                   case(adjust=True), [0]),
}
WARNINGS = {"FX-LUT-009": "MEDIA_MISSING", "FX-LUT-010": "EFFECT_PARAMETER_INVALID",
            "FX-LUT-011": "EFFECT_PARAMETER_INVALID"}

# Checked by the test on its own, as opening the file cannot know: a lookup file that is there but
# cannot be read is found when a frame reads it.
UNREADABLE = ("FX-LUT-013", "The lookup asset's file is luts/refused/count.cube, one line short: "
              "the drawing, untouched, with MEDIA_DECODE_FAILED at every frame and nothing on "
              "opening; the asset and the setting are kept.",
              case(file="refused/count.cube"))


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": "bands", "shift": c["shift"], "softness": 0,
                            "threshold": 0})
    p["assets"][0]["path"] = "media/bands.png"
    if c["file"]:
        p["assets"].append({"id": "asset-lut", "kind": "lut", "name": Path(c["file"]).stem,
                            "path": f"luts/{c['file']}"})
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    art = comp["layers"][0]
    t = art["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = [{"instance_id": "fx-0-0", "type_id": "core.color_lookup", "enabled": True,
                "parameters": {"lut": c["lut"]}}]
    if c["adjust"]:
        adjust = {k: v for k, v in art.items()
                  if k not in ("asset_id", "exposure_spans", "source_offset_frames")}
        adjust.update(id="adjust", name="adjust", kind="adjustment", effects=effects)
        art["effects"] = []
        comp["layers"].append(adjust)
        comp["layer_order"].append("adjust")
    else:
        art["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "refused").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bands.png").write_bytes(S.png(DRAWINGS["bands"]))
    for name, text in LUTS.items():
        (OUT / "luts" / name).write_bytes(text.encode("ascii"))
    for name, (text, _) in REFUSED.items():
        (OUT / "luts" / "refused" / name).write_bytes(text.encode("ascii"))
    (OUT / "luts" / "refused" / NOT_TEXT[0]).write_bytes(NOT_TEXT[1])

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if fx in WARNINGS:
            expected["cases"][fx]["warning"] = WARNINGS[fx]
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    fx, says, c = UNREADABLE
    expected["unreadable"] = {"case": fx, "says": says, "project": write(fx, c),
                              "frames": {"0": plain(c), "4": plain(c)},
                              "on_opening": [], "at_each_frame": "MEDIA_DECODE_FAILED"}
    expected["refused"] = {f"refused/{name}": reason for name, (_, reason) in REFUSED.items()}
    expected["refused"][f"refused/{NOT_TEXT[0]}"] = NOT_TEXT[2]
    expected["collect"] = {
        "says": "Collect Files on FX-LUT-002 copies the lookup file beside the drawing, under its "
                "asset's folder, lists it in the manifest as a lookup file used by the layer "
                "whose effect names it, and the collected project names the copy; on FX-LUT-009 "
                "the missing file is listed as missing and the asset is kept.",
        "project": "fx_lut_002.json",
        "copied": {"media/asset-bands/bands.png": "copied",
                   "media/asset-lut/warm_17.cube": "copied"},
        "kind": "lut",
        "used_by": [{"composition": "comp-main", "layer": "art"}],
        "missing_project": "fx_lut_009.json",
        "missing": {"media/asset-lut/gone.cube": "missing"},
    }
    (OUT / "expected_color_lookup.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    src = [p for row in DRAWINGS["bands"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    enc = lambda px: [round(S.linear_to_srgb(v / px[3]) * 255) for v in px[:3]]  # noqa: E731

    # The files read back as written, and every refused file is refused for its reason.
    for name, text in LUTS.items():
        cube = read_cube(text.encode("ascii"))
        assert cube[0] == (1 if "1d" in name else 3), name
    assert cube_of("cool_3.cube")[4][0] == [-0.05, 0.0, 0.1]
    assert b"\r\n" in (OUT / "luts" / "cool_3.cube").read_bytes()
    for name, (text, reason) in REFUSED.items():
        try:
            read_cube(text.encode("ascii"))
            raise AssertionError(name + " was read")
        except Refused as r:
            assert str(r) == reason, (name, str(r))
    try:
        read_cube(NOT_TEXT[1])
        raise AssertionError("not text was read")
    except Refused as r:
        assert str(r) == NOT_TEXT[2]
    assert read_cube(b"\xef\xbb\xbf" + LUTS["identity_2.cube"].encode()) == cube_of("identity_2.cube")

    # A 1D table is each channel through its own curve; a 3D table of a line is that line; the
    # rule leaves every covering, and every empty pixel, as it was.
    tint = cube_of("tint_1d.cube")
    for e in (0.0, 0.1, 1 / 3, 0.5, 0.9, 1.0):
        u = e * 3
        i = min(math.floor(u), 2)
        want = [tint[4][i][k] * (1 - (u - i)) + tint[4][i + 1][k] * (u - i) for k in range(3)]
        assert near(lookup(tint, [e] * 3), want)
    ident = cube_of("identity_2.cube")
    for e in ([0.1, 0.5, 0.9], [0.0, 1.0, 0.25], [0.3, 0.3, 0.3]):
        assert near(lookup(ident, e), e)
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-LUT-008" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert abs(px[i][3] - base[i][3]) < 1e-15, (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)

    assert like(c["FX-LUT-001"]["0"], drawn)
    two = c["FX-LUT-002"]["0"]
    assert not like(two, drawn, 1e-3)
    assert enc(two[at(13, 4)]) == [0xfe, 0xf4, 0xd8], enc(two[at(13, 4)])
    assert enc(two[at(1, 4)]) == [0x0f, 0x09, 0x04]     # black lifted
    assert enc(two[at(11, 4)]) == [0x42, 0x6c, 0xb3]
    assert enc(c["FX-LUT-003"]["0"][at(13, 4)]) == [0xe5, 0xf2, 0xff]
    three = c["FX-LUT-003"]["0"]
    assert enc(three[at(13, 4)])[2] == 255              # 1.1 clamped
    assert enc(three[at(1, 4)]) == [0, 0, round(0.1 * 255)]  # -0.05 clamped, 0.1 blue
    four = c["FX-LUT-004"]["0"]
    assert enc(four[at(1, 4)]) == [0x14, 0x0d, 0x08] and enc(four[at(13, 4)]) == [0xff, 0xf0, 0xcc]
    five = c["FX-LUT-005"]["0"]
    assert enc(five[at(1, 4)]) == [0, 0, 0] and enc(five[at(13, 4)]) == [255, 255, 255]
    assert enc(five[at(5, 4)])[0] == 255                # red 200/255 is above 0.75
    six = c["FX-LUT-006"]["0"]
    for x in (1, 3, 5, 7, 9, 11, 13):
        want = [min(255, round(2 * v)) for v in src[at(x, 4)][:3]]
        assert all(abs(p - q) <= 1 for p, q in zip(enc(six[at(x, 4)]), want)), x
    assert c["FX-LUT-007"]["0"] == drawn
    eight = c["FX-LUT-008"]
    assert eight["0"] == eight["3"]
    for y in range(H):
        assert eight["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    for fx in ("FX-LUT-009", "FX-LUT-010", "FX-LUT-011"):
        assert c[fx]["0"] == drawn
    assert c["FX-LUT-012"]["0"] == two
    assert expected["unreadable"]["frames"]["0"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
