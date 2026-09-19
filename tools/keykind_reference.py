"""Separate X and Y for position, and auto, continuous and roving keys, worked a second way.

D-69 proposes four things the graph editor's owner asked for, and this file is the reference for
the numbers document 25 pins against it.

**Separate dimensions** change what a frame looks like: a position written as `{"x", "y"}` is two
numbers' properties, each with its own keys and its own ease, and the position at a frame is
`[x at the frame, y at the frame]`.

**Auto, continuous and roving change no frame.** The file always holds the ease and the frame
that are rendered; a key's `kind` or `roving` says what an edit must keep true of them. So those
cases are an edit and the curve or frame it must leave behind:

    speed out of a key  = y1 / x1 * L_out / (next frame - frame)
    speed into a key    = (1 - y2) / (1 - x2) * L_in / (frame - previous frame)

where `[x1, y1, x2, y2]` is the ease of the segment (document 19) and L is how far the value goes
over the segment: the difference for a number, signed; for a pair the straight distance, or three
times the length of the path handle at that end when the position has one (D-53). This is the
speed the window's Keyframe Velocity box has shown since W-25.

    continuous: speed in = speed out.
    auto:       both are (L_in + L_out) / (next frame - previous frame), handles a third long.
    roving:     the key's frame is where it falls if the run is travelled at one speed.

**This file never runs the build's code path**, and its ease is `tools/ease_reference.py`'s
bisection.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/keykind_reference.py
"""

import json
import sys
from math import floor, hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import adjust_reference as adj  # noqa: E402
from ease_reference import ease, EASE_IN_OUT  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "keykind"
TOLERANCE = 1e-9
LINEAR = [1 / 3, 1 / 3, 2 / 3, 2 / 3]
PIECES = 64  # a curved path's length is measured along this many straight pieces


def key(frame, value, interp="linear", **more):
    return {"frame": frame, "value": value, "interp": interp, **more}


def curve(k):
    return list(k["interp"]) if not isinstance(k["interp"], str) else list(LINEAR)


# --- a number's value at a frame (document 20), for the separated position ------------------

def value_at(keys, frame):
    if frame <= keys[0]["frame"]:
        return keys[0]["value"]
    if frame >= keys[-1]["frame"]:
        return keys[-1]["value"]
    a, b = next((a, b) for a, b in zip(keys, keys[1:]) if a["frame"] <= frame < b["frame"])
    if a["interp"] == "hold":
        return a["value"]
    f = (frame - a["frame"]) / (b["frame"] - a["frame"])
    if a["interp"] != "linear":
        f = ease(a["interp"], f)
    return a["value"] + (b["value"] - a["value"]) * f


# --- how far a segment goes, at each of its ends ----------------------------------------------

def reach(a, b):
    """(L at a's end, L at b's end) of the segment from key a to key b."""
    if not isinstance(a["value"], list):
        d = b["value"] - a["value"]
        return d, d
    chord = hypot(b["value"][0] - a["value"][0], b["value"][1] - a["value"][1])
    out = a.get("spatial")
    into = b.get("spatial")
    return (3 * hypot(out[2], out[3]) if out else chord,
            3 * hypot(into[0], into[1]) if into else chord)


def speeds(keys, i):
    """(speed in, speed out) at key i; None on a side with no segment, a hold or no reach."""
    k = keys[i]
    s_in = s_out = None
    if i > 0 and keys[i - 1]["interp"] != "hold":
        p = keys[i - 1]
        L = reach(p, k)[1]
        c = curve(p)
        if L:
            s_in = (1 - c[3]) / (1 - c[2]) * L / (k["frame"] - p["frame"])
    if i + 1 < len(keys) and k["interp"] != "hold":
        n = keys[i + 1]
        L = reach(k, n)[0]
        c = curve(k)
        if L:
            s_out = c[1] / c[0] * L / (n["frame"] - k["frame"])
    return s_in, s_out


