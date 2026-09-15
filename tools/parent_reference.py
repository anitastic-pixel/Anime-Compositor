"""Parenting, worked a second way.

D-57 proposes that a layer may name another layer in the same composition as its parent, and
that the parent's transform is applied after the layer's own:

    p_comp = M_parent_world * M_child * p_child_layer,   M = T(position) R(rotation) S(scale/100) T(-anchor)

chained until a layer with no parent. This file is the reference for the numbers document 25
pins against that sentence.

**This file never builds a matrix.** It takes a point and walks it through document 21's four
steps one at a time - subtract the anchor, scale, rotate clockwise, add the position - then hands
the result to the parent and does the same again. Undoing a transform walks the same steps
backwards. The build multiplies 3x3 matrices, so an agreement between the two is two answers and
not one answer twice.

Keeping a layer's place on screen when it gains or loses a parent (D-57) is worked here from its
definition: the new position is the old anchor point carried through the parent's chain
backwards; the new rotation is the old one less the chain's rotations; the new scale is the old
one divided by the chain's scales. The script then checks the claim document 25 makes about it,
by carrying corners of the layer through before and after.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/parent_reference.py
"""

from math import cos, sin, radians


def value(prop, frame):
    """A property's value at a frame: its base, or linear between keys, held beyond them."""
    keys = prop.get("keys")
    if not keys:
        return prop["base"]
    if frame <= keys[0][0]:
        return keys[0][1]
    for (f0, v0), (f1, v1) in zip(keys, keys[1:]):
        if frame <= f1:
            t = (frame - f0) / (f1 - f0)
            if isinstance(v0, list):
                return [a + t * (b - a) for a, b in zip(v0, v1)]
            return v0 + t * (v1 - v0)
    return keys[-1][1]


def at(layer, frame):
    return {k: value(layer[k], frame) for k in ("anchor", "position", "scale", "rotation")}


def forward(xf, p):
    x, y = p[0] - xf["anchor"][0], p[1] - xf["anchor"][1]
    x, y = x * xf["scale"][0] / 100, y * xf["scale"][1] / 100
    c, s = cos(radians(xf["rotation"])), sin(radians(xf["rotation"]))
    x, y = c * x - s * y, s * x + c * y
    return [x + xf["position"][0], y + xf["position"][1]]


def backward(xf, p):
    x, y = p[0] - xf["position"][0], p[1] - xf["position"][1]
    c, s = cos(radians(xf["rotation"])), sin(radians(xf["rotation"]))
    x, y = c * x + s * y, -s * x + c * y
    x, y = x * 100 / xf["scale"][0], y * 100 / xf["scale"][1]
    return [x + xf["anchor"][0], y + xf["anchor"][1]]


def chain(layers, name):
    """The layer, then its parent, then its parent's parent, up to a layer with none."""
    out = []
    while name is not None:
        out.append(name)
        name = layers[name].get("parent")
    return out


def to_comp(layers, name, p, frame):
    for n in chain(layers, name):
        p = forward(at(layers[n], frame), p)
    return p


def keep_place(layers, child, parent, frame):
    """The child's new static values when it gains `parent`, keeping its anchor on screen."""
    xf = at(layers[child], frame)
    up = chain(layers, parent)
    pos = xf["position"]
    for n in reversed(up):
        pos = backward(at(layers[n], frame), pos)
    rot = xf["rotation"] - sum(at(layers[n], frame)["rotation"] for n in up)
    sx, sy = xf["scale"]
    for n in up:
        sx /= at(layers[n], frame)["scale"][0] / 100
        sy /= at(layers[n], frame)["scale"][1] / 100
    return {"anchor": xf["anchor"], "position": pos, "scale": [sx, sy], "rotation": rot}


def static(anchor=(0, 0), position=(0, 0), scale=(100, 100), rotation=0, parent=None):
    layer = {
        "anchor": {"base": list(anchor)},
        "position": {"base": list(position)},
        "scale": {"base": list(scale)},
        "rotation": {"base": rotation},
    }
    if parent:
        layer["parent"] = parent
    return layer


def table(title, cols, rows):
    print(title)
    print("| " + " | ".join(cols) + " |")
    print("| " + " | ".join("---" for _ in cols) + " |")
    for r in rows:
        print("| " + " | ".join(str(v) for v in r) + " |")
    print()


def fx_001():
    layers = {
        "P": static(anchor=(50, 50), position=(400, 300), rotation=90),
        "C": static(position=(100, 0), parent="P"),
    }
    rows = []
    for p in ([0, 0], [10, 0], [0, 10]):
        rows.append([str(p), *to_comp(layers, "C", p, 0)])
    table("FX-PARENT-001", ["child point", "x", "y"], rows)


def fx_002():
    layers = {
        "G": static(position=(960, 540), scale=(50, 50)),
        "P": static(position=(200, 0), parent="G"),
        "C": static(position=(100, 0), parent="P"),
    }
    layers["P"]["rotation"] = {"base": 0, "keys": [(0, 0), (24, 90)]}
    rows = []
    for f in (0, 6, 12, 24):
        rows.append([f, *to_comp(layers, "C", [0, 0], f), *to_comp(layers, "C", [20, 0], f)])
    table("FX-PARENT-002", ["frame", "origin x", "origin y", "(20,0) x", "(20,0) y"], rows)


CORNERS = ([0, 0], [100, 0], [0, 100], [100, 100])


def keep_case(title, parent_xf, child_xf):
    layers = {"P": static(**parent_xf), "C": static(**child_xf)}
    before = [to_comp(layers, "C", p, 0) for p in CORNERS]
    new = keep_place(layers, "C", "P", 0)
    layers["C"] = static(**{k: tuple(v) if isinstance(v, list) else v for k, v in new.items()}, parent="P")
    after = [to_comp(layers, "C", p, 0) for p in CORNERS]
    table(title + " new values", ["anchor", "position x", "position y", "scale x", "scale y", "rotation"],
          [[str(new["anchor"]), *new["position"], *new["scale"], new["rotation"]]])
    table(title + " corners", ["corner", "before x", "before y", "after x", "after y"],
          [[str(c), *b, *a] for c, b, a in zip(CORNERS, before, after)])
    return before, after


def fx_005():
    before, after = keep_case(
        "FX-PARENT-005",
        dict(anchor=(50, 50), position=(400, 300), scale=(200, 200), rotation=90),
        dict(anchor=(10, 20), position=(500, 400), rotation=30),
    )
    assert all(abs(b - a) < 1e-9 for pb, pa in zip(before, after) for b, a in zip(pb, pa)), "005 moved"


def fx_006():
    before, after = keep_case(
        "FX-PARENT-006",
        dict(position=(300, 200), scale=(200, 100)),
        dict(position=(500, 400), rotation=45),
    )
    assert all(abs(b - a) < 1e-9 for b, a in zip(before[0], after[0])), "006 anchor moved"
    assert any(abs(b - a) > 1 for b, a in zip(before[1], after[1])), "006 was exact"


if __name__ == "__main__":
    fx_001()
    fx_002()
    fx_005()
    fx_006()
