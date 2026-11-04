"""Warp Stabilizer's Crop Less <-> Smooth More, worked a second way.

D-1012 adds `crop_less_smooth_more` to D-653's Warp Stabilizer (`tools/warp_stabilizer_reference.py`,
whose rule this file runs unchanged but for one step). After Effects' slider trades how much of
the picture is cut away against how steady it is; Adobe publishes no formula, so the rule is ours.

With Result `smooth_motion` and Framing `stabilize_crop` or `stabilize_crop_auto_scale`, step 5's
steadied map S is taken part way from the unchanged map I, number by number:

    S' = I + (c / 100) (S - I),   c = Crop Less <-> Smooth More, 0 to 100,

and steps 6 and 7 use S'. At 100 it is D-653's picture; at 0 the layer is not moved, so nothing is
cut. With Stabilize Only or No Motion the setting does nothing. It is 50 when added, as After
Effects'; a file without it reads as 100, D-653's picture, and keeps reading so. Keyable, read at
frame n.

Every case is a project in `Fixtures/warp_stabilizer_2/`, on D-653's 32 by 32 shaken card; the
expected pixels are in `Fixtures/warp_stabilizer_2/expected_warp_stabilizer_2.json`, tolerance
2e-5 as D-653's.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python -I tools/warp_stabilizer_2_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import warp_stabilizer_reference as WS  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "warp_stabilizer_2"
NAME = "crop_less_smooth_more"
_steadied = WS.steadied


def steadied(c, n):
    """Step 5's S, taken part way from I by D-1012."""
    s = _steadied(c, n)
    if c["result"] != "smooth_motion" or c["framing"] == "stabilize_only":
        return s
    k = value_at(c.get(NAME, 100), n) / 100
    return [[i + k * (v - i) for v, i in zip(row, one)] for row, one in zip(s, WS.IDENTITY)]


WS.steadied = steadied


def effect(fid, c):
    p = {key: setting_json(c[key]) for key in WS.KEYS}
    if NAME in c:
        p[NAME] = setting_json(c[NAME])
    return {"instance_id": fid, "type_id": "core.warp_stabilizer", "enabled": True, "parameters": p}


WS.effect = effect
WS.OUT = OUT


def case(**kw):
    return WS.case(**kw)


AUTO = {}
CROP = {"framing": "stabilize_crop"}
ONLY = {"framing": "stabilize_only"}
CASES = {
    "FX-WS2-001": ("As added, Crop Less <-> Smooth More 50: steadied half as far as D-653's, so cut "
                   "and scaled up less (FX-WS-001 at 100).", case(**{NAME: 50}), (2, 3)),
    "FX-WS2-002": ("At 100: D-653's own picture, FX-WS-001.", case(**{NAME: 100}), (2,)),
    "FX-WS2-003": ("At 0: not moved, so nothing cut or scaled: the drawing as it is.",
                   case(**{NAME: 0}), (2,)),
    "FX-WS2-004": ("At 50, Stabilize, Crop: fewer pixels cut away than FX-WS-003's.",
                   case(**{NAME: 50}, **CROP), (2,)),
    "FX-WS2-005": ("At 50 with Stabilize Only: the setting does nothing, FX-WS-002.",
                   case(**{NAME: 50}, **ONLY), (2,)),
    "FX-WS2-006": ("At 50 with No Motion: the setting does nothing, No Motion's own picture.",
                   case(result="no_motion", **{NAME: 50}), (3,)),
    "FX-WS2-007": ("Keyed from 0 at frame 0 to 100 at frame 4: 50 at frame 2, FX-WS2-001's frame 2.",
                   case(**{NAME: keyed((0, 0), (4, 100))}), (2,)),
    "FX-WS2-008": ("A D-653 file without the setting: read as 100, FX-WS-001's picture.", case(), (2,)),
}
INVALID = {
    "FX-WS2-009": ("Crop Less <-> Smooth More 101, above 100.", case(**{NAME: 101})),
    "FX-WS2-010": ("Crop Less <-> Smooth More -1, below 0.", case(**{NAME: -1})),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in WS.DRAWINGS.items():
        if name.startswith("shake_"):
            (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": WS.TOLERANCE, "width": WS.W, "height": WS.H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): WS.render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": WS.write(
            f"{fx.lower().replace('-', '_')}.json", WS.project_json(fx, c)), "frames": rendered}
        print(fx)
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": WS.write(
            f"{fx.lower().replace('-', '_')}.json", WS.project_json(fx, c)),
            "frames": {"0": WS.plain(c, 0), "4": WS.plain(c, 4)}, "warning": "EFFECT_PARAMETER_INVALID"}
    (OUT / "expected_warp_stabilizer_2.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = lambda **kw: WS.render(case(**kw), 2)  # noqa: E731
    assert c["FX-WS2-002"]["2"] == old() == c["FX-WS2-008"]["2"]
    assert c["FX-WS2-003"]["2"] == WS.plain(case(), 2)
    assert c["FX-WS2-005"]["2"] == old(**ONLY)
    assert c["FX-WS2-006"]["3"] == WS.render(case(result="no_motion"), 3)
    assert c["FX-WS2-007"]["2"] == c["FX-WS2-001"]["2"]
    assert c["FX-WS2-001"]["2"] != c["FX-WS2-002"]["2"] and c["FX-WS2-001"]["2"] != c["FX-WS2-003"]["2"]
    # Crop less: Stabilize, Crop at 50 clears fewer pixels than at 100 (FX-WS-003).
    clear = lambda px: sum(p[3] < 1e-9 for p in px)  # noqa: E731
    assert clear(c["FX-WS2-004"]["2"]) < clear(old(**CROP)), (clear(c["FX-WS2-004"]["2"]), clear(old(**CROP)))
    assert clear(c["FX-WS2-001"]["2"]) == 0
    print("checked")


if __name__ == "__main__":
    main()
