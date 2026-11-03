# B-817 playtest: Add Grain's Viewing Mode

What changed (D-937): Add Grain (and the Add Grain inside Match Grain) has After Effects'
**Viewing Mode**: Final Output or Preview. **Final Output** puts the grain over the whole layer,
as before. **Preview** shows the grain only inside a 200 by 200 pixel box in the middle of the
layer, framed by a one-pixel white line, so you can judge the grain on one spot; outside the box
the layer is untouched. A layer smaller than 200 pixels is boxed round its own edge.
An Add Grain added from the Effect menu starts on **Final Output** (After Effects starts on
Preview; we start on Final Output so nothing renders with a white box by accident). Saved
projects look exactly as before.

1. Import a picture (at least 400 by 400). Add Effect > Noise & Grain > Add Grain.
   - Expect: Viewing Mode **Final Output**, the first switch under the numbers; grain all over.
2. Set Viewing Mode to **Preview**.
   - Expect: a white square 200 by 200 pixels in the middle of the layer; grain only inside it;
     outside it the picture is clean.
3. Set Intensity to 0.
   - Expect: the white square is still drawn; no grain anywhere.
4. Put Add Grain in Preview on a small layer (under 200 pixels across).
   - Expect: the white line runs round the small layer's own edge.
5. Add Match Grain and open its settings.
   - Expect: it has Viewing Mode too, and Preview boxes its grain the same way.
6. Turn the viewer to Half.
   - Expect: the box looks bigger, since it is 200 pixels of the smaller preview picture.
7. Save, close and reopen; also open a project saved before this change that has Add Grain.
   - Expect: your setting reads as you left it; the old project shows Final Output, unchanged.
8. Export a frame with Preview on.
   - Expect: the white box is in the export too (as in After Effects); set Final Output before
     exporting.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b817_grain_view` (23 of 23, `B-817_grain_view_table.md`): every
pixel of FX-ADDGRAIN-042 to 047 against `tools/addgrain_view_reference.py`, the wrong word
refused, round trips, old files saved without the word, and tiles. The old cases still pass:
`b323_addgrain` (264), `b334_match_grain` (152) and `b107_gpu_fx` (unchanged; Preview stays on
the processor); the app's contract tests 10 of 10; the page check 5 of 5
(`B-817_grain_view_page_check.md`).

Tutorial sheet: `verification/B-817 pictures/grain_view_vs_tutorial.png`, rendered by the release
app (processor, full size) on a 560 by 332 still from the tutorial (e2_ReXZHsUE, 7:49) with its
settings (Size 0.5, Shadows 0.25, Highlights 0.2, Animation Speed 0.2). The tutorial's AE panel
shows Viewing Mode, and the author sets it to Final Output. The small white rectangle near the eyes
is part of the tutorial's picture itself, so it shows in our "before" too. Row 2: before, Final
Output (grain all over), Preview (the 200 by 200 white box, grain inside only). Row 3, at
Intensity 4 and zoomed three times: Final Output grains everywhere; Preview's white corner with
grain only inside it.
