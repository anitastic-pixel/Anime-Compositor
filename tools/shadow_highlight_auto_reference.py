"""Shadow/Highlight's Auto Amounts, Temporal Smoothing and Scene Detect, worked a second way.

D-400 added `core.shadow_highlight`, After Effects' Shadow/Highlight, and D-563 its Midtone
Contrast and Black and White Clip. D-564 adds the last three settings it lacked: Auto Amounts,
Temporal Smoothing and Scene Detect. `tools/shadow_highlight_reference.py` and
`tools/shadow_highlight_clip_reference.py` are not changed; this file takes D-563's rule from the
second and the statistics of D-351 (`tools/auto_tone_reference.py`) unchanged.

What After Effects does is not published. For D-564 two pages were tried and neither was read:
Adobe's colour correction effects page (helpx.adobe.com, refused with 403) and its copy at
web.archive.org (not reachable from here). What is known is the manual's words as recalled: with
Auto Amounts on, the two amount sliders are not used and the effect picks amounts of its own to
lighten the shadows and bring back detail, and Temporal Smoothing comes into use; Temporal
Smoothing is the seconds of neighbouring frames read to judge each frame's correction, 0 judging
each frame alone; Scene Detect leaves out the frames past a cut. The rule below is this program's
own and unverified; nothing is ported.

The settings: `auto_amounts`, "off" or "on"; `temporal_smoothing`, 0 to 10 seconds, keyable;
`scene_detect`, "off" or "on". All three start "off", 0 and "off", and a file without them reads
those, so a file written before D-564 draws as it did (FX-SHAUTO-001). After Effects is recalled
to start with Auto Amounts on; "off" is chosen here so the effect as added is D-400's and stays
on the card (document 14 records it, an owner question).

The rule. With `auto_amounts` "off" nothing changes: the sliders are used, and Temporal Smoothing
and Scene Detect do nothing, as After Effects greys them out without Auto Amounts (as recalled).
With it "on", the statistics of D-351 (document 21) are read: the brightness histogram h of the
layer's picture as the effects before this one left it, 256 bins, each pixel showing adding its
covering a to bin floor(255 y + 1/2) of its brightness y = 0.2126 eR + 0.7152 eG + 0.0722 eB on
its straight sRGB-encoded colour; T the sum of h, and y_k = k / 255. Then

    d = sum of h_k max(0, 1 - 2 y_k)^2 / T,   b = sum of h_k max(0, 2 y_k - 1)^2 / T
    shadow amount = 100 min(1, 2 d),          highlight amount = 100 min(1, 2 b)

replace the two sliders, and D-563's rule runs on as before (its midtone contrast and clips
count the frame's own lifted lightness, unchanged). So a picture all black is lifted at 100, one
all mid-grey not at all, one spread evenly from black to white at 33 each way; a picture where
nothing shows has no statistics, both amounts are 0, and it is left as it is.

Temporal Smoothing and Scene Detect are D-351's, unchanged: N = floor(seconds x frame rate) frames
each side of frame n, each frame's statistics those of the layer's picture at that frame with
the effects before this one, a frame outside the layer or where nothing shows passed over; with
Scene Detect each side walked outward and stopped at the first frame whose brightness differs
from the last one counted by more than half (16 groups of 16 bins, half the sum of the
differences). The frames counted are added, each over its own total, and d and b read from the
sum, so they are the mean of each frame's own. On an adjustment layer the frames beneath at other
times are not to hand: each frame is judged by itself, with TEMPORAL_SMOOTHING_SKIPPED each frame.

**This file never runs the build's code path.** It counts with numpy's bincount on the drawings'
exact 8-bit values in double precision and lifts with D-563's double-precision lists, where the
build counts row by row from its single-precision buffers. D-351's asserts keep every value a
thousandth of a bin from a bin's edge and every scene difference a thousandth from the half.

Every case is a project of one composition 16 by 10, eight frames, in
`Fixtures/shadow_highlight_auto/`: the layer `holder`, D-351's run of drawings flickering in
brightness, with a cut to a darker, colder scene at frame 4. The expected pixels are in
`Fixtures/shadow_highlight_auto/expected_shadow_highlight_auto.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/shadow_highlight_auto_reference.py
"""

import json
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import recolor_reference as R  # noqa: E402
import auto_tone_reference as A  # noqa: E402
import shadow_highlight_reference as SH  # noqa: E402
import shadow_highlight_clip_reference as CL  # noqa: E402

W, H, FRAMES = A.W, A.H, A.FRAMES
# D-563's rule reads its picture's size from these; here it is D-351's drawing.
SH.W = CL.W = W
SH.H = CL.H = H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "shadow_highlight_auto"
NEW = ("auto_amounts", "temporal_smoothing", "scene_detect")
NUMBERS = CL.NAMES  # every number D-563's rule reads
START = dict(CL.START, auto_amounts="off", temporal_smoothing=0, scene_detect="off")
RANGE = dict(CL.RANGE, temporal_smoothing=(0, 10))
SMOOTH = A.SMOOTH  # 0.1 seconds at 24 a second: two frames each side


