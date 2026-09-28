"""D-181, B-117: the starter presets the program comes with.

Writes `Fixtures/starter_presets/` and prints nothing but "checked". Written before any code.

`starter.fxpreset` is the starter presets themselves, in the preset file format of D-180
(document 19, "Effect preset files"). The build ships this file as it is: it is the one place
the starter presets are written, so changing one is a specification decision, made here.

`expected_starter_presets.json` says what the build must show: the presets in this order, each
with its effects by type, the sentence the Effects panel shows for it, and where the build's
pictures put it: on `layer-3` of the reference shot, whose shapes have hard edges, or on an
adjustment layer above all four
when the preset is for a whole picture.

The rules the build keeps (document 19, "Starter presets"):
- the starter presets read whole by `persist::read_presets`, as an imported file would;
- each effect's settings are in range, so each preset draws where it is applied, and each one
  changes the reference shot's picture;
- there are six to ten of them, as the owner asked;
- they are listed first under Presets, marked as built in; they cannot be removed; they can be
  copied, and the copy is an ordinary preset of the owner's;
- no preset of the owner's has a starter preset's name: saving under one is refused, an import
  gives the next free number, and a preset the window kept from before, whose name a new
  starter preset now has, is given the next free number too, and told so;
- Export presets writes the owner's presets only; a starter preset can be exported alone.

The effects are taken from committed fixture projects, so each is one the build already reads,
and only their settings are changed here. **This file never runs the build's code path.**
Fixtures are read-only to implementation work.
"""
import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "starter_presets"

# The fixture each effect is taken from, so every field but its settings is one the build reads.
SOURCES = {
    "core.bloom": ("bloom", "fx_bloom_001.json"),
    "core.color_balance": ("color_balance", "fx_balance_001.json"),
    "core.brightness_contrast": ("brightness_contrast", "fx_bricon_001.json"),
    "core.vignette": ("vignette", "fx_vignette_001.json"),
    "core.gradient": ("gradient", "fx_grad_001.json"),
    "core.drop_shadow": ("drop_shadow", "fx_shadow_001.json"),
    "core.rim_light": ("rim_light", "fx_rim_001.json"),
    "core.gradient_map": ("gradient_map", "fx_gradmap_001.json"),
    "core.noise": ("noise", "fx_noise_001.json"),
    "core.exposure_flicker": ("exposure_flicker", "fx_flicker_001.json"),
    "core.cross_glare": ("cross_glare", "fx_glare_001.json"),
    "core.chromatic_aberration": ("chromatic_aberration", "fx_chroma_001.json"),
    "core.camera_shake": ("camera_shake", "fx_shake_001.json"),
    "core.diffusion": ("diffusion", "fx_diffuse_001.json"),
    "core.vibrance": ("vibrance", "fx_vibrance_001.json"),
    "core.speed_lines": ("speed_lines", "fx_speed_001.json"),
}

