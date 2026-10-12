"""CC Force Motion Blur, worked a second way.

D-581 (a), accepted by the owner on 2026-10-12 ("go with your recommendations on all six"),
adds CC Force Motion Blur (`core.force_motion_blur`), after CycoreFX's CC Force Motion Blur
(Time): motion blur for what moves inside a composition layer. The CycoreFX manual names the
four controls, says the effect is meant for a pre-composed layer, that it uses the composition's
shutter angle only where the composition, the layer and the compositions inside have motion blur
switched on, and that it ignores the shutter phase. It publishes no formula, so the rule and the
settings when added are ours.

At the holder's composition frame n, its settings held inside their ranges at n:

1. Only a composition layer is blurred. On any other layer, an adjustment layer included, the
   effect leaves the layer as it is and each frame says EFFECT_UNSUPPORTED. So does a holder with
   a Time Remap, whose moments this rule does not yet say how to read.
2. The shutter angle A is `shutter_angle` when `override_native_shutter` is `on`; otherwise the
   composition's own shutter angle when the composition's motion blur and the holder's switch
   are both on; otherwise 0.
3. N = 1 when A is 0, else floor(`motion_blur_samples`). The shutter opens on the frame (the
   phase taken as 0, as CycoreFX ignores it), and the moments are n + (A / 360) * k / N for
   k = 0 .. N-1, so the first is the frame itself.
4. At moment k the composition inside the holder is drawn at its whole source frame with every
   key inside it -- transforms, parents, opacity, effect settings -- read (A / 360) * (k / N) *
   100 / s frames later, s the holder's time stretch. Drawings, in and out points, text, shapes
   and expressions hold at the whole frame.
5. The N pictures are summed in linear premultiplied light and divided by N. The result replaces
   what effects before it made, as CC Wide Time's does, and the holder's transform, its own
   motion blur, opacity and blend then apply as for any layer.
6. `native_motion_blur` `on`, as added: the compositions inside keep their own motion blur, each
   moment drawn with it. `off`: they are drawn with it switched off.

`motion_blur_samples` 1 to 64, 8 when added; `override_native_shutter` `off` or `on`, `off`
when added; `shutter_angle` 0 to 720, 180 when added; `native_motion_blur` `off` or `on`, `on`
when added. The two numbers are keyable.

**This file never runs the build's code path.** It reads the very project files it writes and
draws each frame pixel by pixel through `tools/motion_blur_reference.py`'s slide and resampling,
reading each key at the moment's time directly where the build shifts every key time by one
amount.

Every case is a project whose composition `comp-main` is 20 pixels by 1, five frames, in
`Fixtures/force_motion_blur/`; the holder shows `comp-in`, also 20 by 1, where a bar four pixels
wide runs 16 pixels a frame, its left edge at 6 on frame 1. The expected pixels are in
`Fixtures/force_motion_blur/expected_force_motion_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/force_motion_blur_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import motion_blur_reference as M  # noqa: E402
from motion_blur_reference import prop, key, png, shifted, over, sample_times, slide  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "force_motion_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 20, 1, 5
CLEAR = [0.0] * 4
TYPE = "core.force_motion_blur"
ON = M.shutter()  # 180, phase -90, 4 samples
MOVING = M.MOVING  # 16 pixels a frame, the left edge at 6 on frame 1
LATE = [key(1, [-10, 0]), key(3, [22, 0])]  # the same run a frame later


def setting(v, n):
    return M.value_at(v, n) if isinstance(v, dict) else v


def keyed_setting(*keys):
    return prop(keys[0][1], [key(f, v) for f, v in keys])


# --- the rule -------------------------------------------------------------------------------

RANGES = {"motion_blur_samples": (1, 64), "shutter_angle": (0, 720)}
WORDS = {"override_native_shutter": ("off", "on"), "native_motion_blur": ("off", "on")}


def valid(params):
    """Every key and every plain value inside its range, every word one of its two."""
    for k, (lo, hi) in RANGES.items():
        v = params[k]
        values = [x["value"] for x in v["keyframes"]] + [v["base"]] if isinstance(v, dict) else [v]
        if not all(lo <= x <= hi for x in values):
            return False
    return all(params[k] in words for k, words in WORDS.items())


def effect_of(layer):
    fx = [e for e in layer["effects"] if e["type_id"] == TYPE and e["enabled"]]
    return fx[0]["parameters"] if fx else None


def applies(layer):
    """The settings when the effect blurs this layer; None when it leaves the layer alone."""
    p = effect_of(layer)
    if p is None or not valid(p) or layer["kind"] != "composition" or "time_remap" in layer:
        return None
    return p


def moments(comp, layer, p, n):
    """Steps 2 and 3: the nudges, in the holder's source frames, of each moment."""
    blur = comp.get("motion_blur", M.OFF)
    if p["override_native_shutter"] == "on":
        a = setting(p["shutter_angle"], n)
    elif blur["enabled"] and layer.get("motion_blur"):
        a = blur["shutter_angle"]
    else:
        a = 0
    count = 1 if a == 0 else floor(setting(p["motion_blur_samples"], n))
    stretch = layer.get("time_stretch", 100)
    return [(a / 360) * (k / count) * 100 / stretch for k in range(count)]


