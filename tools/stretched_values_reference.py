"""D-742, B-617: the boxes of a stretched layer read its keys where the picture does.

Writes `Fixtures/stretched_values/` and prints nothing but "checked" (or, with `--table`, document
25's table). Written before any code.

A layer stretched to `s` percent (D-216) plays its keys at its key time,
`in + (frame - in) * 100 / s`, and the picture of composition frame `frame` is drawn from the
keys there: its position, rotation, depth, effect settings and mask paths and numbers. The
inspector's boxes show the value under the playhead, and must show that same value, not the
value at `frame` itself, which belongs to another moment of the animation.

`host.json` is one composition holding a composition layer, `nest`, starting at frame 4 and
stretched to 200 percent, so key time 9 plays at frame 14. Its position, rotation and depth, one
mask's path and feather, and one Fractal Noise's Brightness and Offset are keyed at 4 and 24, every
key linear, so each value between is the straight line between the two keys. The nested
composition holds one solid and no keys.

`expected_stretched_values.json` lists, for each case, a frame, its key time, and the value of
each box under the name and in the form the window gives it: the five numbers as the file holds
them, an effect setting as a list of its numbers, the mask's
path as six numbers a point (point, in handle, out handle) under `masks`.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / 'Fixtures' / 'stretched_values'

IN, OUT_FRAME, STRETCH = 4, 44, 200
FRAMES = [2, 4, 11, 14, 24, 43]


def key(frame, value):
    return {'frame': frame, 'value': value, 'interp': 'linear'}


def keyed(a, b, base=None):
    return {'base': a if base is None else base, 'keyframes': [key(4, a), key(24, b)]}


def square(x, y, size):
    return {'points': [{'point': p, 'in': [-5, 0], 'out': [5, 0]}
                       for p in ([x, y], [x + size, y], [x + size, y + size], [x, y + size])]}


TRANSFORM = {
    'anchor': {'base': [50, 50], 'keyframes': []},
    'position': keyed([60, 60], [140, 100]),
    'scale': {'base': [100, 100], 'keyframes': []},
    'rotation': keyed(0, 90),
    'opacity': {'base': 1, 'keyframes': []},
}
DEPTH = keyed(0, -100)
FEATHER = keyed(0, 40)
PATH = keyed(square(10, 10, 40), square(30, 20, 60))
BRIGHTNESS = keyed(-80, 0)
OFFSET = keyed([0, 0], [100, -50])


def host():
    fx = {
        'enabled': True,
        'instance_id': 'fx-1',
        'type_id': 'core.fractal_noise',
        'parameters': {
            'blend': 'normal', 'brightness': BRIGHTNESS, 'complexity': 4, 'contrast': 100,
            'dark_color': '#000000', 'evolution': {'base': 0, 'keyframes': []},
            'fractal_type': 'basic', 'invert': 'off', 'light_color': '#ffffff',
            'offset': OFFSET, 'opacity': 100, 'seed': 0, 'size': 50, 'speed': 0,
        },
    }
    plain = {
        'anchor': {'base': [0, 0], 'keyframes': []},
        'position': {'base': [0, 0], 'keyframes': []},
        'scale': {'base': [100, 100], 'keyframes': []},
        'rotation': {'base': 0, 'keyframes': []},
        'opacity': {'base': 1, 'keyframes': []},
    }
    inner = {
        'id': 'comp-inner', 'name': 'inner', 'width': 100, 'height': 100,
        'pixel_aspect_ratio': 1, 'frame_rate': {'numerator': 24, 'denominator': 1},
        'start_frame': 0, 'duration_frames': 48, 'background_color': [0, 0, 0],
        'layer_order': ['fill'],
        'layers': [{
            'id': 'fill', 'kind': 'solid', 'name': 'fill', 'enabled': True, 'locked': False,
            'in_frame': 0, 'out_frame': 48, 'solid': {'color': [0.9, 0.5, 0.2], 'width': 100, 'height': 100},
            'transform': plain, 'matte': None, 'blend_mode': 'normal', 'effects': [], 'masks': [],
        }],
    }
    main = {
        'id': 'comp-main', 'name': 'main', 'width': 200, 'height': 160,
        'pixel_aspect_ratio': 1, 'frame_rate': {'numerator': 24, 'denominator': 1},
        'start_frame': 0, 'duration_frames': 48, 'background_color': [0, 0, 0],
        'layer_order': ['nest'],
        'layers': [{
            'id': 'nest', 'kind': 'composition', 'name': 'nest', 'enabled': True, 'locked': False,
            'in_frame': IN, 'out_frame': OUT_FRAME, 'composition_id': 'comp-inner',
            'source_offset_frames': 0, 'time_stretch': STRETCH,
            'transform': TRANSFORM, 'depth': DEPTH, 'matte': None, 'blend_mode': 'normal',
            'effects': [fx],
            'masks': [{'enabled': True, 'name': 'Mask 1', 'mode': 'add', 'inverted': False,
                       'opacity': 1, 'expansion_px': 0, 'feather_px': FEATHER, 'path': PATH}],
        }],
    }
    return {
        'schema_version': 0,
        'project_id': 'proj-fx-svals-host',
        'color_settings': {'working_space': 'linear-srgb', 'alpha_mode': 'premultiplied'},
        'assets': [],
        'compositions': [main, inner],
    }


def key_time(frame):
    return IN + (frame - IN) * 100 / STRETCH


def at(prop, t):
    """A linear property's value at key time `t`: the first key before it, the last after."""
    a, b = prop['keyframes']
    u = min(max((t - a['frame']) / (b['frame'] - a['frame']), 0), 1)

    def mix(p, q):
        if isinstance(p, list):
            return [mix(x, y) for x, y in zip(p, q)]
        if isinstance(p, dict):
            return {k: mix(p[k], q[k]) for k in p}
        return p + (q - p) * u
    return mix(a['value'], b['value'])


