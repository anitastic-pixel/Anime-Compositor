# B-118a: Color Lookup, a .cube file on any layer (D-182)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build: ".cube colour lookup: 3D trilinear, 1D if cheap. Collect/package copies the file; a missing .cube is diagnosed and kept."

## What you will see

A new effect, **Color Lookup**, under Color Correction in the Effects panel. Its card has one row: the lookup file's name, or *No .cube file chosen, so nothing changes.*, and a button **Choose .cube file…**.

- Choose a .cube file (DaVinci Resolve, Photoshop and most grading programs export them, and many free looks are sold as them) and the layer takes on the look at once. 3D files and 1D files both work.
- The file becomes part of the project, listed in the Project panel as a *colour lookup*, as a drawing is. Choosing the same file on another layer uses the same entry.
- Undo takes it back in two steps: the effect's setting, then the file's entry in the project.
- **Collect Files** copies the .cube file into the collected folder, beside the drawings.
- If the file is moved or deleted, the project still opens: the layer is drawn without the look, the warning says which file is missing, and **Relink** in the Project panel takes a new .cube file. Nothing is dropped.
- A file that is not a proper .cube file is refused when you choose it, with the reason (for example "line 4: a table line must be three numbers"). If a file that was fine is later damaged, the layer is drawn without the look and the warning gives the reason.

`looks.png` in this folder shows the six test files on frame 100 of the reference shot: before, then a warm look, a cool look, a 1D tint, a contrast made by a file's domain, and a file that doubles each colour by its input range.

## Known limits

- The lookup works on ordinary screen colour (sRGB). A creative look made for pictures suits it; a camera's *log* lookup, made to turn a flat camera recording into normal colour, does not, as our drawings are not log.
- A file that holds both a 1D table and a 3D table, which DaVinci Resolve can write, is refused with a reason.
- A lookup file stays in the project once chosen until the project is closed, as no kind of file can be removed from the Project panel yet. It costs nothing when no effect uses it.
- The effect runs on the processor. The graphics card learns it in item 9, with the other new effects.

## How a file is read

Strictly, so a damaged file is never half-used. These are the files the test must refuse, each with its reason word for word:

| File | Reason |
| --- | --- |
| `count.cube` | the table has 7 lines where LUT_3D_SIZE 2 needs 8 |
| `both.cube` | the file gives both LUT_1D_SIZE and LUT_3D_SIZE, a 1D table before a 3D one, which this program does not read |
| `no_size.cube` | the file gives no LUT_3D_SIZE or LUT_1D_SIZE |
| `empty.cube` | the file gives no LUT_3D_SIZE or LUT_1D_SIZE |
| `word.cube` | line 4: a table line must be three numbers |
| `four.cube` | line 2: a table line must be three numbers |
| `nan.cube` | line 2: a table line must be three numbers |
| `late.cube` | line 3: DOMAIN_MIN comes after the table has begun |
| `unknown.cube` | line 2: LUT_4D_SIZE is not a keyword of a .cube file this program reads |
| `twice.cube` | line 2: LUT_3D_SIZE is given twice |
| `big.cube` | line 1: LUT_3D_SIZE must be one whole number from 2 to 256 |
| `half.cube` | line 1: LUT_3D_SIZE must be one whole number from 2 to 256 |
| `domain_order.cube` | the domain's top must be above its bottom in each colour |
| `domain_short.cube` | line 2: DOMAIN_MAX must be three numbers |
| `domain_twice.cube` | the file gives its domain more than one way |
| `range_short.cube` | line 2: LUT_1D_INPUT_RANGE must be two numbers |
| `not_text.cube` | the file is not text |

## How you will check it

The build's test draws the thirteen fixture cases and compares every pixel with the numbers `tools/cube_lut_reference.py` worked out, checks each refused file's reason, and collects a project to see the file copied. It writes `verification/B-118_color_lookup_table.md`. A playtest sheet walks you through choosing a file, undoing, collecting, and moving the file away.
