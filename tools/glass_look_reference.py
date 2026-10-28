"""CC Glass's AE Lights and rim look (D-934), worked a second way.

In the Liquid Glass tutorial (n0g4HXxx03s, 2:00 to 3:20 and 14:40) After Effects' CC Glass, with
its own Effect Light at its starting settings, gives the pill a dark rim all round and leaves its
middle about as it was, with a thin highlight on the side facing the light; ours (D-561) lights
the whole surface, so the middle comes out brighter. The tutorial then switches Light Type to AE
Lights with one white ambient light, and the picture is the bend alone. Lane w12c matched that
with Ambient 100, Diffuse 0, Specular 0. Neither rule is published, so these are ours.

`tools/glass_reference.py` is the rule this adds to; everything there holds. Two settings:

1. `light_type` gains `ae_lights`: After Effects' light layers, which this program does not have
   (D-355). The bent picture P is the output, unlit, as one white ambient light at 100 per cent
   shows it; the shading and the light's own settings are kept in the file and not read. At
   Displacement 0, or Height 0, the output is the input.
2. `light_look`, `lit` or `rim`, read for `distant` and `point` only. `lit` is D-561's, and what a
   file without the word reads. `rim`: the flat surface keeps its colour and every slope darkens,
   whichever way it faces: with N, nl and the highlight as D-561's, each channel
   out = C (ambient + diffuse I Lc Nz) / (ambient + diffuse I Lc) + specular I (Lc + (C - Lc)
   metal) highlight, where the share is 0 when its bottom is 0, times P's covering, with that
   covering. Nz is 1 where the bump is flat and falls toward 0 as it steepens, so the rim goes
   down to ambient / (ambient + diffuse I Lc) of its colour; the highlight still faces the light.
   A CC Glass added from the Effect menu starts on `rim`; the word is written only when it is
   `rim`.

**This file never runs the build's code path.** It works each pixel in double precision from the
drawings' 8-bit values, reusing `glass_reference.py`'s cases, drawings and bump.

Every case is a project in `Fixtures/glass/` as `glass_reference.py`'s are. The expected pixels
are in `Fixtures/glass/expected_glass_look.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glass_look_reference.py
"""

import json
import sys
from math import sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import glass_reference as G  # noqa: E402
from glass_reference import W, H, L, decoded, case, plain  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402

OUT = G.OUT


def glass(o, m, s, look):
    """D-561's glass with D-934's two settings."""
    if s["light_type"] == "ae_lights":
        # The bend alone: classic's own rule with no light.
        return G.glass(o, m, dict(s, light_type="classic", light_intensity=0))
    if look != "rim" or s["light_type"] == "classic":
        return G.glass(o, m, s)
    w, h = o["w"], o["h"]
    f = [G.bump_phase(s["property"], p) for p in m["px"]]
    r, one = G.kernel(s["softness"] / 2)

    def bv(x, y):
        if r == 0:
            return f[y * w + x] if 0 <= x < w and 0 <= y < h else 0.0
        return sum(f[j * w + i] * one[i - x + r] * one[j - y + r]
                   for j in range(max(0, y - r), min(h, y + r + 1))
                   for i in range(max(0, x - r), min(w, x + r + 1)))

    k = s["height"] / 100 * 1.25 * max(s["softness"], 1.0)
    lc = G.hex_linear(s["light_color"])
    inten = s["light_intensity"] / 100
    ux, uy = G.along(s["light_angle"])
    amb, dif, spc, metal = (s[n] / 100 for n in ("ambient", "diffuse", "specular", "metal"))
    out = []
    for y in range(h):
        for x in range(w):
            nx, ny = (bv(x + 1, y) - bv(x - 1, y)) / 2 * k, (bv(x, y + 1) - bv(x, y - 1)) / 2 * k
            d = s["displacement"]
            p = L.bilinear(o, x + 0.5 + d * nx, y + 0.5 + d * ny) if d != 0 else list(L.at(o, x, y))
            a = p[3]
            if a <= 0:
                out.append([0.0] * 4)
                continue
            col = [p[i] / a for i in range(3)]
            nn = [-nx, -ny, 1.0]
            ln = sqrt(sum(v * v for v in nn))
            nn = [v / ln for v in nn]
            if s["light_type"] == "point":
                lx, ly = s["light_position"][0] / 100 * w, s["light_position"][1] / 100 * h
                lv = [lx - (x + 0.5), ly - (y + 0.5), s["light_height"]]
            else:
                lv = [100 * ux, 100 * uy, s["light_height"]]
            ll = sqrt(sum(v * v for v in lv))
            lv = [v / ll for v in lv] if ll > 0 else [0.0] * 3
            nl = sum(nn[i] * lv[i] for i in range(3))
            rz = 2 * nl * nn[2] - lv[2]
            shine = rz ** (1 / s["roughness"]) if nl > 0 and rz > 0 else 0.0
            lit = []
            for i in range(3):
                bottom = amb + dif * inten * lc[i]
                share = (amb + dif * inten * lc[i] * nn[2]) / bottom if bottom > 0 else 0.0
                lit.append(col[i] * share
                           + spc * inten * (lc[i] + (col[i] - lc[i]) * metal) * shine)
            out.append([v * a for v in lit] + [a])
    return L.pic(w, h, out)


def look_case(look=None, **kw):
    c = case(**kw)
    c["look"] = look
    return c


def render(c, frame):
    s = G.settings(c, frame)
    out = glass(decoded(c["holder"]), G.the_map(c), s, c["look"] or "lit")
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


