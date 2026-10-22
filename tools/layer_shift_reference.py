"""D-740, B-615: every key travels with its layer when the layer is moved.

Writes `Fixtures/layer_shift/expected_layer_shift.json` and prints nothing but "checked" (or,
with `--table`, document 25's table). Written before any code.

The owner's decision of 2026-09-13: moving a layer along the timeline (`layer.shift`, a dragged
bar, [ and ]) moves every key it holds by the same number of frames, as in After Effects. A key is
any entry with a `frame` in an array named `keyframes` or `keys` anywhere in the layer's record:
the transform's, depth's, Time Remap's, an effect setting's, a mask's path, feather and expansion,
a shape's path, styles, gradient points, parametric settings and trim, a shape group's transform,
repeater and operations, and a sound layer's level and pan. Nothing else in the record moves but
`in_frame` and `out_frame`; the exposures and the timesheet are in source frames and stay.

Each case names a project in `Fixtures/`, a layer in it and the frame its in point is moved
to, and lists every key of that layer as `[where, frame]` after the move, in order of `where`
(by character code) and within one `where` in the file's order: `where` is the record's path from the layer, array places
written `*`. The cases cover each kind of key. The projects are other fixtures, read and never
written here, and `host.json`, written here for the kinds no other fixture keys: a shape layer of
`shape_gradients/fx_shp_051.json` given a keyed depth, keyed anchor, scale, rotation and opacity,
and a stroke whose gradient's start and end are keyed.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIX = ROOT / 'Fixtures'
OUT = FIX / 'layer_shift'

# (project, how far its keyed layers move). Every layer holding a key is a case.
PROJECTS = [
    ('sound_layer/fx_slay_001.json', 10),
    ('sound_layer/fx_slay_005.json', 10),
    ('fill/fx_fill_018.json', 10),
    ('projects/keyed_settings_project.json', 10),
    ('shapes/fx_shp_017.json', 10),
    ('shape_styles/fx_shp_115.json', 10),
    ('shape_styles/fx_shp_116.json', 10),
    ('shape_styles/fx_shp_117.json', 10),
    ('shape_gradients/fx_shp_051.json', 10),
    ('parametric_shapes/host.json', 10),
    ('shape_trims/fx_shp_083.json', 10),
    ('shape_groups/host.json', 10),
    ('shape_repeaters/host.json', 10),
    ('shape_operations/host.json', -7),
    ('time_remap/fx_tremap_010.json', 10),
    ('keykind/fx_sep_001.json', 10),
    ('shape_merge/host.json', 10),
    ('layer_shift/host.json', 5),
]


def host():
    doc = json.loads((FIX / 'shape_gradients/fx_shp_051.json').read_text(encoding='utf-8'))
    doc['project_id'] = 'proj-fx-lshift-host'
    layer = doc['compositions'][0]['layers'][0]

    def keyed(a, b):
        return {'base': a, 'keyframes': [{'frame': 0, 'value': a, 'interp': 'linear'},
                                         {'frame': 4, 'value': b, 'interp': 'linear'}]}
    layer['depth'] = keyed(0, 50)
    t = layer['transform']
    t['anchor'] = keyed([3.0, 1.0], [2.0, 1.0])
    t['scale'] = keyed([100, 100], [50, 50])
    t['rotation'] = keyed(0, 90)
    t['opacity'] = keyed(1, 0.5)
    fill = layer['shapes'][0]['fill']
    layer['shapes'][0]['stroke'] = {
        'color': [0.1, 0.1, 0.1], 'opacity': 1.0, 'width_px': 1.0,
        'gradient': {'type': 'linear', 'start': keyed([0, 0], [6, 0]), 'end': keyed([6, 2], [0, 2]),
                     'stops': fill['gradient']['stops']}}
    return doc


def keys(v, where=''):
    """Every key under `v`, in file order, as [where, frame]."""
    out = []
    if isinstance(v, dict):
        for k, x in v.items():
            if k in ('keyframes', 'keys') and isinstance(x, list):
                out += [[f'{where}/{k}', key['frame']] for key in x
                        if isinstance(key, dict) and isinstance(key.get('frame'), int)]
            out += keys(x, f'{where}/{k}')
    elif isinstance(v, list):
        for x in v:
            out += keys(x, f'{where}/*')
    return out


def cases():
    out = []
    for project, by in PROJECTS:
        doc = json.loads((FIX / project).read_text(encoding='utf-8'))
        for comp in doc['compositions']:
            for layer in comp['layers']:
                before = keys(layer)
                if not before:
                    continue
                out.append({
                    'id': f'FX-LSHIFT-{len(out) + 1:03}',
                    'project': project,
                    'composition': comp['id'],
                    'layer': layer['id'],
                    'kind': layer['kind'],
                    'in_frame': layer.get('in_frame', 0) + by,
                    'keys': sorted([[w, f + by] for w, f in before], key=lambda k: k[0]),
                })
    return out


def kinds(case):
    """The distinct places keys sit, for the table."""
    return sorted({w.rsplit('/', 1)[0].replace('/*', '').lstrip('/') for w, _ in case['keys']})


def main():
    OUT.mkdir(exist_ok=True)
    (OUT / 'host.json').write_text(json.dumps(host(), indent=1) + '\n', encoding='utf-8', newline='\n')
    got = cases()
    assert got, 'no keyed layers'
    every = {w for c in got for w, _ in c['keys']}
    for word in ('/depth/', '/anchor/', '/position/x/', '/stroke/gradient/', '/parametric/position/',
                 '/gain_db/', '/pan/', '/masks/', '/shapes/', '/groups/', '/effects/', '/transform/',
                 '/time_remap/', '/trim/', '/gradient/', '/parametric/', '/repeater/', '/operations/'):
        assert any(word in w for w in every), word
    if '--table' in sys.argv:
        print('| Case | Project | Layer | Moved to | Keys | Where they sit |')
        print('|---|---|---|---|---|---|')
        for c in got:
            print(f"| {c['id']} | `{c['project']}` | {c['layer']} | {c['in_frame']} | {len(c['keys'])} | "
                  f"{', '.join(kinds(c))} |")
        return
    text = json.dumps({'decision': 'D-740', 'cases': got}, indent=1) + '\n'
    (OUT / 'expected_layer_shift.json').write_text(text, encoding='utf-8', newline='\n')
    print('checked')


if __name__ == '__main__':
    main()
