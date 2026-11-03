# B-885 playtest: Layer Style Gradient Overlay and its gradient editor (D-1005)

What changed: a layer can now have After Effects' Gradient Overlay layer style, from Layer >
Layer style: Gradient Overlay (or the command palette, or the Effects browser's Layer Styles
group). It has After Effects' settings and defaults: Blend Mode (Normal), Opacity 100, the
gradient (white at 0 to black at 100, both opacity stops 100), Angle 90, Style (Linear,
Radial, Angle, Reflected, Diamond), Reverse (Off), Scale 100 and Offset 0, 0. The gradient
paints only where the layer already is; it never changes the layer's transparency. It is drawn
after the layer's effects and before Inner Shadow and Drop Shadow, as in After Effects.
Not built (left in D-1005): Gradient Smoothness, Align with Layer off, Dither and the small
midpoint diamonds between stops.

## Steps

1. Open the program and make a composition with a photo in it.
2. Select the photo and pick Layer > Layer style: Gradient Overlay. The photo turns into a
   white-to-black ramp, white at the bottom, black at the top.
3. In the Effect Controls, under "Gradient", there is a bar over a checkerboard with small
   squares above it (opacity stops) and below it (colour stops).
4. Click the colour stop at the left end; pick #232444 (a dark navy) in the Colour box under
   the bar. Do the same for the right-hand colour stop.
5. Click the opacity stop at the left end and set its Opacity to 0, then drag it right until
   its Location reads about 65.6.
6. Set Style to Radial and Scale to 150 (the tutorial's settings, PVUPTcfJ5HY at 4:00).
7. Click just under the bar in the middle: a new colour stop appears there, in the colour the
   bar already had. Click it and press Delete (or the Delete stop button): it goes again.
   With only two stops left, Delete stop is greyed out.
8. Set Blend Mode to Overlay.
9. Edit, Undo a few times: each stop edit comes back one at a time.
10. Save, close, open again: the stops are as you left them.

## What to see

- After step 6: the photo is untouched in the middle and goes dark navy towards its edges and
  corners, an oval vignette, as in the tutorial's frame.
- The bar always shows the gradient as it is: transparent (checkerboard) on the left, navy on
  the right after step 5.
- The stop lists are also typed just above the bar ("Colour Stops: 0 #232444, 100 #232444",
  "Opacity Stops: 65.6 0, 100 100"); typing there does the same as dragging.
- After step 8 the vignette is much softer, a darkening rather than a navy paint.

## Pictures

`verification/B-885 pictures/gradient_overlay_sheet.png`: left, the tutorial before the style
(its own frame, the phone mock-up on the left is part of that frame); middle, After Effects with
the overlay (the tutorial's frame at about 4:00, Normal); right, ours: the left picture's photo
area rendered by this build with the same settings.

`verification/B-885 pictures/gradient_editor_panel.png`: the panel in a private copy of the app
after a scripted check. The left opacity stop was dragged from 65.6 to 30, a colour stop was
added under the middle of the bar and then removed with Delete stop, and the left colour stop is
chosen. With only two colour stops left, Delete stop is greyed out. Each of these steps was read
back from the project and all five checks passed.

## Fixtures

`verification/B-885_gradient_overlay_table.md`: every row "yes" (FX-GOVER-001 to 025, saving
and opening, the refused and taken commands, and the tile checks).