def auto_amounts(s):
    """The shadow and highlight amounts statistics s give; none, nothing."""
    if s is None:
        return 0.0, 0.0
    h = s["hist"][3]
    y = np.arange(256) / 255
    d = float((h * np.maximum(0, 1 - 2 * y) ** 2).sum() / h.sum())
    b = float((h * np.maximum(0, 2 * y - 1) ** 2).sum() / h.sum())
    return 100 * min(1.0, 2 * d), 100 * min(1.0, 2 * b)


def pixels(n):
    """The holder's drawing at frame n, 8-bit, row by row."""
    return [p for row in A.DRAWINGS[f"tone_{A.RUN[n]}"] for p in row]


def stats_at(c, n):
    """D-351's statistics at frame n, its smoothing, scene detect and adjustment layer."""
    return A.stats_at({"temporal_smoothing": c["temporal_smoothing"], "scene_detect": c["scene_detect"],
                       "on": c["on"], "before": None}, n)


def amounts_at(c, n):
    if c["auto_amounts"] != "on":
        return value_at(c["shadow_amount"], n), value_at(c["highlight_amount"], n)
    return auto_amounts(stats_at(c, n))


def render(c, n):
    v = {k: min(RANGE[k][1], max(RANGE[k][0], value_at(c[k], n))) for k in NUMBERS}
    v["shadow_amount"], v["highlight_amount"] = amounts_at(c, n)
    return CL.rule(pixels(n), v)


def plain(c, n):
    return [R.working(p) for p in pixels(n)]


# --- the cases ------------------------------------------------------------------------------

def case(written=NEW, on="holder", **settings):
    c = dict(START)
    c.update(settings)
    c["written"] = written
    c["on"] = on
    return c


CASES = {
    "FX-SHAUTO-001": ("A file written before D-564, without the three settings, at "
                      "Shadow/Highlight's starting settings: the sliders' lift, as D-400 drew it.",
                      case(written=()), (0, 4)),
    "FX-SHAUTO-002": ("The three written at their starting values, off, 0 and off: the same as "
                      "FX-SHAUTO-001.", case(), (0, 4)),
    "FX-SHAUTO-003": ("Auto Amounts on: frame 0, the light warm scene, lifted a little by its own "
                      "lightness; frame 4, the darker cold scene, lifted more.",
                      case(auto_amounts="on"), (0, 4)),
    "FX-SHAUTO-004": ("Auto Amounts on with the sliders at 90 and 90: the sliders are not used, so "
                      "the same as FX-SHAUTO-003.",
                      case(auto_amounts="on", shadow_amount=90, highlight_amount=90), (0, 4)),
    "FX-SHAUTO-005": ("Auto Amounts on, midtone contrast 40 and both clips 5: the auto lift, then "
                      "D-563's steps on the frame's own lightness.",
                      case(auto_amounts="on", midtone_contrast=40, black_clip=5, white_clip=5),
                      (0, 4)),
    "FX-SHAUTO-006": ("Temporal Smoothing 0.1 and Scene Detect on with Auto Amounts off: they do "
                      "nothing without it, so the same as FX-SHAUTO-001.",
                      case(temporal_smoothing=SMOOTH, scene_detect="on"), (0, 4)),
    "FX-SHAUTO-007": ("Auto Amounts on, Temporal Smoothing 0.1 seconds, two frames each side: frame "
                      "0 judged by frames 0 to 2 (the two before are outside the layer); frame 3 by "
                      "1 to 5, across the cut, so lifted more than by itself; frame 7 by 5 to 7.",
                      case(auto_amounts="on", temporal_smoothing=SMOOTH), (0, 3, 7)),
    "FX-SHAUTO-008": ("FX-SHAUTO-007 with Scene Detect: frame 3 judged by 1 to 3 only and frame 4 "
                      "by 4 to 6 only, each side of the cut.",
                      case(auto_amounts="on", temporal_smoothing=SMOOTH, scene_detect="on"), (3, 4)),
    "FX-SHAUTO-009": ("Auto Amounts on, on an adjustment layer above the holder: the frame beneath "
                      "is the drawing, so the same as FX-SHAUTO-003.",
                      case(auto_amounts="on", on="adjust"), (0, 4)),
    "FX-SHAUTO-010": ("Auto Amounts on with Temporal Smoothing 0.1 on an adjustment layer: the "
                      "frames beneath at other times are not to hand, so each frame is judged by "
                      "itself, as FX-SHAUTO-003, with a warning each frame.",
                      case(auto_amounts="on", temporal_smoothing=SMOOTH, on="adjust"), (0, 3)),
}

