"""Energy Stroke's extras, worked a second way.

D-565 adds to D-508's `core.energy_stroke` the rest of P0-22's Energy Stroke row
(docs/effects/EFFECTS.md row 53): the shape of each end, the noise fixed on the layer instead of
travelling along the path, and masks cutting the core or the glow. They are after the purpose of
Video Copilot's Saber's Start Roundness and End Roundness, Lock Noise to Saber, and Alpha Mode with
Invert Masks, as docs/effects/PLUGINS.md section 2.4 names them; no source publishes a formula or
a setting when added, so the rule below and every setting when added are this program's own.
Nothing is ported. A core from two points (`source` "points", `core_start`, `core_end`) and the
size at each end (`start_size`, `end_size`) were built by D-508 and are unchanged. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it. It
imports D-508's reference (tools/energy_stroke_reference.py) and changes nothing in it.

The settings, each with the value when added that draws exactly as D-508 drew, so a file from
before is drawn and written as it was:

- `start_roundness` and `end_roundness`, 0 to 100, 100 when added, keyable: the shape of the
  core at the stretch's first and last end. 100 is D-508's round end, 0 a flat end cut square
  across the beam at the end's point.
- `noise_along_path`, "on" (when added) or "off": with it on the distortion's noise is read along
  the path as D-508 reads it, so it travels with the beam; off, it is read where each cut of the
  path lies on the layer, so it stays fixed on the layer and the beam moves through it.
- `alpha_mode`, "off" (when added), "masks", "core" or "glow": what the layer's masks cut. Off
  cuts nothing; masks cuts the core and the glow; core cuts the core only, so the glow spills past
  the masks' edges; glow cuts the glow only. `invert_masks`, "off" (when added) or "on": what is
  kept is outside the masks instead of inside.

The rule, beyond D-508's (whose words in that file are not repeated):

Ends. A path's stretch has two ends, its first cut and its last, unless it is a closed path drawn
whole (start and end offsets 0 and 100), whose ends meet. At each end, with q its roundness over
100, r = core_size w / 2 the core's radius there (w the width at that end), soft = max(r
core_softness / 100, 1), and o the unit direction from the cut next to it to the end's point (an
end whose cuts all lie on one point has no direction and is left round): when q < 1 and r > 0,
at each pixel's centre within r + 1/2 of the end's point and past its line (along = (X - E) . o >
0), with across the distance from the end's axis, the core's covering is at most
smoothstep(clamp((r + 1/2 - d) / soft, 0, 1)) for d = hypot(across, along / q), and 0 when q is
0: the round end squeezed along the beam into half an ellipse q r long. The glow is not shaped
by the ends; it stays round, as a glow would.

Noise. With `noise_along_path` off, each cut is moved by A (2 n1 - 1) across and A (2 n2 - 1)
down, n1 and n2 now value noise in three axes at (x / S, y / S, tau), (x, y) the cut's point on
the drawing before it moves, S the distortion scale and tau = frame distortion_speed / 10 as
before: on the whole-number grid each corner (i, j, k) has hashed(seed, -1, channel, i, j, k),
D-508's hash with slot -1, and between corners each axis is eased by smoothstep and mixed. The
noise is the same for every path, so two beams crossing one place are moved alike there; a
closed path's two cuts at one point are moved alike, so it stays joined.

Masks. The matte is every mask of the layer that is on and is drawn (document 19), whatever its
mode, each with its own feather, expansion, inversion and opacity, joined: m = m + v - m v from
nothing. With `invert_masks` on it is 1 - m. Then the core c (masks, core) and the glow g (masks,
glow) are each multiplied by it, before the light is worked out; the layer itself is not cut. A
layer with no such mask is drawn uncut, and EFFECT_PATH_MISSING says so every frame. Masks are
always closed here; an open mask is a gap (document 19).

On the card the ends are drawn as here; with Alpha Mode on, the effect runs on the processor,
which holds the masks.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, with no reach cut off round the glow.

Every case is a composition 16 pixels by 10 holding Lightning Bolt's night drawing, as D-508's,
or a shape layer as its open roof. The projects go into `Fixtures/energy_stroke_extras`, the
expected frames into `Fixtures/energy_stroke_extras/expected_energy_stroke_extras.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/energy_stroke_extras_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import smooth_reference as S  # noqa: E402
import stroke_reference as SR  # noqa: E402
import mask_reference as MR  # noqa: E402
import energy_stroke_reference as E  # noqa: E402
from lightning_bolt_reference import DRAWINGS  # noqa: E402
from mask_reference import points  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "energy_stroke_extras"
TOLERANCE = 2e-5
NUMBERS = E.NUMBERS + ("start_roundness", "end_roundness")
WORDS = E.WORDS + ("noise_along_path", "alpha_mode", "invert_masks")
ADDED = dict(E.ADDED, start_roundness=100, end_roundness=100, noise_along_path="on",
             alpha_mode="off", invert_masks="off")


# --- the rule -------------------------------------------------------------------------------

def smooth(f):
    return f * f * (3 - 2 * f)


def noise3(seed, channel, x, y, z):
    i, j, l = math.floor(x), math.floor(y), math.floor(z)
    sx, sy, sz = smooth(x - i), smooth(y - j), smooth(z - l)

    def plane(c):
        v = lambda a, b: E.hashed(seed, -1, (channel, a, b, c))  # noqa: E731
        top = v(i, j) + (v(i + 1, j) - v(i, j)) * sx
        low = v(i, j + 1) + (v(i + 1, j + 1) - v(i, j + 1)) * sx
        return top + (low - top) * sy
    return plane(l) + (plane(l + 1) - plane(l)) * sz


def bits(paths, k, frame_no):
    """D-508's bits, with the noise read on the layer when asked, and each stretch's ends:
    (point, outward direction, radius, roundness share)."""
    s, e = min(k["start_offset"], k["end_offset"]) / 100, max(k["start_offset"], k["end_offset"]) / 100
    seed = math.floor(k["random_seed"])
    amount, scale = k["distortion_amount"], k["distortion_scale"]
    tau = frame_no * k["distortion_speed"] / 10
    step = min(2.0, max(0.5, scale / 8))
    along_path = k["noise_along_path"] == "on"
    out, ends = [], []
    for n, (outline, closed) in enumerate(paths):
        pieces, L = SR.measured(outline, closed)
        lo, hi = s * L, e * L
        if L == 0 or lo >= hi:
            continue
        g0 = (k["evolution"] / 360 % 1) * L if closed else 0.0
        stops = {g0 + lo, g0 + hi}
        corners = [at for _, _, at, _ in pieces] + ([at + L for _, _, at, _ in pieces] if closed else [])
        stops |= {c for c in corners if g0 + lo < c < g0 + hi}
        if amount > 0:
            stops |= {q * step for q in range(math.floor((g0 + lo) / step), math.ceil((g0 + hi) / step) + 1)
                      if g0 + lo < q * step < g0 + hi}
        period = max(1, math.floor(L / scale + 0.5))
        cuts = []
        for g in sorted(stops):
            round_g = g - L if closed and g >= L else g
            p = SR.place(pieces, round_g)
            if amount > 0:
                if along_path:
                    x = round_g * period / L if closed else g / scale
                    per = period if closed else None
                    n1, n2 = E.noise(seed, n, 1, x, tau, per), E.noise(seed, n, 2, x, tau, per)
                else:
                    n1 = noise3(seed, 1, p[0] / scale, p[1] / scale, tau)
                    n2 = noise3(seed, 2, p[0] / scale, p[1] / scale, tau)
                p = (p[0] + amount * (2 * n1 - 1), p[1] + amount * (2 * n2 - 1))
            w = (k["start_size"] + (k["end_size"] - k["start_size"]) * (g - g0 - lo) / (hi - lo)) / 100
            cuts.append((p, w))
        out += [(a, b, wa, wb) for (a, wa), (b, wb) in zip(cuts, cuts[1:])]
        if closed and s == 0 and e == 1:
            continue
        for q, order in ((k["start_roundness"], cuts), (k["end_roundness"], cuts[::-1])):
            (p, w), r = order[0], k["core_size"] * order[0][1] / 2
            nxt = next((c for c, _ in order[1:] if c != p), None)
            if q >= 100 or r <= 0 or nxt is None:
                continue
            dx, dy = p[0] - nxt[0], p[1] - nxt[1]
            n_ = math.hypot(dx, dy)
            ends.append((p, (dx / n_, dy / n_), r, q / 100))
    return out, ends


def shaped(core, ends, k, X, Y):
    for (ex, ey), (ox, oy), r, q in ends:
        along = (X - ex) * ox + (Y - ey) * oy
        if along <= 0 or math.hypot(X - ex, Y - ey) >= r + 0.5:
            continue
        cap = 0.0
        if q > 0:
            d = math.hypot((X - ex) * oy - (Y - ey) * ox, along / q)
            soft = max(r * k["core_softness"] / 100, 1.0)
            cap = smooth(min(1.0, max(0.0, (r + 0.5 - d) / soft)))
        core = min(core, cap)
    return core


def matte(c, frame_no):
    """Every mask that is on and drawn, whatever its mode, joined; None with none."""
    MR.W, MR.H = W, H  # mask_reference's field at this composition's size, not its own 6 by 2
    acc = None
    for m in c["masks"]:
        pts = MR.points_at(m, frame_no)
        if not MR.takes_part(m, pts):
            continue
        acc = acc or [0.0] * (W * H)
        acc = [max(0.0, min(1.0, a + v - a * v)) for a, v in zip(acc, MR.mask_field(m, pts))]
    return acc


def energy(layer, paths, k, frame_no, cut):
    G = [S.srgb_to_linear(v / 255) for v in R.hex_color(k["glow_color"].lower())]
    i = k["glow_intensity"] / 100
    seed = math.floor(k["random_seed"])
    F = 1 - k["flicker_amount"] / 100 * E.noise(seed, 0, 3, frame_no * k["flicker_speed"] / 10, 0)
    segs, ends = bits(paths, k, frame_no)
    px = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            p = layer["px"][y * layer["w"] + x]
            X, Y = layer["left"] + x + 0.5, layer["top"] + y + 0.5
            c, g = E.shade(segs, k, X, Y)
            c = shaped(c, ends, k, X, Y)
            if cut is not None:
                m = cut[y * W + x]
                m = 1 - m if k["invert_masks"] == "on" else m
                if k["alpha_mode"] in ("masks", "core"):
                    c *= m
                if k["alpha_mode"] in ("masks", "glow"):
                    g *= m
            light = [F * (c + (1 - c) * g * G[ch] * i) for ch in range(3)]
            a = min(1.0, max(light))
            if k["composite"] == "add":
                px.append([p[ch] + light[ch] for ch in range(3)] + [p[3] + (1 - p[3]) * a])
            elif k["composite"] == "transparent":
                px.append(light + [a])
            else:
                px.append(light + [1.0])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(base=None, **settings):
    c = dict(base if base is not None else SR.case(masks=[]))
    c["energy"] = dict(ADDED)
    for n, v in settings.items():
        assert n in ADDED, n
        c["energy"][n] = v
    return c


def settings(c, frame_no):
    e = c["energy"]
    k = {n: value_at(e[n], frame_no) for n in NUMBERS}
    k.update({w: e[w] for w in WORDS})
    return k


def render(c, frame_no):
    k = settings(c, frame_no)
    paths = E.paths_of(c, k, frame_no)
    if paths is None:
        return SR.plain(c)
    cut = matte(c, frame_no) if k["alpha_mode"] != "off" else None
    return SR.frame(energy(SR.layer_of(c), paths, k, frame_no, cut), c["shift"])


def masked(*masks):
    return SR.case(masks=list(masks))


none = SR.none
BOX = SR.BOX
SMALL = SR.SMALL                                      # (5, 4) to (9, 6)
LEFT = points([0, 0], [6, 0], [6, 10], [0, 10])       # the left six columns
RIGHT = points([11, 0], [16, 0], [16, 10], [11, 10])  # the right five columns
THICK = dict(core_size=6, glow_spread=0, composite="transparent")   # a core 6 across, no glow
GLOW = dict(core_size=3, glow_spread=6, composite="transparent")
WOBBLE = dict(distortion_amount=1, distortion_scale=3, composite="transparent", core_size=3,
              glow_spread=6)
BOX_WOBBLE = dict(source="masks", distortion_amount=1.5, distortion_scale=4, distortion_speed=0,
                  composite="transparent", core_size=3, glow_spread=6)
ON_BOX = SR.case()

CASES = {
    "FX-ENERGYX-001": ("A core 6 across from (4, 5) to (12, 5), no glow, Start and End "
                       "Roundness 100 written out: D-508's round ends, the frame without them.",
                       case(**THICK), [0]),
    "FX-ENERGYX-002": ("Start and End Roundness 0: both ends flat, cut square across at x 4 and "
                       "x 12.", case(start_roundness=0, end_roundness=0, **THICK), [0]),
    "FX-ENERGYX-003": ("Start Roundness 0, End Roundness 100: flat at the left, round at the "
                       "right.", case(start_roundness=0, **THICK), [0]),
    "FX-ENERGYX-004": ("Both 50: each end half an ellipse, half as long as the round end.",
                       case(start_roundness=50, end_roundness=50, **THICK), [0]),
    "FX-ENERGYX-005": ("Both 0 with a glow, spread 6: the core's ends flat, the glow still "
                       "round past them.",
                       case(start_roundness=0, end_roundness=0, core_size=6, glow_spread=6,
                            composite="transparent"), [0]),
    "FX-ENERGYX-006": ("End Roundness keyed from 0 at frame 0 to 100 at frame 4, linear: the "
                       "right end rounds out, frames 0, 2 and 4.",
                       case(end_roundness=keyed((0, 0), (4, 100)), **THICK), [0, 2, 4]),
    "FX-ENERGYX-007": ("The box mask as the path, drawn whole, both 0: a closed path drawn "
                       "whole has no ends, the frame with them round.",
                       case(ON_BOX, source="masks", start_roundness=0, end_roundness=0,
                            core_size=4, glow_spread=0, composite="transparent"), [0]),
    "FX-ENERGYX-008": ("The box, End offset 50, both 0: half way round, flat at its top-left "
                       "and bottom-right corners' ends.",
                       case(ON_BOX, source="masks", end_offset=50, start_roundness=0,
                            end_roundness=0, core_size=4, glow_spread=0,
                            composite="transparent"), [0]),
    "FX-ENERGYX-009": ("Both 0, End size 50: the right end flat and half as wide.",
                       case(start_roundness=0, end_roundness=0, end_size=50, **THICK), [0]),
    "FX-ENERGYX-010": ("Both 0 with distortion 1 at scale 3: the ends flat across the wobbled "
                       "beam's first and last bit.",
                       case(start_roundness=0, end_roundness=0, distortion_amount=1,
                            distortion_scale=3, **THICK), [0]),
    "FX-ENERGYX-011": ("The open roof shape, Start Roundness 0, End Roundness 25, core 4.",
                       case(SR.shaped(SR.sh(SR.ROOF, closed=False)), source="shapes",
                            start_roundness=0, end_roundness=25, core_size=4, glow_spread=0,
                            composite="transparent"), [0]),
    "FX-ENERGYX-012": ("Noise Along Path on, written out, with distortion 1 at scale 3: "
                       "D-508's wobble, the frame without the setting.",
                       case(**WOBBLE), [0]),
    "FX-ENERGYX-013": ("Noise Along Path off: the noise read on the layer, a different wobble.",
                       case(noise_along_path="off", **WOBBLE), [0]),
    "FX-ENERGYX-014": ("Off on the box, distortion 1.5 at scale 4, speed 0: still joined round "
                       "the box, the same at frames 0 and 3.",
                       case(ON_BOX, noise_along_path="off", **BOX_WOBBLE), [0, 3]),
    "FX-ENERGYX-015": ("Off on the box, speed 50: the wobble moves, frames 0, 1 and 2.",
                       case(ON_BOX, noise_along_path="off", **dict(BOX_WOBBLE, distortion_speed=50)),
                       [0, 1, 2]),
    "FX-ENERGYX-016": ("Off, End offset keyed from 0 at frame 0 to 100 at frame 4: the beam "
                       "draws on through a wobble that stays put, frames 2 and 4.",
                       case(noise_along_path="off", end_offset=keyed((0, 0), (4, 100)),
                            **dict(WOBBLE, distortion_speed=0)), [2, 4]),
    "FX-ENERGYX-017": ("Alpha Mode masks, the small box (5, 4) to (9, 6) as a mask of mode "
                       "None: the core and glow only inside it.",
                       case(masked(none(SMALL)), alpha_mode="masks", **GLOW), [0]),
    "FX-ENERGYX-018": ("Alpha Mode core: the core only inside the small box, the glow "
                       "everywhere.", case(masked(none(SMALL)), alpha_mode="core", **GLOW), [0]),
    "FX-ENERGYX-019": ("Alpha Mode glow: the glow only inside, the core all along.",
                       case(masked(none(SMALL)), alpha_mode="glow", **GLOW), [0]),
    "FX-ENERGYX-020": ("Alpha Mode masks, Invert Masks on: everything but the inside.",
                       case(masked(none(SMALL)), alpha_mode="masks", invert_masks="on", **GLOW),
                       [0]),
    "FX-ENERGYX-021": ("Alpha Mode off with the small box: nothing cut, the frame without it.",
                       case(masked(none(SMALL)), **GLOW), [0]),
    "FX-ENERGYX-022": ("Alpha Mode masks, the small box feathered 2: a soft cut.",
                       case(masked(none(SMALL, feather=2.0)), alpha_mode="masks", **GLOW), [0]),
    "FX-ENERGYX-023": ("Two masks, the left six columns in mode Add and the right five in mode "
                       "None: both cut alike, the mode not read; the Add mask also cuts the "
                       "drawing, as a mask always does, and the light is added over it.",
                       case(masked({"points": LEFT, "mode": "add"}, none(RIGHT)),
                            alpha_mode="masks", core_size=3, glow_spread=6), [0]),
    "FX-ENERGYX-024": ("The small box inverted as a mask (its own Inverted): kept outside it.",
                       case(masked(none(SMALL, inverted=True)), alpha_mode="masks", **GLOW), [0]),
    "FX-ENERGYX-025": ("The box as both the path and the cut: the beam along its edge, half of "
                       "it outside, the core and glow cut at the box's edge.",
                       case(ON_BOX, source="masks", alpha_mode="masks", **GLOW), [0]),
    "FX-ENERGYX-026": ("Alpha Mode masks, composite add, the small box: the drawing beneath "
                       "is not cut, only the light.",
                       case(masked(none(SMALL)), alpha_mode="masks", core_size=3,
                            glow_spread=6), [0]),
}

UNCUT = {
    "FX-ENERGYX-027": ("Alpha Mode masks with no mask.", case(alpha_mode="masks", **GLOW)),
    "FX-ENERGYX-028": ("Alpha Mode core with the one mask switched off.",
                       case(masked(none(SMALL, enabled=False)), alpha_mode="core", **GLOW)),
}

INVALID = {
    "FX-ENERGYX-029": ("Start roundness -1, below 0.", case(start_roundness=-1)),
    "FX-ENERGYX-030": ("End roundness 101, above 100.", case(end_roundness=101)),
    "FX-ENERGYX-031": ("Noise Along Path \"yes\", not \"on\" or \"off\".",
                       case(noise_along_path="yes")),
    "FX-ENERGYX-032": ("Alpha Mode \"enable\", not \"off\", \"masks\", \"core\" or \"glow\".",
                       case(alpha_mode="enable")),
    "FX-ENERGYX-033": ("Invert Masks \"1\", not \"on\" or \"off\".", case(invert_masks="1")),
}


# --- the project files ----------------------------------------------------------------------

def effect_json(c):
    e = c["energy"]
    parameters = {n: (e[n] if n in WORDS else setting_json(e[n])) for n in ADDED}
    return {"instance_id": "fx-0-0", "type_id": "core.energy_stroke", "enabled": True,
            "parameters": parameters}


def project_json(fx, c):
    p = SR.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"] = [effect_json(c)]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = SR.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(len(px)))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in UNCUT.items():
        says += (" No mask to cut by: the beam is drawn uncut, the effect is kept as written, "
                 "and EFFECT_PATH_MISSING is said every frame.")
        frames = {str(f): render(c, f) for f in (0, 4)}
        assert frames["0"] == render(dict(c, energy=dict(c["energy"], alpha_mode="off")), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": frames,
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: uncut")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = SR.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_energy_stroke_extras.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    old = lambda base=None, **s: E.render(E.case(base, **s), 0)  # noqa: E731 - D-508's frame
    clear = [0.0] * 4
    thick = dict(THICK)
    round_ = c["FX-ENERGYX-001"]["0"]
    assert round_ == old(**thick)
    flat = c["FX-ENERGYX-002"]["0"]
    assert flat[at(3, 5)] == clear and round_[at(3, 5)][3] > 0.5     # past the left end
    assert flat[at(12, 5)] == clear and round_[at(12, 5)][3] > 0.5   # past the right end
    assert near(flat[at(4, 5)], round_[at(4, 5)]) and flat[at(8, 5)] == round_[at(8, 5)]
    one = c["FX-ENERGYX-003"]["0"]
    assert one[at(3, 5)] == clear and one[at(12, 5)] == round_[at(12, 5)]
    half = c["FX-ENERGYX-004"]["0"]
    assert flat[at(12, 5)][3] < half[at(12, 5)][3] <= round_[at(12, 5)][3]
    assert 0 < half[at(13, 5)][3] < round_[at(13, 5)][3]
    glow = c["FX-ENERGYX-005"]["0"]
    glow_round = old(core_size=6, glow_spread=6, composite="transparent")
    assert 0 < glow[at(15, 5)][2] and glow[at(15, 5)] == glow_round[at(15, 5)]
    assert glow[at(12, 5)][0] < glow_round[at(12, 5)][0]
    keyed_ = c["FX-ENERGYX-006"]
    assert keyed_["0"] == render(case(end_roundness=0, **THICK), 0)
    assert keyed_["2"] == render(case(end_roundness=50, **THICK), 0) and keyed_["4"] == round_
    assert c["FX-ENERGYX-007"]["0"] == old(ON_BOX, source="masks", core_size=4, glow_spread=0,
                                           composite="transparent")
    hb = c["FX-ENERGYX-008"]["0"]
    hb_round = old(ON_BOX, source="masks", end_offset=50, core_size=4, glow_spread=0,
                   composite="transparent")
    assert hb != hb_round and hb[at(1, 2)] == clear and hb_round[at(1, 2)][3] > 0
    taper = c["FX-ENERGYX-009"]["0"]
    assert taper[at(12, 5)] == clear and taper[at(11, 3)][3] < flat[at(11, 3)][3]
    wob = c["FX-ENERGYX-010"]["0"]
    assert wob != old(distortion_amount=1, distortion_scale=3, **thick)
    roof = c["FX-ENERGYX-011"]["0"]
    assert roof != old(SR.shaped(SR.sh(SR.ROOF, closed=False)), source="shapes", core_size=4,
                       glow_spread=0, composite="transparent")
    along = c["FX-ENERGYX-012"]["0"]
    assert along == old(**WOBBLE)
    fixed = c["FX-ENERGYX-013"]["0"]
    assert fixed != along
    box = c["FX-ENERGYX-014"]
    assert box["0"] == box["3"] and box["0"] != old(ON_BOX, **BOX_WOBBLE)
    moving = c["FX-ENERGYX-015"]
    assert moving["0"] == box["0"] and moving["1"] != moving["0"] and moving["2"] != moving["1"]
    # Fixed on the layer: a beam and its reverse are cut at the same places, every half pixel,
    # and with the noise on the layer each place is moved alike in both; along the path, not.
    k = settings(case(noise_along_path="off", **dict(WOBBLE, distortion_speed=0)), 0)
    ahead, back = [([(2, 5), (14, 5)], False)], [([(14, 5), (2, 5)], False)]
    moved = lambda kk, p: sorted(q for bit in bits(p, kk, 0)[0] for q in bit[:2])  # noqa: E731
    assert moved(k, ahead) == moved(k, back)
    k_on = dict(k, noise_along_path="on")
    assert moved(k_on, ahead) != moved(k_on, back)
    drawn = c["FX-ENERGYX-016"]
    assert drawn["4"] == render(case(noise_along_path="off", **dict(WOBBLE, distortion_speed=0)), 0)
    inside = c["FX-ENERGYX-017"]["0"]
    assert inside[at(2, 5)] == clear and inside[at(14, 5)] == clear
    assert near(inside[at(7, 4)], [1, 1, 1, 1], 1e-6)
    core = c["FX-ENERGYX-018"]["0"]
    assert core[at(2, 5)][3] > 0 and core[at(2, 5)][0] < 0.5 and near(core[at(7, 4)], [1, 1, 1, 1], 1e-6)
    gl = c["FX-ENERGYX-019"]["0"]
    plain = old(**GLOW)
    assert near(gl[at(11, 5)], [1, 1, 1, 1]) and gl[at(8, 0)] == clear != plain[at(8, 0)]
    inv = c["FX-ENERGYX-020"]["0"]
    assert inv[at(7, 4)] == clear and inv[at(2, 5)] == plain[at(2, 5)]
    assert c["FX-ENERGYX-021"]["0"] == plain
    soft = c["FX-ENERGYX-022"]["0"]
    assert 0 < soft[at(4, 4)][3] < plain[at(4, 4)][3] and inside[at(4, 4)] == clear
    two = c["FX-ENERGYX-023"]["0"]
    lit = old(SR.case(masks=[{"points": LEFT, "mode": "add"}, none(RIGHT)]), core_size=3,
              glow_spread=6)
    assert two[at(3, 4)] == lit[at(3, 4)] and two[at(12, 4)] == lit[at(12, 4)]
    assert two[at(8, 4)] != lit[at(8, 4)]
    assert c["FX-ENERGYX-024"]["0"] == inv
    edge = c["FX-ENERGYX-025"]["0"]
    full = old(ON_BOX, source="masks", **GLOW)
    assert edge[at(8, 1)] == clear and full[at(8, 1)][3] > 0 and edge[at(8, 2)] == full[at(8, 2)]
    added = c["FX-ENERGYX-026"]["0"]
    art = SR.plain(case())
    assert added[at(1, 1)] == art[at(1, 1)] and added[at(7, 4)] != art[at(7, 4)]
    for fx in UNCUT:
        assert c[fx]["0"] == plain
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"]
    print("checked")


if __name__ == "__main__":
    main()
