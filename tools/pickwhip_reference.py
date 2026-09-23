"""The pick whip, worked a second way.

D-83 proposes After Effects' pick whip: drag from one property's spiral onto another property,
and the first property gets an expression that reads the second. This file pins the text each
drop writes and what that text then evaluates to, so a build can be checked against both.

**This file never runs the build's code path.** The text is written by the rule below, and it is
evaluated by `tools/expression_reference.py`, the second implementation D-59's fixtures use.

The project is written to `Fixtures/pickwhip/pickwhip_project.json` and the expected text and
values to `Fixtures/pickwhip/expected_pickwhip.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/pickwhip_reference.py
"""

import copy
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from expression_reference import (  # noqa: E402
    CAMERA_PROPS, LAYER_PROPS, Project, evaluate, key, layer, project as expr_project, prop,
    show, with_expression)

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "pickwhip"
FRAMES = [0, 12, 24, 36, 48]


def dims(target, name):
    return (CAMERA_PROPS if target == "camera" else LAYER_PROPS)[name]


def link(source, destination):
    """The text a drop writes on `destination`, reading `source`. Each is (target, property)."""
    target, name = source
    if target == "camera":
        text = f"thisComp.activeCamera.{name}"
    else:
        # After Effects' own words: anchorPoint, not the file's anchor (D-59 reads both).
        text = f'thisComp.layer("{target}").transform.{"anchorPoint" if name == "anchor" else name}'
    have, want = dims(*source), dims(*destination)
    if have == 2 and want == 1:
        return text + "[0]"
    if have == 1 and want == 2:
        return f"temp = {text};\n[temp, temp]"
    return text


def project():
    """D-59's fixture composition, with an animated null and a camera whose depth is keyed."""
    data = expr_project()
    comp = data["compositions"][0]
    null = layer("null-1", "Null 1", position_prop=prop(
        [960, 540], [key(0, [960, 540]), key(48, [1440, 300])]),
        rotation=prop(0, [key(0, 0), key(48, 180)]),
        opacity=prop(1, [key(0, 1), key(48, 0.25)]),
        scale_prop=prop([100, 100], [key(0, [100, 100]), key(48, [50, 200])]))
    for gone in ("asset_id", "exposure_spans", "source_offset_frames"):
        null.pop(gone)
    null["kind"] = "null"
    null["transform"]["anchor"] = prop([50, 50], [key(0, [50, 50]), key(48, [0, 100])])
    target = layer("layer-target", "Target")
    comp["camera"]["depth"] = prop(-1920, [key(0, -1920), key(48, -960)])
    comp["layers"] += [null, target]
    comp["layer_order"] += ["null-1", "layer-target"]
    return data


