"""D-180, B-116: effect presets kept in a file, to move them between machines or keep them safe.

Writes `Fixtures/preset_file/` and prints nothing but "checked". Written before any code.

The rule, in plain words. A preset file is JSON, an object with exactly two things in it:
`preset_file_version`, which is 0, the only version there has been, and `presets`, a list of at
least one preset. A preset is an object with exactly two things: `name`, text that is not empty
once the spaces at its ends are taken off, and no two presets in one file with the same name; and
`effects`, a list of at least one effect, each written exactly as a layer's `effects` list is
written in the project file (document 19), keys and all. The effects are read by the rules that
read pasted effects (`persist::read_effects`, D-166): an effect this build does not have, or any
setting or field it would not write back, is refused.

A file is read whole or refused whole (document 28). One thing wrong anywhere refuses the file,
the window's presets are left exactly as they were, and the message names what was wrong: a file
that is not JSON, a project file chosen by mistake, a newer version, anything the rule does not
name, an empty or repeated name, a preset without effects, and an effect or a setting this build
does not have. A setting outside its range is read and kept, as a project file's is (D-46): the
effect is not drawn where it is applied, and the layer says why.

A file the build writes is read back by the build as the same presets, in the same order. The
build writes a preset's effects exactly as the window keeps them.

The effects in these files are taken from committed fixture projects, so each is one the build
already reads. **This file never runs the build's code path.** Fixtures are read-only to
implementation work: changing one is a specification decision, never a step in making a build pass.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "preset_file"
PREFIX = "FX-PRE-"


def first_effect(folder, file, layer=0):
    """The first effect of a layer in a committed fixture project, exactly as it is written there."""
    project = json.loads((ROOT / "Fixtures" / folder / file).read_text(encoding="utf-8"))
    return project["compositions"][0]["layers"][layer]["effects"][0]


def effect_of(folder, file, type_id):
    """The first effect of `type_id` anywhere in a committed fixture project."""
    project = json.loads((ROOT / "Fixtures" / folder / file).read_text(encoding="utf-8"))
    for composition in project["compositions"]:
        for layer in composition["layers"]:
            for fx in layer.get("effects", []):
                if fx["type_id"] == type_id:
                    return fx
    raise KeyError((folder, file, type_id))


GLOW = first_effect("glow", "fx_glow_001.json")
INVERT = first_effect("invert", "fx_invert_001.json")
POSTER = first_effect("posterize", "fx_poster_001.json")
SHADOW = first_effect("drop_shadow", "fx_shadow_001.json")
VIGNETTE = first_effect("vignette", "fx_vignette_001.json")
KEYED = effect_of("fxkey", "fx_fxk_001.json", "core.exposure")


def invalid_glow():
    """A Glow whose radius is outside its range: read and kept, not drawn where applied."""
    fx = json.loads(json.dumps(GLOW))
    fx["parameters"]["radius"] = -3
    return fx


def turned_off(fx):
    fx = json.loads(json.dumps(fx))
    fx["enabled"] = False
    return fx


def with_setting(fx, name, value):
    fx = json.loads(json.dumps(fx))
    fx["parameters"][name] = value
    return fx


def with_field(fx, name, value):
    fx = json.loads(json.dumps(fx))
    fx[name] = value
    return fx


def preset(name, *effects):
    return {"name": name, "effects": list(effects)}


def file(*presets, **extra):
    return {"preset_file_version": 0, "presets": list(presets), **extra}


def valid(says, content, names_and_types):
    return {"says": says, "content": content,
            "presets": [{"name": n, "effects": t} for n, t in names_and_types]}


def refused(says, content, diagnostic, names):
    return {"says": says, "content": content, "refused": diagnostic, "names": names}


def cases():
    unknown = with_field(GLOW, "type_id", "core.lens_sparkle")
    out = [
        valid("One preset holding one effect: the smallest file.",
              file(preset("Soft glow", GLOW)),
              [("Soft glow", ["core.glow"])]),
        valid("Three presets, read in the file's order, each with its effects in their order.",
              file(preset("Night", INVERT, VIGNETTE), preset("Poster", POSTER),
                   preset("Lifted", SHADOW, GLOW, POSTER)),
              [("Night", ["core.invert", "core.vignette"]), ("Poster", ["core.posterize"]),
               ("Lifted", ["core.drop_shadow", "core.glow", "core.posterize"])]),
        valid("A keyed setting: the keys come with the preset, at the frames they were at.",
              file(preset("Flash", KEYED)),
              [("Flash", ["core.exposure"])]),
        valid("An effect switched off stays off.",
              file(preset("Off for now", turned_off(GLOW), INVERT)),
              [("Off for now", ["core.glow", "core.invert"])]),
        valid("A name in Japanese, with inner spaces, is kept exactly as written.",
              file(preset("撃ち合い  光 2", GLOW)),
              [("撃ち合い  光 2", ["core.glow"])]),
        valid("A setting outside its range is read and kept, as a project file's is (D-46); the "
              "effect is not drawn where the preset is applied, and the layer says why.",
              file(preset("Wrong radius", invalid_glow())),
              [("Wrong radius", ["core.glow"])]),
        valid("The same effect id in two presets does not matter: pasting gives each new ids.",
              file(preset("A", GLOW), preset("B", GLOW)),
              [("A", ["core.glow"]), ("B", ["core.glow"])]),
        valid("Names differing only by capitals are two presets.",
              file(preset("glow", GLOW), preset("Glow", GLOW)),
              [("glow", ["core.glow"]), ("Glow", ["core.glow"])]),
        refused("Not JSON: a text file chosen by mistake.",
                "These are my presets.\n", "PRESET_FILE_INVALID", "JSON"),
        refused("A project file chosen by mistake is named as one.",
                json.loads((ROOT / "Fixtures" / "glow" / "fx_glow_001.json").read_text(encoding="utf-8")),
                "PRESET_FILE_INVALID", "project"),
        refused("A newer version is refused, never guessed at.",
                {"preset_file_version": 1, "presets": [preset("Soft glow", GLOW)]},
                "PRESET_FILE_INVALID", "newer"),
        refused("Something the rule does not name, beside the presets, would be lost, so the "
                "file is refused.",
                file(preset("Soft glow", GLOW), author="someone"),
                "PRESET_FILE_INVALID", "author"),
        refused("Something the rule does not name, in a preset, likewise.",
                file({"name": "Soft glow", "effects": [GLOW], "thumbnail": "glow.png"}),
                "PRESET_FILE_INVALID", "thumbnail"),
        refused("A name of spaces only.",
                file(preset("   ", GLOW)), "PRESET_FILE_INVALID", "name"),
        refused("Two presets with one name: which one was meant is not known.",
                file(preset("Soft glow", GLOW), preset("Soft glow", INVERT)),
                "PRESET_FILE_INVALID", "Soft glow"),
        refused("A preset with no effects.",
                file(preset("Empty")), "PRESET_FILE_INVALID", "Empty"),
        refused("A file with no presets.",
                file(), "PRESET_FILE_INVALID", "presets"),
        refused("An effect this build does not have, in the last of three presets, refuses the "
                "whole file: the first two are not taken either.",
                file(preset("Soft glow", GLOW), preset("Poster", POSTER), preset("Sparkle", INVERT, unknown)),
                "EFFECT_UNSUPPORTED", "core.lens_sparkle"),
        refused("A setting this build does not have would be lost, so the file is refused.",
                file(preset("Soft glow", with_setting(GLOW, "sparkle", 3))),
                "EFFECT_UNSUPPORTED", "sparkle"),
    ]
    return {f"{PREFIX}{i:03d}": c for i, c in enumerate(out, 1)}


def check(expected):
    assert len(expected) >= 12
    for cid, case in expected.items():
        text = (OUT / case["file"]).read_text(encoding="utf-8")
        if "refused" in case:
            assert case["refused"] in ("PRESET_FILE_INVALID", "EFFECT_UNSUPPORTED"), cid
            assert case["names"], cid
            continue
        content = json.loads(text)
        # A valid case is valid by the rule itself, checked here on the file as written.
        assert set(content) == {"preset_file_version", "presets"} and content["preset_file_version"] == 0, cid
        names = [p["name"] for p in content["presets"]]
        assert names and len(set(names)) == len(names) and all(n.strip() for n in names), cid
        for p, want in zip(content["presets"], case["presets"]):
            assert set(p) == {"name", "effects"} and p["effects"], cid
            assert p["name"] == want["name"], cid
            assert [fx["type_id"] for fx in p["effects"]] == want["effects"], cid
    # The last refusal differs from a valid file only by its unknown effect, and the one before by
    # its unknown setting, so it is those that refuse them.
    assert '"core.lens_sparkle"' in (OUT / expected["FX-PRE-018"]["file"]).read_text(encoding="utf-8")
    assert '"sparkle": 3' in (OUT / expected["FX-PRE-019"]["file"]).read_text(encoding="utf-8")
    # The not-JSON case really is not JSON, and the project case really is a project.
    try:
        json.loads((OUT / expected["FX-PRE-009"]["file"]).read_text(encoding="utf-8"))
        raise AssertionError("FX-PRE-009 parses")
    except json.JSONDecodeError:
        pass
    assert "schema_version" in json.loads((OUT / expected["FX-PRE-010"]["file"]).read_text(encoding="utf-8"))
    print("checked")


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    expected = {}
    for cid, case in cases().items():
        name = cid.lower().replace("-", "_") + ".fxpreset"
        content = case.pop("content")
        text = content if isinstance(content, str) else json.dumps(content, indent=2, ensure_ascii=False) + "\n"
        (OUT / name).write_text(text, encoding="utf-8", newline="\n")
        expected[cid] = {"file": name, **case}
    (OUT / "expected_preset_file.json").write_text(
        json.dumps({"cases": expected}, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    check(expected)


if __name__ == "__main__":
    main()
