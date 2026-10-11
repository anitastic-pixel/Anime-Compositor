"""Energy Stroke, worked a second way.

D-508 adds `core.energy_stroke`: a glowing beam along a path, a hot white core inside a coloured
glow, that can draw on, taper, wobble and flicker. It is pick #11 of `docs/effects/PLUGINS.md`,
after the purpose of Video Copilot's Saber, and built separately from Path Stroke as that file
recommends. Saber's controls are named in its launch tutorial and in two written walkthroughs
(webdew.com, "How to use Saber in After Effects"; Noble Desktop, "Neon text effect"); none
publishes a formula or a setting when added, so the rule below and every setting when added are
this program's own. Nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The settings. `source`, where the beam runs: "points" (when added), a straight beam from
`core_start` to `core_end`, x then y in per cent of the drawing's own width and height, -1000 to
1000, [25, 50] and [75, 50] when added; "masks", every mask of the layer that is on with two
points or more, whatever its mode, each closed as document 19 flattens it; "shapes", every shape
of a shape layer that is on with two points or more, open or closed as it says (D-357's paths);
"text", every outline of a text layer's words as they are drawn (D-373's paths). `evolution`,
-3600 to 3600 degrees, 0 when added: where a closed path begins, a whole turn its whole length.
`start_offset` and `end_offset`, 0 to 100 per cent of each path's length, 0 and 100 when added,
taken smaller first: the stretch drawn. `start_size` and `end_size`, 0 to 100 per cent, 100 when
added: the beam's width at each end of that stretch. `core_size`, 0 to 200 pixels across, 8 when
added; `core_softness`, 0 to 100, 30 when added. `glow_color`, `#rrggbb`, "#3080ff" when added;
`glow_intensity`, 0 to 1000 per cent, 100 when added; `glow_spread`, 0 to 1000 pixels, 40 when
added. `flicker_amount`, 0 to 100, 0 when added; `flicker_speed`, 0 to 100, 20 when added.
`distortion_amount`, 0 to 500 pixels, 0 when added; `distortion_scale`, 1 to 1000 pixels, 60
when added; `distortion_speed`, 0 to 100, 20 when added. `random_seed`, 0 to 10000, its floor
taken, 0 when added. Every number keyable. `composite`, "add" (when added), "transparent" or
"black", exact. Copies on one layer stack: with Add, each beam is added to what is below it.

The rule. Each path is measured along its length L from its first point (an open path without
the piece from its last point to its first). With s and e the smaller and larger offset over
100, the stretch drawn is u from lo = s L to hi = e L; with lo = hi, or L = 0, the path draws
nothing. On a closed path the stretch starts g0 = frac(evolution / 360) L along it: the point at
u is the path's point at g = g0 + u, taken round the path (g less L once it passes L). An open
path's g is u. The stretch is cut into straight bits at its two ends, at every corner of the path
inside it and, with distortion, at every g inside it that is a whole number of steps
D = clamp(distortion_scale / 8, 0.5, 2) from g = 0. Each cut has a width w = (start_size +
(end_size - start_size)(u - lo) / (hi - lo)) / 100.

Distortion moves each cut by A (2 n1 - 1) across and A (2 n2 - 1) down, A the amount, n1 and n2
the path's noise channels 1 and 2 at x and tau = frame distortion_speed / 10. On a closed path
x = (g, round the path) P / L with P = max(1, floor(L / distortion_scale + 1/2)) cells round it,
the noise repeating every P cells so the path stays joined; on an open path x = g /
distortion_scale. The noise is value noise: on the whole-number grid each corner (i, j) has
hashed(seed, path number, channel, i, j), Scribble's and P0-22's hash (along.rs), i taken modulo P
on a closed path; between corners each axis is eased by smoothstep and mixed. The path number
counts from 0 in the order the paths are taken.

At the pixel's centre (X, Y), for each bit from a to b: t the nearest point's share along it, d
the distance to it, w the width there (mixed from its ends by t), r = core_size w / 2 and soft =
max(r core_softness / 100, 1). The core's covering is smoothstep(clamp((r + 1/2 - d) / soft, 0,
1)) when r > 0, else 0. The glow is w (1/4) sum over k = 0 to 3 of exp(-d^2 / (2 sigma_k^2)),
sigma_k = max(glow_spread / 3 / 2^k, 1/4), and 0 when glow_spread is 0: four soft rings, each half
the last, P0-21's doubling worked out for a line. Core c and glow g are each the largest over
every bit of every path. The flicker F = 1 - flicker_amount / 100 n3, n3 the noise's channel 3
of path slot 0 at (frame flicker_speed / 10, 0). The light, with G the glow colour's linear value
and i = glow_intensity / 100: L = F (c + (1 - c) g G i), per channel, its covering a =
min(1, max(L)). On the premultiplied pixel p:

- add: (p.rgb + L, p.a + (1 - p.a) a);
- transparent: (L, a), the layer itself gone;
- black: (L, 1), the light over black.

The layer never grows: the beam is drawn inside the layer's own picture. A draft scales
core_size, glow_spread, distortion_amount and distortion_scale as distances; the paths are scaled
as they always are; the points are shares of the drawing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, with no reach cut off round the glow, where the build looks
only near each bit and writes single precision into its buffers.

Every case but the text ones is a composition 16 pixels by 10 holding Lightning Bolt's night
drawing (its left half a night sky, its right half empty), with masks of mode None, as Path
Stroke's cases (tools/stroke_reference.py), or a shape layer as D-357's. The text cases are
D-373's: "Yes" on a text layer 48 by 30 (tools/stroke_text_reference.py). The projects go into
`Fixtures/energy_stroke`, the expected frames into
`Fixtures/energy_stroke/expected_energy_stroke.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/energy_stroke_reference.py
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
import stroke_text_reference as ST  # noqa: E402
from lightning_bolt_reference import DRAWINGS  # noqa: E402
from scribble_reference import mix  # noqa: E402
from mask_reference import points  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "energy_stroke"
TOLERANCE = 2e-5
M32 = 0xFFFFFFFF
NUMBERS = ("core_start", "core_end", "evolution", "start_offset", "end_offset", "start_size",
           "end_size", "core_size", "core_softness", "glow_intensity", "glow_spread",
           "flicker_amount", "flicker_speed", "distortion_amount", "distortion_scale",
           "distortion_speed", "random_seed")
WORDS = ("source", "glow_color", "composite")
ADDED = {"source": "points", "core_start": [25, 50], "core_end": [75, 50], "evolution": 0,
         "start_offset": 0, "end_offset": 100, "start_size": 100, "end_size": 100,
         "core_size": 8, "core_softness": 30, "glow_color": "#3080ff", "glow_intensity": 100,
         "glow_spread": 40, "flicker_amount": 0, "flicker_speed": 20, "distortion_amount": 0,
         "distortion_scale": 60, "distortion_speed": 20, "random_seed": 0, "composite": "add"}


# --- the rule -------------------------------------------------------------------------------

def hashed(seed, slot, rest):
    h = mix(seed & M32)
    for v in (slot, *rest):
        h = mix(h ^ (v & M32))
    return h / 4294967296.0


def noise(seed, slot, channel, x, y, period=None):
    i, j = math.floor(x), math.floor(y)
    fx, fy = x - i, y - j
    sx, sy = fx * fx * (3 - 2 * fx), fy * fy * (3 - 2 * fy)

    def v(a, b):
        return hashed(seed, slot, (channel, a % period if period else a, b, 0))
    top = v(i, j) + (v(i + 1, j) - v(i, j)) * sx
    low = v(i, j + 1) + (v(i + 1, j + 1) - v(i, j + 1)) * sx
    return top + (low - top) * sy


def bits(paths, k, frame_no):
    """Every straight bit drawn: (a, b, width at a, width at b)."""
    s, e = min(k["start_offset"], k["end_offset"]) / 100, max(k["start_offset"], k["end_offset"]) / 100
    seed = math.floor(k["random_seed"])
    amount, scale = k["distortion_amount"], k["distortion_scale"]
    tau = frame_no * k["distortion_speed"] / 10
    step = min(2.0, max(0.5, scale / 8))
    out = []
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
                x = round_g * period / L if closed else g / scale
                per = period if closed else None
                p = (p[0] + amount * (2 * noise(seed, n, 1, x, tau, per) - 1),
                     p[1] + amount * (2 * noise(seed, n, 2, x, tau, per) - 1))
            w = (k["start_size"] + (k["end_size"] - k["start_size"]) * (g - g0 - lo) / (hi - lo)) / 100
            cuts.append((p, w))
        out += [(a, b, wa, wb) for (a, wa), (b, wb) in zip(cuts, cuts[1:])]
    return out


def shade(segs, k, X, Y):
    """The core's covering and the glow at (X, Y)."""
    core = glow = 0.0
    spread = k["glow_spread"]
    sig = [max(spread / 3 / 2 ** j, 0.25) for j in range(4)]
    for a, b, wa, wb in segs:
        dx, dy = b[0] - a[0], b[1] - a[1]
        l2 = dx * dx + dy * dy
        t = 0.0 if l2 == 0 else min(1.0, max(0.0, ((X - a[0]) * dx + (Y - a[1]) * dy) / l2))
        d = math.hypot(X - a[0] - t * dx, Y - a[1] - t * dy)
        w = wa + (wb - wa) * t
        r = k["core_size"] * w / 2
        if r > 0:
            soft = max(r * k["core_softness"] / 100, 1.0)
            u = min(1.0, max(0.0, (r + 0.5 - d) / soft))
            core = max(core, u * u * (3 - 2 * u))
        if spread > 0:
            glow = max(glow, w * sum(math.exp(-d * d / (2 * q * q)) for q in sig) / 4)
    return core, glow