def local_frame(layer, n):
    if "time_remap" in layer:
        return floor(M.value_at(layer["time_remap"], n))
    t = (n - layer["in_frame"]) * 100 / layer.get("time_stretch", 100) + layer["source_offset_frames"]
    return floor(t)


def source(project, layer, n, nudge, native):
    """The layer's own picture at whole frame n, before its transform."""
    if layer["kind"] == "composition":
        return render(project, layer["composition_id"], local_frame(layer, n), nudge, native)
    return M.source(project, layer, n)


def holder_source(project, comp, layer, n):
    """Steps 4 and 5 for a holder the effect blurs; its plain picture otherwise."""
    p = applies(layer)
    if p is None:
        return source(project, layer, n, 0.0, True)
    native = p["native_motion_blur"] == "on"
    drawn = [source(project, layer, n, d, native) for d in moments(comp, layer, p, n)]
    return [[sum(d[i][c] for d in drawn) / len(drawn) for c in range(4)] for i in range(W)]


def picture(project, comp, layers, layer, n, blur, nudge, native, top):
    """One layer at frame n, every key read `nudge` later; `top` marks comp-main's layers."""
    if not layer["in_frame"] <= n < layer["out_frame"]:
        return None
    if top and effect_of(layer) is not None:
        src = holder_source(project, comp, layer, n)
    else:
        src = source(project, layer, n, nudge, native)
    times = sample_times(n, blur) if layer.get("motion_blur") else [n]
    slides = [slide(comp, layers, layer, t + nudge) for t in times]
    if all(s == slides[0] for s in slides):
        return shifted(src, slides[0])
    drawn = [shifted(src, s) for s in slides]
    return [[sum(d[i][c] for d in drawn) / len(drawn) for c in range(4)] for i in range(W)]


def render(project, comp_id, n, nudge=0.0, native=True):
    """A composition at frame n. Inside the effect (`nudge` set, or `native` off) its own blur
    is used only when `native`; comp-main's always is."""
    top = comp_id == "comp-main"
    comp = next(c for c in project["compositions"] if c["id"] == comp_id)
    blur = comp.get("motion_blur", M.OFF) if (native or top) else M.OFF
    layers = {l["id"]: l for l in comp["layers"]}
    frame = [CLEAR[:] for _ in range(W)]
    for lid in comp["layer_order"]:
        layer = layers[lid]
        if layer["kind"] in ("null", "adjustment"):
            continue
        pic = picture(project, comp, layers, layer, n, blur, nudge, native, top)
        if pic is None:
            continue
        o = M.value_at(layer["transform"]["opacity"], n + nudge)
        frame = [over([c * o for c in p], d) for p, d in zip(pic, frame)]
    return frame