def six(path):
    return [p['point'] + p['in'] + p['out'] for p in path['points']]


def cases():
    out = []
    for f in FRAMES:
        t = key_time(f)
        out.append({
            'id': f'FX-SVAL-{len(out) + 1:03}',
            'frame': f,
            'key_time': t,
            'values': {
                'position': at(TRANSFORM['position'], t),
                'rotation': at(TRANSFORM['rotation'], t),
                'depth': at(DEPTH, t),
                'mask:0:feather': at(FEATHER, t),
                # The window sends every effect setting as a list, one number for Brightness.
                'fx:fx-1:brightness': [at(BRIGHTNESS, t)],
                'fx:fx-1:offset': at(OFFSET, t),
                'masks': [six(at(PATH, t))],
            },
        })
    return out


def main():
    OUT.mkdir(exist_ok=True)
    (OUT / 'host.json').write_text(json.dumps(host(), indent=1) + '\n', encoding='utf-8', newline='\n')
    got = cases()
    # Frame 14 plays key time 9, a quarter of the way from 4 to 24.
    assert got[3]['key_time'] == 9 and got[3]['values']['rotation'] == 22.5, got[3]
    # Before the in point the first key holds; at the out point the line is nearly done.
    assert got[0]['values']['rotation'] == 0 and got[5]['key_time'] == 23.5
    if '--table' in sys.argv:
        print('| Case | Frame | Key time | Position | Rotation | Depth | Feather | Brightness | Offset |')
        print('|---|---|---|---|---|---|---|---|---|')
        for c in got:
            v = c['values']
            print(f"| {c['id']} | {c['frame']} | {c['key_time']:g} | "
                  f"{', '.join(f'{x:g}' for x in v['position'])} | {v['rotation']:g} | {v['depth']:g} | "
                  f"{v['mask:0:feather']:g} | {v['fx:fx-1:brightness'][0]:g} | "
                  f"{', '.join(f'{x:g}' for x in v['fx:fx-1:offset'])} |")
        return
    text = json.dumps({'decision': 'D-742', 'host': 'stretched_values/host.json',
                       'composition': 'comp-main', 'layer': 'nest', 'cases': got}, indent=1) + '\n'
    (OUT / 'expected_stretched_values.json').write_text(text, encoding='utf-8', newline='\n')
    print('checked')


if __name__ == '__main__':
    main()