def energy(layer, paths, k, frame_no):
    G = [S.srgb_to_linear(v / 255) for v in R.hex_color(k["glow_color"].lower())]
    i = k["glow_intensity"] / 100
    seed = math.floor(k["random_seed"])
    F = 1 - k["flicker_amount"] / 100 * noise(seed, 0, 3, frame_no * k["flicker_speed"] / 10, 0)
    segs = bits(paths, k, frame_no)
    px = []
    for y in range(layer["h"]):
        for x in range(layer["w"]):
            p = layer["px"][y * layer["w"] + x]
            c, g = shade(segs, k, layer["left"] + x + 0.5, layer["top"] + y + 0.5)
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
    """An Energy Stroke on `base`, a Path Stroke case from stroke_reference or a text case
    from stroke_text_reference (its masks or shapes are the paths); unsaid settings as added."""
    c = dict(base if base is not None else SR.case(masks=[]))
    c["energy"] = dict(ADDED)
    for n, v in settings.items():
        assert n in ADDED, n
        c["energy"][n] = v
    return c


def text(**settings):
    return case(ST.case(all_masks="on"), source="text", **settings)


def settings(c, frame_no):
    e = c["energy"]
    k = {n: value_at(e[n], frame_no) for n in NUMBERS}
    k.update({w: e[w] for w in WORDS})
    return k


