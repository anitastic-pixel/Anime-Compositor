"""A layer of this composition as an effect's setting, worked a second way.

D-189 proposes one new kind of effect setting, shared by Compound Blur, Displacement Map and
Gradient Wipe when they come: **a layer of this composition**. The effect holding it reads that
layer's picture as a map. This file pins what the map is, before any effect uses one.

The map for a holder layer H naming layer L at frame n:

1. L's own picture at n, by L's own timing: document 21's steps 1 to 3 - its drawing, solid,
   shapes or composition, its masks, its enabled effects - cut back to its step-1 rectangle, so an
   effect that grows the picture (a blur) is cut. Nothing after step 3 counts: not L's transform,
   parent, camera, matte, opacity, blend, its enabled switch, its matte-only role or its motion
   blur. If L is H itself, steps 1 and 2 only. An adjustment layer's picture is an opaque white
   rectangle the size of the composition through its masks, its effects not run. A null, a layer
   outside its in and out points, a composition layer outside its composition's frames, a missing
   composition or a missing drawing give an empty picture, with that layer's own warning if it
   has one.
2. The picture is fitted to H's step-1 picture, W by H pixels, by H's fit word. `center` puts it
   at dx = floor((W - w) / 2), dy likewise, and transparent black outside it. `tile` repeats it
   from there: ((x - dx) mod w, (y - dy) mod h). `stretch` reads it at u = (x + 1/2) w / W - 1/2,
   v likewise, each held within the picture, bilinear in premultiplied light. An empty picture
   fits to transparent black.
3. In Draft, the map is made at the size H's effects run at. For a solid or shape holder that is
   full size, so the map is the Full map. For a drawing holder with an enabled effect (D-99), a
   composition layer (D-67) or an adjustment layer (D-66) it is the draft divisor d: L's step-1
   picture is taken down by d exactly as D-99 takes a drawing down - a composition layer's picture
   is already drawn at d - and L's masks and effects run at the small size with every distance
   divided by d.
4. A name that is empty is no map and nothing said. A name that is not a layer of H's own
   composition - a deleted layer, a layer of another composition - is no map, and
   `EFFECT_LAYER_MISSING` every frame; the effect holding it is skipped.

**This file never runs the build's code path.** It draws each map pixel by pixel from document
21, blurs with the two-dimensional kernel summed directly where the build runs two passes, and
takes pictures down with its own bilinear sampler.

Every case reads the one project `Fixtures/layer_map/layer_map.json`, a composition 8 by 6 of
three frames holding every layer the cases name. The drawings are written into
`Fixtures/layer_map/media`, the expected maps into `Fixtures/layer_map/expected_layer_map.json`,
and a picture of the three fits into `verification/B-125a proposal/layer_map.png`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/effect_layer_reference.py
"""

import json
import sys
from math import ceil, exp, floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from mask_reference import pixel_coverage  # noqa: E402
from motion_blur_reference import png  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "layer_map"
PICTURE = ROOT / "verification" / "B-125a proposal" / "layer_map.png"
TOLERANCE = 1e-6
W, H = 8, 6
DRAFT = 4
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- drawings -------------------------------------------------------------------------------

