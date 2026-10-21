"""D-739, B-608: animation presets.

Writes `Fixtures/animation_preset/` and `Fixtures/starter_presets/animation.fxpreset`, and prints
nothing but "checked" (or, with `--table`, document 25's table). Written before any code.

An animation preset is a preset of D-180's file with `"kind": "animation"`. Besides effects it may
hold a layer's `transform` properties, `masks`, and a shape's `trim`, each written as a project
writes them (document 19, "Animation presets"). Applied at a frame, every key in it moves by that
frame less its earliest key, so its first key lands on the frame; its numbers are never scaled to
the layer. A transform property marked `"added": true` has each value added to the layer's own
value of it at that frame.

`host.json` is the project presets are applied to. Its layers are taken from committed fixtures,
so each is one the build already reads:
- `big`, a 1920 by 1080 solid in the middle of a 1920 by 1080 composition, with an opacity key
  of its own at frame 40, which a preset must leave;
- `small`, a 200 by 100 solid at (300, 200) turned 30 degrees, the different size D-739 asks for;
- `lines`, a shape layer of two open lines, the first without trim, the second trimmed;
- `words`, a text layer.

`expected_animation_preset.json` gives, for each case, the preset file, which preset in it (the
first unless named), the layer, the frame, and what the layer's record in the saved project must
then hold, by a path inside the layer's record (`effects/-1` is the last effect):
- `keys`: every key as [frame, value], values in the file's own units (scale in per cent, opacity
  0 to 1); frames exact, values within 1e-9;
- `frames`: the frames of a mask's or a shape's path keys;
- `kinds`: D-69's kind of each key, "bezier" where the file writes none;
- `expression`: the expression's text; `base`: the value with no keys; `length`: a list's length;
  `equals`: exactly this JSON.
A refused file names the diagnostic and a word the message must contain, and nothing is applied.
A refused application names a word the sentence must contain, and the layer must be unchanged.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIX = ROOT / "Fixtures"
OUT = FIX / "animation_preset"
STARTERS = FIX / "starter_presets" / "animation.fxpreset"
EASY = [1 / 3, 0, 2 / 3, 1]


def load(rel):
    return json.loads((FIX / rel).read_text(encoding="utf-8"))


def first_layer(project, kind):
    for comp in project["compositions"]:
        for layer in comp["layers"]:
            if layer["kind"] == kind:
                return copy.deepcopy(layer)
    raise SystemExit(f"no {kind} layer")


def prop(base, keys=(), ease=False):
    """A property record as a project writes one: `keys` are (frame, value)."""
    out = {"base": base, "keyframes": []}
    for frame, value in keys:
        key = {"frame": frame, "value": value, "interp": "ease" if ease else "linear"}
        if ease:
            key["ease"] = EASY
        out["keyframes"].append(key)
    return out


def transform(anchor, position, scale=(100, 100), rotation=0, opacity=1):
    return {
        "anchor": prop(list(anchor)),
        "position": prop(list(position)),
        "scale": prop(list(scale)),
        "rotation": prop(rotation),
        "opacity": prop(opacity),
    }


# The host project.
TEXT = first_layer(load("text_animator/fx_txa_020.json"), "text")
SHAPE = first_layer(load("shape_styles/fx_shp_109.json"), "shape")
MASK = load("projects/keyed_settings_project.json")
MASK = next(m for c in MASK["compositions"] for l in c["layers"] for m in (l.get("masks") or []))
WIPE = next(
    e
    for c in load("linear_wipe/fx_lwipe_001.json")["compositions"]
    for l in c["layers"]
    for e in l["effects"]
    if e["type_id"] == "core.linear_wipe"
)
TYPER = copy.deepcopy(TEXT["effects"][0])


def solid(name, w, h, at, colour):
    return {
        "id": name,
        "kind": "solid",
        "name": name,
        "enabled": True,
        "locked": False,
        "in_frame": 0,
        "out_frame": 96,
        "transform": transform((w / 2, h / 2), at),
        "matte": None,
        "blend_mode": "normal",
        "effects": [],
        "masks": [],
        "solid": {"color": colour, "width": w, "height": h},
    }


def host():
    big = solid("big", 1920, 1080, (960, 540), [0.2, 0.4, 0.8])
    big["transform"]["opacity"] = prop(1, [(40, 0.5)])
    small = solid("small", 200, 100, (300, 200), [0.9, 0.5, 0.1])
    small["transform"]["rotation"] = prop(30)
    lines = copy.deepcopy(SHAPE)
    lines.update(id="lines", name="lines", in_frame=0, out_frame=96)
    lines["transform"] = transform((960, 540), (960, 540))
    second = copy.deepcopy(lines["shapes"][0])
    second["name"] = "Shape 2"
    first = copy.deepcopy(second)
    first["name"] = "Shape 1"
    first.pop("trim")
    lines["shapes"] = [first, second]
    words = copy.deepcopy(TEXT)
    words.update(id="words", name="words", in_frame=0, out_frame=96, effects=[])
    words["transform"] = transform((640, 180), (960, 540))
    layers = [big, small, lines, words]
    return {
        "schema_version": 0,
        "project_id": "proj-animation-preset-host",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [
            {
                "id": "comp-main",
                "name": "comp-main",
                "width": 1920,
                "height": 1080,
                "pixel_aspect_ratio": 1,
                "frame_rate": {"numerator": 24, "denominator": 1},
                "start_frame": 0,
                "duration_frames": 96,
                "work_area": {"start_frame": 0, "end_frame_exclusive": 96},
                "layer_order": [l["id"] for l in layers],
                "layers": layers,
            }
        ],
    }


HOST = host()


def layer_of(name):
    return next(l for l in HOST["compositions"][0]["layers"] if l["id"] == name)


def wipe(keys):
    e = copy.deepcopy(WIPE)
    e["instance_id"] = "anp-wipe"
    e["parameters"].update(completion=prop(100, keys, ease=True), angle=90, feather=40)
    return e


def typewriter():
    e = copy.deepcopy(TYPER)
    e["instance_id"] = "anp-typewriter"
    e["parameters"]["start"] = prop(0, [(0, 0), (24, 100)])
    return e


def animation(name, **parts):
    return {"name": name, "kind": "animation", **parts}


# The starter set D-739 names. Twenty-four frames is a second at the 24 fps the program starts a
# composition at; the numbers are kept, never scaled, whatever the layer or the frame rate.
SLIDES = {
    "left": [-1920, 0],
    "right": [1920, 0],
    "top": [0, -1080],
    "bottom": [0, 1080],
}
STARTER_LIST = [
    (
        animation("Fade in", transform={"opacity": prop(0, [(0, 0), (12, 1)])}),
        "The layer fades up from nothing over half a second.",
        "big",
    ),
    (
        animation("Fade out", transform={"opacity": prop(1, [(0, 1), (12, 0)])}),
        "The layer fades away over half a second, from the playhead.",
        "big",
    ),
    (
        animation(
            "Pop in",
            transform={
                "scale": prop([0, 0], [(0, [0, 0]), (8, [110, 110]), (12, [100, 100])], ease=True)
            },
        ),
        "The layer grows from nothing, overshoots a little and settles at full size.",
        "small",
    ),
] + [
    (
        animation(
            f"Slide in from the {side}",
            transform={
                "position": {
                    **prop([0, 0], [(0, by), (12, [0, 0])], ease=True),
                    "added": True,
                }
            },
        ),
        f"The layer slides in from the {side} and stops where it is now.",
        "small",
    )
    for side, by in SLIDES.items()
] + [
    (
        animation(
            "Spin in",
            transform={
                "rotation": {**prop(0, [(0, -180), (12, 0)], ease=True), "added": True},
                "scale": prop([0, 0], [(0, [0, 0]), (12, [100, 100])], ease=True),
            },
        ),
        "The layer turns in half a turn while it grows to full size.",
        "big",
    ),
    (
        animation(
            "Wiggle shake",
            transform={
                "position": {
                    "base": [0, 0],
                    "keyframes": [],
                    "expression": {"text": "wiggle(12, 15)", "enabled": True},
                }
            },
        ),
        "The layer shakes about where it is, twelve times a second, up to fifteen pixels. An expression, so it has no keys.",
        "big",
    ),
    (
        animation("Typewriter", effects=[typewriter()]),
        "The letters appear one by one over a second. For a text layer.",
        "words",
    ),
    (
        animation(
            "Write-on",
            trim={
                "start": prop(0),
                "end": prop(0, [(0, 0), (24, 100)], ease=True),
                "offset": prop(0),
            },
        ),
        "Each line draws itself along its path over a second. For a shape layer.",
        "lines",
    ),
    (
        animation("Wipe reveal", effects=[wipe([(0, 100), (12, 0)])]),
        "The layer is wiped on from the left over half a second.",
        "big",
    ),
]


def preset_file(*presets):
    return {"preset_file_version": 0, "presets": list(presets)}


def keys_of(record):
    return [[k["frame"], k["value"]] for k in record["keyframes"]]


def moved(record, by):
    return [[f + by, v] for f, v in keys_of(record)]


def added(record, own, by):
    def plus(v):
        return [a + b for a, b in zip(v, own)] if isinstance(v, list) else v + own

    return [[f + by, plus(v)] for f, v in keys_of(record)]


def own(layer, name):
    return layer["transform"][name]["base"]


def first_key(preset):
    """The earliest key anywhere in the preset, or None."""
    frames = []

    def walk(v):
        if isinstance(v, dict):
            if isinstance(v.get("keyframes"), list):
                frames.extend(k["frame"] for k in v["keyframes"])
            for x in v.values():
                walk(x)
        elif isinstance(v, list):
            for x in v:
                walk(x)

    walk({k: v for k, v in preset.items() if k != "name"})
    return min(frames) if frames else None


def expect_applied(preset, layer_name, at):
    """What the layer's record holds after `preset` goes on at `at`, by D-739's rule."""
    layer = layer_of(layer_name)
    first = first_key(preset)
    by = 0 if first is None else at - first
    out = []
    for name, record in preset.get("transform", {}).items():
        path = f"transform/{name}"
        theirs = record.get("keyframes", [])
        mine = layer["transform"][name]
        if theirs:
            new = added(record, own(layer, name), by) if record.get("added") else moved(record, by)
            kept = [k for k in keys_of(mine) if k[0] not in [f for f, _ in new]]
            out.append({"path": path, "keys": sorted(new + kept, key=lambda k: k[0])})
            kinds = {k["frame"] + by: k.get("kind", "bezier") for k in theirs}
            if any(v != "bezier" for v in kinds.values()):
                out.append({"path": path, "kinds": [kinds.get(f, "bezier") for f, _ in out[-1]["keys"]]})
        elif "expression" not in record:
            base = record["base"]
            if record.get("added"):
                o = own(layer, name)
                base = [a + b for a, b in zip(base, o)] if isinstance(base, list) else base + o
            out.append({"path": path, "base": base, "keys": keys_of(mine)})
        if "expression" in record:
            out.append({"path": path, "expression": record["expression"]["text"]})
    if "effects" in preset:
        out.append({"path": "effects", "length": len(layer["effects"]) + len(preset["effects"])})
        for i, e in enumerate(preset["effects"]):
            at_ = i - len(preset["effects"])
            for setting, v in e["parameters"].items():
                if isinstance(v, dict) and v.get("keyframes"):
                    out.append({"path": f"effects/{at_}/parameters/{setting}", "keys": moved(v, by)})
    if "masks" in preset:
        have = len(layer["masks"])
        out.append({"path": "masks", "length": have + len(preset["masks"])})
        for i, m in enumerate(preset["masks"]):
            out.append({"path": f"masks/{have + i}/path", "frames": [f + by for f, _ in keys_of(m["path"])]})
            for key in ("feather_px", "expansion_px", "opacity"):
                if isinstance(m.get(key), dict):
                    out.append({"path": f"masks/{have + i}/{key}", "keys": moved(m[key], by)})
            for key, value in m.items():
                if key not in ("name", "enabled", "inverted", "mode", "opacity", "feather_px", "expansion_px", "path"):
                    out.append({"path": f"masks/{have + i}/{key}", "equals": value})
    if "trim" in preset:
        for i, _ in enumerate(layer["shapes"]):
            for part in ("start", "end", "offset"):
                record = preset["trim"][part]
                if record["keyframes"]:
                    out.append({"path": f"shapes/{i}/trim/{part}", "keys": moved(record, by)})
                else:
                    out.append({"path": f"shapes/{i}/trim/{part}", "base": record["base"], "keys": []})
    return out


CASES = {}
FILES = {}


def case(n, says, file_presets, **rest):
    cid = f"FX-ANP-{n:03d}"
    name = f"fx_anp_{n:03d}.fxpreset"
    FILES[name] = file_presets
    entry = {"file": name, "says": says}
    if "layer" in rest and "apply_refused" not in rest and "refused" not in rest:
        rest["expect"] = expect_applied(file_presets["presets"][0], rest["layer"], rest["at"])
    entry.update(rest)
    CASES[cid] = entry


def starter(name):
    return next(copy.deepcopy(p) for p, _, _ in STARTER_LIST if p["name"] == name)


case(1, "Fade in applied at frame 10: its first key lands on frame 10, and the layer's own key at 40 stays.",
     preset_file(starter("Fade in")), layer="big", at=10)
case(2, "Pop in on a 200 by 100 layer: the same numbers as on a 1920 by 1080 one, never scaled to the layer.",
     preset_file(starter("Pop in")), layer="small", at=10)
case(3, "Pop in on the 1920 by 1080 layer, to set beside FX-ANP-002.",
     preset_file(starter("Pop in")), layer="big", at=10)
case(4, "Slide in from the left on the small layer: the keys are added to where it is, so it ends where it was.",
     preset_file(starter("Slide in from the left")), layer="small", at=10)
case(5, "The same slide on the big layer: the same distance, 1920 pixels, from its own place.",
     preset_file(starter("Slide in from the left")), layer="big", at=10)
case(6, "Wiggle shake has no keys: the expression goes on and nothing moves in time.",
     preset_file(starter("Wiggle shake")), layer="big", at=30)
case(7, "An animation preset of effects only: the wipe's keys move to the frame, where an effect preset's would not.",
     preset_file(starter("Wipe reveal")), layer="big", at=30)
case(8, "The first key is the earliest anywhere in the preset: here the effect's, at 2, before the opacity's at 4.",
     preset_file(animation("Mixed", transform={"opacity": prop(1, [(4, 0), (16, 1)])},
                           effects=[wipe([(2, 100), (8, 0)])])), layer="big", at=20)
case(9, "Write-on on a shape layer: every shape's trim, the untrimmed one's too, takes the keys.",
     preset_file(starter("Write-on")), layer="lines", at=10)
case(10, "Write-on on a solid, which has no shapes: refused, and the layer is unchanged.",
     preset_file(starter("Write-on")), layer="big", at=10, apply_refused="shape layer")


def mask_preset():
    m = copy.deepcopy(MASK)
    m["name"] = "Reveal"
    for key in ("path", "feather_px", "expansion_px"):
        for k, f in zip(m[key]["keyframes"], (5, 15)):
            k["frame"] = f
    m["tension"] = 2
    return animation("Moving mask", masks=[m])


case(11, "A mask goes on after the layer's own, its path and feather keys moved; a field this build does not know, "
         "`tension`, is kept with it and saved, never dropped.",
     preset_file(mask_preset()), layer="small", at=10)
case(12, "Typewriter on a text layer: the Text Animator's Start keys begin at the frame.",
     preset_file(starter("Typewriter")), layer="words", at=10)


def kinds_preset():
    r = prop(0, [(0, 0), (6, 90), (12, 0)], ease=True)
    r["keyframes"][1]["kind"] = "continuous"
    return animation("Swing", transform={"rotation": r})


case(13, "A key's kind (D-69) comes with it.", preset_file(kinds_preset()), layer="small", at=10)
case(14, "A property with neither keys nor an expression sets the layer's value.",
     preset_file(animation("Tilt", transform={"rotation": prop(45)})), layer="small", at=10)
case(15, "Spin in on the small layer, turned 30 degrees: the turn is added to its own, the scale is not.",
     preset_file(starter("Spin in")), layer="small", at=10)

# Refused files.
skew = starter("Fade in")
skew["transform"]["skew"] = prop(0, [(0, 0), (12, 10)])
case(20, "A transform property this build does not have would be lost, so the file is refused.",
     preset_file(skew), refused="PRESET_FILE_INVALID", names="skew")
loop = starter("Fade in")
loop["transform"]["opacity"]["loop"] = "cycle"
case(21, "A field in a property this build would not write back.", preset_file(loop),
     refused="PRESET_FILE_INVALID", names="loop")
velocity = starter("Fade in")
velocity["transform"]["opacity"]["keyframes"][0]["velocity"] = 3
case(22, "A field in one key, likewise.", preset_file(velocity), refused="PRESET_FILE_INVALID", names="velocity")
motion = starter("Fade in")
motion["kind"] = "motion"
case(23, "A kind of preset this build does not have.", preset_file(motion), refused="PRESET_FILE_INVALID",
     names="motion")
plain = starter("Fade in")
plain.pop("kind")
plain["effects"] = [wipe([])]
case(24, "An effect preset, without `kind`, holding a transform is refused as D-180 refuses it.",
     preset_file(plain), refused="PRESET_FILE_INVALID", names="transform")
case(25, "An animation preset with nothing in it.", preset_file(animation("Nothing")),
     refused="PRESET_FILE_INVALID", names="Nothing")
split = animation("Apart", transform={"position": {"x": prop(0, [(0, 0), (12, 100)]), "y": prop(0)}})
case(26, "A position taken apart into X and Y is not one a preset holds.", preset_file(split),
     refused="PRESET_FILE_INVALID", names="position")
bounce = starter("Write-on")
bounce["trim"]["bounce"] = 1
case(27, "A field in a trim this build would not write back.", preset_file(bounce),
     refused="PRESET_FILE_INVALID", names="bounce")
thumb = starter("Fade in")
thumb["thumbnail"] = "fade.png"
case(28, "Something else the rule does not name, on an animation preset.", preset_file(thumb),
     refused="PRESET_FILE_INVALID", names="thumbnail")
empty = animation("Hollow", transform={})
case(29, "An empty transform is nothing in it too.", preset_file(empty), refused="PRESET_FILE_INVALID",
     names="Hollow")

# Every starter applied at frame 10 to the layer it is for.
for n, (p, about, layer) in enumerate(STARTER_LIST, start=1):
    cid = f"FX-ANP-S{n:02d}"
    CASES[cid] = {
        "file": "../starter_presets/animation.fxpreset",
        "preset": p["name"],
        "about": about,
        "says": f"The starter preset \"{p['name']}\" applied at frame 10 to `{layer}`.",
        "layer": layer,
        "at": 10,
        "expect": expect_applied(p, layer, 10),
    }


def check():
    starters = [p for p, _, _ in STARTER_LIST]
    names = [p["name"] for p in starters]
    assert len(names) == len(set(names)) == 12, names
    for p in starters:
        assert p["kind"] == "animation"
        assert set(p) <= {"name", "kind", "effects", "transform", "masks", "trim"}, p
        f = first_key(p)
        assert f in (0, None), (p["name"], f)
    for cid, c in CASES.items():
        if "expect" in c:
            assert c["expect"], cid
    # FX-ANP-001's first key is the frame, and FX-ANP-002 and 003 give the same numbers.
    one = CASES["FX-ANP-001"]["expect"][0]["keys"]
    assert one == [[10, 0], [22, 1], [40, 0.5]], one
    assert CASES["FX-ANP-002"]["expect"] == CASES["FX-ANP-003"]["expect"]
    assert CASES["FX-ANP-004"]["expect"][0]["keys"][0] == [10, [-1620, 200]]
    assert CASES["FX-ANP-008"]["expect"][0]["keys"] == [[22, 0], [34, 1], [40, 0.5]]


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def table():
    rows = ["| case | says | applied | the build must give |", "| --- | --- | --- | --- |"]
    for cid, c in CASES.items():
        if "refused" in c:
            where, want = "read only", f"refused, `{c['refused']}`, naming {c['names']}"
        elif "apply_refused" in c:
            where, want = f"`{c['layer']}` at {c['at']}", f"refused, naming {c['apply_refused']}; nothing changed"
        else:
            where = f"`{c['layer']}` at {c['at']}"
            want = "; ".join(
                f"{e['path']}: " + ", ".join(f"{k} {json.dumps(v)}" for k, v in e.items() if k != "path")
                for e in c["expect"]
            )
        rows.append(f"| {cid} | {c['says']} | {where} | {want} |")
    print("\n".join(rows))


if __name__ == "__main__":
    check()
    if "--table" in sys.argv:
        table()
        raise SystemExit
    write(OUT / "host.json", HOST)
    for name, value in FILES.items():
        write(OUT / name, value)
    write(STARTERS, preset_file(*[p for p, _, _ in STARTER_LIST]))
    write(OUT / "expected_animation_preset.json", {"cases": CASES})
    print("checked")
