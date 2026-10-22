"""D-741, B-616: chosen keys of a mask's, a shape's and a group's numbers moved, deleted and
reversed together, as a property's are.

Writes `Fixtures/key_requests/` and prints nothing but "checked" (or, with `--table`, document
25's table). Written before any code.

The timeline sends the keys a person has chosen, each named `layer|property|frame`, to three
requests: `keyframe.move` (dragged, or the arrow keys) moves each by `by` frames, the keys
furthest along going first; `keyframe.add_remove` with keys named (Delete) takes each out; and
`keyframe.reverse` puts each chosen key at `lo + hi - frame`, `lo` and `hi` the first and last
chosen frame, giving the key that lands first the ease the last one had and each other the ease
of the one before it, turned (D-314). A key keeps its value wherever it goes. Every property name
the single-key requests answer to is answered here: `mask:<n>` (a mask's path), `mask:<n>:feather`
and its siblings, `shape:<n>` (a shape's path), `shape:<n>:trim_end` and its siblings,
`group:<n>:rotation`, `group:<n>:repeater_copies` and `group:<n>:op<k>_radius` and theirs. The
last key of a number taken out leaves the number at the value the first chosen key of it held, as
a path's does (B-24f). A key moved onto one that is not chosen is refused, and then nothing moves.

Each case names a project, a layer, the request and the keys chosen, and lists every key the layer
holds afterwards as `[where, frame, value, interp, ease]`, in order of `where` and frame, `where`
the record's path from the layer; `bases` lists the numbers left with no keys and their value.
A refused case lists the layer's keys as they were. The projects are other fixtures, read and never
written here, and `host.json`, written here: `projects/keyed_settings_project.json` with its mask
copied as a second mask, so one layer holds two keyed paths.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FIX = ROOT / 'Fixtures'
OUT = FIX / 'key_requests'

TRIM = ('shape_trims/fx_shp_083.json', 'shapes')
MASKS = ('key_requests/host.json', 'layer-1')
GROUPS = ('shape_groups/host.json', 'shapes')
REPEATERS = ('shape_repeaters/host.json', 'shapes')
OPS = ('shape_operations/host.json', 'shapes')
SIZE = ('parametric_shapes/host.json', 'shapes')

# (project and layer, request, by, chosen keys as property|frame)
CASES = [
    (TRIM, 'move', 3, ['shape:0:trim_end|0', 'shape:0:trim_end|4']),
    (TRIM, 'delete', 0, ['shape:0:trim_end|0', 'shape:0:trim_end|4']),
    (TRIM, 'reverse', 0, ['shape:0:trim_end|0', 'shape:0:trim_end|4']),
    (TRIM, 'move', 4, ['shape:0:trim_end|0']),
    (MASKS, 'move', 2, ['mask:0|0', 'mask:0|34', 'mask:1|0', 'mask:1|34',
                        'mask:0:feather|0', 'mask:0:feather|34']),
    (MASKS, 'delete', 0, ['mask:0:feather|34', 'mask:1|0']),
    (MASKS, 'reverse', 0, ['mask:0|0', 'mask:0|34', 'mask:0:expansion|0', 'mask:0:expansion|34']),
    (GROUPS, 'move', 5, ['group:0:rotation|0', 'group:0:rotation|24',
                         'group:2:position|0', 'group:2:position|24']),
    (GROUPS, 'delete', 0, ['group:0:rotation|24', 'group:2:position|0']),
    (GROUPS, 'reverse', 0, ['group:0:rotation|0', 'group:0:rotation|24',
                            'group:2:position|0', 'group:2:position|24']),
    (REPEATERS, 'move', 4, ['group:0:repeater_copies|0', 'group:0:repeater_copies|24',
                            'group:1:repeater_offset|0', 'group:1:repeater_offset|24']),
    (REPEATERS, 'reverse', 0, ['group:0:repeater_copies|0', 'group:0:repeater_copies|24',
                               'group:1:repeater_offset|0', 'group:1:repeater_offset|24']),
    (OPS, 'move', 6, ['group:0:op0_radius|0', 'group:0:op0_radius|24', 'group:1:op0_amount|24',
                      'group:7:op0_amount|0', 'group:7:op0_amount|24']),
    (OPS, 'delete', 0, ['group:1:op0_amount|0', 'group:1:op0_amount|24']),
    (SIZE, 'move', 2, ['shape:1:path_size|0', 'shape:1:path_size|24']),
    (SIZE, 'reverse', 0, ['shape:1:path_size|0', 'shape:1:path_size|24']),
]

MASK_FILE = {'feather': 'feather_px', 'expansion': 'expansion_px', 'opacity': 'opacity'}


def where(prop):
    """The record's path, from the layer, of the keyed number or path a property name names."""
    kind, n, *rest = prop.split(':')
    name = rest[0] if rest else None
    if kind == 'mask':
        return f'/masks/{n}/' + (MASK_FILE[name] if name else 'path')
    if kind == 'shape':
        if name is None:
            return f'/shapes/{n}/path'
        if name.startswith('trim_'):
            return f'/shapes/{n}/trim/' + name[5:]
        if name.startswith('path_'):
            return f'/shapes/{n}/parametric/' + name[5:]
        raise ValueError(prop)
    if name.startswith('repeater_'):
        return f'/groups/{n}/repeater/' + name[9:]
    if name.startswith('op'):
        k, num = name[2:].split('_', 1)
        return f'/groups/{n}/operations/{k}/{num}'
    return f'/groups/{n}/transform/{name}'