# Each setting's range in this build (document 21), so a starter preset never carries one outside
# it. Only the settings the presets below change are listed.
RANGES = {
    ("core.bloom", "threshold"): (0, 100), ("core.bloom", "radius"): (0, 500),
    ("core.bloom", "intensity"): (0, 10),
    ("core.color_balance", "shadows"): (-100, 100), ("core.color_balance", "midtones"): (-100, 100),
    ("core.color_balance", "highlights"): (-100, 100),
    ("core.brightness_contrast", "brightness"): (-150, 150),
    ("core.brightness_contrast", "contrast"): (-100, 100),
    ("core.vignette", "amount"): (0, 100), ("core.vignette", "size"): (1, 200),
    ("core.vignette", "roundness"): (0, 100), ("core.vignette", "softness"): (0, 100),
    ("core.gradient", "start"): (-1000, 1000), ("core.gradient", "end"): (-1000, 1000),
    ("core.gradient", "start_opacity"): (0, 100), ("core.gradient", "end_opacity"): (0, 100),
    ("core.drop_shadow", "opacity"): (0, 100), ("core.drop_shadow", "direction"): (-3600, 3600),
    ("core.drop_shadow", "distance"): (0, 1000), ("core.drop_shadow", "softness"): (0, 500),
    ("core.rim_light", "direction"): (-3600, 3600), ("core.rim_light", "width"): (0, 100),
    ("core.rim_light", "softness"): (0, 100), ("core.rim_light", "intensity"): (0, 100),
    ("core.gradient_map", "midpoint"): (1, 99), ("core.gradient_map", "amount"): (0, 100),
    ("core.noise", "amount"): (0, 100),
    ("core.exposure_flicker", "amount"): (0, 4), ("core.exposure_flicker", "hold"): (1, 100),
    ("core.cross_glare", "threshold"): (0, 100), ("core.cross_glare", "length"): (0, 1000),
    ("core.cross_glare", "points"): (1, 8), ("core.cross_glare", "intensity"): (0, 10),
    ("core.chromatic_aberration", "amount"): (0, 100),
    ("core.camera_shake", "amount"): (0, 1000), ("core.camera_shake", "rotation"): (0, 45),
    ("core.camera_shake", "hold"): (1, 100),
    ("core.diffusion", "radius"): (0, 500), ("core.diffusion", "amount"): (0, 100),
    ("core.vibrance", "vibrance"): (-100, 100), ("core.vibrance", "saturation"): (-100, 100),
    ("core.speed_lines", "count"): (4, 1000), ("core.speed_lines", "thickness"): (0, 30),
    ("core.speed_lines", "inner"): (0, 100000), ("core.speed_lines", "opacity"): (0, 100),
}
WORDS = {
    ("core.bloom", "streaks"): ("none", "cross", "star"),
    ("core.gradient", "shape"): ("linear", "radial"),
    ("core.gradient", "blend"): ("normal", "multiply", "screen", "add"),
    ("core.rim_light", "blend"): ("normal", "add", "screen", "multiply"),
    ("core.noise", "mode"): ("mono", "color"),
    ("core.noise", "animate"): ("on", "off"),
    ("core.diffusion", "blend"): ("screen", "lighten", "normal"),
}


def effect(type_id, n, **settings):
    """The fixture's effect of `type_id` with `settings` changed, and its own instance id."""
    folder, file = SOURCES[type_id]
    project = json.loads((ROOT / "Fixtures" / folder / file).read_text(encoding="utf-8"))
    found = next(fx for c in project["compositions"] for layer in c["layers"]
                 for fx in layer.get("effects", []) if fx["type_id"] == type_id)
    fx = copy.deepcopy(found)
    fx["instance_id"] = f"starter-{n}"
    for name, value in settings.items():
        assert name in fx["parameters"], (type_id, name)
        fx["parameters"][name] = value
    return fx


