# B-557: Foam

Built on 2026-10-13 as D-678 under your /loop request: **Foam**, in **Generate** beside Polygon
Shards, after what After Effects' Foam is for. Bubbles are born in a producer. They grow, drift on
the wind, wander in turbulence, push each other apart, stick together, wobble and pop. They are
drawn as outlines (the Draft view) or as textured bubbles (Rendered), and the layer becomes the
foam alone.

Nothing is remembered from frame to frame. Every frame steps the bubbles from the layer's start,
so any frame is the same picture whether it is drawn alone, scrubbed to or rendered out of order.
This means a long shot gets slower to draw towards its end.

The settings (the value when added in brackets):

- **View** (Draft): Draft shows the producer as a red ring and each bubble as a blue ring;
  Rendered shows the bubbles themselves.
- **Producer**: Producer Point (the middle), Producer X Size and Y Size (0.05), Producer
  Orientation (0), Zoom Producer Point (off), Production Rate (1, thirty bubbles a second).
- **Bubbles**: Size (0.5), Size Variance (0.5), Lifespan (300), Bubble Growth Speed (0.1),
  Strength (10, how long bubbles last before popping).
- **Physics**: Initial Speed (0), Initial Direction (0), Wind Speed (0.5), Wind Direction (90),
  Turbulence (0.5), Wobble Amount (0.05), Repulsion (1), Pop Velocity (0), Viscosity (0.1),
  Stickiness (0.75).
- **Zoom** (1), **Universe Size** (1, how far bubbles can go before they are dropped).
- **Rendering**: Blend Mode (Transparent), Bubble Texture (Default Bubble; also Spit, Bubblegum,
  Dishwater, Layer), Bubble Orientation (Fixed), Reflection Strength (0), Reflection Convergence
  (0).
- **Simulation Quality** (Normal), **Random Seed** (1).

Adobe publishes no formula, so the rule and the values are ours. Adobe's bubble pictures are not
copied: each texture is drawn by our own rule. They are written down in document 21 and in
`tools/foam_reference.py`.

The check is `verification/D-678_foam_table.md`, and 326 of 326 checks pass. It compares
every pixel with numbers that a separate program worked out before the code existed. It also
checks that the graphics card's picture stays within 1 level of the processor's (79 of 79
comparisons). The pictures are in `verification/D-678 pictures/` and use the street picture from
D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png` (frame 24): the Draft view on white. A small red oval
   (the producer) in the middle, with a little cluster of thin blue rings of mixed sizes just to its
   right.
2. **Rendered close look.** `2_rendered_close_look.png` (frame 48): a wide cloud of pale, nearly
   white bubbles of mixed sizes filling the middle of the picture, each softly shaded with a faint
   highlight. They are faint on the white background; that is expected.
3. **Crowded bubblegum.** `3_crowded_bubblegum.png`: the whole picture packed with big pink
   bubbles, each with a bright white highlight at its upper left, overlapping and crowding each
   other, smaller ones toward the middle.
4. **The street in every bubble.** `4_the_street_in_every_bubble.png`: big round bubbles filling
   the picture, each showing the whole street (sky, buildings, road) inside it, with a pale
   reflection laid over it.
5. **Windy and wandering.** `5_windy_and_wandering.png`: the pale bubble cloud has been blown off
   to the upper right; it trails in a loose, uneven band and the lower left of the picture is empty.
6. **Spit, newest on top.** `6_spit_new_on_top.png`: a large cloud of whitish, solid (not see-through)
   bubbles of mixed sizes, faint on the white background, overlapping each other.
7. **In the app.** Put Foam (Generate) on the street picture and play. Rings should stream out of
   the producer. Switch View to Rendered to see the bubbles.
8. **Scrub.** Jump to frame 40, then play from 0 to 40. Frame 40 should be the same picture both
   times.
9. **Keyed production rate.** Key Production Rate from 0 to 2. Bubbles should start appearing and
   thicken smoothly.
10. **Out of range.** Type 200 in Production Rate. It should be refused with a sentence that gives
    the range.
11. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added and the units;
- the look of the five Bubble Textures (ours, not Adobe's pictures);
- that the Layer texture and the reflection read the effect's own layer, not another layer;
- that the Flow Map group is not offered;
- that a long shot gets slower to draw towards its end.

## Speed

`verification/B-557_foam_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 34.1 / 70.8, as added 102.8 / 169.1; Foam, Rendered, bigger bubbles, the Layer texture with a reflection 66.2 / 193.3; Foam, Rendered, production rate keyed 0 to 2 (the settings read back to the in point) 64.5 / 151.6; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
