# B-148: Bevel Alpha and Bevel Edges, by hand

Built on 2026-09-29 against D-213, which you accepted with the rest of the After Effects picks
("take everything"). B11 asked for the bevel style After Effects has; its own two, **Bevel
Alpha** and **Bevel Edges**, are both here as two effects, and After Effects' CC Glass is left
out. They make a flat drawing look raised, as a light from one side catches its edges:

- **Bevel Alpha** bevels the drawing's own outline, whatever its shape: a badge, a logo, a
  title.
- **Bevel Edges** bevels the four sides of the layer's rectangle, as a raised tile or button.

The side toward the light goes toward the light's colour, the side away goes darker, and what
the drawing covers never changes. The layer does not grow. Both sit in the **Stylize** group
after Emboss.

The generated halves are `verification/B-148_bevel_table.md`, 134 of 134 checks passing, which
renders every FX-BEVEL case against the numbers written before the code and draws the pictures
below, and `verification/B-12b_state_fields_table.md`, which checks each card sends every
setting the command reads. This sheet covers what the tables cannot: how they look in the
window.

## The pictures

`verification/B-148 pictures/`, three times enlarged, a grey and white check where nothing is:

- `badge.png`: a made-up badge at its own size, 160 by 100: a red disc with a yellow diamond in
  it, on nothing.
- `badge_start.png`: Bevel Alpha as it starts: the disc's rim lit at the upper left and shaded
  at the lower right, the middle and the diamond untouched.
- `badge_off.png`: Light Intensity 0: the badge as it was.
- `badge_wide.png`: Edge Thickness 8 at intensity 0.8: a wider, stronger, rounder bevel.
- `badge_low_light.png`: Light Angle 120, the light from the lower right: the lit and shaded
  sides swap.
- `badge_blue.png`: a blue light, #2040a0, at intensity 1: the lit rim goes toward blue.
- `panel.png`: a made-up panel filling the whole layer, blue with a pale stripe across it.
- `panel_start.png`: Bevel Edges as it starts, a tenth of the panel's height: a frame 10 pixels
  wide, the left and top sides lit, the right and bottom shaded, its corners cut at 45 degrees.
- `panel_below.png`: Light Angle 180, the light from below: the bottom lit, the top shaded, and
  the left and right sides, square to the light, unchanged.
- `panel_gold.png`: a quarter of the panel's height with a gold light, #ffd070, at 0.9: a deep
  picture-frame.
- `panel_pyramid.png`: half the panel's height: every pixel is on a side, a four-sided pyramid.

## Before you start

Make the composition 160 by 100 and import `badge.png` and `panel.png` from
`verification/B-148 pictures/`, each as its own layer. Press **Full resolution**.

## What to check

1. **Adding them.** On the badge, pick **Bevel Alpha** in **Add effect...**, under **Stylize**,
   after Emboss; typing "bevel", "raised" or "button" in the search finds both. The card shows
   Edge Thickness 2, Light Angle -60 on a dial, Light Color white and Light Intensity 0.4, and
   the badge looks as `badge_start.png`.
2. **Thickness.** Edge Thickness 8 and Light Intensity 0.8: as `badge_wide.png`. Back to 2 and
   0.4.
3. **The light.** Turn the Light Angle dial to 120: as `badge_low_light.png`. Turn it round
   slowly: the lit side follows the light round the rim. Back to -60.
4. **Colour.** Light Color #2040a0 and Light Intensity 1: as `badge_blue.png`. Back to white
   and 0.4.
5. **Off.** Light Intensity 0: the badge as it was. Back to 0.4.
6. **Bevel Edges.** On the panel, add **Bevel Edges**. The card shows Edge Thickness 0.1, the
   same light, and the panel looks as `panel_start.png`.
7. **Its light.** Light Angle 180: as `panel_below.png`. Back to -60.
8. **Its thickness.** Edge Thickness 0.25, Light Color #ffd070 and Light Intensity 0.9: as
   `panel_gold.png`. Edge Thickness 0.5 and the colour back to white: as `panel_pyramid.png`.
9. **Keys.** On the badge, key Light Angle from -60 at the first frame to 300 at frame 24 and
   play: the light goes once round the badge.
10. **Moved.** Move and scale either layer: its bevel moves and scales with it.
11. **Out of range.** Type 201 in Bevel Alpha's Edge Thickness, 0.6 in Bevel Edges', or 1.5 in
    either's Light Intensity: it is refused with a sentence saying what it runs to, and the card
    keeps its old number.
12. **Draft.** Press **Draft**: the picture is smaller and both bevels look the same.
13. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- Bevel Edges bevels the layer's whole rectangle, so on a drawing with empty space round it the
  bevel is on that empty edge, where nothing shows; use Bevel Alpha there, as in After Effects.
- Bevel Alpha works from what the drawing covers only, so a line or colour change inside the
  drawing, the diamond in the badge, is not bevelled.
- The light's colour cannot be keyed, as Drop Shadow's cannot.
- They run on the processor; a graphics card version is their own later unit.
- They are modelled on After Effects' Bevel Alpha and Bevel Edges and are not claimed to match
  them.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 49 to 53 ms without it; Bevel Alpha as it starts, 104 to 114 ms; Bevel Alpha at Edge
  Thickness 20, 109 to 124 ms; Bevel Edges as it starts, 53 to 58 ms; Bevel Edges at 0.5, 54 to
  60 ms. **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