def at(record, path):
    for part in path.strip('/').split('/'):
        record = record[int(part)] if isinstance(record, list) else record[part]
    return record


def turned(key):
    """D-314: an ease played backwards, the curve turned half a turn about its middle."""
    if key.get('interp') == 'ease' and 'ease' in key:
        x1, y1, x2, y2 = key['ease']
        return {'interp': 'ease', 'ease': [1 - x2, 1 - y2, 1 - x1, 1 - y1]}
    return {k: key[k] for k in ('interp', 'ease') if k in key}


def ease_of(key):
    return {k: key[k] for k in ('interp', 'ease') if k in key}


def apply(layer, request, by, chosen):
    """The layer after the request, or None when it is refused."""
    layer = copy.deepcopy(layer)
    named = {}
    for c in chosen:
        prop, frame = c.rsplit('|', 1)
        named.setdefault(where(prop), []).append(int(frame))
    every = [int(c.rsplit('|', 1)[1]) for c in chosen]
    lo, hi = min(every), max(every)
    bases = []
    for place, frames in named.items():
        number = at(layer, place)
        keys = number['keyframes']
        assert all(any(k['frame'] == f for k in keys) for f in frames), (place, frames)
        mine = sorted((k for k in keys if k['frame'] in frames), key=lambda k: k['frame'])
        if request == 'delete':
            number['keyframes'] = [k for k in keys if k['frame'] not in frames]
            if not number['keyframes']:
                number['base'] = mine[0]['value']
                bases.append([place, mine[0]['value']])
            continue
        if request == 'move':
            new = {id(k): (k['frame'] + by, ease_of(k)) for k in mine}
        else:
            new = {id(k): (lo + hi - k['frame'],
                           ease_of(mine[-1]) if j == 0 else turned(mine[j - 1]))
                   for j, k in enumerate(mine)}
        landed = []
        for k in keys:
            if id(k) in new:
                frame, ease = new[id(k)]
                k.pop('ease', None)
                k.update(ease, frame=frame)
            landed.append(k['frame'])
        if len(set(landed)) != len(landed):
            return None, []
        keys.sort(key=lambda k: k['frame'])
    return layer, bases


def keys(layer):
    """Every key the layer holds, as [where, frame, value, interp, ease], in order of where."""
    out = []

    def walk(v, path):
        if isinstance(v, dict):
            for k, x in v.items():
                if k == 'keyframes' and isinstance(x, list):
                    out.extend([path, key['frame'], key.get('value'), key.get('interp'), key.get('ease')]
                               for key in x)
                else:
                    walk(x, f'{path}/{k}')
        elif isinstance(v, list):
            for i, x in enumerate(v):
                walk(x, f'{path}/{i}')
    walk(layer, '')
    return sorted(out, key=lambda k: (k[0], k[1]))


def host():
    doc = json.loads((FIX / 'projects/keyed_settings_project.json').read_text(encoding='utf-8'))
    doc['project_id'] = 'proj-fx-kreq-host'
    layer = doc['compositions'][0]['layers'][0]
    second = copy.deepcopy(layer['masks'][0])
    second['name'] = 'Mask 2'
    second['mode'] = 'subtract'
    for k in second['path']['keyframes']:
        for p in k['value']['points']:
            p['point'] = [p['point'][0] + 40, p['point'][1] + 300]
    layer['masks'].append(second)
    return doc


def cases():
    out = []
    for (project, lid), request, by, chosen in CASES:
        doc = json.loads((FIX / project).read_text(encoding='utf-8'))
        comp = next(c for c in doc['compositions'] if any(l['id'] == lid for l in c['layers']))
        layer = next(l for l in comp['layers'] if l['id'] == lid)
        after, bases = apply(layer, request, by, chosen)
        out.append({
            'id': f'FX-KREQ-{len(out) + 1:03}',
            'project': project,
            'composition': comp['id'],
            'layer': lid,
            'request': request,
            'by': by,
            'chosen': chosen,
            'refused': after is None,
            'keys': keys(layer if after is None else after),
            'bases': bases,
        })
    return out


def main():
    OUT.mkdir(exist_ok=True)
    (OUT / 'host.json').write_text(json.dumps(host(), indent=1) + '\n', encoding='utf-8', newline='\n')
    got = cases()
    assert [c['refused'] for c in got].count(True) == 1, 'one refusal, FX-KREQ-004'
    assert got[3]['refused']
    # The trim's two keys reversed: frame 0 holds what frame 4 did.
    trim = [k for k in got[2]['keys'] if k[0] == '/shapes/0/trim/end']
    assert [(k[1], k[2]) for k in trim] == [(0, 100), (4, 0)], trim
    if '--table' in sys.argv:
        print('| Case | Project | Layer | Request | Keys chosen | Keys afterwards |')
        print('|---|---|---|---|---|---|')
        for c in got:
            what = {'move': f"move by {c['by']}", 'delete': 'delete', 'reverse': 'reverse'}[c['request']]
            places = sorted({k[0] for k in c['keys']})
            print(f"| {c['id']} | `{c['project']}` | {c['layer']} | {what} | "
                  f"{', '.join(c['chosen'])} | "
                  + ('refused, nothing moves' if c['refused'] else
                     f"{len(c['keys'])} keys in {len(places)} place" + "s" * (len(places) != 1)) + ' |')
        return
    text = json.dumps({'decision': 'D-741', 'cases': got}, indent=1) + '\n'
    (OUT / 'expected_key_requests.json').write_text(text, encoding='utf-8', newline='\n')
    print('checked')


if __name__ == '__main__':
    main()
