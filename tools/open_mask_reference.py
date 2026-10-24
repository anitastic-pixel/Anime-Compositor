"""D-737, B-611: open masks.

Writes `Fixtures/open_masks/` and prints one line a case. Written before any code.

A mask may carry `closed`, true or false; without it the mask is closed, as every mask was. An
open mask has no piece from its last point back to its first. It encloses nothing, so its mode is
`none` and it cuts nothing from its layer; it needs two points, where a closed mask needs three,
and may cross itself. An effect that draws along a path takes it open, as D-357 takes an open
shape: Path Stroke measures it without the closing piece, so nothing is drawn along it. The file
writes `"closed": false` and `"mode": "none"`, so a reader without D-737 keeps the switch (as
document 28 keeps any field it does not know) and cuts nothing with the mask either; it draws a
Path Stroke along it closed, and warns that a two-point mask cannot be drawn.

Every case is `tools/stroke_reference.py`'s: a composition 16 by 10 holding Lightning Bolt's night
drawing with Path Stroke on it, Brush Size 1.5 (3 before D-773: Brush Size is the radius), Hardness 75, Spacing 15, white, On Original Image.
The stroke's numbers are that file's, worked on the open path as it works an open shape's
(FX-STROKE-049). `expected_open_masks.json` gives each case's frames, every sample within 2e-5,
and the two refused files, each with the diagnostic and a word its message must contain.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import json
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import stroke_reference as ST  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "open_masks"
W, H = ST.W, ST.H

ROOF, BOX, RING = ST.ROOF, ST.BOX, ST.RING
LINE = ST.points([3, 8], [12, 3])  # two points: a line, 10.3 long


def opened(pts, **more):
    return {"points": pts, "mode": "none", "closed": False, **more}


def render(c):
    """The frame as stroke_reference draws it, each mask's path open or closed as it says."""
    chosen = c["masks"] if c["all_masks"] == "on" else [c["masks"][c["mask"] - 1]]
    paths = [(ST.SH.flatten(m["points"], m.get("closed", True)), m.get("closed", True))
             for m in chosen if len(m["points"]) >= 2]
    n = {k: ST.held(c, k, 0) for k in ST.RANGES}
    n["sequential"] = c["all_masks"] == "on" and c["stroke_sequentially"] == "on"
    return ST.frame(ST.stroke(ST.layer_of(c), paths, n, c["color"], c["paint_style"]), 0)


def project(fx, c):
    p = ST.project_json(fx, c)
    for m, written in zip(c["masks"], p["compositions"][0]["layers"][0]["masks"]):
        if "closed" in m:
            # Where the build writes it: after the path, the last of a mask's own fields.
            written["closed"] = m["closed"]
    return p


CASES = {
    "FX-MSKO-001": ("An open roof, two legs from (2, 7) up to (8, 2) and down to (14, 7), as Mask "
                    "1, stroked: drawn open, nothing along the bottom, FX-STROKE-049's line on "
                    "the night drawing.", ST.case(masks=[opened(ROOF)])),
    "FX-MSKO-002": ("The same roof closed, `closed` left out: the bottom is drawn too, as every "
                    "mask was.", ST.case(masks=[ST.none(ROOF)])),
    "FX-MSKO-003": ("The open roof, End 50: the left leg alone, half of its open length.",
                    ST.case(masks=[opened(ROOF)], end=50)),
    "FX-MSKO-004": ("A mask of two points, open, a line from (3, 8) to (12, 3): drawn along, and "
                    "no warning that it has too few points.", ST.case(masks=[opened(LINE)])),
    "FX-MSKO-005": ("The curved ring open: a circle with no piece back from its last point, "
                    "(4, 5), to its first, (8, 1).", ST.case(masks=[opened(RING)])),
    "FX-MSKO-006": ("Every mask, sequentially, End 60: the open roof then the closed box, one "
                    "length end to end, the roof's without its bottom.",
                    ST.case(masks=[opened(ROOF), ST.none(BOX)], all_masks="on",
                            stroke_sequentially="on", end=60)),
}

# An open mask alone, no effect: every pixel of the layer as it was. `closed: true` written out is
# kept as written.
LEFT_ALONE = {
    "FX-MSKO-010": ("Two open masks, the roof and the line, `closed` written on a third, closed "
                    "box of mode Add that keeps the drawing inside it, and no effect: the open "
                    "masks cut nothing; the box cuts as it always has.",
                    [opened(ROOF), opened(LINE), {"points": BOX, "mode": "add", "closed": True}]),
}

REFUSED = {
    "FX-MSKO-020": ("`closed` that is not true or false.", "refused_closed_word.json",
                    lambda m: m.update(closed="no"), "closed"),
    "FX-MSKO-021": ("An open mask of mode Add: an open mask encloses nothing to add.",
                    "refused_open_add.json", lambda m: m.update(mode="add"), "closed"),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ST.OUT / "media" / "night.png", OUT / "media" / "night.png")
    expected = {"tolerance": ST.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project(fx, c), indent=2) + "\n", encoding="utf-8")
        px = render(c)
        expected["cases"][fx] = {"says": says, "project": name, "frames": {"0": px}}
        before = ST.plain(c)
        print(f"{fx}: {sum(px[i] != before[i] for i in range(W * H))} changed")
    for fx, (says, masks) in LEFT_ALONE.items():
        c = ST.case(masks=masks)
        p = project(fx, c)
        del p["compositions"][0]["layers"][0]["effects"][-1]
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
        px = ST.plain(c)  # the box's Add cut, and nothing of the open masks
        expected["cases"][fx] = {"says": says, "project": name, "frames": {"0": px, "4": px}}
        print(f"{fx}: the drawing, cut by the box alone")
    for fx, (says, file, spoil, word) in REFUSED.items():
        p = copy.deepcopy(json.loads((OUT / "fx_msko_001.json").read_text(encoding="utf-8")))
        spoil(p["compositions"][0]["layers"][0]["masks"][0])
        (OUT / file).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
        expected.setdefault("refused", {})[fx] = {
            "says": says, "file": file, "refused": "PROJECT_SCHEMA_INVALID", "names": word}
        print(f"{fx}: refused")
    (OUT / "expected_open_masks.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    art = ST.plain(ST.case())
    # FX-STROKE-049's open roof is this roof on the shape layer: the same pixels along the legs.
    open_roof, closed_roof = c["FX-MSKO-001"], c["FX-MSKO-002"]
    assert open_roof[at(8, 7)] == art[at(8, 7)] and closed_roof[at(8, 7)] != art[at(8, 7)]
    assert open_roof[at(5, 4)] == closed_roof[at(5, 4)] != art[at(5, 4)]
    # End 50 draws the left leg and not the right.
    half = c["FX-MSKO-003"]
    assert half[at(5, 4)] != art[at(5, 4)] and half[at(11, 4)] == art[at(11, 4)]
    # The line is drawn; the ring's gap is not.
    assert c["FX-MSKO-004"][at(7, 6)] != art[at(7, 6)]
    ring = c["FX-MSKO-005"]
    assert ring[at(5, 2)] == art[at(5, 2)] and ring[at(11, 2)] != art[at(11, 2)]
    print("checked")


if __name__ == "__main__":
    main()
