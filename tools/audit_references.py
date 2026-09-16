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
Since B-12d it is a CI gate. It cannot know that a name in a sentence is a name rather than a
path, so the names that are deliberately not files - the drawing the reference shot does not
have, an exported frame written to an ignored folder, an example filename in a walkthrough -
are listed in ALLOWED with the reason each one is there, and a new one of those is added to
ALLOWED rather than left to fail.
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

# A named artifact followed closely by a score claimed for it. The count may be written as
# "N of N checks", as a bare total "N checks", or as a word - all three are in the pack, and a
# pattern that reads only the first let document 18 go on claiming eighteen checks for a table
# that had grown to thirty-one.
WORDS = {'one': 1, 'two': 2, 'three': 3, 'four': 4, 'five': 5, 'six': 6, 'seven': 7,
         'eight': 8, 'nine': 9, 'ten': 10, 'eleven': 11, 'twelve': 12, 'thirteen': 13,
         'fourteen': 14, 'fifteen': 15, 'sixteen': 16, 'seventeen': 17, 'eighteen': 18,
         'nineteen': 19, 'twenty': 20, 'thirty': 30, 'forty': 40, 'fifty': 50, 'sixty': 60}
COUNT = r'(?:\d+|' + '|'.join(WORDS) + r')'
QUOTED_SCORE = re.compile(
    r'`(verification/[A-Za-z0-9_.-]+\.md)`[^.]{0,120}?'
    r'(?:(\d+) of (\d+) checks|(' + COUNT + r') checks)')

# The score an artifact states for itself, which is the first one in the file.
OWN_SCORE = re.compile(r'(\d+) of (\d+) checks')

# A named artifact, and a claim in the PRESENT TENSE about what it reads now. These are
# photographs and they get retaken; three records went on saying one reads a number it stopped
# reading two captures ago. Only "reads N" is checked: a record saying what a picture *read*
# before is describing history, which is exactly what these records have to be able to do.
QUOTED_PLAY = re.compile(
    r'`(verification/[A-Za-z0-9_.-]+\.md)`[^.]{0,160}?'
    r'reads\s+(\d+)\s+(?:draft\s+)?(?:frames?\s+)?played')

# What a photograph reports today, which is the count in its own quoted capture.
OWN_PLAY = re.compile(r'^>\s*Played\s+(\d+)', re.M)


def as_count(word):
    """The number a claim spells, however it spells it."""
    return int(word) if word.isdigit() else WORDS[word.lower()]

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
            if not own:
                continue
            total = int(own.group(2))
            if m.group(2) is not None:
                claimed = f'{m.group(2)} of {m.group(3)}'
                wrong = (m.group(2), m.group(3)) != (own.group(1), own.group(2))
            else:
                # A bare total, in digits or in words: "41 checks", "eighteen checks".
                # "grew from 46 checks to 60" names both numbers and is not a claim about
                # the count today, so it is history rather than drift.
                if re.match(r'\s+to\s+\d+', text[m.end():m.end() + 20]):
                    continue
                claimed = f'{m.group(4)} checks'
                wrong = as_count(m.group(4)) != total
            if wrong:
                found.append(f'{p}: says {m.group(1)} is {claimed}, '
                             f'the file itself says {own.group(1)} of {own.group(2)}')

        for m in QUOTED_PLAY.finditer(text):
            other = ROOT / m.group(1)
            if not other.exists():
                continue
            own = OWN_PLAY.search(other.read_text(encoding='utf-8', errors='replace'))
            if own and m.group(2) != own.group(1):
                found.append(f'{p}: says {m.group(1)} reads {m.group(2)} frames played, '
                             f'the file itself reports {own.group(1)}')

    for line in found:
        print(line)
    print(f'{len(found)} inconsistencies' if found else 'clean')
    return 1 if found else 0


if __name__ == '__main__':
    sys.exit(main())
