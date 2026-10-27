"""D-916: the colour-management switch in the window (ADR-023), the expected outcomes.

Writes Fixtures/colour/switch/: two projects with colour management off whose composition is in
an After Effects depth (8 bpc (After Effects), 32 bpc (After Effects)), made from D-910's
refused_eight_bpc.json and refused_ae_32bpc.json by taking colour management off and nothing
else; and expected_switch.json, what `project.set_color_management` must do in each case.

No number here comes from the build. A case that is accepted names an existing fixture project
written by an earlier reference tool (D-910's tools/c4_reference.py, D-915's
tools/view_reference.py): after the switch the project must be that one (its project ID aside)
and the viewer must send that project's frame. A case that is refused names D-910's diagnostic
and requires the opened file to save back byte for byte. Python's standard library only:

    python -I tools/switch_reference.py
"""

import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
COLOUR = os.path.join(HERE, "..", "Fixtures", "colour")
OUT = os.path.join(COLOUR, "switch")


def load(rel):
    with open(os.path.join(COLOUR, rel), encoding="utf-8") as f:
        return json.load(f)


def save(rel, value):
    with open(os.path.join(OUT, rel), "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


os.makedirs(OUT, exist_ok=True)
for src, dst, pid in [
    ("c4/refused_eight_bpc.json", "off_eight_bpc.json", "proj-switch-eight"),
    ("c4/refused_ae_32bpc.json", "off_ae_32bpc.json", "proj-switch-ae32"),
]:
    p = load(src)
    p["project_id"] = pid
    p["color_settings"] = {"working_space": "linear-srgb", "alpha_mode": "premultiplied"}
    save(dst, p)


def applied(cid, opened, working, becomes, note):
    return {"id": cid, "open": opened, "working": working, "expect": "applied",
            "same_project_as": becomes, "same_frame_as": becomes, "note": note}


cases = [
    applied("off-to-acescg", "view/hdr_scene_off.json", "acescg", "view/hdr_scene_aces.json",
            "Off to on: ACEScg, the default when on; the View menu's Colour view appears, "
            "showing ACES 2.0 SDR. Undo gives the opened file's bytes back."),
    applied("acescg-to-off", "view/hdr_scene_aces.json", "off", "view/hdr_scene_off.json",
            "On to off: today's picture, and the Colour view gone from the View menu."),
    applied("rec709-to-acescg", "c4/solid_rec709.json", "acescg", "c4/solid_acescg.json",
            "The working space changed with colour management on."),
    applied("acescg-to-rec709", "c4/solid_acescg.json", "linear-srgb", "c4/solid_rec709.json",
            "Back to linear Rec.709 working, colour management still on."),
]
for name, depth in [("off_eight_bpc", "8 bpc (AE)"), ("off_ae_32bpc", "32 bpc (AE)")]:
    for working in ["acescg", "linear-srgb"]:
        cases.append({
            "id": "refused-" + name[4:].replace("_", "-") + "-" + working,
            "open": "switch/" + name + ".json", "working": working, "expect": "refused",
            "diagnostic": "COLOR_DEPTH_REFUSED", "names": ["Main", depth],
            "note": "D-910's depth refusal: colour management stays off, the composition's depth "
                    "is not changed, nothing goes into the history and the file saves back byte "
                    "for byte."})

save("expected_switch.json", {
    "decision": "D-916",
    "command": "project.set_color_management",
    "working": {"off": "colour management off (the default)",
                "acescg": "on, working in ACEScg (the default when on)",
                "linear-srgb": "on, working in linear Rec.709"},
    "compare": "same_project_as: the project after the command equals that file's, read as JSON, "
               "with project_id left out; same_frame_as: the window's frame 0 at Full is that "
               "file's frame 0 at Full, byte for byte",
    "cases": cases,
})