INVALID = {
    "FX-SHAUTO-011": ("Auto Amounts written \"yes\".", case(auto_amounts="yes")),
    "FX-SHAUTO-012": ("Scene Detect written \"maybe\".", case(scene_detect="maybe")),
    "FX-SHAUTO-013": ("Temporal Smoothing 11 seconds, above 10.", case(temporal_smoothing=11)),
    "FX-SHAUTO-014": ("Temporal Smoothing -1 seconds, below 0.", case(temporal_smoothing=-1)),
}


def effect(c):
    names = CL.NAMES + tuple(n for n in NEW if n in c["written"])
    return {"instance_id": "fx-1", "type_id": "core.shadow_highlight", "enabled": True,
            "parameters": {n: setting_json(c[n]) for n in names}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-tone", spans=A.SPANS, position=(0, 0), out_frame=FRAMES)
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect(c)]
        layers.append(adjust)
    else:
        holder["effects"] = [effect(c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-tone", "kind": "image_sequence", "name": "tone",
                        "pattern": "tone_####.png",
                        "frames": {str(k): f"media/tone_{k}.png" for k in range(1, 6)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, px in A.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(px))
    expected = {"tolerance": A.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in render(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["on"] == "adjust" and c["auto_amounts"] == "on" and c["temporal_smoothing"] != 0:
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} amounts {tuple(round(a, 3) for a in amounts_at(c, int(f)))}" for f in frames))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(c, 0), "4": plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_shadow_highlight_auto.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda f, g, e=1e-12: all(abs(p - q) < e for u, v in zip(f, g)  # noqa: E731
                                     for p, q in zip(u, v))
    on = case(auto_amounts="on")
    s13 = case(auto_amounts="on", temporal_smoothing=SMOOTH)
    s14 = case(auto_amounts="on", temporal_smoothing=SMOOTH, scene_detect="on")

    # The rule's own ends: all black 100 and 0, all mid-grey 0 and 0, an even spread a third each.
    one = lambda k: {"hist": np.array([np.eye(256)[k]] * 4)}  # noqa: E731
    assert auto_amounts(one(0)) == (100.0, 0.0) and auto_amounts(one(255)) == (0.0, 100.0)
    assert auto_amounts({"hist": np.array([np.ones(256)] * 4)})[0] > 33
    assert auto_amounts(None) == (0.0, 0.0)
    # Old files and the starting values are D-400's lift.
    for fx in ("FX-SHAUTO-002", "FX-SHAUTO-006"):
        for f, px in c[fx].items():
            assert near(px, c["FX-SHAUTO-001"][f]), fx
    assert "auto_amounts" not in (OUT / "fx_shauto_001.json").read_text(encoding="utf-8")
    assert near(c["FX-SHAUTO-001"]["0"], CL.rule(pixels(0), dict(CL.START)))
    # The dark scene is lifted more than the light one, and the sliders are not read.
    s0, s4 = amounts_at(on, 0), amounts_at(on, 4)
    assert 0 < s0[0] < s4[0] and s0[0] != CL.START["shadow_amount"]
    for f in ("0", "4"):
        assert near(c["FX-SHAUTO-004"][f], c["FX-SHAUTO-003"][f])
        assert not near(c["FX-SHAUTO-005"][f], c["FX-SHAUTO-003"][f])
        assert near(c["FX-SHAUTO-009"][f], c["FX-SHAUTO-003"][f])
    assert not near(c["FX-SHAUTO-003"]["0"], c["FX-SHAUTO-001"]["0"])
    # The frames each judged by, as the cases say; the amounts their own amounts' mean.
    fs = lambda n: A.frame_stats(A.case(), n)  # noqa: E731
    mean = lambda ns: tuple(np.mean([auto_amounts(fs(n)) for n in ns], axis=0))  # noqa: E731
    for got, ns in ((amounts_at(s13, 0), (0, 1, 2)), (amounts_at(s13, 3), (1, 2, 3, 4, 5)),
                    (amounts_at(s13, 7), (5, 6, 7)), (amounts_at(s14, 3), (1, 2, 3)),
                    (amounts_at(s14, 4), (4, 5, 6))):
        assert np.allclose(got, mean(ns), atol=1e-12, rtol=0), (got, ns)
    assert amounts_at(s13, 3)[0] > amounts_at(on, 3)[0]
    assert not near(c["FX-SHAUTO-008"]["3"], c["FX-SHAUTO-007"]["3"])
    # The adjustment layer with smoothing: each frame by itself.
    assert near(c["FX-SHAUTO-010"]["0"], c["FX-SHAUTO-003"]["0"])
    assert near(c["FX-SHAUTO-010"]["3"], render(on, 3))
    # Every pixel a picture can hold, the covering never changed.
    for fx, v in c.items():
        for f, px in v.items():
            for p, q in zip(px, plain(on, int(f))):
                assert abs(p[3] - q[3]) < 1e-15 and all(-1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