STILL = G.STILL
CASES = {
    "FX-GLASS-028": ("As a CC Glass is now added: Edge Look Rim with the rest as FX-GLASS-001: the "
                     "two blocks bent outward, their middles their own colour, their soft edges "
                     "darker all round, a highlight toward the top left.",
                     look_case("rim"), (0,)),
    "FX-GLASS-029": ("Rim, Softness 4, Displacement 0: the light alone. The blocks keep about "
                     "their colour, darker than D-561's light makes them, a little darker toward "
                     "every edge, the bottom and right as well as the top and left, with a "
                     "faint highlight at the top left.", look_case("rim", **STILL), (0,)),
    "FX-GLASS-030": ("Rim with a point light at 25, 30 per cent, 10 pixels up: the darkening as "
                     "FX-GLASS-029, the highlight moved toward the light.",
                     look_case("rim", light_type="point", light_position=(25, 30),
                               light_height=10, **STILL), (0,)),
    "FX-GLASS-031": ("Rim at Height 0: a flat surface, so every pixel keeps its colour, plus the "
                     "faintest highlight in its own colour.", look_case("rim", height=0), (0,)),
    "FX-GLASS-032": ("Rim with Light Intensity 0, Displacement 20: no darkening and no highlight, "
                     "the blocks bent and their colours untouched.",
                     look_case("rim", light_intensity=0, displacement=20), (0,)),
    "FX-GLASS-033": ("Rim with Ambient 0, Diffuse 100, Specular 0, Softness 4, Displacement 0: "
                     "each pixel its own colour times Nz, never brighter, darkest where the "
                     "surface is steepest.",
                     look_case("rim", ambient=0, diffuse=100,
                               specular=0, **STILL), (0,)),
    "FX-GLASS-034": ("Light Type AE Lights, Displacement 20: the blocks bent and not lit, as "
                     "After Effects shows them under one white ambient light.",
                     look_case(light_type="ae_lights", displacement=20), (0,)),
    "FX-GLASS-035": ("AE Lights with Displacement 0: nothing bent and nothing lit, the drawing "
                     "comes back untouched.", look_case(light_type="ae_lights", **STILL), (0,)),
    "FX-GLASS-036": ("Rim with the classic light, Displacement 20, Light Intensity 70: the look "
                     "is not read, so the same as FX-GLASS-003.",
                     look_case("rim", light_type="classic", displacement=20,
                               light_intensity=70), (0,)),
    "FX-GLASS-037": ("Edge Look written `lit`: D-561's look, the same as FX-GLASS-001.",
                     look_case("lit"), (0,)),
}

INVALID = {
    "FX-GLASS-038": ("An edge look written \"dark\".", look_case("dark")),
}


def effect(c):
    p = {k: setting_json(c[k]) for k in G.NAMES}
    if c["look"] is not None:
        p["light_look"] = c["look"]
    return {"instance_id": "fx-1", "type_id": "core.glass", "enabled": True, "parameters": p}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    doc = G.project_json(fx, c)
    doc["compositions"][0]["layers"][0]["effects"] = [effect(c)]
    (OUT / name).write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": G.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_glass_look.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    old = lambda **kw: G.render(case(**kw), 0)  # noqa: E731
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-9: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                    for p, q in zip(u, v))
    straight = lambda p: [v / p[3] for v in p[:3]] if p[3] > 0 else [0.0] * 3  # noqa: E731
    for fx, px in c.items():
        for p in px:
            assert -1e-12 <= p[3] <= 1 + 1e-12 and all(u >= -1e-12 for u in p[:3]), (fx, p)
    assert c["FX-GLASS-028"] not in (drawn, old())
    # FX-GLASS-029: no pixel far from its colour; darker toward every edge, bottom right too.
    nine = c["FX-GLASS-029"]
    red = straight(drawn[at(3, 4)])
    assert all(abs(u - v) < 0.05 for p, q in zip(nine, drawn) if q[3] > 0
               for u, v in zip(straight(p), straight(q)))
    lum = lambda p: sum(straight(p))  # noqa: E731
    lit = old(**STILL)
    assert lum(lit[at(3, 4)]) > lum(nine[at(3, 4)])  # D-561's middle is brighter
    edges = [(x, y) for y in range(H) for x in range(W)
             if drawn[at(x, y)][3] > 0 and straight(drawn[at(x, y)]) == red
             and lum(nine[at(x, y)]) < lum(drawn[at(x, y)]) - 1e-3]
    assert any(x < 3 for x, _ in edges) and any(x > 4 for x, _ in edges), edges
    assert all(p[3] == q[3] for p, q in zip(nine, drawn))
    assert c["FX-GLASS-030"] != nine
    assert near([straight(c["FX-GLASS-031"][at(3, 4)])], [red], 1e-5)
    assert c["FX-GLASS-031"] != old(height=0)
    assert near(c["FX-GLASS-032"], old(light_intensity=0, displacement=20, ambient=100))
    assert c["FX-GLASS-032"] != drawn
    three = c["FX-GLASS-033"]
    assert all(u <= v + 1e-12 for p, q in zip(three, drawn) for u, v in zip(p, q))
    assert lum(three[at(5, 7)]) < lum(three[at(3, 4)]) and lum(three[at(1, 2)]) < lum(three[at(3, 4)])
    assert near(c["FX-GLASS-034"], c["FX-GLASS-032"])
    assert near(c["FX-GLASS-035"], drawn, 1e-12)
    assert c["FX-GLASS-036"] == old(light_type="classic", displacement=20, light_intensity=70)
    assert c["FX-GLASS-037"] == old()
    print("checked")


if __name__ == "__main__":
    main()
