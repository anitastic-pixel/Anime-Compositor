"""The camera and depth, worked a second way.

D-58 proposes that a composition is seen through a camera, and that every layer sits on a plane
at a depth. The planes stay parallel to the screen; the camera has a place, a depth of its own
and a zoom, and never tilts or turns. A point that document 21 has already put in composition
space is then carried to the screen by

    s = zoom / (world_depth(layer) - camera_depth)

    screen = comp_centre + (p_comp - camera_position) * s

with every one of the three camera numbers read at the frame being drawn. This file is the
reference for the numbers document 25 pins against those two lines.

**This file never builds a matrix.** It reuses `parent_reference.py`, which walks a point
through document 21's four steps one at a time, and then does the two lines above by hand. The
build folds the projection into the layer's transform and multiplies 3x3 matrices, so an
agreement between the two is two answers and not one answer twice.

A layer's depth is measured from its parent's plane, the way its position is measured in its
parent's space, so `world_depth` adds the depths up the chain. That is worked here too, and so
is the draw order, which is the planes sorted from far to near with the layer stack breaking a
tie.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/camera_reference.py
"""

from parent_reference import chain, static, table, to_comp, value

W, H = 1920, 1080
CENTRE = [W / 2, H / 2]

# The camera a composition has when its file says nothing about one. It sits at the centre of
# the frame, `zoom` in front of the depth-0 plane, so that a layer at depth 0 lands exactly
# where document 21 puts it and every G1 fixture written before this entry still holds.
DEFAULT = {
    "position": {"base": list(CENTRE)},
    "depth": {"base": -W},
    "zoom": {"base": W},
}


def camera(position=CENTRE, depth=-W, zoom=W, **keyed):
    cam = {
        "position": {"base": list(position)},
        "depth": {"base": depth},
        "zoom": {"base": zoom},
    }
    for name, keys in keyed.items():
        cam[name] = {"base": cam[name]["base"], "keys": keys}
    return cam


def world_depth(layers, name):
    """A layer's depth from the camera's axis: its own, plus every plane it rides on."""
    return sum(layers[n].get("depth", 0) for n in chain(layers, name))


def scale(cam, layers, name, frame):
    """How big a plane is drawn. 1 on the plane the camera draws at true size."""
    ahead = world_depth(layers, name) - value(cam["depth"], frame)
    if ahead <= 0:
        return None  # CAMERA_PLANE_BEHIND: level with the camera or behind it.
    return value(cam["zoom"], frame) / ahead


def to_screen(cam, layers, name, p, frame):
    """A point of a layer, in the pixels of the picture that is drawn."""
    s = scale(cam, layers, name, frame)
    if s is None:
        return None
    at_comp = to_comp(layers, name, p, frame)
    eye = value(cam["position"], frame)
    return [c + (q - e) * s for c, q, e in zip(CENTRE, at_comp, eye)]


def order(layers, stack):
    """What is drawn first. Farthest away first; the layer stack breaks a tie."""
    return sorted(stack, key=lambda n: -world_depth(layers, n))


def num(v):
    """An exact number written as one, and anything else written in full."""
    if v is None:
        return "not drawn"
    r = round(v)
    return r if abs(v - r) < 1e-9 else v


def row(*values):
    return [num(v) if isinstance(v, float) or v is None else v for v in values]


def fx_001():
    """The camera nobody has touched changes nothing."""
    layers = {"A": static(anchor=(50, 50), position=(400, 300), scale=(150, 150), rotation=30)}
    rows = []
    for p in ([0, 0], [10, 0], [0, 10]):
        comp = to_comp(layers, "A", p, 0)
        screen = to_screen(DEFAULT, layers, "A", p, 0)
        rows.append(row(str(p), *comp, *screen))
        # Mathematically the identity, and not bitwise the identity: carrying a point out to the
        # camera and back again is two roundings. That is why D-58 requires a build to leave out
        # a projection that is the identity, rather than to apply it and land within tolerance.
        assert all(abs(a - b) < 1e-9 for a, b in zip(comp, screen)), f"the default camera moved {p}"
    table("FX-CAM-001", ["layer point", "comp x", "comp y", "screen x", "screen y"], rows)


def fx_002():
    """One plane, at four depths, seen by the camera nobody has touched."""
    rows = []
    for d in (-960, 0, 960, 1920):
        layers = {"A": static(position=(960, 540))}
        layers["A"]["depth"] = d
        s = scale(DEFAULT, layers, "A", 0)
        origin = to_screen(DEFAULT, layers, "A", [0, 0], 0)
        right = to_screen(DEFAULT, layers, "A", [100, 0], 0)
        rows.append(row(d, s, *origin, *right))
    table("FX-CAM-002", ["depth", "drawn at", "origin x", "origin y", "(100,0) x", "(100,0) y"], rows)


PARALLAX = {"near": 0, "middle": 640, "far": 1920}