CASES = [
    ("FX-WHIP-001", "Target's Position whipped to Null 1's Position: two numbers onto two.",
     [(("null-1", "position"), ("layer-target", "position"))]),
    ("FX-WHIP-002", "Target's Rotation whipped to Null 1's Rotation: one number onto one.",
     [(("null-1", "rotation"), ("layer-target", "rotation"))]),
    ("FX-WHIP-003", "Target's Rotation whipped to Null 1's Position: two numbers onto one "
     "takes the first, x.",
     [(("null-1", "position"), ("layer-target", "rotation"))]),
    ("FX-WHIP-004", "Target's Scale whipped to Null 1's Rotation: one number onto two uses it "
     "for both.",
     [(("null-1", "rotation"), ("layer-target", "scale"))]),
    ("FX-WHIP-005", "Target's Opacity whipped to Null 1's Opacity: the same percentage, "
     "though the file keeps 0 to 1 and an expression 0 to 100.",
     [(("null-1", "opacity"), ("layer-target", "opacity"))]),
    ("FX-WHIP-006", "Target's Position whipped to Null 1's Anchor Point: written anchorPoint, "
     "as After Effects writes it.",
     [(("null-1", "anchor"), ("layer-target", "position"))]),
    ("FX-WHIP-007", "Target's Scale whipped to Null 1's Scale, which is keyed on both numbers.",
     [(("null-1", "scale"), ("layer-target", "scale"))]),
    ("FX-WHIP-008", "Target's Position whipped to the camera's Position, which already has a "
     "wiggle: the link reads the wiggled value, not the keys.",
     [(("camera", "position"), ("layer-target", "position"))]),
    ("FX-WHIP-009", "The camera's Depth whipped to Null 1's Rotation: a whip can start on "
     "the camera.",
     [(("null-1", "rotation"), ("camera", "depth"))]),
    ("FX-WHIP-010", "Null 1's Position whipped to Trail's Position: a whip can start on a null, "
     "and reads a layer that is itself an expression.",
     [(("layer-trail", "position"), ("null-1", "position"))]),
    ("FX-WHIP-011", "Shake's Position, which already has a wiggle, whipped to Null 1's "
     "Position: the drop replaces the old text whole.",
     [(("null-1", "position"), ("layer-shake", "position"))]),
    ("FX-WHIP-012", "Target's Position whipped to the camera's Depth, which is keyed: one "
     "number onto two, from the camera.",
     [(("camera", "depth"), ("layer-target", "position"))]),
    ("FX-WHIP-013", "Target's Position whipped to Null 1's, then Null 1's back to Target's: "
     "the second drop is written, both ask for each other, and each falls back to its keys "
     "with EXPRESSION_CYCLE, as any expression loop does under D-59.",
     [(("null-1", "position"), ("layer-target", "position")),
      (("layer-target", "position"), ("null-1", "position"))]),
]

# Dropped where nothing is written: the property's own spiral, or anything that is not a
# property an expression can read.
NOTHING = [
    ("FX-WHIP-020", "Target's Position dropped on its own Position: nothing is written.",
     ("layer-target", "position"), ("layer-target", "position")),
]


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    data = project()
    (OUT / "pickwhip_project.json").write_text(
        json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")
    expected = {"tolerance": 1e-6, "project": "pickwhip_project.json", "frames": FRAMES,
                "cases": {}, "nothing": {}}
    print("| case | drop | text written | frame | source | linked | error |")
    print("| --- | --- | --- | --- | --- | --- | --- |")
    for fx, says, drops in CASES:
        after = copy.deepcopy(data)
        written = []
        for source, destination in drops:
            text = link(source, destination)
            after = with_expression(after, *destination, text)
            written.append({"source": list(source), "destination": list(destination),
                            "text": text})
        # The first drop's destination is the property the case is about.
        (src, dst) = drops[0]
        p = Project(after)
        frames = {}
        for f in FRAMES:
            linked, code, _ = evaluate(p, *dst, f)
            source, _, _ = evaluate(p, *src, f)
            frames[str(f)] = {"source": source, "linked": linked, "error": code}
            print(f"| {fx} | {dst[0]} {dst[1]} to {src[0]} {src[1]} | "
                  f"`{written[0]['text'].replace(chr(10), ' ')}` | {f} | {show(source)} | "
                  f"{show(linked)} | {code or 'none'} |")
        expected["cases"][fx] = {"says": says, "drops": written, "frames": frames}
        # The check a build repeats: the linked value is the source, its x, or it twice.
        if len(drops) == 1:
            for f, v in frames.items():
                s, got = v["source"], v["linked"]
                want = s[0] if isinstance(s, list) and not isinstance(got, list) else \
                    [s, s] if isinstance(got, list) and not isinstance(s, list) else s
                assert got == want and v["error"] is None, (fx, f, s, got)
        else:
            assert all(v["error"] == "EXPRESSION_CYCLE" for v in frames.values()), fx
    for fx, says, source, destination in NOTHING:
        expected["nothing"][fx] = {"says": says, "source": list(source),
                                   "destination": list(destination)}
        print(f"\n- {fx}: {says}")
    (OUT / "expected_pickwhip.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
