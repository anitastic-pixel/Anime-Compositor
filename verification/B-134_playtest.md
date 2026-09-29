# B-134: Light Sweep, by hand

Built on 2026-09-29 against D-199, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' CC Light Sweep does, by a rule of our own: it
lays a band of light across the drawing along a line, brightest on the line and fading to its
sides, lights the drawing's outer edges more than its middle, and lights only what is drawn.
Key the band's centre and a shine runs across a sword, a badge or a logo. The layer does not
grow. It sits in the **Generate** group after Rain, and its settings carry After Effects' own
names.

The generated halves are `verification/B-134_light_sweep_table.md`, 106 of 106 checks passing,
which renders every FX-SWEEP case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Light Sweep card
sends every setting the command reads. This sheet covers what the tables cannot: how it looks
and feels in the window.

## The pictures

`verification/B-134 pictures/`, a badge (a blue plate with a dark border, a red bar across and a
gold disc in the middle, nothing round it), three times enlarged, over a dark blue where nothing
is drawn:

- `before.png`: no effect.
- `start.png`: as it is added: a soft white band leaning left through the middle, the border
  brightest where the band crosses it.
- `sweep_1.png`, `sweep_2.png` and `sweep_3.png`: a sharp band, Center at 30, 50 then 50, 50
  then 70, 50: the shine as it runs across, one step at a time.
- `edges.png`: Width 10000, Sweep Intensity 0, Edge Thickness 3: only the border lit, three
  pixels deep, the inside untouched.
- `composite.png`: Light Reception Composite in orange: the band paints the badge orange, never
  brighter than the orange.
- `cutout.png`: Light Reception Cutout: the badge gone, only the light left in its shape.

## Before you start

Make the composition 160 by 100 and import `plate.png` from `verification/B-134 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the plate, pick **Light Sweep** in **Add effect…**, under **Generate**,
   after Rain; typing "light sweep", "shine", "glint" or "sheen" in the search finds it too. The
   card shows **Center** 50, 50, **Direction** -30, **Width** 50, **Sweep Intensity** 50, **Edge
   Intensity** 100, **Edge Thickness** 1, **Shape** Smooth, **Light Color** white and **Light
   Reception** Add, and above them a box standing for the drawing with a mark at the centre and
   a pale line through it for the band. The picture is as `start.png`.
2. **Dragging.** Drag the mark in the box to the left: the line and the shine in the picture
   follow. Turn the Direction dial: the line and the band turn together.
3. **The sweep.** Set Shape to Sharp, Width 24, Sweep Intensity 70, and Center to 30, 50, then
   50, 50, then 70, 50: as `sweep_1.png`, `sweep_2.png` and `sweep_3.png`.
4. **Keyed.** Key Center from -20, 50 at the first frame to 120, 50 a second later and play: the
   shine runs across the badge from left to right and is gone before and after.
5. **Only what is drawn.** Wherever the band is, the empty space round the badge stays empty.
6. **The edges.** Set Width to 10000 and Sweep Intensity to 0: as `edges.png`, only the border
   lit. Edge Thickness 10 lights it deeper; Edge Intensity 0 turns it off.
7. **The shapes.** Back to Width 50 and Sweep Intensity 50: Linear fades evenly, Smooth softly,
   Sharp is a hard bar.
8. **Colour and reception.** Set Light Color to orange (#ffb040), Width 80, Sweep Intensity 90
   and Light Reception Composite: as `composite.png`. Set Light Color back to white and Light
   Reception to Cutout: as `cutout.png`.
9. **Out of range.** Type 101 in Sweep Intensity, -1 in Width or 51 in Edge Thickness: it is
   refused with a sentence saying what it runs from and to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller and the band crosses it in the same place,
    as wide for the smaller picture.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- An edge is only where the drawing meets empty space. A line drawn inside the drawing, the gold
  disc's rim, is not an edge.
- Edge Thickness counts whole pixels: 2.7 is 2.
- There is no handle for the band in the viewer yet: move it by the card's box or its numbers.
- The box on the card shows the band's middle line, not its width, the box not knowing the
  drawing's pixels.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' CC Light Sweep and is not claimed to match it.
- It costs little: the reference shot's frames 100 and 101 at full size take 51 to 52 ms
  without it, 68 to 75 ms with Light Sweep 400 pixels wide on all four layers, and 118 to 124 ms
  with Edge Thickness 50 on all four.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