DRAWINGS = {
    # 5 by 3, every pixel different: red across, green down, blue full.
    "card": [[(51 * x, 127 * y, 255, 255) for x in range(5)] for y in range(3)],
    # The same card's second drawing, for timing: red full, green across, blue down.
    "card_b": [[(255, 51 * x, 127 * y, 255) for x in range(5)] for y in range(3)],
    # 10 by 8, bigger than the composition each way.
    "big": [[(25 * x, 36 * y, 0, 255) for x in range(10)] for y in range(8)],
    # 8 by 8, white in columns 0-1 and 4-5, clear in 2-3 and 6-7.
    "stripes": [[(255, 255, 255, 255) if (x // 2) % 2 == 0 else (0, 0, 0, 0) for x in range(8)]
                for _ in range(8)],
}


def decoded(name):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    rows = DRAWINGS[name]
    px = []
    for row in rows:
        for r, g, b, a in row:
            a = a / 255
            px.append([srgb_to_linear(r / 255) * a, srgb_to_linear(g / 255) * a,
                       srgb_to_linear(b / 255) * a, a])
    return pic(len(rows[0]), len(rows), px)


# --- pictures -------------------------------------------------------------------------------

def pic(w, h, px=None):
    return {"w": w, "h": h, "px": px if px is not None else [list(CLEAR) for _ in range(w * h)]}


EMPTY = pic(0, 0)


def at(p, x, y):
    return p["px"][y * p["w"] + x] if 0 <= x < p["w"] and 0 <= y < p["h"] else CLEAR


def bilinear(p, sx, sy):
    """Document 21's resampling: `(sx, sy)` a place in the picture, pixel centres at halves;
    transparent black outside."""
    fx, fy = sx - 0.5, sy - 0.5
    x0, y0 = floor(fx), floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            s = at(p, x0 + dx, y0 + dy)
            for c in range(4):
                out[c] += s[c] * wx * wy
    return out


def extent(n, d):
    return -(-n // d)


def placed(src, w, h, d, pos=(0, 0)):
    """`src` put at `pos` in a w by h picture and read at 1/d: the draft frame's sampling, and at
    d = 1 a whole-pixel move."""
    ew, eh = extent(w, d), extent(h, d)
    return pic(ew, eh, [bilinear(src, (x + 0.5) * d - pos[0], (y + 0.5) * d - pos[1])
                        for y in range(eh) for x in range(ew)])


def masked(p, mask, d):
    """A polygon mask, its corners divided by d, by ADR-016's sixteen samples."""
    if mask is None:
        return p
    outline = [(x / d, y / d) for x, y in mask]
    out = []
    for y in range(p["h"]):
        for x in range(p["w"]):
            m = pixel_coverage(outline, x, y, 0)
            out.append([c * m for c in at(p, x, y)])
    return pic(p["w"], p["h"], out)


def exposure(p, stops):
    g = 2 ** stops
    return pic(p["w"], p["h"], [[q[0] * g, q[1] * g, q[2] * g, q[3]] for q in p["px"]])


def blur(p, sigma):
    """The two-dimensional kernel summed directly, radius ceil(3 sigma), transparent outside.
    Only the picture's own rectangle is worked: the map is cut back to it."""
    r = ceil(3 * sigma)
    if r == 0:
        return p
    one = [exp(-(k * k) / (2 * sigma * sigma)) for k in range(-r, r + 1)]
    total = sum(one)
    one = [v / total for v in one]
    out = []
    for y in range(p["h"]):
        for x in range(p["w"]):
            acc = [0.0] * 4
            for j in range(-r, r + 1):
                for i in range(-r, r + 1):
                    s = at(p, x + i, y + j)
                    k = one[i + r] * one[j + r]
                    for c in range(4):
                        acc[c] += s[c] * k
            out.append(acc)
    return pic(p["w"], p["h"], out)


def run_effects(p, effects, d):
    for kind, *args in effects:
        if kind == "off":
            continue
        if kind == "exposure":
            p = exposure(p, args[0])
        elif kind == "blur":
            p = blur(p, args[0] / d)
        else:
            raise AssertionError(kind)
    return p


# --- fitting --------------------------------------------------------------------------------

def fit(p, word, fw, fh):
    w, h = p["w"], p["h"]
    if w == 0 or h == 0:
        return pic(fw, fh)
    dx, dy = floor((fw - w) / 2), floor((fh - h) / 2)
    out = []
    for y in range(fh):
        for x in range(fw):
            if word == "center":
                out.append(list(at(p, x - dx, y - dy)))
            elif word == "tile":
                out.append(list(at(p, (x - dx) % w, (y - dy) % h)))
            elif word == "stretch":
                u = min(max((x + 0.5) * w / fw - 0.5, 0.0), w - 1)
                v = min(max((y + 0.5) * h / fh - 0.5, 0.0), h - 1)
                i0, j0 = floor(u), floor(v)
                i1, j1 = min(i0 + 1, w - 1), min(j0 + 1, h - 1)
                fu, fv = u - i0, v - j0
                acc = [0.0] * 4
                for (i, wi) in ((i0, 1 - fu), (i1, fu)):
                    for (j, wj) in ((j0, 1 - fv), (j1, fv)):
                        s = at(p, i, j)
                        for c in range(4):
                            acc[c] += s[c] * wi * wj
                out.append(acc)
            else:
                raise AssertionError(word)
    return pic(fw, fh, out)


# --- the project ----------------------------------------------------------------------------

def prop(base):
    return {"base": base, "keyframes": []}


def rect(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


EFFECT_JSON = {
    "exposure": lambda s: ("core.exposure", {"stops": s}),
    "blur": lambda s: ("core.gaussian_blur", {"sigma_px": s}),
}


def layer(id, kind, *, effects=(), mask=None, in_frame=0, out_frame=3, enabled=True,
          position=(0, 0), anchor=(0, 0), scale=(100, 100), rotation=0, opacity=1,
          blend="normal", matte=None, **extra):
    record = {
        "id": id, "kind": kind, "name": id, "enabled": enabled, "locked": False,
        "in_frame": in_frame, "out_frame": out_frame,
        "transform": {"anchor": prop(list(anchor)), "position": prop(list(position)),
                      "scale": prop(list(scale)), "rotation": prop(rotation),
                      "opacity": prop(opacity)},
        "mask": None if mask is None else {"vertices": mask, "enabled": True, "inverted": False},
        "matte": matte, "blend_mode": blend, "effects": [],
    }
    for n, e in enumerate(effects):
        enabled_fx = e[0] != "off"
        kind_fx, *args = e[1:] if not enabled_fx else e
        type_id, params = EFFECT_JSON[kind_fx](*args)
        record["effects"].append({"instance_id": f"fx-{id}-{n}", "type_id": type_id,
                                  "enabled": enabled_fx, "parameters": params})
    record.update(extra)
    return record


def raster(id, asset, **kw):
    spans = kw.pop("spans", [])
    offset = kw.pop("offset", 0)
    return layer(id, "raster", asset_id=asset, source_offset_frames=offset,
                 exposure_spans=spans, **kw)


def solid(id, w, h, color, **kw):
    return layer(id, "solid", solid={"color": color, "width": w, "height": h}, **kw)


def nested(id, comp_id, **kw):
    return layer(id, "composition", composition_id=comp_id, source_offset_frames=0, **kw)


SHAPE_FILL = [0.25, 1.0, 0.5]
DOT = [0.25, 0.5, 1.0]
FLIP_SPANS = [{"start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1},
              {"start_frame": 1, "end_frame_exclusive": 3, "drawing_number": 2}]


def shape(id, x0, y0, x1, y1):
    record = layer(id, "shape")
    del record["mask"]
    record["masks"] = []  # a shape layer keeps a list
    pts = [{"point": p, "in": [0, 0], "out": [0, 0]} for p in rect(x0, y0, x1, y1)]
    record["shapes"] = [{"name": "Box", "enabled": True, "closed": True,
                         "path": {"base": {"points": pts}, "keyframes": []},
                         "fill": {"color": SHAPE_FILL, "opacity": 1.0}, "stroke": None}]
    return record


def adjustment(id, **kw):
    return layer(id, "adjustment", **kw)


# Every layer the cases name, in one composition. Each is described where it is first used.
LAYERS = [
    solid("holder", W, H, [0.0, 0.0, 0.0],
          matte={"layer_id": "card_hidden", "mode": "alpha", "matte_only": True}),
    raster("card", "asset-card"),
    raster("card_placed", "asset-card", position=(3, 2), anchor=(2.5, 1.5), scale=(200, 50),
           rotation=30, opacity=0.5, blend="multiply"),
    raster("card_hidden", "asset-card", enabled=False),
    raster("card_masked", "asset-card", mask=rect(0, 0, 3, 3)),
    raster("card_bright", "asset-card", effects=[("exposure", 1)]),
    raster("card_off_fx", "asset-card", effects=[("off", "exposure", 1)]),
    raster("card_soft", "asset-card", effects=[("blur", 0.5)]),
    raster("self", "asset-card", mask=rect(0, 0, 3, 3), effects=[("exposure", 1)]),
    raster("flip", "asset-flip", spans=FLIP_SPANS),
    raster("flip_late", "asset-flip", spans=FLIP_SPANS, in_frame=1),
    raster("flip_ahead", "asset-flip", spans=FLIP_SPANS, offset=1),
    solid("solid", 4, 2, [1.0, 0.5, 0.25], position=(5, 5)),
    shape("shape", 1, 1, 5, 4),
    nested("nest", "comp-inner"),
    nested("nest_bright", "comp-inner", effects=[("exposure", 1)]),
    nested("nest_gone", "comp-gone"),
    layer("rig", "null"),
    adjustment("adjust", mask=rect(0, 0, 4, 6), effects=[("exposure", 1)]),
    raster("gone_file", "asset-gone"),
    raster("big", "asset-big"),
    solid("holder_card", 5, 3, [0.0, 0.0, 0.0]),
    solid("holder_small", 3, 2, [0.0, 0.0, 0.0]),
    raster("stripes_fx", "asset-stripes", effects=[("exposure", -1)]),
    raster("card_draft", "asset-card", mask=rect(0, 0, 4, 3),
           effects=[("exposure", 1), ("blur", 2)]),
]
INNER_W, INNER_H = 8, 4
INNER = [solid("dot", 4, 2, DOT, position=(2, 1))]


def composition(id, w, h, frames, layers):
    return {"id": id, "name": id, "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": [l["id"] for l in layers], "layers": layers}


def still(name, path):
    return {"id": "asset-" + name, "kind": "still", "name": name, "path": path,
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}


PROJECT = {
    "schema_version": 0, "project_id": "proj-layer-map",
    "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
    "assets": [still("card", "media/card.png"), still("big", "media/big.png"),
               still("stripes", "media/stripes.png"), still("gone", "media/gone.png"),
               {"id": "asset-flip", "kind": "image_sequence", "name": "flip",
                "pattern": "flip_####.png",
                "frames": {"1": "media/card.png", "2": "media/card_b.png"},
                "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
    "compositions": [composition("comp-main", W, H, 3, LAYERS),
                     composition("comp-inner", INNER_W, INNER_H, 2, INNER)],
}
BY_ID = {l["id"]: l for l in LAYERS}


# --- the rule -------------------------------------------------------------------------------

def local_frame(l, n):
    if not l["in_frame"] <= n < l["out_frame"]:
        return None
    return n - l["in_frame"] + l.get("source_offset_frames", 0)


def drawing_at(l, n):
    """The drawing a raster layer shows at comp frame n, or None."""
    local = local_frame(l, n)
    if local is None:
        return None
    asset = next(a for a in PROJECT["assets"] if a["id"] == l["asset_id"])
    if asset["kind"] == "still":
        path = asset["path"]
    else:
        span = next(s for s in l["exposure_spans"]
                    if s["start_frame"] <= local < s["end_frame_exclusive"])
        path = asset["frames"][str(span["drawing_number"])]
    return Path(path).stem


def inner_picture(d):
    """comp-inner's frame: its one solid, drawn at 1/d (D-67: the inner frame is drawn at the
    draft divisor)."""
    (dot,) = INNER
    s = dot["solid"]
    src = pic(s["width"], s["height"], [s["color"] + [1.0] for _ in range(s["width"] * s["height"])])
    return placed(src, INNER_W, INNER_H, d, dot["transform"]["position"]["base"])


def step1(l, n, d, said):
    """Layer l's step-1 picture at comp frame n, made at 1/d, or EMPTY."""
    kind = l["kind"]
    if kind == "null" or local_frame(l, n) is None:
        return EMPTY
    if kind == "composition":
        if l["composition_id"] != "comp-inner":
            said.append("COMPOSITION_REFERENCE_MISSING")
            return EMPTY
        if not 0 <= local_frame(l, n) < 2:
            return EMPTY
        return inner_picture(d)
    if kind == "raster":
        name = drawing_at(l, n)
        if name not in DRAWINGS:
            said.append("MEDIA_MISSING")
            return EMPTY
        full = decoded(name)
    elif kind == "solid":
        s = l["solid"]
        full = pic(s["width"], s["height"], [s["color"] + [1.0] for _ in range(s["width"] * s["height"])])
    elif kind == "shape":
        (sh,) = l["shapes"]
        (x0, y0), _, (x1, y1), _ = [p["point"] for p in sh["path"]["base"]["points"]]
        full = pic(W, H, [SHAPE_FILL + [1.0] if x0 <= x < x1 and y0 <= y < y1 else list(CLEAR)
                          for y in range(H) for x in range(W)])
    elif kind == "adjustment":
        full = pic(W, H, [[1.0, 1.0, 1.0, 1.0] for _ in range(W * H)])
    else:
        raise AssertionError(kind)
    return full if d == 1 else placed(full, full["w"], full["h"], d)


def layer_picture(l, n, d, own, said):
    """Steps 1 to 3 (1 and 2 for the holder itself), cut to the step-1 rectangle."""
    p = step1(l, n, d, said)
    if p is EMPTY:
        return EMPTY
    p = masked(p, l.get("mask") and l["mask"]["vertices"], d)
    if own or l["kind"] == "adjustment":
        return p
    effects = [(("off",) if not e["enabled"] else ()) + (
        ("exposure", e["parameters"]["stops"]) if e["type_id"] == "core.exposure"
        else ("blur", e["parameters"]["sigma_px"])) for e in l["effects"]]
    return run_effects(p, effects, d)


def holder_scale(h, quality):
    """The divisor the holder's effects run at."""
    if quality == "full":
        return 1
    if h["kind"] in ("composition", "adjustment"):
        return DRAFT
    if h["kind"] == "raster" and any(e["enabled"] for e in h["effects"]):
        return DRAFT
    return 1


def layer_map(holder, named, word, n, quality):
    said = []
    if named == "":
        return None, said
    if named not in BY_ID:
        return None, ["EFFECT_LAYER_MISSING"]
    h = BY_ID[holder]
    d = holder_scale(h, quality)
    size = step1(h, n, d, [])
    p = layer_picture(BY_ID[named], n, d, named == holder, said)
    return fit(p, word, size["w"], size["h"]), said


# --- the cases ------------------------------------------------------------------------------

def case(says, named, word="center", holder="holder", frame=0, quality="full"):
    return {"says": says, "holder": holder, "named": named, "fit": word, "frame": frame,
            "quality": quality}


CASES = {
    "FX-LMAP-001": case("`card`, a plain drawing 5 by 3, in `holder`, a solid the size of the "
                        "composition: centred at (1, 1), transparent round it.", "card"),
    "FX-LMAP-002": case("`card_placed`, the same drawing moved, scaled 200 by 50, turned 30 "
                        "degrees, at half opacity and Multiply: the same map as FX-LMAP-001, "
                        "because nothing after step 3 counts.", "card_placed"),
    "FX-LMAP-003": case("`card_hidden`, switched off and used only as `holder`'s matte: the same "
                        "map as FX-LMAP-001.", "card_hidden"),
    "FX-LMAP-004": case("`card_masked`, its mask keeping columns 0 to 2: the map is cut the same "
                        "way.", "card_masked"),
    "FX-LMAP-005": case("`card_bright`, with Exposure +1: the map is twice as bright.",
                        "card_bright"),
    "FX-LMAP-006": case("`card_off_fx`, the same Exposure switched off: FX-LMAP-001's map.",
                        "card_off_fx"),
    "FX-LMAP-007": case("`card_soft`, with Gaussian Blur sigma 0.5: blurred, and cut back to the "
                        "card's own 5 by 3, so none of the blur's spread shows outside it.",
                        "card_soft"),
    "FX-LMAP-008": case("`self` names itself: its drawing through its mask, and not its "
                        "Exposure, fitted to its own 5 by 3.", "self", holder="self"),
    "FX-LMAP-009": case("`flip` shows the card on frame 0 and its second drawing from frame 1: "
                        "the map follows the drawing exposed.", "flip", frame=1),
    "FX-LMAP-010": case("`flip_late` starts on frame 1: on frame 0 the map is empty, and nothing "
                        "is said.", "flip_late"),
    "FX-LMAP-011": case("`flip_late` on frame 1 shows its first drawing, by its own timing.",
                        "flip_late", frame=1),
    "FX-LMAP-012": case("`flip_ahead`, its source one frame ahead: on frame 0 it shows the second "
                        "drawing.", "flip_ahead"),
    "FX-LMAP-013": case("`solid`, a solid 4 by 2: centred at (2, 2).", "solid"),
    "FX-LMAP-014": case("`shape`, a box from (1, 1) to (5, 4): a shape layer's picture is the "
                        "composition's size, so the map is the box where it is drawn.", "shape"),
    "FX-LMAP-015": case("`nest`, a composition layer: the inner composition's frame, 8 by 4, "
                        "centred.", "nest"),
    "FX-LMAP-016": case("`nest_bright`, the same with Exposure +1: brighter.", "nest_bright"),
    "FX-LMAP-017": case("`nest` on frame 2, past the inner composition's two frames: empty, "
                        "nothing said.", "nest", frame=2),
    "FX-LMAP-018": case("`nest_gone` shows a composition not in the project: empty, and "
                        "`COMPOSITION_REFERENCE_MISSING`.", "nest_gone"),
    "FX-LMAP-019": case("`rig`, a null, with tile: empty, nothing said.", "rig", "tile"),
    "FX-LMAP-020": case("`adjust`, an adjustment layer: white through its mask over columns 0 "
                        "to 3, its Exposure not run.", "adjust"),
    "FX-LMAP-021": case("`gone_file`, a drawing whose file is missing: empty, and "
                        "`MEDIA_MISSING`.", "gone_file"),
    "FX-LMAP-022": case("Centre, `big` 10 by 8 into 8 by 6: dx = dy = -1, the middle shows.",
                        "big"),
    "FX-LMAP-023": case("Tile, `card` into 8 by 6: from (1, 1), repeating every 5 across and 3 "
                        "down.", "card", "tile"),
    "FX-LMAP-024": case("Tile, `big` into 8 by 6: a picture bigger than the holder tiles to "
                        "FX-LMAP-022's centre.", "big", "tile"),
    "FX-LMAP-025": case("Stretch, `card` into 8 by 6: bilinear, held at the card's edges.",
                        "card", "stretch"),
    "FX-LMAP-026": case("Stretch, `big` into 8 by 6.", "big", "stretch"),
    "FX-LMAP-027": case("Stretch, `card` into `holder_card`, a solid 5 by 3: the card exactly.",
                        "card", "stretch", holder="holder_card"),
    "FX-LMAP-028": case("Centre, `card` into `holder_small`, 3 by 2: dx = -1 and dy = "
                        "floor(-1/2) = -1.", "card", holder="holder_small"),
    "FX-LMAP-029": case("Tile, `card` into `holder_small`: ((x + 1) mod 5, (y + 1) mod 3).",
                        "card", "tile", holder="holder_small"),
    "FX-LMAP-030": case("Stretch, `card_masked` into 8 by 6: premultiplied, so the clear columns "
                        "fade the covering and never tint the colour.", "card_masked", "stretch"),
    "FX-LMAP-031": case("Draft, `stripes_fx`, a drawing with an effect, so its effects run at a "
                        "quarter (D-99) on 2 by 2: `card_draft` is taken down to 2 by 1, its mask "
                        "(0, 0)-(4, 3) becomes (0, 0)-(1, 0.75), then Exposure +1, then Gaussian "
                        "Blur sigma 2 / 4.", "card_draft", holder="stripes_fx", quality="draft"),
    "FX-LMAP-032": case("Draft, `holder`, a solid, whose effects run at full size: FX-LMAP-001's "
                        "map exactly.", "card", quality="draft"),
    "FX-LMAP-033": case("Draft, `nest` holding: its picture is 2 by 1 at Draft. `nest_bright`'s "
                        "inner frame is drawn at a quarter, then its Exposure.", "nest_bright",
                        holder="nest", quality="draft"),
    "FX-LMAP-034": case("Draft, `adjust` holding: its effects run on the 2 by 2 frame. `big` is "
                        "taken down to 3 by 2 and stretched.", "big", "stretch", holder="adjust",
                        quality="draft"),
    "FX-LMAP-040": case("A name that is no layer: no map, and `EFFECT_LAYER_MISSING`.",
                        "layer-deleted"),
    "FX-LMAP-041": case("`dot`, a layer of another composition: no map, and "
                        "`EFFECT_LAYER_MISSING`.", "dot"),
    "FX-LMAP-042": case("An empty name: no map, nothing said.", ""),
}


# --- printing -------------------------------------------------------------------------------

def fmt(v):
    return f"{v:.9g}"


def print_map(m):
    if m is None:
        print("No map.\n")
        return
    print(f"{m['w']} by {m['h']}.\n")
    print("| y | " + " | ".join(f"x = {x}" for x in range(m["w"])) + " |")
    print("| --- " * (m["w"] + 1) + "|")
    for y in range(m["h"]):
        print(f"| {y} | " + " | ".join(" ".join(fmt(v) for v in at(m, x, y))
                                       for x in range(m["w"])) + " |")
    print()


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, rows in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    (OUT / "layer_map.json").write_text(json.dumps(PROJECT, indent=2) + "\n", encoding="utf-8")

    expected = {"tolerance": TOLERANCE, "project": "layer_map.json", "composition": "comp-main",
                "cases": {}}
    for fx, c in CASES.items():
        m, said = layer_map(c["holder"], c["named"], c["fit"], c["frame"], c["quality"])
        expected["cases"][fx] = dict(c, map=None if m is None else
                                     {"width": m["w"], "height": m["h"], "pixels": m["px"]},
                                     diagnostics=said)
        named = f"`{c['named']}`" if c["named"] else "nothing"
        print(f"{fx}: {c['says']} Holder `{c['holder']}`, named {named}, fit "
              f"`{c['fit']}`, frame {c['frame']}, {c['quality'].capitalize()}. "
              f"Said: {', '.join(f'`{s}`' for s in said) or 'nothing'}.\n")
        print_map(m)
    (OUT / "expected_layer_map.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    checks(expected)
    draw_picture(expected)


def checks(e):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["map"] and v["map"]["pixels"] for fx, v in e["cases"].items()}
    said = {fx: v["diagnostics"] for fx, v in e["cases"].items()}
    card = decoded("card")["px"]
    one = c["FX-LMAP-001"]
    assert all(one[(y + 1) * W + x + 1] == card[y * 5 + x] for y in range(3) for x in range(5))
    assert sum(p[3] for p in one) == 15
    for fx in ("FX-LMAP-002", "FX-LMAP-003", "FX-LMAP-006", "FX-LMAP-032"):
        assert c[fx] == one, fx
    assert all(p[3] == 0 for y in range(6) for x in range(4, 8) for p in [c["FX-LMAP-004"][y * W + x]])
    assert c["FX-LMAP-005"] == [[q * 2 for q in p[:3]] + p[3:] for p in one]
    soft = c["FX-LMAP-007"]
    assert all(soft[y * W + x][3] == 0 for y in range(6) for x in range(8)
               if not (1 <= x < 6 and 1 <= y < 4)), "the blur is cut to the card"
    assert soft[2 * W + 3][3] < 1 and soft != one
    assert c["FX-LMAP-008"] == [p if i % 5 < 3 else CLEAR for i, p in enumerate(card)]
    assert c["FX-LMAP-009"][W + 1] == decoded("card_b")["px"][0]
    assert all(p == CLEAR for p in c["FX-LMAP-010"]) and c["FX-LMAP-011"] == one
    assert c["FX-LMAP-012"] == c["FX-LMAP-009"]
    assert sum(p[3] for p in c["FX-LMAP-013"]) == 8 and c["FX-LMAP-013"][2 * W + 2][3] == 1
    assert sum(p[3] for p in c["FX-LMAP-014"]) == 12 and c["FX-LMAP-014"][W + 1][3] == 1
    assert sum(p[3] for p in c["FX-LMAP-015"]) == 8 and c["FX-LMAP-015"][2 * W + 2] == DOT + [1.0]
    assert c["FX-LMAP-016"][2 * W + 2] == [0.5, 1.0, 2.0, 1.0]
    for fx in ("FX-LMAP-017", "FX-LMAP-018", "FX-LMAP-019", "FX-LMAP-021"):
        assert all(p == CLEAR for p in c[fx]), fx
    assert said["FX-LMAP-018"] == ["COMPOSITION_REFERENCE_MISSING"]
    assert said["FX-LMAP-021"] == ["MEDIA_MISSING"]
    assert c["FX-LMAP-020"] == [[1.0] * 4 if x < 4 else CLEAR for y in range(6) for x in range(8)]
    assert c["FX-LMAP-024"] == c["FX-LMAP-022"]
    assert c["FX-LMAP-027"] == card
    assert c["FX-LMAP-028"] == [card[5 + 1], card[5 + 2], card[5 + 3],
                                card[10 + 1], card[10 + 2], card[10 + 3]]
    assert c["FX-LMAP-029"] == c["FX-LMAP-028"]
    assert all(p[3] == 1 for p in c["FX-LMAP-025"]), "stretch covers the holder"
    edge = [p for p in c["FX-LMAP-030"] if 0 < p[3] < 1]
    assert edge and all(abs(p[2] / p[3] - 1) < 1e-12 for p in edge), "blue stays full, straight"
    draft = e["cases"]["FX-LMAP-031"]["map"]
    assert (draft["width"], draft["height"]) == (2, 2)
    assert (e["cases"]["FX-LMAP-033"]["map"]["width"], e["cases"]["FX-LMAP-033"]["map"]["height"]) == (2, 1)
    assert [p[3] for p in c["FX-LMAP-033"]] == [0.5, 0.5]
    assert (e["cases"]["FX-LMAP-034"]["map"]["width"], e["cases"]["FX-LMAP-034"]["map"]["height"]) == (2, 2)
    for fx in ("FX-LMAP-040", "FX-LMAP-041", "FX-LMAP-042"):
        assert c[fx] is None, fx
    assert said["FX-LMAP-040"] == said["FX-LMAP-041"] == ["EFFECT_LAYER_MISSING"]
    assert said["FX-LMAP-042"] == []
    quiet = {"FX-LMAP-018", "FX-LMAP-021", "FX-LMAP-040", "FX-LMAP-041"}
    assert all(said[fx] == [] for fx in said if fx not in quiet)


# --- the picture in the proposal ------------------------------------------------------------

def draw_picture(e):
    """The card and the three fits of it into 8 by 6, the card centred in 3 by 2, and a drawing
    bigger than the holder centred: each pixel a 24-pixel square, a grey check where it is clear."""
    import numpy as np
    from PIL import Image, ImageDraw

    s, gap = 24, 16
    maps = [("the card, 5 by 3", decoded("card")),
            ("centre in 8 by 6", e["cases"]["FX-LMAP-001"]["map"]),
            ("tile in 8 by 6", e["cases"]["FX-LMAP-023"]["map"]),
            ("stretch in 8 by 6", e["cases"]["FX-LMAP-025"]["map"]),
            ("centre in 3 by 2", e["cases"]["FX-LMAP-028"]["map"]),
            ("centre, 10 by 8 in 8 by 6", e["cases"]["FX-LMAP-022"]["map"])]
    width = sum(8 * s + gap for _ in maps) + gap
    img = Image.new("RGB", (width, 6 * s + 2 * gap + 20), (32, 32, 36))
    draw = ImageDraw.Draw(img)
    x0 = gap
    for label, m in maps:
        w = m["w"] if "w" in m else m["width"]
        h = m["h"] if "h" in m else m["height"]
        px = m["px"] if "px" in m else m["pixels"]
        a = np.zeros((h * s, w * s, 3))
        for y in range(h):
            for x in range(w):
                r, g, b, al = px[y * w + x]
                check = 0.6 if ((x + y) % 2 == 0) else 0.45
                rgb = [min(1.0, c + check * (1 - al)) for c in (r, g, b)]
                srgb = [12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055 for c in rgb]
                a[y * s:(y + 1) * s, x * s:(x + 1) * s] = srgb
        img.paste(Image.fromarray((a * 255 + 0.5).astype("uint8")), (x0, gap))
        draw.text((x0, gap + 6 * s + 4), label, fill=(230, 230, 230))
        x0 += 8 * s + gap
    PICTURE.parent.mkdir(parents=True, exist_ok=True)
    img.save(PICTURE)


if __name__ == "__main__":
    main()
