"""D-410's Line Boil, worked a second way.

PLUGINS.md's pick #4: lines that wobble a little and jump to a new random shape every few
frames, as hand-drawn lines redrawn on twos or threes do. The usual recipe is Turbulent Displace
at a small amount with its seed changed every 2 to 4 frames by an expression (`posterizeTime`);
PLUGINS.md says to merge it into `core.turbulent_displace` as "new seed every N frames", like
Camera Shake's hold.

`core.turbulent_displace` gains `new_seed_every`, 0 to 100 frames, keyable, its whole part
counted. 0, what a file without it means, keeps one seed for ever: D-127's and D-328's rule
exactly. With a whole part N of 1 or more, P0-23's held step m = floor(frame / N) (frame the
composition frame, rounded toward minus infinity) is added to the seed's whole part, and the
field F takes that seed: the warp holds for N frames and then jumps to another, as if the seed
were keyed up by one every N frames. Evolution and speed are read as before, so with speed 0
the warp holds still between jumps.

Every case is turbulent_displace_reference's 16 by 10 composition and stripes, in After
Effects' units (D-328) unless the case says. The projects go into `Fixtures/line_boil`, the
expected frames into `Fixtures/line_boil/expected_line_boil.json`.

**This file never runs the build's code path.** It is turbulent_ae_reference's double
precision rule, given the seed for the frame.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/line_boil_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import turbulent_displace_reference as T  # noqa: E402
import turbulent_ae_reference as A  # noqa: E402

W, H = T.W, T.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "line_boil"
TOLERANCE = T.TOLERANCE
RANGE = (0, 100)


def every(c, frame_no):
    """The setting's whole part at the frame, held inside its range."""
    if c["new_seed_every"] is None:
        return 0
    return math.floor(min(RANGE[1], max(RANGE[0], value_at(c["new_seed_every"], frame_no))))


def seed_at(seed, n, frame_no):
    """D-410: the seed's whole part plus P0-23's held step."""
    return seed if n < 1 else seed + math.floor(frame_no / n)


def case(new_seed_every=2, units="after_effects", **settings):
    """`new_seed_every` None: the file does not say."""
    c = A.case(units=units, **settings)
    c["new_seed_every"] = new_seed_every
    return c


def render(c, frame_no):
    if c["units"] not in ("classic", "after_effects", None):
        return T.plain(c)
    layer = T.drawn_layer(c["drawing"])
    n = [T.held(c, k, frame_no) for k in T.NAMES[:-1]]
    n[0] = A.strength(n[0], n[1], c["units"])
    n[5] = seed_at(n[5], every(c, frame_no), frame_no)
    return [T.displaced(layer, *n, c["edges"], frame_no, x - c["shift"], y)
            for y in range(H) for x in range(W)]


BOIL = {"amount": 30, "size": 8, "speed": 0}  # FX-TURB-AE-007's wave, held still

CASES = {
    "FX-BOIL-001": ("New seed every 2 frames, amount 30, size 8, speed 0 (a push of 2.4 pixels "
                    "at most): frames 0 and 1 are FX-TURB-AE-007 (seed 0), frames 2 and 3 the "
                    "same warp with seed 1, frame 4 seed 2: the stripes jump to a new wobble "
                    "every second frame and hold still between, the hand-drawn boil.",
                    case(**BOIL), [0, 1, 2, 3, 4]),
    "FX-BOIL-002": ("New seed every 0, written: one seed for ever, so every frame is "
                    "FX-TURB-AE-007.", case(new_seed_every=0, **BOIL), [0, 2, 4]),
    "FX-BOIL-003": ("New seed every 1: a new seed on every frame, frame f drawn with seed f.",
                    case(new_seed_every=1, **BOIL), [0, 1, 2, 3, 4]),
    "FX-BOIL-004": ("New seed every 3 from seed 8: frames 0 to 2 seed 8, frames 3 and 4 "
                    "seed 9.",
                    case(new_seed_every=3, seed=8, **BOIL), [0, 2, 3, 4]),
    "FX-BOIL-005": ("New seed every 2.9: the whole part, 2, is counted, so this is "
                    "FX-BOIL-001.", case(new_seed_every=2.9, **BOIL), [0, 1, 2, 3, 4]),
    "FX-BOIL-006": ("New seed every 2 with speed 20: the evolution still moves every frame, "
                    "so no two frames are alike, and frames 2 and 3 use seed 1.",
                    case(amount=30, size=8, speed=20), [0, 1, 2, 3]),
    "FX-BOIL-007": ("New seed every keyed from 1 at frame 0 to 4 at frame 4, linear: frame 0 "
                    "seed 0; at frame 2 the setting is 2.5, whole part 2, step 1; at frame 4 "
                    "it is 4, step 1.",
                    case(new_seed_every=keyed((0, 1), (4, 4)), **BOIL), [0, 2, 4]),
    "FX-BOIL-008": ("New seed every 2 in a file without units (D-127's classic push), amount "
                    "3, size 8, speed 0: FX-TURB-004's warp on frames 0 and 1, seed 1 on frames "
                    "2 and 3.", case(units=None, amount=3, size=8, speed=0), [0, 1, 2, 3]),
    "FX-BOIL-009": ("FX-BOIL-001 moved three pixels right: the layer grows by 3, so the three "
                    "columns left of the drawing show the stripes pushed into them, and the "
                    "boil is the same, moved.", case(shift=3, **BOIL), [0, 2]),
    "FX-BOIL-010": ("FX-BOIL-001 with edges repeat: nothing grows, a push past the edge reads "
                    "the nearest edge pixel; the seed steps as before.",
                    case(edges="repeat", **BOIL), [0, 2]),
}