def paths_of(c, k, frame_no):
    if k["source"] == "points":
        a, b = k["core_start"], k["core_end"]
        return [([(a[0] / 100 * W, a[1] / 100 * H), (b[0] / 100 * W, b[1] / 100 * H)], False)]
    if k["source"] == "text":
        return ST.paths_of(c, frame_no)
    base = dict(c, source=k["source"], all_masks="on")
    return SR.paths_of(base, frame_no)


def is_text(c):
    return "words" in c


def plain(c):
    if is_text(c):
        return [[0.0] * 4 for _ in range(ST.W * ST.H)]
    return SR.plain(c)


def render(c, frame_no):
    k = settings(c, frame_no)
    paths = paths_of(c, k, frame_no)
    if paths is None:
        return plain(c)
    if is_text(c):
        layer = {"px": plain(c), "left": 0, "top": 0, "w": ST.W, "h": ST.H}
        return energy(layer, paths, k, frame_no)["px"]
    return SR.frame(energy(SR.layer_of(c), paths, k, frame_no), c["shift"])


BOX = SR.BOX
TWO = SR.TWO
none, sh, shaped = SR.none, SR.sh, SR.shaped
ON_BOX = SR.case()
SMALL_LINE = dict(core_size=3, glow_spread=6)
LINE = points([2, 5], [13, 5])                     # an open line across the middle, 11 long

