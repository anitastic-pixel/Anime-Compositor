# B-116: presets in a file, by hand

Built on 2026-09-28 against the B-116 entry in document 15 (D-180). **Awaiting the owner's playtest.**

`verification/B-116_preset_file_table.md` is the comparison: the nineteen preset files of `Fixtures/preset_file/`, what each should do and what the build did, 34 of 34. Every good file is also exported and imported again and comes back the same; every broken one is refused by Export as well as by Import, so the program never writes a file it would refuse.

## Before you start

- Use the release build. It opens on the reference shot.
- Have at least two presets of your own in the Effects panel's **Presets** folder. If you have none, select a layer, add a Glow, and choose **Save the effects as a preset** in the command palette (Ctrl+Shift+P); do it again with a second effect under another name.

## What to check

1. **Export them all.** In the Effects panel, right-click an empty part of the list and choose **Export presets…**. A Save dialog opens, suggesting `presets.fxpreset`. Save it on the desktop. The status line says how many presets were exported, and where.
2. **Export one.** Right-click one preset and choose **Export this preset…**. The dialog suggests the preset's own name. Save it. The status line says 1 preset.
3. **Import into the same window.** Right-click the list and choose **Import presets…**, and pick the file from step 1. Every preset comes in again with " 2" after its name, because you already have those names, and the status line lists each one it renamed. Nothing you had is changed.
4. **Import on a fresh window.** Remove the presets (the ✕ beside each, pressed twice), then import the file from step 1. They come back with their own names, and applying one to a layer gives the same effects and settings as before.
5. **Someone else's file.** Import `Fixtures/preset_file/fx_pre_002.fxpreset` from the project folder. Three presets arrive: Night, Poster and Lifted, in that order.
6. **A broken file.** Import `Fixtures/preset_file/fx_pre_018.fxpreset`. It is refused: the status line says the effect `core.lens_sparkle` is not one this build has, and **not one** preset is added, even though the first two in that file are fine.
7. **The wrong kind of file.** Make a copy of a project file, rename the copy to end `.fxpreset`, and import it. The status line says it is a project file and to use Open instead. Nothing is added.
8. **Cancel.** Start an Export and an Import and press Cancel in each dialog. Nothing happens and nothing is written.
9. **Nothing to export.** With no presets at all, choose **Export presets** from the command palette. No dialog opens; the status line says there are no presets to export yet.
10. **The project is untouched.** None of this marks the project as changed or adds a step to undo: presets belong to the window, not the project.

## What to report

- Any preset that comes back different from how it went out: a setting, a key, an effect switched off that came back on.
- Any file that was partly imported.
- Any message you could not understand.
