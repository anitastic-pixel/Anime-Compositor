# B-342: Mr. Smoothie

Built on 2026-10-11 as D-462, under your /loop request and your go-ahead: **Mr. Smoothie**, in
**Stylize**, after After Effects' CC Mr. Smoothie. The layer is repainted in flowing bands of
colour: a gradient of the colours found between two points of the layer, laid over the light
and dark shapes of a layer (its own, or another you pick). The layer keeps its shape; only its
colours change.

- **Flow Layer** (the layer itself): whose shapes the bands follow. **Flow Layer Placement**
  (Stretch) fits another layer of a different size: stretched, centred or tiled.
- **Property** (Luminance): which part of the flow layer makes the shapes: Red, Green, Blue,
  Alpha, Luminance, Lightness, Hue or Saturation.
- **Smoothness** (5, 0 to 100 pixels): softens the shapes, so the bands are wider and smoother.
- **Sample A** (25%, 50%) and **Sample B** (75%, 50%): the two points of the layer whose
  colours, and every colour on the line between them, make the gradient.
- **Phase** (0 degrees): turns the gradient round, so the bands take other colours; 360 comes
  back to where it started. Key it to make the colours cycle.
- **Color Loop** (ABA): the order of the gradient: AB from A to B, BA back, ABA and BAB there and
  back, which never jumps from one end to the other.

The controls are After Effects' own; how it draws is our own rule beyond what the manual says,
and the starting values apart from Property are ours (After Effects' are not published), so it
is not matched to After Effects pixel for pixel. After Effects' Blend w. Original is done by
the effect's own mix.

The check, `verification/D-462_mr_smoothie_table.md` (172 of 172), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-462 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street repainted in the colours found across its middle,
   from a quarter to three quarters of the way across, in bands following its light and dark
   shapes; the outline of the street is unchanged.
2. **Hue, BAB.** `3_hue_bab.png` (Property Hue, Smoothness 10, Color Loop BAB): different bands,
   following the colours' hues instead of their brightness.
3. **Phase.** `4_phase_180.png` (Phase 180): the same bands as `2_as_added.png` in other colours.
4. **One point.** `5_one_point.png` (Sample A and B both at the middle): one flat colour everywhere,
   the colour at the middle of the street.
5. **Smoother.** `6_smoothness_40.png` (Smoothness 40): wider, softer bands than `2_as_added.png`.
6. **In the app.** Add Mr. Smoothie (Stylize) to a picture, drag Sample A and Sample B, switch
   Property and Color Loop, pick another layer as the Flow Layer, key Phase from 0 to 360 over
   two seconds and play it.
7. **Out of range.** Type 101 in Smoothness: it is refused with a sentence giving the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-342_mr_smoothie_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Mr. Smoothie on three layers, played again, 26.5 ms a frame on the card as
added against 17.7 for the Noise alone; on the processor 109.0 against
72.5: a busy machine cannot say whether the card is quicker than the processor for this effect.