def set_speed(keys, i, s, side, third=False):
    """Give one side of key i the speed s. The handle keeps its length, or becomes a third."""
    k = keys[i]
    if side == "in":
        p = keys[i - 1]
        c = curve(p)
        if third:
            c[2] = 2 / 3
        c[3] = 1 - s * (1 - c[2]) * (k["frame"] - p["frame"]) / reach(p, k)[1]
        p["interp"] = c
    else:
        n = keys[i + 1]
        c = curve(k)
        if third:
            c[0] = 1 / 3
        c[1] = s * c[0] * (n["frame"] - k["frame"]) / reach(k, n)[0]
        k["interp"] = c


def settle(keys, touched=None):
    """Make every auto and continuous key true again. `touched` is (key index, side) when the
    edit set that side's curve by hand: the other side follows it."""
    for i, k in enumerate(keys):
        kind = k.get("kind")
        if not kind:
            continue
        s_in, s_out = speeds(keys, i)
        if kind == "auto":
            if i == 0 or i + 1 == len(keys):  # an end key leaves or arrives as a straight line
                side = "out" if i == 0 else "in"
                if (s_out if i == 0 else s_in) is not None:
                    a, b = (k, keys[1]) if i == 0 else (keys[i - 1], k)
                    L = reach(a, b)[0 if i == 0 else 1]
                    set_speed(keys, i, L / (b["frame"] - a["frame"]), side, third=True)
                continue
            p, n = keys[i - 1], keys[i + 1]
            s = (reach(p, k)[1] + reach(k, n)[0]) / (n["frame"] - p["frame"])
            if s_in is not None:
                set_speed(keys, i, s, "in", third=True)
            if s_out is not None:
                set_speed(keys, i, s, "out", third=True)
        elif s_in is not None and s_out is not None:
            if touched and touched[0] == i:
                s = s_out if touched[1] == "out" else s_in
            else:
                s = (s_in + s_out) / 2
            set_speed(keys, i, s, "in")
            set_speed(keys, i, s, "out")
    return keys


# --- roving -----------------------------------------------------------------------------------

def length(a, b):
    """The length of the path from key a to key b: straight, or D-53's curve in 64 pieces."""
    if not a.get("spatial") and not b.get("spatial"):
        return hypot(b["value"][0] - a["value"][0], b["value"][1] - a["value"][1])
    out = a.get("spatial") or [0, 0] + [(b["value"][c] - a["value"][c]) / 3 for c in (0, 1)]
    into = b.get("spatial") or [(a["value"][c] - b["value"][c]) / 3 for c in (0, 1)] + [0, 0]
    pts = [[a["value"][c], a["value"][c] + out[2 + c], b["value"][c] + into[c], b["value"][c]]
           for c in (0, 1)]

    def at(t):
        return [(1 - t) ** 3 * p[0] + 3 * (1 - t) ** 2 * t * p[1] + 3 * (1 - t) * t * t * p[2]
                + t ** 3 * p[3] for p in pts]

    steps = [at(n / PIECES) for n in range(PIECES + 1)]
    return sum(hypot(q[0] - p[0], q[1] - p[1]) for p, q in zip(steps, steps[1:]))


def rove(keys):
    """Put every roving key on its frame. None when a run has too few frames for its keys."""
    i = 0
    while i < len(keys):
        if not keys[i].get("roving"):
            i += 1
            continue
        j = i
        while keys[j].get("roving"):
            j += 1
        a, b, run = keys[i - 1], keys[j], keys[i:j]
        if b["frame"] - a["frame"] - 1 < len(run):
            return None
        lengths = [length(p, q) for p, q in zip(keys[i - 1:j], keys[i:j + 1])]
        total, so_far = sum(lengths), 0.0
        for n, k in enumerate(run):
            so_far += lengths[n]
            share = so_far / total if total else (n + 1) / (len(run) + 1)
            k["frame"] = floor(a["frame"] + (b["frame"] - a["frame"]) * share + 0.5)
        for n, k in enumerate(run):  # never two keys on one frame, never outside the run
            k["frame"] = max(k["frame"], (run[n - 1]["frame"] if n else a["frame"]) + 1)
        for n in range(len(run) - 1, -1, -1):
            after = run[n + 1]["frame"] if n + 1 < len(run) else b["frame"]
            run[n]["frame"] = min(run[n]["frame"], after - 1)
        i = j
    return keys