def plain(project, n):
    """The frame with the effect left out."""
    q = json.loads(json.dumps(project))
    for c in q["compositions"]:
        for l in c["layers"]:
            l["effects"] = [e for e in l["effects"] if e["type_id"] != TYPE]
    return render(q, "comp-main", n)


# --- the project files ----------------------------------------------------------------------

def fmb(samples=8, override="off", angle=180, native="on"):
    return {"instance_id": "fx-1", "type_id": TYPE, "enabled": True,
            "parameters": {"motion_blur_samples": samples, "override_native_shutter": override,
                           "shutter_angle": angle, "native_motion_blur": native}}


STANDARD = dict(samples=4, override="on", angle=180)  # four moments, k / 8 of a frame apart


def five(record):
    record["out_frame"] = FRAMES
    return record


def comp(id, layers, blur=None):
    c = M.composition(id, W, layers, blur)
    c.update({"duration_frames": FRAMES, "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES}})
    return c


def holder(fx, position=(0, 0), inner="comp-in", time_stretch=None, **kw):
    h = five(M.nested("holder", position=list(position) if isinstance(position, tuple) else position, **kw))
    h["composition_id"] = inner
    if time_stretch is not None:
        h["time_stretch"] = time_stretch
    if fx is not None:
        h["effects"] = [fx]
    return h


def bar(**kw):
    return five(M.solid("bar", **kw))


def project(name, main_layers, inner_layers=None, main_blur=None, inner_blur=None, extra=()):
    comps = [comp("comp-main", main_layers, main_blur)]
    if inner_layers is not None:
        comps.append(comp("comp-in", inner_layers, inner_blur))
    comps.extend(extra)
    p = {"schema_version": 0, "project_id": "proj-" + name.lower(),
         "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
         "assets": [], "compositions": comps}
    if any(l["kind"] == "raster" for c in comps for l in c["layers"]):
        p["assets"].append({"id": "asset-bar", "kind": "image_sequence", "name": "bar",
                            "pattern": "bar_####.png",
                            "frames": {"1": "media/bar_0001.png", "2": "media/bar_0002.png"},
                            "interpretation": {"color_space": "srgb", "alpha": "straight"}})
    return p


def simple(name, fx, holder_kw=None, main_blur=None, inner=None, inner_blur=None):
    """The holder over `comp-in`, which holds the moving bar unless `inner` says otherwise."""
    return project(name, [holder(fx, **(holder_kw or {}))], inner or [bar()], main_blur, inner_blur)


def red_then_blue():
    r = five(M.raster("bar"))
    r["exposure_spans"] = [{"start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1},
                           {"start_frame": 1, "end_frame_exclusive": FRAMES, "drawing_number": 2}]
    return r


def adjusted(name):
    h = holder(None)
    adj = five(M.common("adjust", "adjustment", [0, 0])[0])
    adj = M.finish(adj, M.transform([0, 0]), {})
    adj["effects"] = [fmb(**STANDARD)]
    return project(name, [h, adj], [bar()])


def two_deep(name):
    mid = holder(None)
    mid["id"], mid["name"] = "mid", "mid"
    return project(name, [holder(fmb(**STANDARD), inner="comp-mid")], [bar()],
                   extra=[comp("comp-mid", [mid])])


def solid_holder(name):
    s = bar(position=[6, 0])
    s["id"] = s["name"] = "holder"
    s["effects"] = [fmb(**STANDARD)]
    return project(name, [s])


def remapped(name):
    h = holder(fmb(**STANDARD))
    h["time_remap"] = prop(0, [key(0, 0), key(4, 4)])
    return project(name, [h], [bar()])


CASES = {
    "FX-FMB-001": ("As added (8 samples, the composition's 180 degrees, its blur and the holder's "
                   "switch on): on frame 1 the bar is drawn at 6 to 13, a pixel apart, an eighth "
                   "each. The holder itself is still, so its own blur draws it once.",
                   simple("FX-FMB-001", fmb(), {"blur": True}, ON), (1,)),
    "FX-FMB-002": ("Override on at 180 degrees and 4 samples, in a composition whose blur is off "
                   "and a holder whose switch is off: the bar at 6, 8, 10 and 12, a quarter each.",
                   simple("FX-FMB-002", fmb(**STANDARD)), (1, 2)),
    "FX-FMB-003": ("Override off and the composition's blur off: no shutter, so the bar sharp at "
                   "6, as without the effect.",
                   simple("FX-FMB-003", fmb(), {"blur": True}, M.shutter(enabled=False)), (1,)),
    "FX-FMB-004": ("Override off and the holder's switch off: sharp at 6.",
                   simple("FX-FMB-004", fmb(), {}, ON), (1,)),
    "FX-FMB-005": ("Override on at 360 degrees, 4 samples: a whole frame, the bar at 6, 10, 14 "
                   "and 18, the last cut at the edge.",
                   simple("FX-FMB-005", fmb(4, "on", 360)), (1,)),
    "FX-FMB-006": ("Override on at 0 degrees: one moment, the frame itself.",
                   simple("FX-FMB-006", fmb(8, "on", 0)), (1,)),
    "FX-FMB-007": ("One sample: the frame itself, whatever the angle.",
                   simple("FX-FMB-007", fmb(1, "on", 180)), (1,)),
    "FX-FMB-008": ("2.9 samples count as 2: the bar at 6 and 10, a half each.",
                   simple("FX-FMB-008", fmb(2.9, "on", 180)), (1,)),
    "FX-FMB-009": ("Three samples at 180 degrees: moments a sixth of a frame apart, the bar at 6, "
                   "8 2/3 and 11 1/3, so its edges fall between pixels.",
                   simple("FX-FMB-009", fmb(3, "on", 180)), (1,)),
    "FX-FMB-010": ("The bar still at 6 with its opacity keyed 0 to 1 over frames 0 to 2: read at "
                   "each moment, 0.5, 0.5625, 0.625 and 0.6875, so 0.59375 on average.",
                   simple("FX-FMB-010", fmb(**STANDARD),
                          inner=[bar(position=M.STILL, opacity=M.FADE)]), (1,)),
    "FX-FMB-011": ("A held key at 720 degrees, 4 samples: the bar sits at 2 until frame 1, then "
                   "12; on frame 0 the moments are 0, 0.5, 1 and 1.5, so it is seen twice at "
                   "half.", simple("FX-FMB-011", fmb(4, "on", 720),
                                   inner=[bar(position=M.HELD)]), (0,)),
    "FX-FMB-012": ("A drawing red on frame 0 and blue after, at 720 degrees on frame 0: the "
                   "moments reach past frame 1 but the drawing holds, so all four are red.",
                   simple("FX-FMB-012", fmb(4, "on", 720), inner=[red_then_blue()]), (0,)),
    "FX-FMB-013": ("Native Motion Blur on, with comp-in's own blur on (180, -90, 4) and the bar's "
                   "switch on: each of the four moments is itself blurred, the bar at 3, 5, 7 "
                   "and 9 plus 2k.",
                   simple("FX-FMB-013", fmb(**STANDARD), inner=[bar(blur=True)], inner_blur=ON),
                   (1,)),
    "FX-FMB-014": ("The same with Native Motion Blur off: comp-in's blur is switched off inside, "
                   "so FX-FMB-002.",
                   simple("FX-FMB-014", fmb(**STANDARD, native="off"), inner=[bar(blur=True)],
                          inner_blur=ON), (1,)),
    "FX-FMB-015": ("The bar still on a null running 16 pixels a frame: parents are read at each "
                   "moment too, so FX-FMB-002.",
                   simple("FX-FMB-015", fmb(**STANDARD),
                          inner=[five(M.null(position=M.RIG)),
                                 bar(position=[56, 50], parent="rig")]), (1,)),
    "FX-FMB-016": ("The bar's in point at frame 2, its run a frame later (at 6 on frame 2), at "
                   "720 degrees: on frame 1 the moments reach frame 2.5 but the in point holds, "
                   "so nothing; on frame 2 the moments 2 to 3.5 put it at 6, 14, 22 and 30.",
                   simple("FX-FMB-016", fmb(4, "on", 720),
                          inner=[bar(in_frame=2, position=LATE)]), (1, 2)),
    "FX-FMB-017": ("Two levels deep: the holder shows comp-mid, which shows comp-in. The moments "
                   "reach every level, so FX-FMB-002.", two_deep("FX-FMB-017"), (1,)),
    "FX-FMB-018": ("The holder itself moving 4 pixels a frame with its own switch on, in a "
                   "composition blurred at 180, -90, 4, override off and 4 samples: the effect's "
                   "picture is FX-FMB-002's, then the holder's own blur moves it at -0.75, -0.25, "
                   "0.25 and 0.75.",
                   simple("FX-FMB-018", fmb(4), {"blur": True,
                                                 "position": [key(0, [-4, 0]), key(2, [4, 0])]},
                          ON), (1,)),
    "FX-FMB-019": ("The holder stretched to 200%: on frame 2 it shows comp-in's frame 1, and its "
                   "moments are half as far apart there, the bar at 6, 7, 8 and 9.",
                   simple("FX-FMB-019", fmb(**STANDARD), {"time_stretch": 200}), (2,)),
    "FX-FMB-020": ("Samples keyed from 1 at frame 0 to 8 at frame 2: 4.5 on frame 1, so four "
                   "moments, FX-FMB-002's frame; one on frame 0.",
                   simple("FX-FMB-020", fmb(keyed_setting((0, 1), (2, 8)), "on", 180)), (0, 1)),
    "FX-FMB-021": ("The effect on a solid: only a composition layer is blurred, so the solid is "
                   "drawn as it is, and each frame says so.", solid_holder("FX-FMB-021"), (1,),
                   "EFFECT_UNSUPPORTED"),
    "FX-FMB-022": ("The effect on an adjustment layer above the holder: the composition below is "
                   "drawn sharp, and each frame says the effect does nothing there.",
                   adjusted("FX-FMB-022"), (1,), "EFFECT_UNSUPPORTED"),
    "FX-FMB-023": ("A holder with a Time Remap (here one that changes nothing): not blurred yet, "
                   "drawn as it is, and each frame says so.", remapped("FX-FMB-023"), (1,),
                   "EFFECT_UNSUPPORTED"),
}

INVALID = {
    "FX-FMB-024": ("Motion Blur Samples 0, below 1.", fmb(0, "on")),
    "FX-FMB-025": ("Motion Blur Samples 65, above 64.", fmb(65, "on")),
    "FX-FMB-026": ("Shutter Angle 721, above 720.", fmb(4, "on", 721)),
    "FX-FMB-027": ("Shutter Angle -1, below 0.", fmb(4, "on", -1)),
    "FX-FMB-028": ("Override Native Shutter written \"maybe\".", fmb(4, "maybe")),
    "FX-FMB-029": ("Native Motion Blur written \"maybe\".", fmb(4, "on", 180, "maybe")),
    "FX-FMB-030": ("Motion Blur Samples keyed to 100 at frame 4.",
                   fmb(keyed_setting((0, 8), (4, 100)), "on")),
}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def file_name(fx):
    return f"{fx.lower().replace('-', '_')}.json"


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in M.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, p, frames, *warn) in CASES.items():
        rendered = {str(f): render(p, "comp-main", f) for f in frames}
        entry = {"says": says, "project": write(file_name(fx), p), "frames": rendered}
        if warn:
            entry["frame_warning"] = warn[0]
        expected["cases"][fx] = entry
        h = next(l for c in p["compositions"] for l in c["layers"] if effect_of(l) is not None)
        main_comp = p["compositions"][0]
        times = {f: ([round(f + d * h.get("time_stretch", 100) / 100, 6)
                      for d in moments(main_comp, h, applies(h), f)] if applies(h) else [f])
                 for f in frames}
        print(f"{fx}: moments {times}; " + ", ".join(
            f"frame {f} {sum(px[i] != plain(p, int(f))[i] for i in range(W))} changed"
            for f, px in rendered.items()))
    for fx, (says, fx_json) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        p = simple(fx, fx_json)
        expected["cases"][fx] = {"says": says, "project": write(file_name(fx), p),
                                 "frames": {"0": plain(p, 0), "1": plain(p, 1)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_force_motion_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b) for p, q in zip(u, v))  # noqa: E731
    colour = M.BAR + [1.0]

    def bars(*lefts, weight=None, rgb=colour):
        """Bars four wide at whole-pixel lefts, each weighted 1 / len(lefts)."""
        w = weight or 1 / len(lefts)
        out = [CLEAR[:] for _ in range(W)]
        for x0 in lefts:
            for x in range(x0, x0 + 4):
                if 0 <= x < W:
                    out[x] = [a + w * b for a, b in zip(out[x], rgb)]
        return out

    assert near(c["FX-FMB-001"]["1"], bars(*range(6, 14)))
    assert near(c["FX-FMB-002"]["1"], bars(6, 8, 10, 12))
    assert c["FX-FMB-002"]["2"] == [CLEAR] * W  # the bar is past the edge by frame 2
    for fx in ("FX-FMB-003", "FX-FMB-004", "FX-FMB-006", "FX-FMB-007"):
        assert near(c[fx]["1"], bars(6)), fx
    assert near(c["FX-FMB-005"]["1"], bars(6, 10, 14, 18))
    assert near(c["FX-FMB-008"]["1"], bars(6, 10))
    assert not near(c["FX-FMB-009"]["1"], c["FX-FMB-002"]["1"])
    assert near(c["FX-FMB-010"]["1"], bars(6, weight=0.59375))
    assert near(c["FX-FMB-011"]["0"], bars(2, 12))
    assert near(c["FX-FMB-012"]["0"], bars(-10, -2, 6, 14, rgb=[1.0, 0.0, 0.0, 1.0]))
    assert near(c["FX-FMB-013"]["1"], [[sum(p) / 4 for p in zip(*px)] for px in zip(
        *[bars(3 + 2 * k, 5 + 2 * k, 7 + 2 * k, 9 + 2 * k) for k in range(4)])])
    for fx in ("FX-FMB-014", "FX-FMB-015", "FX-FMB-017", "FX-FMB-020"):
        assert near(c[fx]["1"], c["FX-FMB-002"]["1"]), fx
    assert c["FX-FMB-016"]["1"] == [CLEAR] * W
    assert near(c["FX-FMB-016"]["2"], bars(6, 14, weight=0.25))
    assert near(c["FX-FMB-019"]["2"], bars(6, 7, 8, 9))
    assert near(c["FX-FMB-020"]["0"], bars(-10))
    for fx in ("FX-FMB-021", "FX-FMB-022", "FX-FMB-023"):
        assert near(c[fx]["1"], bars(6)), fx
    # The holder's own blur moves FX-FMB-002's picture by -0.75 to 0.75 and averages.
    base = c["FX-FMB-002"]["1"]
    held = [shifted(base, s) for s in (-0.75, -0.25, 0.25, 0.75)]
    assert near(c["FX-FMB-018"]["1"], [[sum(p) / 4 for p in zip(*px)] for px in zip(*held)])
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