def fx_003():
    """The move the whole entry is for: the camera tracks sideways and the planes disagree."""
    layers = {}
    for name, d in PARALLAX.items():
        layers[name] = static(position=(960, 540))
        layers[name]["depth"] = d
    cam = camera(position=(760, 540))
    cam["position"]["keys"] = [(0, [760, 540]), (48, [1160, 540])]

    rows = []
    for f in (0, 16, 32, 48):
        rows.append(row(f, *[to_screen(cam, layers, n, [0, 0], f)[0] for n in PARALLAX]))
    table("FX-CAM-003", ["frame", "near x", "middle x", "far x"], rows)

    travelled = []
    for n in PARALLAX:
        first = to_screen(cam, layers, n, [0, 0], 0)[0]
        last = to_screen(cam, layers, n, [0, 0], 48)[0]
        travelled.append(row(n, PARALLAX[n], first - last))
    table("FX-CAM-003 travel", ["plane", "depth", "pixels travelled"], travelled)
    assert travelled[0][2] > travelled[1][2] > travelled[2][2], "the planes did not disagree"


def fx_004():
    """A zoom and a dolly are different moves, which is why the camera has both numbers."""
    layers = {"near": static(position=(960, 540)), "far": static(position=(960, 540))}
    layers["far"]["depth"] = 1920

    zoomed = camera()
    zoomed["zoom"]["keys"] = [(0, 1920), (48, 2880)]
    dollied = camera()
    dollied["depth"]["keys"] = [(0, -1920), (48, -960)]

    rows = []
    for title, cam in (("zoom to 2880", zoomed), ("dolly in 960", dollied)):
        for f in (0, 48):
            near = to_screen(cam, layers, "near", [100, 0], f)[0]
            far = to_screen(cam, layers, "far", [100, 0], f)[0]
            rows.append(row(title, f, near, far, (near - 960) / (far - 960)))
    table("FX-CAM-004", ["move", "frame", "near x", "far x", "near over far"], rows)
    assert rows[0][4] == rows[1][4], "a zoom changed the parallax"
    assert rows[2][4] != rows[3][4], "a dolly did not change the parallax"


def fx_005():
    """What is drawn first, and what a tie does."""
    layers = {}
    for name, d in (("A", 0), ("B", 1920), ("C", 1920), ("D", -960)):
        layers[name] = static()
        layers[name]["depth"] = d
    stack = ["A", "B", "C", "D"]
    drawn = order(layers, stack)
    table("FX-CAM-005", ["layer stack", "depths", "drawn first to last"],
          [[", ".join(stack),
            ", ".join(str(layers[n]["depth"]) for n in stack),
            ", ".join(drawn)]])
    assert drawn == ["B", "C", "A", "D"], drawn


def fx_006():
    """Level with the camera, and behind it."""
    rows = []
    for d in (-2000, -1920, -1919, -960):
        layers = {"A": static(position=(960, 540))}
        layers["A"]["depth"] = d
        s = scale(DEFAULT, layers, "A", 0)
        rows.append(row(d, d - value(DEFAULT["depth"], 0), s))
    table("FX-CAM-006", ["depth", "in front of the camera by", "drawn at"], rows)
    assert rows[0][2] == "not drawn" and rows[1][2] == "not drawn"


def fx_007():
    """A layer rides on its parent's plane, the way it rides on its parent's place."""
    layers = {
        "P": static(anchor=(50, 50), position=(400, 300), rotation=90),
        "C": static(position=(100, 0), parent="P"),
    }
    layers["P"]["depth"] = 1920
    rows = []
    for name in ("P", "C"):
        rows.append(row(name, layers[name].get("depth", 0), world_depth(layers, name),
                        scale(DEFAULT, layers, name, 0)))
    table("FX-CAM-007 depths", ["layer", "own depth", "world depth", "drawn at"], rows)

    points = []
    for p in ([0, 0], [10, 0], [0, 10]):
        points.append(row(str(p), *to_comp(layers, "C", p, 0), *to_screen(DEFAULT, layers, "C", p, 0)))
    table("FX-CAM-007 child", ["child point", "comp x", "comp y", "screen x", "screen y"], points)
    assert world_depth(layers, "C") == 1920, "the child did not ride its parent's plane"


def fx_009():
    """A matte on another plane slides, because it is a plane and not a stencil."""
    layers = {"A": static(position=(960, 540)), "M": static(position=(960, 540))}
    layers["M"]["depth"] = 1920
    cam = camera(position=(760, 540))
    rows = []
    for name in ("A", "M"):
        x = to_screen(cam, layers, name, [0, 0], 0)[0]
        rows.append(row(name, layers[name].get("depth", 0), x))
    table("FX-CAM-009", ["layer", "depth", "origin x with the camera at 760"], rows)
    assert rows[0][2] != rows[1][2], "the matte did not slide"


if __name__ == "__main__":
    fx_001()
    fx_002()
    fx_003()
    fx_004()
    fx_005()
    fx_006()
    fx_007()
    fx_009()