# --- separate and join --------------------------------------------------------------------------

def separate(keys):
    """A position's keys as X's and Y's: the same frames, eases and kinds; no path, no roving."""
    return [[{k2: v for k2, v in {**k, "value": k["value"][c]}.items()
              if k2 not in ("spatial", "roving")} for k in keys] for c in (0, 1)]


def join(xs, ys):
    """One position again: a key wherever either had one, holding both values at that frame,
    with X's ease where X had a key and else Y's."""
    frames = sorted({k["frame"] for k in xs + ys})
    own = {k["frame"]: k for k in ys} | {k["frame"]: k for k in xs}
    return [{**own[f], "value": [value_at(xs, f), value_at(ys, f)]} for f in frames]


# --- the cases ----------------------------------------------------------------------------------

SEP = {
    "FX-SEP-001": ("X goes 0 to 8 at a steady speed while Y goes 0 to 4 on the ease-in-out "
                   "curve: each has its own ease.",
                   [key(0, 0), key(4, 8)], [key(0, 0, EASE_IN_OUT), key(4, 4)]),
    "FX-SEP-002": ("X and Y keyed on different frames, Y with a hold: neither needs a key where "
                   "the other has one.",
                   [key(0, 0), key(4, 8)], [key(1, 2, "hold"), key(3, 6)]),
}

EASED = [0.25, 0.5, 0.75, 1.0]


def edits():
    """Each case: what it says, the keys before, the edit in words, and the keys after."""
    cases = {}

    def case(fx, says, before, edit, after):
        cases[fx] = {"says": says, "before": json.loads(json.dumps(before)), "edit": edit,
                     "after": after}

    k = [key(0, 0), key(4, 8), key(12, 4)]
    case("FX-KIND-001", "A key made auto: it passes through at the slope between its neighbours, "
         "4 over 12 frames, with handles a third long.",
         k, "make the key at frame 4 auto",
         settle([k[0], {**k[1], "kind": "auto"}, k[2]]))
    k = [key(0, 0), key(4, 8)]
    case("FX-KIND-002", "Auto on the first and last keys: a straight line out and in.",
         k, "make both keys auto", settle([{**x, "kind": "auto"} for x in k]))
    k = [key(0, [0, 0]), key(4, [3, 4]), key(8, [3, 16])]
    case("FX-KIND-003", "An auto key on a position: 5 pixels in and 12 out over 8 frames, so "
         "17/8 of a pixel a frame on both sides.",
         k, "make the key at frame 4 auto",
         settle([k[0], {**k[1], "kind": "auto"}, k[2]]))
    k = cases["FX-KIND-001"]["after"]
    moved = json.loads(json.dumps(k))
    moved[2]["value"] = 12
    case("FX-KIND-004", "An auto key follows its neighbours: the next key's value goes from 4 "
         "to 12 and the slope through the auto key becomes 1.",
         k, "set the value of the key at frame 12 to 12", settle(moved))
    k = [key(0, 0), key(4, 8), key(8, 12)]
    case("FX-KIND-005", "A corner made continuous: 2 a frame in and 1 a frame out become 1.5 "
         "on both sides, the handles keeping their lengths.",
         k, "make the key at frame 4 continuous",
         settle([k[0], {**k[1], "kind": "continuous"}, k[2]]))
    k = cases["FX-KIND-005"]["after"]
    pulled = json.loads(json.dumps(k))
    pulled[1]["interp"] = list(EASED)
    case("FX-KIND-006", "One handle of a continuous key pulled by hand: the other side follows "
         "it to the same speed, 2 a frame.",
         k, "set the ease of the key at frame 4 to [0.25, 0.5, 0.75, 1]",
         settle(pulled, touched=(1, "out")))
    k = [key(0, 0), key(4, 8), key(8, 8)]
    case("FX-KIND-007", "An auto key beside a segment that goes nowhere: that side has no speed "
         "to set and is left as it was.",
         k, "make the key at frame 4 auto",
         settle([k[0], {**k[1], "kind": "auto"}, k[2]]))

    k = [key(0, [0, 0]), key(2, [30, 40]), key(10, [30, 140])]
    case("FX-ROVE-001", "A roving key: 50 pixels then 100, so it sits a third of the way "
         "through the ten frames, on frame 3.",
         k, "make the key at frame 2 roving",
         rove([k[0], {**k[1], "roving": True}, k[2]]))
    k = [key(0, [0, 0]), key(1, [1, 0]), key(2, [2, 0]), key(3, [102, 0])]
    case("FX-ROVE-002", "Two roving keys crowded at the start of a short run: each still gets "
         "a frame of its own.",
         k, "make the keys at frames 1 and 2 roving",
         rove([k[0], {**k[1], "roving": True}, {**k[2], "roving": True}, k[3]]))
    k = [key(0, [0, 0]), key(1, [10, 0]), key(3, [20, 0]), key(4, [30, 0])]
    squeezed = json.loads(json.dumps(k))
    squeezed[1]["roving"] = squeezed[2]["roving"] = True
    squeezed[3]["frame"] = 2
    assert rove(squeezed) is None
    case("FX-ROVE-003", "A run with fewer frames than roving keys is refused with "
         "COMMAND_INVALID_VALUE and nothing changes.",
         [k[0], {**k[1], "roving": True}, {**k[2], "roving": True}, k[3]],
         "move the key at frame 4 to frame 2", None)
    k = [key(0, [0, 0], spatial=[0, 0, 0, 60]), key(5, [60, 0], spatial=[0, 60, 0, 0]),
         key(10, [120, 0])]
    case("FX-ROVE-004", "A curved path counts at its own length, measured along 64 straight "
         "pieces: the arch is longer than the straight run after it.",
         k, "make the key at frame 5 roving",
         rove([k[0], {**k[1], "roving": True}, k[2]]))

    k = [key(0, [0, 0], EASE_IN_OUT, spatial=[0, 0, 0, 20]), key(4, [8, 4], kind="auto")]
    xs, ys = separate(k)
    case("FX-SEP-003", "Separating a position: each key becomes a key of X and a key of Y with "
         "the same ease and kind; the path handles are dropped.",
         k, "separate the position", {"x": xs, "y": ys})
    xs, ys = SEP["FX-SEP-002"][1:]
    case("FX-SEP-004", "Joining FX-SEP-002 again: a key wherever either had one, holding the "
         "position at that frame, with X's ease where X had a key and else Y's.",
         {"x": xs, "y": ys}, "join the position", join(xs, ys))
    return cases


