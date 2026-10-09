# B-232: Matte Choker, Refine Hard Matte and Refine Soft Matte

Built on 2026-10-08 under your effects loop request, decided as D-352. This is P0-20, the
"Matte refinement kit": the program now has an edge-aware matte filter (the guided filter, kept
in one place for the keyer to come) and three new effects that clean up a keyed layer's edge.
They are ours, named after After Effects' **Matte Choker**, **Refine Hard Matte** and **Refine
Soft Matte**, and all three are in the **Lines & Mattes** group of the effects list, beside Simple Choker:

- **Matte Choker**: two stages, each with **Geometric Softness** (how far each pixel looks
  round it), **Choke** (positive takes the edge in, negative spreads it) and **Gray Level
  Softness** (how soft the new edge is), run **Iterations** times. As added it takes out small
  specks, fills small holes and rounds the corners.
- **Refine Hard Matte**: **Feather** softens the edge but only along the picture's own colour
  edges; **Contrast** steepens it again; **Shift Edge** moves it out or in.
  **Decontaminate Edge Colors** takes the background's colour out of the part-covered edge, with
  **Decontamination Amount**, **Extend Where Decontaminated** and **View Decontamination Map**.
- **Refine Soft Matte**: the same, plus **Edge Radius**, a band about the edge where the
  covering is fitted to the colours (for hair and soft detail), and **View Edge Region** to see
  that band. Decontaminate is On when added.

Matte Choker runs on the graphics card (a hard step, Gray Level Softness 0, stays on the
processor). The two Refine effects run on the processor, and they are slow (see below). The
check, `verification/D-352_matte_refine_table.md`, 195 of 195, holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-352 pictures/`: `before.png` is the test street with a rough key (a soft,
sky-tinted band round every house, specks in the sky, holes in the walls), and the others are it
after each effect.

## What to check

Key a shot with Color Key, HSV Key or Extract, so the edge is rough, then add the effects below
the key from **Lines & Mattes**.

1. **Matte Choker as added.** Specks in the background go, small holes in the subject fill, the
   edge comes in a little and its corners round (`matte_choker.png` against `before.png`).
2. **Choke both ways.** Set **Choke 1** to -60: the matte spreads and holes close. Set it to 60:
   the edge comes in. Raise **Iterations** to 3: each pass does it again.
3. **Gray Level Softness.** Set it to 0 on stage 1: a hard edge. Set it to 100: a soft one.
4. **Refine Hard Matte as added.** The edge softens along the subject's outline and stays fairly
   hard (`refine_hard_matte.png`). Raise **Feather** to 6 and **Contrast** to 0: a wider, softer
   edge. **Shift Edge** 50 moves it out, -50 in.
5. **Decontaminate.** Turn **Decontaminate Edge Colors** On: the background's tint leaves the
   edge, which looks like the subject's own colour again (`refine_hard_matte_decontaminated.png`).
   Turn **View Decontamination Map** On: grey where colour is changed, brightest at half covering
   (`decontamination_map.png`).
6. **Refine Soft Matte.** Add it to a soft-edged subject (hair, fur, smoke): within **Edge
   Radius** of the edge the covering follows the colours. Turn **View Edge Region** On: white in
   the band, black elsewhere (`refine_soft_edge_region.png`, Edge Radius 4).
7. **Save and open.** Save, close and reopen: the settings and keys should be as you left them. A
   project saved before this build opens exactly as it did.
8. **Same as an export.** Export a frame and put it next to the viewer at Full. They should look
   the same.

## Not done, on purpose or for later

- **The Refine effects are slow**: about two thirds of a second to a second a frame with one on
  each of three 1080p layers (`verification/B-232_matte_refine_timing_table.md`), so scrubbing
  is sluggish; export is fine. A graphics-card version would fix it; not done yet. Matte Choker
  adds about 9 ms a frame per 1080p layer.
- **Dark holes left by a key look see-through to Refine Soft Matte.** It only sees the colours
  the layer still has, and a pixel the key took out entirely has none (black), so dark parts near
  such holes can come out part see-through (`refine_soft_matte.png`, the windows). The usual After
  Effects way, Keylight then Refine Soft Matte on hair, waits for our own keyer.
- **Reduce Chatter** and **Extend Where Smoothed** (they read nearby frames), **Use Motion
  Blur** and its settings, **Calculate Edge Details** and **Invert** are not there.
- The radii count whole pixels, so a keyed Feather or Edge Radius steps rather than glides.
- Adobe does not publish how its three work, so how each is worked out is ours; the results will
  be close to After Effects' but not identical. Where After Effects' defaults are not documented,
  ours are used.

## If something is wrong

Say which step. The most likely fault would be in step 5 (the decontaminated edge turning a wrong
colour) or step 1 (Matte Choker eating thin parts of the subject that should stay).
