# B-867 playtest: Fast Zoom Blur's streaks fade as After Effects' do (D-987)

What changed: Fast Zoom Blur (our CC Radial Fast Blur) draws shorter, fainter streaks. Before, a streak ran almost evenly all the way back to the edge of the frame. Now it fades out quickly, as in After Effects. At the default amount, 50, the streaks are short; at 100 they run evenly all the way, as before.

Start with the picture `verification/B-867 pictures/fast_zoom_tutorial_before_after.png`. It shows the neon title from the RIS86giOG-Q tutorial (3:29 and 3:35). Each row has three pictures: After Effects on the left, ours before this change in the middle, and ours now on the right.

- In both rows the middle picture has long, bright grey fans running off the left edge. After Effects has none of that.
- In the right picture the streaks fade out about a third of the way to the edge, closer to After Effects.
- The right picture's streaks are still a little stronger than After Effects' at 3:35. See "Not right yet".
- The "before" pictures are from before the Glow fix (B-866) too, so the glow itself also looks different. Only the grey streaks belong to this change.

## Steps

1. New composition, 1920 by 1080. Add a black solid.
2. Add a text layer that says GLOW, white, large and thin, a little left of centre.
3. Add Effect > Blur & Sharpen > Fast Zoom Blur to the text. Set Amount to 78 and Center to the right end of the word.
   You should see soft grey streaks reaching left from the letters and fading out before they are half as long again as the word. You should not see streaks running to the frame's edge.
4. Set Amount to 50. The streaks become very short, just a soft edge on the letters.
5. Set Amount to 100. The streaks run evenly all the way back, as before this change.
6. Set Blur Type to Brightest, then Darkest, at amount 78. Brightest only ever lightens and Darkest only ever darkens; both fade as quickly as Standard.
7. Ctrl+Z steps back through each change.

## Not right yet

- Cycore does not publish how its streaks fade. The fade is matched to two tutorial stills at amount 78; at other amounts it is our best guess (medium-low confidence).
- At 3:35 After Effects' streaks are a little fainter than ours. In the tutorial the centre sits on the title, while in our test project it sits at the word's right end, so part of the difference is the set-up.

## Checks behind it

- FX-FASTZOOM-001 to 014 (worked again by `tools/fast_zoom_blur_reference.py`, D-987), test b241: pass.
- Card and processor agree within one level (b46, b718, verification/D-362_fast_zoom_blur_table.md).