# --- the project files and the tables -------------------------------------------------------------

def property_json(keys):
    out = []
    for k in keys:
        record = {"frame": k["frame"], "value": k["value"],
                  "interp": k["interp"] if isinstance(k["interp"], str) else "ease"}
        if not isinstance(k["interp"], str):
            record["ease"] = list(k["interp"])
        out.append(record)
    return {"base": keys[0]["value"], "keyframes": out}


def project_json(fx, xs, ys):
    project = adj.project_json(fx, {"layers": [adj.BG]})
    comp = project["compositions"][0]
    comp["duration_frames"] = 5
    comp["work_area"]["end_frame_exclusive"] = 5
    comp["layers"][0]["out_frame"] = 5
    comp["layers"][0]["transform"]["position"] = {"x": property_json(xs), "y": property_json(ys)}
    return project


def show(k):
    words = k["interp"] if isinstance(k["interp"], str) else \
        "ease [" + ", ".join(f"{v:.9g}" for v in k["interp"]) + "]"
    extra = "".join(f", {n}" for n in ("kind", "roving") if k.get(n)
                    for n in [k[n] if n == "kind" else n])
    if k.get("spatial"):
        extra += f", path handles {k['spatial']}"
    return f"| {k['frame']} | {k['value']} | {words}{extra} |"