CASES = {
    "FX-ENERGY-001": ("The settings as added: a beam from a quarter across to three quarters "
                      "across, half way down, core 8 across, softness 30, a blue glow spread 40 "
                      "at intensity 100, added to the drawing. On a drawing this small the glow "
                      "reaches every pixel.", case(), [0]),
    "FX-ENERGY-002": ("Core 3, spread 6: a thin white line with its blue glow, the rest of the "
                      "drawing lit faintly.", case(**SMALL_LINE), [0]),
    "FX-ENERGY-003": ("Composite transparent: the light alone, the night sky gone.",
                      case(composite="transparent", **SMALL_LINE), [0]),
    "FX-ENERGY-004": ("Composite black: the light over black, every pixel whole.",
                      case(composite="black", **SMALL_LINE), [0]),
    "FX-ENERGY-005": ("Glow spread 0: the core alone, no glow anywhere.",
                      case(core_size=3, glow_spread=0, composite="transparent"), [0]),
    "FX-ENERGY-006": ("Core size 0: the glow alone, blue all through, no white.",
                      case(core_size=0, glow_spread=6, composite="transparent"), [0]),
    "FX-ENERGY-007": ("Core softness 0 and 100: hard, then soft from the middle out.",
                      case(core_size=4, core_softness=0, glow_spread=0, composite="transparent"),
                      [0]),
    "FX-ENERGY-008": ("Core softness 100.", case(core_size=4, core_softness=100, glow_spread=0,
                                                 composite="transparent"), [0]),
    "FX-ENERGY-009": ("Glow intensity 400, an orange glow: past 1 where the glow is strong.",
                      case(glow_intensity=400, glow_color="#ff8020", **SMALL_LINE), [0]),
    "FX-ENERGY-010": ("End offset 50: the beam from its start to half way, ending at the middle.",
                      case(end_offset=50, composite="transparent", **SMALL_LINE), [0]),
    "FX-ENERGY-011": ("Start offset 75, End offset 25: taken smaller first, the middle half.",
                      case(start_offset=75, end_offset=25, composite="transparent", **SMALL_LINE),
                      [0]),
    "FX-ENERGY-012": ("Start offset 40, End offset 40: nothing drawn, the drawing as it was.",
                      case(start_offset=40, end_offset=40, **SMALL_LINE), [0]),
    "FX-ENERGY-013": ("End offset keyed from 0 at frame 0 to 100 at frame 4, linear: the beam "
                      "draws on, frames 0, 2 and 4.",
                      case(end_offset=keyed((0, 0), (4, 100)), composite="transparent",
                           **SMALL_LINE), [0, 2, 4]),
    "FX-ENERGY-014": ("Start size 100, End size 0: the beam tapers to a point at its end.",
                      case(end_size=0, core_size=5, glow_spread=4, composite="transparent"), [0]),
    "FX-ENERGY-015": ("Start size 0 and End size 0: no width anywhere, nothing drawn.",
                      case(start_size=0, end_size=0, **SMALL_LINE), [0]),
    "FX-ENERGY-016": ("Core start and end keyed: the beam from the top-left corner to the "
                      "bottom-right at frame 0, swinging to the bottom-left to the top-right at "
                      "frame 4, frames 0 and 4.",
                      case(core_start=keyed((0, (0, 0)), (4, (0, 100))),
                           core_end=keyed((0, (100, 100)), (4, (100, 0))),
                           composite="transparent", **SMALL_LINE), [0, 4]),
    "FX-ENERGY-017": ("Path From Masks, the box from (2, 2) to (13, 7): the beam all round it.",
                      case(ON_BOX, source="masks", composite="transparent", **SMALL_LINE), [0]),
    "FX-ENERGY-018": ("The box, End offset 50: half way round it, from its top-left corner.",
                      case(ON_BOX, source="masks", end_offset=50, composite="transparent",
                           **SMALL_LINE), [0]),
    "FX-ENERGY-019": ("The box, End offset 50, Evolution 90: the same half, begun a quarter of "
                      "the way round, at the top-right corner.",
                      case(ON_BOX, source="masks", end_offset=50, evolution=90,
                           composite="transparent", **SMALL_LINE), [0]),
    "FX-ENERGY-020": ("The box, End offset 50, Evolution 450: a turn and a quarter, "
                      "FX-ENERGY-019's frame.",
                      case(ON_BOX, source="masks", end_offset=50, evolution=450,
                           composite="transparent", **SMALL_LINE), [0]),
    "FX-ENERGY-021": ("The box, End offset 25, Evolution keyed from 0 at frame 0 to 360 at "
                      "frame 4: a quarter of the box travelling round it, frames 0, 1, 2 and 4.",
                      case(ON_BOX, source="masks", end_offset=25,
                           evolution=keyed((0, 0), (4, 360)), composite="transparent",
                           **SMALL_LINE), [0, 1, 2, 4]),
    "FX-ENERGY-022": ("Two masks, the box and a small box inside it: both, the glow of each the "
                      "nearer's.", case(SR.case(masks=TWO), source="masks",
                                        composite="transparent", **SMALL_LINE), [0]),
    "FX-ENERGY-023": ("The box, distortion 1.5 at scale 4, speed 0: the beam wobbles but stays "
                      "joined round the box, and is the same at frame 0 and 3.",
                      case(ON_BOX, source="masks", distortion_amount=1.5, distortion_scale=4,
                           distortion_speed=0, composite="transparent", **SMALL_LINE), [0, 3]),
    "FX-ENERGY-024": ("Distortion 1.5 at scale 4, speed 50: the wobble moves, frames 0, 1 and "
                      "2.", case(ON_BOX, source="masks", distortion_amount=1.5,
                                 distortion_scale=4, distortion_speed=50,
                                 composite="transparent", **SMALL_LINE), [0, 1, 2]),
    "FX-ENERGY-025": ("FX-ENERGY-023 with Random Seed 7: a different wobble.",
                      case(ON_BOX, source="masks", distortion_amount=1.5, distortion_scale=4,
                           distortion_speed=0, random_seed=7, composite="transparent",
                           **SMALL_LINE), [0]),
    "FX-ENERGY-026": ("The straight beam with distortion 1 at scale 3: an open path wobbles too.",
                      case(distortion_amount=1, distortion_scale=3, composite="transparent",
                           **SMALL_LINE), [0]),
    "FX-ENERGY-027": ("Flicker 80 at speed 30: the whole light dims and brightens frame by "
                      "frame, frames 0 to 4.",
                      case(flicker_amount=80, flicker_speed=30, composite="transparent",
                           **SMALL_LINE), [0, 1, 2, 3, 4]),
    "FX-ENERGY-028": ("Flicker 80 at speed 0: held, the same each frame, frames 0 and 4.",
                      case(flicker_amount=80, flicker_speed=0, composite="transparent",
                           **SMALL_LINE), [0, 4]),
    "FX-ENERGY-029": ("Path From Shapes, the box as a shape layer's shape: FX-ENERGY-017's beam "
                      "on the shape layer (transparent), added.",
                      case(shaped(sh(BOX)), source="shapes", **SMALL_LINE), [0]),
    "FX-ENERGY-030": ("An open shape, a roof of two legs: drawn open, nothing along the bottom.",
                      case(shaped(sh(SR.ROOF, closed=False)), source="shapes", **SMALL_LINE), [0]),
    "FX-ENERGY-031": ("The open roof, Evolution 180: an open path has no evolution, "
                      "FX-ENERGY-030's frame.",
                      case(shaped(sh(SR.ROOF, closed=False)), source="shapes", evolution=180,
                           **SMALL_LINE), [0]),
    "FX-ENERGY-032": ("The box mask with Path From Points: the mask is not used, the straight "
                      "beam, FX-ENERGY-002's frame.", case(ON_BOX, **SMALL_LINE), [0]),
    "FX-ENERGY-033": ("FX-ENERGY-002 moved three pixels right: the beam moves with the drawing.",
                      case(SR.case(masks=[], shift=3), **SMALL_LINE), [0]),
    "FX-ENERGY-034": ("Path From Text Outlines, \"Yes\" (D-373's text layer, 48 by 30): every "
                      "outline glows, core 1.5, spread 3, added over nothing.",
                      text(core_size=1.5, glow_spread=3), [0]),
    "FX-ENERGY-035": ("\"Yes\", End offset keyed from 0 at frame 0 to 100 at frame 4: each "
                      "outline draws on round itself, frames 0, 2 and 4.",
                      text(core_size=1.5, glow_spread=3, end_offset=keyed((0, 0), (4, 100))),
                      [0, 2, 4]),
}

