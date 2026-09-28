# B-117a: starter presets (D-181)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build: "6-10 built-in starter presets, marked built-in, cannot be deleted, can be copied".

## What you will see

Open the Effects panel's **Presets** folder on a fresh window and nine presets are already there, before any of your own. Each has a small **built in** mark, and hovering one shows what it does and where it works best.

- Drag one onto a layer, or double-click it, exactly as with your own presets.
- A built-in preset has no ✕ and no *Remove*. Pressing Delete on one says it is built in and cannot be removed.
- **Copy to my presets…** (its right-click menu, or the ⧉ beside it) asks for a name, suggesting "Night copy", and the copy is an ordinary preset of yours: change it, rename it, remove it.
- Their names are kept for them. Saving a preset of your own as "Night" is refused with a message; an imported "Night" comes in as "Night 2". If this window already had a preset of yours called, say, "Sunset", it becomes "Sunset 2" the first time, and the status line says so. Nothing of yours is lost.
- **Export presets…** writes your own presets only. **Export this preset…** on a built-in one writes that one, so you can hand it on.

## The nine

| Preset | What the panel says | Its effects, in order | Pictured on |
| --- | --- | --- | --- |
| Soft bloom | A gentle glow off the brightest parts of the picture. Best on an adjustment layer. | bloom | an adjustment layer |
| Night | Day painted as night: cooler, darker, the corners fall away. Best on an adjustment layer. | color balance, vibrance, brightness contrast, vignette | an adjustment layer |
| Sunset | Warm evening light falling from the top of the frame. Best on an adjustment layer. | color balance, gradient | an adjustment layer |
| Cel shadow | A hard, flat shadow down and to the right, in a deep violet rather than black. For a character layer. | drop shadow | a character layer |
| Rim light | A warm light catching the upper right edge. For a character layer. | rim light | a character layer |
| Old film | Sepia, grain, dark corners and a slight flicker. Best on an adjustment layer. | gradient map, noise, vignette, exposure flicker | an adjustment layer |
| Impact | The frame of a hit: glints on the brights, colour fringes and a shake. Best on an adjustment layer, for a few frames. | cross glare, chromatic aberration, camera shake | an adjustment layer |
| Dream haze | A soft, bright haze with the colour lifted. Best on an adjustment layer. | diffusion, vibrance | an adjustment layer |
| Speed lines | Focus lines rushing in to the middle of the frame. Best on an adjustment layer. | speed lines | an adjustment layer |

`looks.png` in this folder shows each one on frame 100 of the reference shot. The character-layer ones are on the layer with the yellow and blue shapes; the others are on an adjustment layer above everything. Every setting is one you can change after applying; the presets are starting points.

## Where they are written

`Fixtures/starter_presets/starter.fxpreset`, an ordinary preset file of the kind B-116 exports, written by `tools/starter_presets_reference.py`. The program carries that file inside itself, so the presets are the same on every machine and need nothing installed. Changing a starter preset later is a decision made in that tool, not in the code.

## How you will check it

The build's test reads the starter presets as an import would, checks the nine names, their effects and sentences against the fixture, applies each one to the reference shot and requires a changed picture with no warnings. It writes `verification/B-117_starter_presets_table.md` and a picture of each in `verification/B-117 pictures/`, for you to judge the looks. A playtest sheet walks you through applying, copying and trying to remove one.
