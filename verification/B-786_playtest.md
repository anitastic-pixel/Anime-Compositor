# B-786 playtest: export on the graphics card, a trial left switched off (D-907)

You said "Measure first". So the card export is built but switched off, and it is not in the export window. Nothing you can click turns it on. This sheet checks two things: that exports are exactly what they were, and that the measured numbers are there for you to decide on.

## 1. Exports are what they were (2 minutes)

1. Open the app and open the reference shot (or any project of yours).
2. Export a few frames as a PNG sequence, as you always do.
3. Read the export's report when it finishes.

**Pass:** the report says nothing about the graphics card, and the frames look as they always have. (Behind this, every export the B-160 check makes was hashed file by file with its report, 167,631 hashes, and compared with the build before this change: all the same, the MP4 once the date Windows stamps into it is set aside.)

**Fail:** the report mentions "Exported on the graphics card", or a frame looks different.

## 2. Read the measured numbers (3 minutes)

Open `verification/B-786_card_export_table.md`.

- The "Faster than the processor by" column is how much quicker the whole export was on the card. Above 1.00x means the card was quicker.
- "Pixels within 1 level of the processor's" is the share of pixels that are the same as the processor's, or one step of 255 away (the most the viewer is allowed to differ, D-165).
- "Largest difference" is the worst single pixel, in steps of 255.
- "Frames the same bytes" is how many card frames match the processor's file exactly. Expect few or none: the card adds numbers in a different order, so even an invisible difference changes the file.

What was measured, in short: on the light reference shot (4 layers) the card was about the same (1.07x, 11.0 s against 11.8 s for 240 frames). On a heavier 25-layer project it was 3.36x quicker (4.0 s against 13.5 s for 48 frames). These are the quiet re-measure of 2026-10-27, nothing else building; the first run, on a busy machine, gave 0.94x and 2.46x. Every pixel on both was within 1 level of the processor's, and no frame was the same bytes.

**Your decision (D-907):** offer the card export as a choice in the export window, labelled "not the processor's exact pixels", or leave it out. The processor stays the default either way.

## 3. If you say yes

A later step would add a "Draw on the graphics card (faster, not exact)" tick to the export window. Each export made with it would end with a line naming which frames the card drew, which frames the processor drew and why, and which effects the processor ran first.
