# B-886 playtest: Hue/Saturation's Channel Range handles (D-1006)

What it is: in After Effects you can make one of Hue/Saturation's colour ranges, such as Reds,
wider or narrower by dragging four handles under the colour bars. Now you can do the same here.

## Steps

1. Open any project with a colourful picture and select its layer.
2. Effect > Hue/Saturation.
3. In Effect Controls, set Reds' S (saturation) to -100. The reds, and some of the oranges and
   pinks, turn grey.
4. Under the two colour bars, set **Channel Range** to **Reds**. A white band and two grey ends
   appear under the bars, with four small triangles: the two white ones are where the range is
   full, and the two grey ones are where it fades out to nothing.
5. Drag the right-hand grey triangle further right, towards the yellow.
6. Drag the two white triangles close together.
7. Type `50, 55, 65, 70` into the **Yellows Range** box, press Enter, and set Yellows' S to -100.

## What you should see

- After step 4 the triangles sit either side of the red at both ends of the bar, because red sits
  at 0 and 360 on the colour wheel.
- After step 5 more of the oranges and yellows go grey in the viewer, and the lower bar turns grey
  further to the right. The Reds Range box shows the new numbers.
- After step 6 less of the picture goes grey, only the hues between the white triangles fully.
- After step 7 only pure yellows go grey. The oranges and yellow-greens stay coloured.
- Channel Range stays on the range you chose while you work.

## Pictures

`verification/B-886 pictures/channel_range_panel.png`: the panel in a private copy of the app,
after a scripted check. FX-HUESAT-038 was open, with Reds at 275, 320, 35, 85 and saturation -100.
Channel Control was set to Reds, and the right-hand fade-out handle was dragged from 85 to 120.
The triangles sit at 35 and 120 on the left and at 275 and 320 on the right. Grey shows in the
lower bar wherever the reds take the saturation away. All four checks passed: the handles opened
as written, the drag wrote 120 into the project, the Reds Range box showed it, and Channel Control
stayed on Reds.

## Fixtures

`verification/B-886_hue_saturation_range_table.md`: every row "yes". It covers:

- FX-HUESAT-037 to 047;
- saving and opening;
- a refused file shape;
- five refused and three taken commands;
- the tile checks.

FX-HUESAT-001 to 036 (`verification/B-56_hue_saturation_table.md`,
`verification/B-656_hue_saturation_colorize_table.md`) still pass.
