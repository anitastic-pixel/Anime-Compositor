# B-552: Hair

Built on 2026-10-13 as D-673 under your /loop request: **Hair**, in **Generate** beside Drizzle,
after what CycoreFX's CC Hair is for. It grows hair or fur out of the layer. Each strand is rooted
where the picture is opaque, stands out towards you, and hangs down under its own weight. It takes
its colour from the hair colour and the picture under its root, and it is lit the way hair is lit:
brightest where it runs across the light.

Nothing moves by itself. A frame is just the settings as they are at that frame, so keyed settings
animate the coat and an unkeyed coat stays still.

The settings (the value when added in brackets):

- **Length** (30): how long the strands are. At 100 a strand is about a tenth of the layer's
  height.
- **Thickness** (2): how thick a strand is at its root, in pixels on a picture 1080 high. Each
  strand narrows to a quarter of that at its tip.
- **Weight** (50): how far a strand's tip hangs down, as a share of its length. Below 0 the
  strands curl up.
- **Constant Mass** (off): when on, longer strands are heavier and hang further.
- **Density** (100): how many strands.
- **Hair Color** (brown), **Color Inheritance** (50): how much a strand takes the picture's colour
  at its root instead of the hair colour. **Opacity** (100).
- **Light and Shading**: Light Intensity, Light Direction, Ambient, Diffuse, Specular, Roughness,
  Metal.

CycoreFX publishes no formula and no values when added, so the rule and those values are ours. They
are written down in document 21 and in `tools/hair_reference.py`.

The check is `verification/D-673_hair_table.md`, and 189 of 189 checks pass. It compares
every pixel with numbers that a separate program worked out before the code existed. It also checks
that the graphics card's picture stays within 1 level of the processor's (50 of 50
comparisons). The pictures are in `verification/D-673 pictures/` and use the street picture from
D-506.

## What to check

1. **As added.** `1_as_added.png`: thin short pale strands scattered over the whole street, like
   fine scratches. The street is still clear.
2. **Long fur.** `2_long_fur.png`: long strands growing from everywhere and arching over like tall
   grass, pale grey over the sky and brown over the road. The street shows through them.
3. **Curled up.** `3_curled_up.png`: Weight below 0. The strands climb straight up in tangles
   instead of arching over.
4. **Blue hair.** `4_blue_hair.png`: the hair colour blue and Color Inheritance 0. Every strand is
   blue, whatever is under it, some lighter and some darker where the light catches them.
5. **Shiny.** `5_shiny.png`: fewer, thicker long strands. Look for bright white streaks along the
   strands where the shine catches them.
6. **Constant Mass.** `6_constant_mass.png`: long heavy strands with Constant Mass on. They stand
   tall and bend only near their tips.
7. **In the app.** Put Hair (Generate) on a picture. Turn Length and Density up for a fur coat and
   Weight up and down to see it bend over and curl up.
8. **Clear areas.** Put Hair on a layer with a clear part. No hair should grow from the clear part.
9. **Keyed weight.** Key Weight from -100 to 200 over a few seconds and play. The strands should
   swing smoothly from curled up to bent over, with no jump.
10. **Out of range.** Type 2000 in Length. It should be refused with a sentence that gives the
    range.
11. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added;
- the units of Length, Thickness and Weight;
- the shape of a strand and the way it is lit;
- that the manual's Hairfall Map (a second layer steering the hair) is not offered;
- that the strands are seen straight on, not through the composition camera.

## Speed

`verification/B-552_hair_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 53.7 / 54.1, as added 290.7 / 518.2; Hair, a long fur: length 200, thickness 6, density 60 3100.4 / 4496.6; Hair, weight keyed -100 to 200, constant mass 336.5 / 523.0; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