# (name, where the pictures show it, the sentence the panel shows, effects)
PRESETS = [
    ("Soft bloom", "adjustment",
     "A gentle glow off the brightest parts of the picture. Best on an adjustment layer.",
     [effect("core.bloom", "1-1", threshold=65, radius=30, intensity=0.8)]),
    ("Night", "adjustment",
     "Day painted as night: cooler, darker, the corners fall away. Best on an adjustment layer.",
     [effect("core.color_balance", "2-1", shadows=[-15, 0, 35], midtones=[-30, -10, 40], highlights=[-15, 0, 20]),
      effect("core.vibrance", "2-2", vibrance=0, saturation=-35),
      effect("core.brightness_contrast", "2-3", brightness=-60, contrast=15),
      effect("core.vignette", "2-4", amount=55, softness=60)]),
    ("Sunset", "adjustment",
     "Warm evening light falling from the top of the frame. Best on an adjustment layer.",
     [effect("core.color_balance", "3-1", shadows=[10, -5, -10], midtones=[25, 5, -20], highlights=[15, 5, -15]),
      effect("core.gradient", "3-2", shape="linear", start=[50, 0], end=[50, 100], start_color="#ff9a3c",
             end_color="#ffffff", start_opacity=45, end_opacity=0, blend="screen")]),
    ("Cel shadow", "layer",
     "A hard, flat shadow down and to the right, in a deep violet rather than black. For a character layer.",
     [effect("core.drop_shadow", "4-1", color="#1a1030", opacity=70, direction=135, distance=20, softness=0)]),
    ("Rim light", "layer",
     "A warm light catching the upper right edge. For a character layer.",
     [effect("core.rim_light", "5-1", color="#ffe8c0", direction=45, width=6, softness=2, intensity=100, blend="screen")]),
    ("Old film", "adjustment",
     "Sepia, grain, dark corners and a slight flicker. Best on an adjustment layer.",
     [effect("core.gradient_map", "6-1", shadow_color="#2b1d0e", midtone_color="#8a6a43",
             highlight_color="#f3e3c3", midpoint=50, amount=100),
      effect("core.noise", "6-2", amount=12, mode="mono", animate="on"),
      effect("core.vignette", "6-3", amount=55, softness=70),
      effect("core.exposure_flicker", "6-4", amount=0.15, hold=2)]),
    ("Impact", "adjustment",
     "The frame of a hit: glints on the brights, colour fringes and a shake. Best on an adjustment layer, for a few frames.",
     [effect("core.cross_glare", "7-1", threshold=95, length=50, points=4, angle=45, intensity=0.8),
      effect("core.chromatic_aberration", "7-2", amount=4),
      effect("core.camera_shake", "7-3", amount=14, rotation=1.5, hold=1)]),
    ("Dream haze", "adjustment",
     "A soft, bright haze with the colour lifted. Best on an adjustment layer.",
     [effect("core.diffusion", "8-1", radius=30, amount=60, blend="screen"),
      effect("core.vibrance", "8-2", vibrance=25, saturation=-5)]),
    ("Speed lines", "adjustment",
     "Focus lines rushing in to the middle of the frame. Best on an adjustment layer.",
     [effect("core.speed_lines", "9-1", count=100, thickness=1, inner=550, opacity=85)]),
]


def check(text, expected):
    content = json.loads(text)
    assert set(content) == {"preset_file_version", "presets"} and content["preset_file_version"] == 0
    presets = content["presets"]
    assert 6 <= len(presets) <= 10, len(presets)
    names = [p["name"] for p in presets]
    assert len(set(names)) == len(names) and all(n.strip() == n and n for n in names)
    for p, want in zip(presets, expected["presets"]):
        assert set(p) == {"name", "effects"} and p["effects"]
        assert p["name"] == want["name"]
        assert [fx["type_id"] for fx in p["effects"]] == want["effects"]
        assert want["shown_on"] in ("layer", "adjustment") and want["about"]
        for fx in p["effects"]:
            assert fx["enabled"] is True, (p["name"], fx["type_id"])
            for name, value in fx["parameters"].items():
                values = value if isinstance(value, list) else [value]
                if (fx["type_id"], name) in RANGES:
                    low, high = RANGES[(fx["type_id"], name)]
                    assert all(low <= v <= high for v in values), (p["name"], name, value)
                if (fx["type_id"], name) in WORDS:
                    assert value in WORDS[(fx["type_id"], name)], (p["name"], name, value)
                if name.endswith("color") and isinstance(value, str) and value:
                    assert len(value) == 7 and value[0] == "#" and int(value[1:], 16) >= 0, (p["name"], name)
    ids = [fx["instance_id"] for p in presets for fx in p["effects"]]
    assert len(set(ids)) == len(ids)
    print("checked")


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    content = {"preset_file_version": 0,
               "presets": [{"name": name, "effects": effects} for name, _, _, effects in PRESETS]}
    text = json.dumps(content, indent=2, ensure_ascii=False) + "\n"
    (OUT / "starter.fxpreset").write_text(text, encoding="utf-8", newline="\n")
    expected = {
        "file": "starter.fxpreset",
        "count": {"at_least": 6, "at_most": 10},
        "presets": [{"name": name, "shown_on": shown_on, "about": about,
                     "effects": [fx["type_id"] for fx in effects]}
                    for name, shown_on, about, effects in PRESETS],
    }
    (OUT / "expected_starter_presets.json").write_text(
        json.dumps(expected, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    check(text, expected)


if __name__ == "__main__":
    main()
