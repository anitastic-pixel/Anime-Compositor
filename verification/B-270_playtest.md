# B-270: Slant

Built on 2026-10-09 under your effects loop request, decided as D-391. **Slant** (in
**Distort**) is our version of CycoreFX's CC Slant: it leans the layer over like italic type and
squashes or stretches it toward a floor line, and can fill it with one colour, for a quick cast
shadow. The formulas are ours; CycoreFX publishes none.

The settings, with the values they start at:

- **Slant** (0): degrees, -80 to 80; plus leans the top to the right, minus to the left.
- **Stretching** (Off): Off tips the picture over as it leans, so it gets shorter, its leaning
  sides keeping their length; On keeps its full height.
- **Height** (100): per cent of the height kept, squashing (below 100) or stretching (above)
  toward the floor.
- **Floor** (50, 100, the bottom edge): per cent of the layer; only its height counts. The floor
  line stays put while everything above and below it leans and squashes.
- **Set Color** (Off) and **Color** (black): On fills the layer with the colour, keeping its
  shape and soft edges.

Nothing grows: what leans past the layer's edge is cut off.

The check, `verification/D-391_slant_table.md` (164 of 164), holds every pixel to numbers worked
out by a separate program before the code existed, and the graphics card's picture to within 1
level of the processor's. The pictures are in `verification/D-391 pictures/`. They are saved
over white, so parts left see-through show as white.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: upright at full height, so the street is unchanged.
2. **Leaning, tipped.** `3_lean.png`, Slant 30: the street leans right from its bottom edge and
   is a little shorter, the top left corner white.
3. **Leaning, stretched.** `4_lean_stretched.png`, Slant 30 with Stretching on: the same lean at
   full height.
4. **Half height.** `5_half_height.png`, Height 50 with the floor across the middle: the street
   squashed to a band about the middle, white above and below.
5. **Set to red.** `6_red.png`, Slant -20, Stretching on, Set Color on in red: the whole layer,
   which has no see-through parts, becomes solid red, leaning left, the top right corner white.
6. **In the app.** Add Slant to a character layer, turn Set Color on in black, set Slant to 45,
   Height to 40: a shadow lying down from the character's feet. Lower the layer's Opacity for a
   soft cast shadow.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Slant costs too little to read against the timing noise, at most about 2.5 ms a
1080p layer, and about 16.8 ms on the processor (`verification/B-270_distort_timing_table.md`;
provisional, the card figures were within their own noise).

## Not built

- Tipping by the cosine of the angle when Stretching is off, and the level floor line, are our
  own reading of CycoreFX's one-sentence descriptions; no tutorial with numbers was matched.
- There is no floor handle on the viewer yet; set the floor in Effect Controls.
