"""Color Grade, worked a fifth way: HSL Secondary's key tools and Show Mask's colours (D-1007).

The fifth unit of `core.color_grade` (D-397, D-398, D-571, D-637), Lumetri Color's panel under
our own name. Lumetri's HSL Secondary picks its key with three eyedroppers (Set, Add and Remove)
or six colour dots, shows the key's ranges on three bars, and shows the key itself three ways:
Color/Gray, Color/Black and White/Black. D-398 gave the key's numbers and White/Black only.
Adobe publishes no maths for any of it, so every rule below is ours; document 14 records it as a
deviation. Nothing is ported.

The word `key_view` gains two values (not keyable):

    key_view                       "off", "mask" (White/Black, D-398's), "color_gray" or
                                   "color_black"; "off"

The rule, Show Mask. With key_view "color_gray" or "color_black" the HSL Secondary step (step 11
of tools/color_grade_2_reference.py, as D-571 refines its key) gives, for the encoded colour e
the step receives and its key m (inverted with key_invert on, refined by key_denoise and
key_blur): e m + g (1 - m), g = 0.5 for "color_gray" and 0 for "color_black", on each channel.
The correction is not shown and the vignette is not drawn, as with "mask". The step runs
whenever the view is not "off".

The rule, the key tools. These change only the eight key numbers, the way typing them would;
they are worked by the panel, so the expected numbers are for the page check. A sample is the
8-bit colour picked from the picture: H its HLS hue in degrees (none for a grey), S its HLS
saturation and L its lightness, per cent. `round` takes halves up, `floor` and `ceil` as usual.

    Set      With a hue: key_hue = round(H) mod 360, key_hue_range = 15, key_hue_softness = 15;
             a grey: key_hue_range = 180. Then key_saturation_low = max(0, round(S) - 20),
             key_saturation_high = min(100, round(S) + 20), and the lightness likewise from L.
             key_softness is kept.
    Add      The ranges grow just enough to take the sample. With key_hue_range below 180: a grey
             makes it 180; a hue H at d = ((H - key_hue + 180) mod 360) - 180 with |d| above the
             range r grows the side towards it, r2 = (r + |d|) / 2 about c2 = key_hue +
             sign(d) (|d| - r) / 2; key_hue = round(c2) mod 360 and key_hue_range =
             min(180, ceil(r2 + |c2 - round(c2)|)). key_saturation_low = min(low, floor(S)),
             key_saturation_high = max(high, ceil(S)), and the lightness likewise.
    Remove   The key shrinks just enough to leave the sample out, in one of its three bands.
             When the key already leaves it out (its key is 0), nothing changes. Otherwise each
             band the sample is in (with its softness) offers a way out. The hue's, with
             key_hue_range r below 180 and s' = max(key_hue_softness, 1), d as Add's (sign 1
             when 0): the near edge goes to E = |d| - s' from key_hue; when E < -r the range is
             0, otherwise r2 = (r + E) / 2 about c2 = key_hue + sign(d) (E - r) / 2, key_hue =
             round(c2) mod 360 and key_hue_range = max(0, floor(r2 - |c2 - round(c2)|)). The
             saturation's, with k' = max(key_softness, 1): when S - low <= high - S, low =
             min(high, ceil(S + k')), otherwise high = max(low, floor(S - k')); the lightness's
             likewise. The way that takes the smallest share of its band away is used (the
             hue's share of r, the others' of high - low, each at least 1); on a tie the hue's,
             then the saturation's, then the lightness's.
    Dots     Red, yellow, green, cyan, blue and magenta: key_hue 0, 60, 120, 180, 240 or 300,
             key_hue_range 30, key_hue_softness 15, the saturation and lightness 0 to 100;
             key_softness kept.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py). The projects go into
`Fixtures/color_grade_5`, the expected frames into
`Fixtures/color_grade_5/expected_color_grade_5.json` and the key tools' numbers into
`Fixtures/color_grade_5/expected_key_tools.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_5_reference.py
"""

import colorsys
import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_grade_2_reference as C2  # noqa: E402
import color_grade_3_reference as C3  # noqa: E402
import color_grade_4_reference as C4  # noqa: E402


W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade_5"
SETTINGS, LISTS, WORDS = C4.SETTINGS, C4.LISTS, C4.WORDS
clamp = C2.clamp
BACKDROP = {"color_gray": 0.5, "color_black": 0.0}
NEUTRAL = dict(secondary_temperature=0, secondary_tint=0, secondary_contrast=0,
               secondary_saturation=100, secondary_wheel_amount=0, secondary_lightness=0,
               secondary_shadow_wheel_amount=0, secondary_shadow_lightness=0,
               secondary_highlight_wheel_amount=0, secondary_highlight_lightness=0,
               vignette_amount=0)


