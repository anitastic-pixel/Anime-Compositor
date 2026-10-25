"""A layer's Preserve Underlying Transparency, After Effects' T switch (D-828), worked a second way.

After Effects' help ("Track mattes and traveling mattes", Preserve Underlying Transparency): the
layer gets its transparency from the composite of the layers below it, so its opaque areas show
only over opaque areas beneath, "similar to the behavior of a clipping mask in Adobe Photoshop".
That is Porter and Duff's "atop": with document 21's general blend equation

    Co = (1-As) Cd + (1-Ad) Cs + As Ad B,   Ao = As + Ad - As Ad,

the layer's own term over nothing, (1-Ad) Cs, is not drawn, and the alpha beneath is kept:

    Co = (1-As) Cd + As Ad B,               Ao = Ad.

For Normal (B = cs) that is Cs Ad + Cd (1-As). A track matte cuts the layer first (document 21's
step 5) and its opacity scales it (step 6), as without the switch; a matte-only layer is not part
of what is beneath. A stencil draws nothing of its own, so the switch changes nothing on one. A
null or an adjustment layer has no switch: such a file is refused with the reason.

Nothing here is read from the program. Writes Fixtures/preserve_transparency/.

The composition, 10 by 8: a backdrop of five strips 2 pixels wide, encoded (0.25, 0.75, 0.5)
opaque, (0.9, 0.2, 0.1) at 50%, (0.1, 0.3, 0.95) opaque, (0.5, 0.5, 0.5) at 25%, and columns 8
and 9 clear; above it the layer under test, the whole frame, four strips 2 pixels tall encoded
(0.8, 0.3, 0.5), (0.2, 0.9, 0.6), (1, 0.85, 0.1) and (0.35, 0.1, 0.7), each its own solid, the
last at 50% opacity. Linear premultiplied, every pixel of frame 0.
"""

import json
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "preserve_transparency"
W, H = 10, 8
TOLERANCE = 2e-5

BACK = [((0.25, 0.75, 0.5), 100), ((0.9, 0.2, 0.1), 50), ((0.1, 0.3, 0.95), 100),
        ((0.5, 0.5, 0.5), 25)]
TOP = [((0.8, 0.3, 0.5), 100), ((0.2, 0.9, 0.6), 100), ((1.0, 0.85, 0.1), 100),
       ((0.35, 0.1, 0.7), 50)]
# The matte of FX-PRESERVE-006: an opaque solid over columns 0 to 4, matte only.
MATTE_WIDTH = 5
# The stencil of FX-PRESERVE-007, one solid over the whole frame: a stencil keeps only what is
# under it, so four strips would each clear the others' rows.
SINGLE = ((0.8, 0.3, 0.5), 50)


