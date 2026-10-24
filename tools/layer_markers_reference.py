"""A layer's markers (D-765), and a marker's length on either kind, worked out a second way.

The rule (D-765, after W-24's composition markers):
- A marker is a frame, a name and an optional `duration_frames` (After Effects' marker
  duration). The file writes `duration_frames` only when it is not 0.
- A layer's `markers` is read as real data and kept in frame order. A file with none loads as
  before and is saved with none.
- A layer `markers` value this build cannot read (not a list, or an entry without a whole-number
  frame and a text name) gets a PROJECT_FEATURE_UNSUPPORTED warning naming where, the file still
  opens, the layer shows no markers, and saving writes the value back exactly as it was
  (document 28: kept, and said so; never dropped in silence). `null` is no markers; `[]` is kept.
- A layer slid to start later carries its markers with it, by the same number of frames.

Nothing here is read from the program. Writes Fixtures/layer_markers/.
"""

import copy
import json
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "layer_markers"


def project(layer_markers=None, comp_markers=None, has=True):
    layer = {
        "id": "lm", "kind": "solid", "name": "lm", "enabled": True, "locked": False,
        "in_frame": 0, "out_frame": 48,
        "transform": {
            "anchor": {"base": [30.0, 30.0], "keyframes": []},
            "position": {"base": [400, 400], "keyframes": []},
            "scale": {"base": [100, 100], "keyframes": []},
            "rotation": {"base": 0, "keyframes": []},
            "opacity": {"base": 1, "keyframes": []},
        },
        "mask": None, "matte": None, "blend_mode": "normal", "effects": [],
        "solid": {"color": [1, 1, 1], "width": 60, "height": 60},
    }
    if has:
        layer["markers"] = layer_markers
    comp = {
        "id": "comp-main", "name": "Main", "width": 800, "height": 800,
        "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
        "start_frame": 0, "duration_frames": 48,
        "work_area": {"start_frame": 0, "end_frame_exclusive": 48},
    }
    if comp_markers is not None:
        comp["markers"] = comp_markers
    comp["layer_order"] = ["lm"]
    comp["layers"] = [layer]
    return {
        "schema_version": 0, "project_id": "proj-lmark",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [], "compositions": [comp],
    }


def readable(value):
    """The markers as the program should hold them, or None when they cannot be read."""
    if value is None:
        return []
    if not isinstance(value, list):
        return None
    out = []
    for m in value:
        if not isinstance(m, dict):
            return None
        f, n, d = m.get("frame"), m.get("name"), m.get("duration_frames", 0)
        if type(f) is not int or not isinstance(n, str) or type(d) is not int or d < 0:
            return None
        out.append({"frame": f, "name": n, "duration_frames": d})
    return sorted(out, key=lambda m: m["frame"])


def written(markers):
    out = []
    for m in markers:
        w = {"frame": m["frame"], "name": m["name"]}
        if m["duration_frames"]:
            w["duration_frames"] = m["duration_frames"]
        out.append(w)
    return out


def expect(file, has, value, comp_markers=None, slide=0):
    """What loading, sliding by `slide` frames and saving should give."""
    held = readable(value) if has else []
    warn = has and held is None
    held = held or []
    held = [dict(m, frame=m["frame"] + slide) for m in held]
    if not has:
        saved = "absent"
    elif warn:
        saved = value  # written back exactly
    elif held:
        saved = written(held)
    elif isinstance(value, list) and value:
        saved = "absent"  # (all taken off: not a case here)
    else:
        saved = value  # null or [] left as written
    comp = readable(comp_markers) if comp_markers is not None else []
    return {
        "file": file,
        "slide_frames": slide,
        "layer_markers": held,
        "warnings": ["PROJECT_FEATURE_UNSUPPORTED"] if warn else [],
        "warning_names": "layers/0/markers" if warn else None,
        "saved_layer_markers": saved,
        "comp_markers": comp,
        "saved_comp_markers": written(comp) if comp_markers is not None else [],
    }


CASES = [
    # (file, has, layer markers, comp markers, slide, what it shows)
    ("fx_lmark_001.json", True, [{"frame": 12, "name": "hit"}], None, 0),
    ("fx_lmark_002.json", True,
     [{"frame": 30, "name": "end", "duration_frames": 6}, {"frame": 4, "name": "start"}], None, 0),
    ("fx_lmark_003.json", False, None, None, 0),
    ("fx_lmark_004.json", True, "hit", None, 0),
    ("fx_lmark_005.json", True, [{"frame": "twelve", "name": "hit"}], None, 0),
    ("fx_lmark_006.json", True, [{"frame": 2, "name": "in"}],
     [{"frame": 10, "name": "chorus", "duration_frames": 4}, {"frame": 30, "name": "out"}], 0),
    ("fx_lmark_007.json", True, [{"frame": 12, "name": "hit", "duration_frames": 3}], None, 5),
    ("fx_lmark_008.json", True, None, None, 0),
    ("fx_lmark_009.json", True, [], None, 0),
]


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    expected = []
    for file, has, value, comp, slide in CASES:
        with open(OUT / file, "w", encoding="utf-8", newline="") as f:
            f.write(json.dumps(project(copy.deepcopy(value), copy.deepcopy(comp), has), indent=2) + "\n")
        expected.append(expect(file, has, value, comp, slide))
    with open(OUT / "expected_layer_markers.json", "w", encoding="utf-8", newline="") as f:
        f.write(json.dumps({"rule": "D-765", "cases": expected}, indent=2) + "\n")
    # Self-check on what the rule says, not on the program.
    by = {c["file"]: c for c in expected}
    assert by["fx_lmark_002.json"]["layer_markers"][0]["frame"] == 4
    assert by["fx_lmark_004.json"]["saved_layer_markers"] == "hit"
    assert by["fx_lmark_007.json"]["layer_markers"][0]["frame"] == 17
    assert by["fx_lmark_003.json"]["saved_layer_markers"] == "absent"
    print("wrote", len(expected), "cases to", OUT)


if __name__ == "__main__":
    main()