INVALID = {
    "FX-BOIL-011": ("New seed every -1, below 0.", case(new_seed_every=-1, **BOIL)),
    "FX-BOIL-012": ("New seed every 101, above 100.", case(new_seed_every=101, **BOIL)),
    "FX-BOIL-013": ("New seed every keyed from 2 at frame 0 to 150 at frame 4.",
                    case(new_seed_every=keyed((0, 2), (4, 150)), **BOIL)),
}


def project_json(fx, c):
    p = A.project_json(fx, c)
    if c["new_seed_every"] is not None:
        p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["new_seed_every"] = \
            setting_json(c["new_seed_every"])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = T.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = T.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_line_boil.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    ae = json.loads((OUT.parent / "turbulent_ae" / "expected_turbulent_ae.json")
                    .read_text(encoding="utf-8"))["cases"]
    old = json.loads((OUT.parent / "turbulent_displace" / "expected_turbulent_displace.json")
                     .read_text(encoding="utf-8"))["cases"]
    held = lambda seed, f=0, **k: render(case(new_seed_every=None, seed=seed, **(BOIL | k)), f)  # noqa: E731

    assert seed_at(5, 0, 7) == 5 and seed_at(5, 2, 7) == 8 and seed_at(5, 3, -1) == 4
    one = c["FX-BOIL-001"]
    assert one["0"] == one["1"] == ae["FX-TURB-AE-007"]["frames"]["0"]
    assert one["2"] == one["3"] == held(1) != one["0"]
    assert one["4"] == held(2) not in (one["0"], one["2"])
    assert all(v == ae["FX-TURB-AE-007"]["frames"]["0"] for v in c["FX-BOIL-002"].values())
    three = c["FX-BOIL-003"]
    assert all(three[str(f)] == held(f) for f in range(5))
    assert len({json.dumps(v) for v in three.values()}) == 5
    four = c["FX-BOIL-004"]
    assert four["0"] == four["2"] == held(8) and four["3"] == four["4"] == held(9) != four["0"]
    assert c["FX-BOIL-005"] == one
    six = c["FX-BOIL-006"]
    assert len({json.dumps(v) for v in six.values()}) == 4
    assert six["2"] == render(case(new_seed_every=None, seed=1, amount=30, size=8, speed=20), 2)
    seven = c["FX-BOIL-007"]
    assert seven["0"] == held(0) and seven["2"] == held(1) and seven["4"] == held(1)
    eight = c["FX-BOIL-008"]
    assert eight["0"] == eight["1"] == old["FX-TURB-004"]["frames"]["0"]
    assert eight["2"] == eight["3"] != eight["0"]
    nine = c["FX-BOIL-009"]
    assert nine["0"] != nine["2"] and nine["0"] == render(case(new_seed_every=None, shift=3, **BOIL), 0)
    ten = c["FX-BOIL-010"]
    assert ten["0"] == render(case(new_seed_every=None, edges="repeat", **BOIL), 0) != ten["2"]
    print("checked")


if __name__ == "__main__":
    main()