MISSING = {
    "FX-ENERGY-036": ("Path From Masks, no masks.", case(source="masks", **SMALL_LINE)),
    "FX-ENERGY-037": ("Path From Masks, the one mask switched off.",
                      case(SR.case(masks=[none(BOX, enabled=False)]), source="masks",
                           **SMALL_LINE)),
    "FX-ENERGY-038": ("Path From Shapes on the night drawing, which is not a shape layer.",
                      case(ON_BOX, source="shapes", **SMALL_LINE)),
    "FX-ENERGY-039": ("Path From Text Outlines on a shape layer with no shapes.",
                      case(ST.case(kind="shape", all_masks="on"), source="text", **SMALL_LINE)),
}

INVALID = {
    "FX-ENERGY-040": ("Core size 201, above 200.", case(core_size=201)),
    "FX-ENERGY-041": ("Core softness -1, below 0.", case(core_softness=-1)),
    "FX-ENERGY-042": ("Glow spread 1001, above 1000.", case(glow_spread=1001)),
    "FX-ENERGY-043": ("Glow intensity 1001, above 1000.", case(glow_intensity=1001)),
    "FX-ENERGY-044": ("Start offset -1, below 0.", case(start_offset=-1)),
    "FX-ENERGY-045": ("End size 101, above 100.", case(end_size=101)),
    "FX-ENERGY-046": ("Distortion scale 0, below 1.", case(distortion_scale=0)),
    "FX-ENERGY-047": ("Flicker amount 101, above 100.", case(flicker_amount=101)),
    "FX-ENERGY-048": ("Core start x 1001, above 1000.", case(core_start=[1001, 50])),
    "FX-ENERGY-049": ("Evolution 3601, above 3600.", case(evolution=3601)),
    "FX-ENERGY-050": ("Random seed -1, below 0.", case(random_seed=-1)),
    "FX-ENERGY-051": ("Composite \"screen\", which is not \"add\", \"transparent\" or \"black\".",
                      case(composite="screen")),
    "FX-ENERGY-052": ("Path From \"mask\", which is not \"points\", \"masks\", \"shapes\" or "
                      "\"text\".", case(source="mask")),
    "FX-ENERGY-053": ("Glow colour \"#12345\", not six hex digits.", case(glow_color="#12345")),
}