def lin(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def enc(c):
    c = min(1.0, max(0.0, c))
    return c * 12.92 if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def mix(mode, cs, cd):
    """B, the mode's blend of the straight colours (document 21; D-301's Overlay encoded)."""
    if mode == "multiply":
        return [cs[c] * cd[c] for c in range(3)]
    if mode == "screen":
        return [cs[c] + cd[c] - cs[c] * cd[c] for c in range(3)]
    if mode == "add":
        return [min(1.0, cs[c] + cd[c]) for c in range(3)]
    if mode == "overlay":
        out = []
        for c in range(3):
            b, s = enc(cd[c]), enc(cs[c])
            out.append(lin(2 * b * s if b <= 0.5 else 1 - 2 * (1 - b) * (1 - s)))
        return out
    raise ValueError(mode)


def straight(p):
    return [0.0, 0.0, 0.0] if p[3] == 0 else [v / p[3] for v in p[:3]]


def beneath(x):
    if x >= 8:
        return [0.0, 0.0, 0.0, 0.0]
    color, opacity = BACK[x // 2]
    a = opacity / 100
    return [lin(v) * a for v in color] + [a]


def layer(x, y, matte, single=False):
    color, opacity = SINGLE if single else TOP[y // 2]
    a = opacity / 100
    if matte and x >= MATTE_WIDTH:
        a = 0.0
    return [lin(v) * a for v in color] + [a]


def pixel(case, x, y):
    dst = beneath(x)
    src = layer(x, y, case.get("matte", False), case.get("single", False))
    mode, keep = case["mode"], case["preserve"]
    a_s, a_d = src[3], dst[3]
    if mode == "stencil_alpha":
        return [v * a_s for v in dst]
    if mode == "normal":
        b = straight(src)
    else:
        b = mix(mode, straight(src), straight(dst))
    own = 0.0 if keep else (1 - a_d)
    rgb = [(1 - a_s) * dst[c] + own * src[c] + a_s * a_d * b[c] for c in range(3)]
    return rgb + [a_d if keep else a_s + a_d - a_s * a_d]


def transform(x, y, w, h, opacity=100):
    t = lambda v: {"base": v, "keyframes": []}
    return {"anchor": t([w / 2, h / 2]), "position": t([x + w / 2, y + h / 2]),
            "scale": t([100, 100]), "rotation": t(0), "opacity": t(opacity / 100)}


def solid(lid, color, x, y, w, h, opacity, blend="normal", preserve=False, matte=None):
    l = {"id": lid, "kind": "solid", "name": lid, "enabled": True, "locked": False,
         "in_frame": 0, "out_frame": 1,
         "solid": {"color": [lin(v) for v in color], "width": w, "height": h},
         "transform": transform(x, y, w, h, opacity), "masks": [], "matte": matte,
         "blend_mode": blend, "effects": []}
    if preserve:
        l["preserve_transparency"] = True
    return l


def project(case, pid):
    layers = [solid(f"back-{i}", c, 2 * i, 0, 2, H, o) for i, (c, o) in enumerate(BACK)]
    matte = None
    if case.get("matte"):
        layers.insert(0, solid("matte", (1.0, 1.0, 1.0), 0, 0, MATTE_WIDTH, H, 100))
        matte = {"layer_id": "matte", "mode": "alpha", "matte_only": True}
    keep = case["preserve"]
    if case.get("single"):
        layers.append(solid("top", SINGLE[0], 0, 0, W, H, SINGLE[1], case["mode"], keep))
    else:
        layers += [solid(f"top-{j}", c, 0, 2 * j, W, 2, o, case["mode"], keep, matte)
                   for j, (c, o) in enumerate(TOP)]
    if case.get("refused") == "adjustment":
        layers.append({"id": "adj", "kind": "adjustment", "name": "adj", "enabled": True,
                       "locked": False, "in_frame": 0, "out_frame": 1,
                       "transform": transform(0, 0, W, H), "mask": None, "matte": None,
                       "blend_mode": "normal", "effects": [],
                       "preserve_transparency": True})
    if case.get("refused") == "null":
        layers.append({"id": "null", "kind": "null", "name": "null", "enabled": True,
                       "locked": False, "in_frame": 0, "out_frame": 1,
                       "transform": transform(0, 0, W, H), "mask": None, "matte": None,
                       "blend_mode": "normal", "effects": [],
                       "preserve_transparency": True})
    return {
        "schema_version": 0, "project_id": pid,
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 1,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": [l["id"] for l in layers], "layers": layers,
        }],
    }


CASES = [
    ({"mode": "normal", "preserve": True},
     "Normal with T: each strip shows only over the backdrop, by its alpha (Cs Ad + Cd (1-As)); "
     "columns 8 and 9 stay clear and every pixel keeps the backdrop's alpha."),
    ({"mode": "multiply", "preserve": True},
     "Multiply with T: the multiplied colour where the backdrop is, nothing over the clear "
     "columns, the backdrop's alpha kept."),
    ({"mode": "screen", "preserve": True}, "Screen with T, the same way."),
    ({"mode": "add", "preserve": True}, "Add with T, the same way."),
    ({"mode": "overlay", "preserve": True},
     "Overlay (D-301, on the encoded colours) with T, the same way."),
    ({"mode": "normal", "preserve": True, "matte": True},
     "Normal with T and an alpha track matte over columns 0 to 4: the matte cuts the layer first, "
     "so columns 5 to 9 are the backdrop as it is."),
    ({"mode": "stencil_alpha", "preserve": True, "single": True},
     "Stencil Alpha with T, one solid over the whole frame at 50%: a stencil draws nothing of its "
     "own, so the switch changes nothing; the backdrop is kept at half, as D-301 has it."),
    ({"mode": "normal", "preserve": False},
     "The same layers with the switch off, as every file before D-828: plain normal over, the "
     "strips drawn over the clear columns too."),
]
REFUSED = [
    ({"mode": "normal", "preserve": False, "refused": "adjustment"},
     "An adjustment layer with T: refused with the reason, as a motion-blur switch on one is "
     "(D-188); nothing is drawn without it (document 28).",
     "no preserve_transparency on a null or adjustment layer"),
    ({"mode": "normal", "preserve": False, "refused": "null"},
     "A null with T: refused the same way.",
     "no preserve_transparency on a null or adjustment layer"),
]


def write(name, data):
    (OUT / name).write_text(json.dumps(data, indent=1) + "\n", encoding="utf-8", newline="\n")


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    cases, refused = {}, {}
    n = 0
    for case, says in CASES:
        n += 1
        fid, name = f"FX-PRESERVE-{n:03d}", f"fx_preserve_{n:03d}.json"
        write(name, project(case, fid.lower()))
        frame = [pixel(case, x, y) for y in range(H) for x in range(W)]
        cases[fid] = {"says": says, "mode": case["mode"], "preserve": case["preserve"],
                      "project": name, "frames": {"0": frame}}
    n = 19
    for case, says, reason in REFUSED:
        n += 1
        fid, name = f"FX-PRESERVE-{n:03d}", f"fx_preserve_{n:03d}.json"
        write(name, project(case, fid.lower()))
        refused[fid] = {"says": says, "kind": case["refused"], "project": name, "reason": reason}
    write("expected_preserve_transparency.json",
          {"tolerance": TOLERANCE, "width": W, "height": H, "cases": cases, "refused": refused})
    print(f"wrote {len(cases)} drawn cases and {len(refused)} refused cases to {OUT}")


if __name__ == "__main__":
    main()