def table(keys):
    return "\n".join(["| frame | value | to the next key |", "| --- | --- | --- |"]
                     + [show(k) for k in keys])


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bg.png").write_bytes(adj.png(adj.DRAWINGS["bg"]))
    expected = {"tolerance": TOLERANCE, "cases": {}}

    for fx, (says, xs, ys) in SEP.items():
        project = f"{fx.lower().replace('-', '_')}.json"
        (OUT / project).write_text(json.dumps(project_json(fx, xs, ys), indent=2) + "\n",
                                   encoding="utf-8")
        values = {str(f): [value_at(xs, f), value_at(ys, f)] for f in range(5)}
        expected["cases"][fx] = {"says": says, "project": project, "position": values}
        print(f"{fx}: {says}\n\n| frame | X | Y |\n| --- | --- | --- |")
        for f, (x, y) in values.items():
            print(f"| {f} | {x:.9g} | {y:.9g} |")
        print()

    for fx, c in sorted(edits().items()):
        expected["cases"][fx] = c
        print(f"{fx}: {c['says']}\n")
        for word, keys in (("Before", c["before"]), ("After: " + c["edit"], c["after"])):
            if keys is None:
                print(f"{word}: refused.\n")
            elif isinstance(keys, dict):
                for axis in ("x", "y"):
                    print(f"{word}, {axis.upper()}:\n\n{table(keys[axis])}\n")
            else:
                print(f"{word}:\n\n{table(keys)}\n")

    (OUT / "expected_keykind.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")

    # The claims the cases are there to make, worked by hand and checked on the numbers above.
    c = expected["cases"]
    near = lambda a, b: all(abs(p - q) < 1e-12 for p, q in zip(a, b))  # noqa: E731
    one = c["FX-KIND-001"]["after"]
    assert near(one[0]["interp"], [1 / 3, 1 / 3, 2 / 3, 17 / 18])
    assert near(one[1]["interp"], [1 / 3, -2 / 9, 2 / 3, 2 / 3])
    assert all(near(curve(k), LINEAR) for k in c["FX-KIND-002"]["after"])
    three = c["FX-KIND-003"]["after"]
    assert near(three[0]["interp"][2:], [2 / 3, 13 / 30]) and near(three[1]["interp"][:2],
                                                                    [1 / 3, 17 / 72])
    four = c["FX-KIND-004"]["after"]
    assert near(four[0]["interp"][2:], [2 / 3, 5 / 6]) and near(four[1]["interp"][:2],
                                                                 [1 / 3, 2 / 3])
    five = c["FX-KIND-005"]["after"]
    assert near(five[0]["interp"], [1 / 3, 1 / 3, 2 / 3, 0.75])
    assert near(five[1]["interp"], [1 / 3, 0.5, 2 / 3, 2 / 3])
    six = c["FX-KIND-006"]["after"]
    assert near(six[0]["interp"][2:], [2 / 3, 2 / 3]) and six[1]["interp"] == EASED
    for fx in ("FX-KIND-001", "FX-KIND-003", "FX-KIND-004", "FX-KIND-005", "FX-KIND-006"):
        s_in, s_out = speeds(c[fx]["after"], 1)
        assert abs(s_in - s_out) < 1e-12, fx
    seven = c["FX-KIND-007"]["after"]
    assert near(seven[0]["interp"][2:], [2 / 3, 5 / 6]) and seven[1]["interp"] == "linear"
    assert [k["frame"] for k in c["FX-ROVE-001"]["after"]] == [0, 3, 10]
    assert [k["frame"] for k in c["FX-ROVE-002"]["after"]] == [0, 1, 2, 3]
    arch = c["FX-ROVE-004"]["after"]
    assert arch[1]["frame"] > 5, "the arch is longer than the straight run"
    sep = c["FX-SEP-003"]["after"]
    assert [k["value"] for k in sep["x"]] == [0, 8] and [k["value"] for k in sep["y"]] == [0, 4]
    assert all("spatial" not in k for k in sep["x"] + sep["y"]) and sep["y"][1]["kind"] == "auto"
    joined = c["FX-SEP-004"]["after"]
    assert [(k["frame"], k["value"]) for k in joined] == [(0, [0, 2]), (1, [2, 2]), (3, [6, 6]),
                                                          (4, [8, 6])]
    assert c["FX-SEP-002"]["position"]["2"] == [4, 2]


if __name__ == "__main__":
    main()