# --- the project files ----------------------------------------------------------------------

def effect_json(c):
    e = c["energy"]
    parameters = {n: (e[n] if n in WORDS else setting_json(e[n])) for n in ADDED}
    return {"instance_id": "fx-0-0", "type_id": "core.energy_stroke", "enabled": True,
            "parameters": parameters}


def project_json(fx, c):
    p = ST.project_json(fx, c) if is_text(c) else SR.project_json(fx, c)
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
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(len(px)))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in MISSING.items():
        says += (" Nothing to draw along: the layer is drawn without the effect, which is kept "
                 "as written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert render(c, 0) == before
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_energy_stroke.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    clear = [0.0] * 4
    assert all(p != q for p, q in zip(c["FX-ENERGY-001"]["0"], art))
    two = c["FX-ENERGY-002"]["0"]
    assert near(two[at(8, 4)], [1, 1, 1, 1]) and two[at(8, 0)] != art[at(8, 0)]
    alone = c["FX-ENERGY-003"]["0"]
    assert near(alone[at(8, 4)], [1, 1, 1, 1])
    assert alone[at(15, 0)][2] > alone[at(15, 0)][0] > 0          # blue glow, faint, at a corner
    assert all(p[3] == 1.0 for p in c["FX-ENERGY-004"]["0"])
    assert all(near(p[:3], q[:3]) for p, q in zip(c["FX-ENERGY-004"]["0"], alone))
    core = c["FX-ENERGY-005"]["0"]
    assert core[at(8, 0)] == clear and near(core[at(8, 4)], [1, 1, 1, 1])
    glow = c["FX-ENERGY-006"]["0"]
    assert all(p[2] >= p[0] for p in glow) and glow[at(8, 4)][0] < 0.5
    hard, soft = c["FX-ENERGY-007"]["0"], c["FX-ENERGY-008"]["0"]
    assert soft[at(8, 3)][3] < hard[at(8, 3)][3]
    assert any(max(p[:3]) > 1 for p in c["FX-ENERGY-009"]["0"])
    half = c["FX-ENERGY-010"]["0"]
    assert near(half[at(5, 4)], [1, 1, 1, 1]) and half[at(11, 4)][0] < 0.5
    assert c["FX-ENERGY-011"]["0"] == render(case(start_offset=25, end_offset=75,
                                                  composite="transparent", **SMALL_LINE), 0)
    assert c["FX-ENERGY-012"]["0"] == art
    on = c["FX-ENERGY-013"]
    assert all(p == clear for p in on["0"]) and on["2"] == half and on["4"] == alone
    taper = c["FX-ENERGY-014"]["0"]
    assert taper[at(5, 3)][3] > taper[at(11, 3)][3]
    assert c["FX-ENERGY-015"]["0"] == art
    swing = c["FX-ENERGY-016"]
    assert near(swing["0"][at(0, 0)], [1, 1, 1, 1], 0.05) and near(swing["4"][at(0, 9)], [1, 1, 1, 1], 0.05)
    box = c["FX-ENERGY-017"]["0"]
    assert near(box[at(8, 1)], [1, 1, 1, 1]) and near(box[at(13, 4)], [1, 1, 1, 1])
    hb = c["FX-ENERGY-018"]["0"]
    assert near(hb[at(8, 1)], [1, 1, 1, 1]) and hb[at(8, 7)][0] < 0.5
    turned = c["FX-ENERGY-019"]["0"]
    assert near(turned[at(13, 4)], [1, 1, 1, 1]) and near(turned[at(8, 7)], [1, 1, 1, 1])
    assert turned[at(4, 1)][0] < 0.5
    assert c["FX-ENERGY-020"]["0"] == turned
    travel = c["FX-ENERGY-021"]
    assert travel["0"] == travel["4"] and travel["1"] != travel["0"]
    assert travel["2"] == render(case(ON_BOX, source="masks", start_offset=0, end_offset=25,
                                      evolution=180, composite="transparent", **SMALL_LINE), 0)
    both = c["FX-ENERGY-022"]["0"]
    assert near(both[at(8, 1)], [1, 1, 1, 1]) and near(both[at(7, 4)], [1, 1, 1, 1])
    wob = c["FX-ENERGY-023"]
    assert wob["0"] == wob["3"] and wob["0"] != box
    moving = c["FX-ENERGY-024"]
    assert moving["0"] == wob["0"] and moving["1"] != moving["0"] and moving["2"] != moving["1"]
    assert c["FX-ENERGY-025"]["0"] != wob["0"]
    assert c["FX-ENERGY-026"]["0"] != alone
    fl = c["FX-ENERGY-027"]
    lit = [fl[str(f)][at(8, 4)][0] for f in range(5)]
    assert len(set(lit)) > 2 and all(0.2 - 1e-12 <= v <= 1 for v in lit)
    held = c["FX-ENERGY-028"]
    assert held["0"] == held["4"] and held["0"][at(8, 4)][0] < 1
    sbox = c["FX-ENERGY-029"]["0"]
    assert sbox == box
    roof, roof_turned = c["FX-ENERGY-030"]["0"], c["FX-ENERGY-031"]["0"]
    assert roof == roof_turned and roof[at(8, 7)][0] < 0.5 and near(roof[at(8, 2)], [1, 1, 1, 1], 0.05)
    assert c["FX-ENERGY-032"]["0"] == two
    moved = c["FX-ENERGY-033"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    words = c["FX-ENERGY-034"]["0"]
    assert sum(p[3] > 0.9 for p in words) > 50
    draw = c["FX-ENERGY-035"]
    assert all(p == clear for p in draw["0"]) and draw["4"] == words and draw["2"] != words
    for fx in list(MISSING) + list(INVALID):
        assert c[fx]["0"] == c[fx]["4"]
    print("checked")


if __name__ == "__main__":
    main()
