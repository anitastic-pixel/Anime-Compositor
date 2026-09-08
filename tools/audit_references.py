"""Check the prose against what is on disk.

Two things drift silently in this repository and neither is caught by the build.

A document names a file - an artifact, a fixture, a source file - and the file is later
renamed or never existed. Nothing fails; the reader follows the name and finds nothing.

A document quotes another artifact's score - "`verification/B-09_save_table.md`, 15 of 15
checks" - and that table later grows a row. Nothing fails; the artifact says 17 and the
planning document goes on saying 15, and the owner has two numbers for one thing.

Both are read here. Run it from the repository root:

    python tools/audit_references.py

It prints one line per inconsistency and exits non-zero, or prints "clean" and exits zero.
It is a reading aid rather than a gate: it cannot know that a name in a sentence is a name
rather than a path, so the names that are deliberately not files - the drawing the reference
shot does not have, an exported frame written to an ignored folder, an example filename in a
walkthrough - are listed in ALLOWED with the reason each one is there.
"""

import re
import sys
from pathlib import Path

ROOT = Path('.')

# Build output, diagnostic output, working files, and the quarantined G0 spikes. Nothing in
# any of these is prose the owner reads.
SKIP = {'.git', 'target', 'node_modules', '.claude', 'spike-output', 'trace', 'scratchpad',
        'spikes'}

# A path in backticks, which is how every document here writes one.
PATH = re.compile(r'`([A-Za-z0-9_./-]+\.(?:md|png|json|rs|py|ps1|toml|html|yml|yaml))`')

# A named artifact followed closely by a score claimed for it.
QUOTED_SCORE = re.compile(r'`(verification/[A-Za-z0-9_.-]+\.md)`[^.]{0,120}?(\d+) of (\d+) checks')

# The score an artifact states for itself, which is the first one in the file.
OWN_SCORE = re.compile(r'(\d+) of (\d+) checks')

# Names that are deliberately not files on disk, with why.
ALLOWED = {
    'layer3/layer3_007.png': 'the drawing the reference shot deliberately does not have',
    'cel_007.png': 'the same gap, under the fixture catalogue name',
    'layer3_006.png': 'named as the drawing either side of that gap',
    'shot_0012.png': 'an exported frame; exports are written to an ignored folder',
    'shot_-0012.png': 'the same, showing what a negative frame number would be named',
    'shot_0016.png': 'the same',
    'neg_0011.png': 'the same',
    'neg_-0012.png': 'the same',
    'layer00_2_decode.png': 'a trace filename; trace/ is diagnostic output and ignored',
    'layer01_zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz_decode.png': 'the same, truncated',
    'shot.autosave-0.json': 'an autosave snapshot, written beside a project at run time',
    'my_shot.json': 'an example filename in a walkthrough',
}

_names = None


def by_name(ref):
    """True when the basename exists somewhere in the tree.

    A document that names a file by its name rather than its path - `gen_dependencies.py`,
    `DEPENDENCIES.md` - is not making a broken reference, so the basename is enough.
    """
    global _names
    if _names is None:
        _names = {f.name for f in ROOT.rglob('*')
                  if not any(part in SKIP for part in f.parts)}
    return Path(ref).name in _names


def documents():
    for p in sorted(ROOT.rglob('*.md')):
        if not any(part in SKIP for part in p.parts):
            yield p


def main():
    found = []
    for p in documents():
        text = p.read_text(encoding='utf-8', errors='replace')
        for ref in sorted(set(PATH.findall(text))):
            if ref in ALLOWED or Path(ref).name in ALLOWED:
                continue
            if (ROOT / ref).exists() or (p.parent / ref).exists() or by_name(ref):
                continue
            found.append(f'{p}: names {ref}, which does not exist')
        for m in QUOTED_SCORE.finditer(text):
            other = ROOT / m.group(1)
            if not other.exists():
                continue
            own = OWN_SCORE.search(other.read_text(encoding='utf-8', errors='replace'))
            if own and (m.group(2), m.group(3)) != (own.group(1), own.group(2)):
                found.append(f'{p}: says {m.group(1)} is {m.group(2)} of {m.group(3)}, '
                             f'the file itself says {own.group(1)} of {own.group(2)}')

    for line in found:
        print(line)
    print(f'{len(found)} inconsistencies' if found else 'clean')
    return 1 if found else 0


if __name__ == '__main__':
    sys.exit(main())