# --- the rule, Show Mask --------------------------------------------------------------------

def color_grade(px, v, look, lists, words):
    view = words["key_view"]
    if view not in BACKDROP:
        return C4.color_grade(px, v, look, lists, words)
    g = BACKDROP[view]
    # e, the colours the secondary receives: every step before it, the secondary changing nothing.
    before = C4.color_grade(px, dict(v, **NEUTRAL), look, lists, dict(words, key_view="off"))
    # m, D-571's key as White/Black shows it.
    grey = C4.color_grade(px, v, look, lists, dict(words, key_view="mask"))
    out = []
    for p, w, k in zip(px, before, grey):
        if p[3] == 0:
            out.append(w)
            continue
        a = w[3]
        e = [S.linear_to_srgb(clamp(w[c] / a)) for c in range(3)]
        m = S.linear_to_srgb(clamp(k[0] / a))
        out.append([srgb_to_linear(clamp(x * m + g * (1 - m))) * a for x in e] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(look="", shift=0, **given):
    for k in given:
        assert k in SETTINGS or k in LISTS or k in WORDS, k
    return {"look": look, "shift": shift, "old": False, "set": given}


def render(c, frame_no):
    lists = {k: c["set"].get(k, d) for k, d in LISTS.items()}
    words = {k: c["set"].get(k, d) for k, d in WORDS.items()}
    return C4.R.frame(color_grade(B.pixels(), C4.values(c, frame_no), c["look"] == "asset-look",
                                  lists, words), c["shift"])


REDS = dict(key_hue=0, key_hue_range=20)

CASES = {
    "FX-GRADE5-001": ("The red key (hue 0, range 20) shown Color/Gray: the reds in their own "
                      "colour, everything else flat 50% grey.",
                      case(key_view="color_gray", **REDS), [0]),
    "FX-GRADE5-002": ("The same key shown Color/Black: the reds in their own colour, everything "
                      "else black.", case(key_view="color_black", **REDS), [0]),
    "FX-GRADE5-003": ("The red key with hue softness 30, Color/Gray: the warm tones part colour, "
                      "part grey, as much colour as the key takes.",
                      case(key_view="color_gray", key_hue_softness=30, **REDS), [0]),
    "FX-GRADE5-004": ("The red key inverted, Color/Black: everything but the reds in colour, the "
                      "reds black.",
                      case(key_view="color_black", key_invert="on", **REDS), [0]),
    "FX-GRADE5-005": ("Color/Gray with a correction set (secondary saturation 0) and a dark "
                      "vignette: neither shows, only the colours going in and the key.",
                      case(key_view="color_gray", secondary_saturation=0, vignette_amount=-3,
                           **REDS), [0]),
    "FX-GRADE5-006": ("The red key with key blur 20, Color/Black: the key softened as D-571's "
                      "White/Black shows it, the colour fading out with it.",
                      case(key_view="color_black", key_blur=20, **REDS), [0]),
    "FX-GRADE5-007": ("A skin key (hue 25, range 10, softness 20; saturation 20 to 80, "
                      "lightness 30 to 90, softness 10), Color/Gray, after a warmer white "
                      "balance (temperature 30): the colours going in are the warmer ones.",
                      case(key_view="color_gray", key_hue=25, key_hue_range=10,
                           key_hue_softness=20, key_saturation_low=20, key_saturation_high=80,
                           key_lightness_low=30, key_lightness_high=90, key_softness=10,
                           temperature=30), [0]),
    "FX-GRADE5-008": ("The red key shown White/Black (\"mask\", D-398's word, unchanged): D-571's "
                      "own picture.", case(key_view="mask", **REDS), [0]),
}

INVALID = {
    "FX-GRADE5-009": ("Show Key \"color\", not one of the four words.",
                      case(key_view="color", **REDS)),
}


# --- the rule, the key tools ----------------------------------------------------------------

KEY = ["key_hue", "key_hue_range", "key_hue_softness", "key_saturation_low",
       "key_saturation_high", "key_lightness_low", "key_lightness_high", "key_softness"]
AS_ADDED = {k: SETTINGS[k][2] for k in KEY}
DOTS = {"red": 0, "yellow": 60, "green": 120, "cyan": 180, "blue": 240, "magenta": 300}


def rnd(x):
    return math.floor(x + 0.5)


def sample(hex_):
    e = [int(hex_[i:i + 2], 16) / 255 for i in (1, 3, 5)]
    h, l, s = colorsys.rgb_to_hls(*e)
    return (None if max(e) == min(e) else h * 360), min(s, 1.0) * 100, l * 100


def signed(h, c):
    return ((h - c + 180) % 360) - 180


def tool(name, k, given):
    k = dict(k)
    if name == "dot":
        k.update(key_hue=DOTS[given], key_hue_range=30, key_hue_softness=15,
                 key_saturation_low=0, key_saturation_high=100, key_lightness_low=0,
                 key_lightness_high=100)
        return k
    hue, sat, light = sample(given)
    pairs = (("key_saturation_low", "key_saturation_high", sat),
             ("key_lightness_low", "key_lightness_high", light))
    if name == "set":
        if hue is None:
            k["key_hue_range"] = 180
        else:
            k.update(key_hue=rnd(hue) % 360, key_hue_range=15, key_hue_softness=15)
        for lo, hi, x in pairs:
            k[lo], k[hi] = max(0, rnd(x) - 20), min(100, rnd(x) + 20)
        return k
    r, c = k["key_hue_range"], k["key_hue"]
    if name == "add":
        if r < 180 and hue is None:
            k["key_hue_range"] = 180
        elif r < 180:
            d = signed(hue, c)
            if abs(d) > r:
                r2 = (r + abs(d)) / 2
                c2 = c + math.copysign(1, d) * (abs(d) - r) / 2
                k["key_hue"] = rnd(c2) % 360
                k["key_hue_range"] = min(180, math.ceil(r2 + abs(c2 - rnd(c2))))
        for lo, hi, x in pairs:
            k[lo], k[hi] = min(k[lo], math.floor(x)), max(k[hi], math.ceil(x))
        return k
    assert name == "remove"
    if key_of(k, given) == 0.0:
        return k
    # Each way of leaving the sample out, with the share of its band it takes away; the
    # cheapest is used, hue first, then saturation, then lightness, on a tie.
    ways = []
    if r < 180 and hue is not None:
        d = signed(hue, c)
        s1 = max(k["key_hue_softness"], 1)
        if abs(d) < r + s1:
            side = -1 if d < 0 else 1
            e = abs(d) - s1
            if e < -r:
                hue_to = {"key_hue_range": 0}
            else:
                r2 = (r + e) / 2
                c2 = c + side * (e - r) / 2
                hue_to = {"key_hue": rnd(c2) % 360,
                          "key_hue_range": max(0, math.floor(r2 - abs(c2 - rnd(c2))))}
            ways.append(((r - hue_to["key_hue_range"]) / max(r, 1), hue_to))
    k1 = max(k["key_softness"], 1)
    for lo, hi, x in pairs:
        low, high = k[lo], k[hi]
        if low - k["key_softness"] < x < high + k["key_softness"] or low <= x <= high:
            if x - low <= high - x:
                to = {lo: min(high, math.ceil(x + k1))}
            else:
                to = {hi: max(low, math.floor(x - k1))}
            moved = abs(next(iter(to.values())) - k[next(iter(to))])
            ways.append((moved / max(high - low, 1), to))
    if ways:
        k.update(min(ways, key=lambda w: w[0])[1])
    return k


ORANGE, SKY, SKIN, GREY, LEAF = "#e07020", "#4080d0", "#d8a080", "#808080", "#3c8c30"
SET_SKIN = tool("set", AS_ADDED, SKIN)

TOOLS = [
    ("Set on an orange: hue 25, range 15, softness 15, its saturation and lightness 20 either "
     "side.", "set", AS_ADDED, ORANGE),
    ("Set on a sky blue.", "set", AS_ADDED, SKY),
    ("Set on a grey: every hue (range 180), its lightness 20 either side, saturation 0 to 20.",
     "set", AS_ADDED, GREY),
    ("Set on a skin tone.", "set", AS_ADDED, SKIN),
    ("Add an orange to the skin key: the hue range grows towards it, the saturation's high end up "
     "to take it.", "add", SET_SKIN, ORANGE),
    ("Add a colour already inside the skin key: nothing changes.", "add", SET_SKIN, SKIN),
    ("Add a grey to the skin key: every hue (range 180), the saturation's low end down to 0.",
     "add", SET_SKIN, GREY),
    ("Add a sky blue to an orange key: past half a turn the other way, so the range grows the "
     "short way round.", "add", tool("set", AS_ADDED, ORANGE), SKY),
    ("Remove a paler skin tone from the skin key: the saturation's high end comes down past it, "
     "the cheapest way out (a smaller share than the lightness's).",
     "remove", SET_SKIN, "#f0c8b0"),
    ("Remove an orange from a wide warm key (hue 20, range 40, softness 10) whose saturation "
     "(0 to 100) is the cheapest band to trim: its high end comes down below the orange's.",
     "remove", dict(AS_ADDED, key_hue=20, key_hue_range=40, key_hue_softness=10), ORANGE),
    ("Remove an orange from the same warm key with narrow saturation (72 to 78) and lightness "
     "(45 to 55): now the hue is the cheapest, its side towards the orange shrinks.", "remove",
     dict(AS_ADDED, key_hue=20, key_hue_range=40, key_hue_softness=10, key_saturation_low=72,
          key_saturation_high=78, key_lightness_low=45, key_lightness_high=55), ORANGE),
    ("Remove a leaf green from the skin key: the key leaves it out already, so nothing changes.",
     "remove", SET_SKIN, LEAF),
    ("Remove a duller skin tone from the skin key: the saturation's low end comes up past it, "
     "the cheapest way out.", "remove", SET_SKIN, "#d0a890"),
    ("The red dot: hue 0, range 30, softness 15, saturation and lightness 0 to 100.",
     "dot", AS_ADDED, "red"),
    ("The cyan dot after the skin key: the key softness kept.", "dot",
     dict(SET_SKIN, key_softness=10), "cyan"),
]


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(C4.G.look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade_5", CASES, INVALID, render, B.plain,
                             C4.file_json)
    tools = {"rule": "D-1007", "cases": [
        {"says": says, "tool": name, "picked": given, "from": k, "to": tool(name, k, given)}
        for says, name, k, given in TOOLS]}
    (OUT / "expected_key_tools.json").write_text(json.dumps(tools, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected, tools)


def key_of(k, hex_):
    e = [int(hex_[i:i + 2], 16) / 255 for i in (1, 3, 5)]
    return C2.key(e, dict(k), False)


def check(expected, tools):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = B.plain(case())
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    grey, black = srgb_to_linear(0.5), 0.0
    keyed = C4.render(C4.case(key_view="mask", **REDS), 0)
    for i, (p, k) in enumerate(zip(drawn, keyed)):
        if p[3] == 0:
            continue
        m = k[0] / p[3]
        if m > 1 - 1e-9:
            assert near(c["FX-GRADE5-001"][i], p) and near(c["FX-GRADE5-002"][i], p)
            assert near(c["FX-GRADE5-004"][i][:3], [black] * 3)
        elif m < 1e-9:
            assert near(c["FX-GRADE5-001"][i][:3], [grey * p[3]] * 3)
            assert near(c["FX-GRADE5-002"][i][:3], [black] * 3)
            assert near(c["FX-GRADE5-004"][i], p)
    assert like(c["FX-GRADE5-005"], c["FX-GRADE5-001"])
    assert like(c["FX-GRADE5-008"], keyed)
    for fx in ("001", "002", "003", "006", "007"):
        assert not like(c[f"FX-GRADE5-{fx}"], drawn), fx
    assert not like(c["FX-GRADE5-003"], c["FX-GRADE5-001"])
    assert not like(c["FX-GRADE5-006"], c["FX-GRADE5-002"])
    assert not like(c["FX-GRADE5-007"],
                    render(case(**dict(expected_set(), temperature=0)), 0))
    # the tools: Set and Add take the sample wholly, Remove leaves it out, a dot keys its hue
    for t in tools["cases"]:
        k, to, hx = t["from"], t["to"], t["picked"]
        if t["tool"] in ("set", "add"):
            assert key_of(to, hx) == 1.0, t["says"]
        if t["tool"] == "add":
            assert all(to[n] <= k[n] for n in ("key_saturation_low", "key_lightness_low"))
        if t["tool"] == "remove":
            assert key_of(to, hx) == 0.0, t["says"]
        if t["tool"] == "dot":
            assert to["key_softness"] == k["key_softness"]
        assert all(0 <= to[n] <= SETTINGS[n][1] for n in KEY), t["says"]
    for n in (5, 11):
        assert tools["cases"][n]["to"] == tools["cases"][n]["from"]
    print("checked")


def expected_set():
    return dict(key_view="color_gray", key_hue=25, key_hue_range=10, key_hue_softness=20,
                key_saturation_low=20, key_saturation_high=80, key_lightness_low=30,
                key_lightness_high=90, key_softness=10)


if __name__ == "__main__":
    main()
