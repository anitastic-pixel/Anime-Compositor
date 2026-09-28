# B-117: starter presets, by hand

Built on 2026-09-28 against the B-117 entry in document 15 (D-181). **Awaiting the owner's playtest.**

`verification/B-117_starter_presets_table.md` is the comparison, 49 of 49: the nine presets the program carries are the fixture's, in its order, with its effects and sentences, and each one draws on the reference shot with no warning and a changed picture. `verification/B-117 pictures/` has each one on frame 100 (`before.png` has none), for you to judge the looks.

## Before you start

- Use the release build. It opens on the reference shot.
- Open the Effects panel.

## What to check

1. **They are there.** The **Presets** folder lists nine presets first: Soft bloom, Night, Sunset, Cel shadow, Rim light, Old film, Impact, Dream haze and Speed lines, each with a small **built in** mark, then any of your own.
2. **They say what they do.** Hover over **Night**. The tip says it is built in, what it does, where it works best, and its effects.
3. **Apply one.** Make a new adjustment layer above the others and double-click **Old film**. The picture turns sepia with grain, dark corners and a flicker as you play. Undo takes it off in one step.
4. **On a character.** Select `layer3` (the yellow and blue shapes) and drag **Cel shadow** onto it. A hard violet shadow falls down and to the right of each shape.
5. **They cannot be removed.** Select **Night** in the list and press Delete. Nothing is removed; the status line says it is built in and to copy it instead. Its right-click menu has no *Remove*, and there is no ✕ beside it.
6. **Copy one.** Press the ⧉ beside **Sunset** (or right-click it, **Copy to my presets…**). The box suggests "Sunset copy"; press Enter. "Sunset copy" appears under Presets without the mark, with a ✕, and can be removed like any preset of yours.
7. **Their names are kept.** Select a layer with effects and choose **Save the effects as a preset** in the command palette (Ctrl+Shift+P). Type `Night` and press Enter. It is refused: the status line says it is a built-in preset's name. Try again as `My night`: it is saved.
8. **Export.** Right-click the list, **Export presets…**. The file holds your own presets only (open it in Notepad: no Soft bloom). Right-click **Impact**, **Export this preset…**: that file holds Impact alone.
9. **Import.** Import the Impact file from step 8. It comes in as "Impact 2", because the built-in one has the name.
10. **The same on a fresh window.** Close the program and open it again. The nine are still there, first, whatever you removed or copied.

## What to report

- Any preset whose look you would change: which, and how (too strong, too faint, wrong colour).
- Any preset that could be removed or renamed over.
- Any message you could not understand.
