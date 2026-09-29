# B-118: Color Lookup, by hand

Built on 2026-09-28 against the B-118 entry in document 15 (D-182). **Awaiting the owner's playtest.**

`verification/B-118_color_lookup_table.md` is the comparison, 84 of 84. It checks FX-LUT-001 to 013 pixel by pixel, the reason given for each of the seventeen refused files word for word, Collect Files and Check Package, and undo. `verification/B-118 pictures/` has frame 100 of the reference shot through three of the fixture's files, `warm_17.png`, `cool_3.png` and `tint_1d.png`, beside `before.png`, for you to judge.

The files used below are in `Fixtures/cube_lut/luts/`. Any .cube file from a grading program (DaVinci Resolve, Photoshop's Export Color Lookup, a LUT pack) works the same way.

## Before you start

- Use the release build. It opens on the reference shot.
- Save the project somewhere of your own first (File > Save As…), so the lookup file can be stored relative to it.
- Copy the folder `Fixtures/cube_lut/luts/` somewhere of your own and choose the files from the copy, so that step 8 renames a copy and not the fixture's own file.

## What to check

1. **It is there.** In the Effects panel, **Color Correction** lists **Color Lookup** last. Typing `lut` or `cube` in the search box finds it.
2. **Added, it changes nothing yet.** Make a new adjustment layer above the others and add **Color Lookup** to it. The card has one row, **Lookup File**, saying *No .cube file chosen, so nothing changes.* The picture is unchanged, and nothing is written on the status line.
3. **Choose a file.** Press **Choose .cube file…** and pick `warm_17.cube`. The status line says *Color Lookup now uses warm_17.cube.*, the card shows `warm_17`, and the picture turns warm: cream clouds, a duller blue sky (compare `B-118 pictures/warm_17.png`). The Project panel gains `warm_17`, marked **colour lookup**.
4. **One undo.** Press Ctrl+Z once. The picture is back as it was, and `warm_17` is gone from the Project panel: bringing the file in and using it are one step.
5. **Another file.** Redo (Ctrl+Shift+Z), then choose `cool_3.cube` on the same card, then `tint_1d.cube`. Each is a different look (compare the pictures). Choosing `warm_17.cube` again uses the asset already there rather than adding a second.
6. **A file it cannot read.** Choose `refused/count.cube` from your copy. Nothing changes; the status line says it was not used because *the table has 7 lines where LUT_3D_SIZE 2 needs 8*.
7. **On a character only.** Delete the adjustment layer, select `layer3` (the yellow and blue shapes), add Color Lookup and choose `tint_1d.cube`. Only that layer changes.
8. **A missing file is kept and said.** Save and close. Rename your copy of `tint_1d.cube` and open the project again. It opens; the warning panel says the files for "tint_1d" are not where the project expects them, the layer draws without its look, and the card still names `tint_1d`. Select `tint_1d` in the Project panel and press **Relink…** at its foot: the dialog asks for a .cube file. Pick the renamed file and the look is back.
9. **A layer cannot show the file.** Select `warm_17` in the Project panel and press Ctrl+Alt+L, which makes a layer of a drawing. It is refused: the status line says a layer cannot show a colour lookup file, and to add Color Lookup to a layer and choose the file there.
10. **Collect Files.** File > Collect Files… into an empty folder. The folder has the .cube file under `media/`, and the collected project opens with the same picture. Check Package on it says every file is whole.

## Known limits

- A preset or a pasted effect carrying a Color Lookup is refused in a project that does not have its lookup file, because the setting names the file by its asset in the project it came from. Add the effect and choose the file instead.
- Dropping a .cube file on the window does not bring it in; use the card's button.
- It is drawn by the processor, not the graphics card.

## What to report

- Any .cube file of your own that is refused, with the sentence shown, or that looks different from the program that made it.
- Any step whose message you could not understand.
